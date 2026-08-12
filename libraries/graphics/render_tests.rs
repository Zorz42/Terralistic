//! Golden-image tests for the graphics library.
//!
//! Every case draws into the offscreen framebuffer that the renderer already uses, reads it
//! back with `GraphicsContext::capture_frame`, and compares it against a committed
//! `Surface` in `goldens/`. That makes the pixels the renderer produces a checked-in
//! specification rather than something only a human eye ever verifies.
//!
//! # Why these are not `#[test]`s
//!
//! They need a real OpenGL context. On macOS the Cocoa video driver refuses to initialise
//! off the main thread, and libtest always runs a test body on a spawned worker - even
//! under `--test-threads=1`. SDL's `offscreen` driver would sidestep that, but it needs
//! EGL, which macOS does not have. So the suite gets its own main-thread entry point:
//!
//! ```text
//! cargo run --features render-tests -- rendertest             # check against goldens
//! cargo run --features render-tests -- rendertest regenerate  # rewrite the goldens
//! cargo run --features render-tests -- rendertest dump        # also write viewable PPMs
//! ```
//!
//! `cargo test` is deliberately untouched, and still needs no graphics context at all.
//!
//! # Determinism
//!
//! A golden is only useful if the same code always produces the same pixels. Three things
//! in this toolkit fight that, and each has a `#[cfg(feature = "render-tests")]` hook to
//! pin it: animations driven by `Instant::elapsed` (`AnimationTimer::freeze`,
//! `Button::settle_hover`, `Toggle::settle_animation`, `TextInput::settle_animation`), the
//! blur and scale fades on the context itself (`GraphicsContext::settle_animations`), and
//! hover states that read the real mouse position - which the settle hooks also neutralise,
//! because they stop the animation advancing towards whatever target hover reports.
//!
//! # What these cover that the draw-list tests cannot
//!
//! Drawing records a `DrawCommand` and the backend replays it, so `cargo test` can assert on
//! what a primitive *asks* for without a window - see the draw list tests in `tests.rs`.
//! These cases are the other half: they are the only check that the backend turns those
//! commands into the right pixels, and the only coverage of anything that has to own a GPU
//! object before it can draw at all (`RectArray`, `TextureAtlas`, `ShadowContext`, fonts).
//!
//! They also happen to be a hard test of deferred resource deletion. `fixture_texture()`
//! returns a temporary, so in a line like
//! `fixture_texture().render(graphics, ..)` the texture is dropped at the end of the
//! statement - long before the frame is executed in `capture_frame`. These cases only match
//! their goldens because `gpu_garbage` holds the OpenGL name until the frame has run.
//!
//! Running a new case five times before committing its golden is what caught the atlas bug:
//! `TextureAtlas::new` packed in `HashMap` iteration order, which Rust randomises per
//! process, and `Texture::render` inflated the source rectangle by `size + 0.1`, so the last
//! column of a region rounded into whichever region happened to be packed next to it. Both
//! are fixed - the atlas packs in key order and the source rectangle is mapped exactly - and
//! `texture_atlas_multiple_regions` is the case that would catch a regression.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{BaseUiElement, DrawTarget, UiContext};

/// Small on purpose: the goldens are committed, and a diff a human has to look at is much
/// easier to read at this size than at window size.
const WINDOW_WIDTH: u32 = 320;
const WINDOW_HEIGHT: u32 = 240;

/// How far a capture may drift from its golden before the case fails.
#[derive(Clone, Copy)]
struct Tolerance {
    /// Largest allowed difference on any single channel of any single pixel.
    max_channel_delta: u8,
    /// Largest allowed fraction of pixels that may differ at all.
    max_differing_fraction: f32,
}

impl Tolerance {
    /// Flat colour and `NEAREST`-sampled geometry, which has no interpolation anywhere in
    /// it, should come back bit for bit identical.
    const EXACT: Self = Self {
        max_channel_delta: 0,
        max_differing_fraction: 0.0,
    };

    /// Only for the gaussian blur shader, whose float error differs between drivers and
    /// between GPU and software rasterisers.
    ///
    /// Use this sparingly. The shadow looks like it belongs here and does not:
    /// `ShadowContext` bakes its gaussian into a CPU `Surface` once and then draws it as an
    /// ordinary `NEAREST` texture, so it is exactly reproducible. Everything except
    /// `render_rect_blur` holds at `EXACT`, and a loose tolerance hides real changes - this
    /// constant once absorbed a genuine one pixel shift in the text input cases.
    const BLURRY: Self = Self {
        max_channel_delta: 4,
        max_differing_fraction: 0.02,
    };
}

struct Case {
    name: &'static str,
    tolerance: Tolerance,
    draw: fn(&mut gfx::GraphicsContext),
}

// --- fixtures -------------------------------------------------------------------------

/// An 8x8 image with four differently coloured quadrants, a translucent one among them, and
/// a single white pixel in the top left corner.
///
/// The asymmetry is the point: a flipped, rotated or offset blit is obvious in the diff
/// rather than silently matching.
fn fixture_surface() -> gfx::Surface {
    let mut surface = gfx::Surface::new(gfx::IntSize(8, 8));
    for (pos, pixel) in surface.iter_mut() {
        *pixel = match (pos.0 < 4, pos.1 < 4) {
            (true, true) => gfx::Color::new(220, 40, 40, 255),
            (false, true) => gfx::Color::new(40, 220, 40, 255),
            (true, false) => gfx::Color::new(40, 40, 220, 255),
            (false, false) => gfx::Color::new(240, 240, 40, 128),
        };
    }
    if let Ok(pixel) = surface.get_pixel_mut(gfx::IntPos(0, 0)) {
        *pixel = gfx::Color::new(255, 255, 255, 255);
    }
    surface
}

fn fixture_texture() -> gfx::Texture {
    gfx::Texture::load_from_surface(&fixture_surface())
}

fn solid_surface(size: u32, color: gfx::Color) -> gfx::Surface {
    let mut surface = gfx::Surface::new(gfx::IntSize(size, size));
    for (_, pixel) in surface.iter_mut() {
        *pixel = color;
    }
    surface
}

/// An opaque background, so alpha blending has something to blend against.
fn background(graphics: &gfx::GraphicsContext) {
    gfx::Rect::new(gfx::FloatPos(0.0, 0.0), graphics.get_window_size()).render(graphics, gfx::Color::new(30, 30, 60, 255));
}

/// Vertical stripes, so a blur has high frequency detail to smear.
fn striped_background(graphics: &gfx::GraphicsContext) {
    background(graphics);
    let mut x = 0.0;
    while x < graphics.get_window_size().0 {
        gfx::Rect::new(gfx::FloatPos(x, 0.0), gfx::FloatSize(8.0, graphics.get_window_size().1)).render(graphics, gfx::Color::new(220, 220, 90, 255));
        x += 16.0;
    }
}

fn parent_of(graphics: &gfx::GraphicsContext) -> gfx::Container {
    gfx::Container::default(graphics)
}

// --- cases: rect ----------------------------------------------------------------------

fn case_rect_solid(graphics: &mut gfx::GraphicsContext) {
    gfx::Rect::new(gfx::FloatPos(20.0, 30.0), gfx::FloatSize(160.0, 90.0)).render(graphics, gfx::Color::new(200, 60, 60, 255));
}

fn case_rect_outline(graphics: &mut gfx::GraphicsContext) {
    gfx::Rect::new(gfx::FloatPos(20.0, 30.0), gfx::FloatSize(160.0, 90.0)).render_outline(graphics, gfx::Color::new(90, 220, 120, 255));
}

fn case_rect_alpha_blend(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    gfx::Rect::new(gfx::FloatPos(30.0, 30.0), gfx::FloatSize(140.0, 100.0)).render(graphics, gfx::Color::new(255, 0, 0, 128));
    gfx::Rect::new(gfx::FloatPos(90.0, 80.0), gfx::FloatSize(140.0, 100.0)).render(graphics, gfx::Color::new(0, 255, 0, 128));
}

/// A rect entirely off screen takes the early return in `Rect::render`, so the frame stays
/// exactly as the clear left it.
fn case_rect_offscreen_culled(graphics: &mut gfx::GraphicsContext) {
    gfx::Rect::new(gfx::FloatPos(-400.0, -400.0), gfx::FloatSize(50.0, 50.0)).render(graphics, gfx::Color::new(255, 255, 255, 255));
    gfx::Rect::new(gfx::FloatPos(600.0, 600.0), gfx::FloatSize(50.0, 50.0)).render(graphics, gfx::Color::new(255, 255, 255, 255));
}

fn case_rect_partially_offscreen(graphics: &mut gfx::GraphicsContext) {
    gfx::Rect::new(gfx::FloatPos(-30.0, -40.0), gfx::FloatSize(120.0, 120.0)).render(graphics, gfx::Color::new(230, 180, 40, 255));
    gfx::Rect::new(gfx::FloatPos(260.0, 190.0), gfx::FloatSize(120.0, 120.0)).render(graphics, gfx::Color::new(40, 180, 230, 255));
}

fn case_rect_zero_alpha_is_skipped(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    gfx::Rect::new(gfx::FloatPos(40.0, 40.0), gfx::FloatSize(100.0, 100.0)).render(graphics, gfx::Color::new(255, 255, 255, 0));
}

// --- cases: rect array ----------------------------------------------------------------

fn case_rect_array_gradient(graphics: &mut gfx::GraphicsContext) {
    let mut array = gfx::RectArray::new();
    let empty = gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
    for i in 0..4 {
        let x = 20.0 + i as f32 * 70.0;
        array.add_rect(
            &gfx::Rect::new(gfx::FloatPos(x, 40.0), gfx::FloatSize(50.0, 150.0)),
            &[
                gfx::Color::new(255, 0, 0, 255),
                gfx::Color::new(0, 255, 0, 255),
                gfx::Color::new(0, 0, 255, 255),
                gfx::Color::new(255, 255, 0, 255),
            ],
            &empty,
        );
    }
    array.update();
    array.render(graphics, None, gfx::FloatPos(0.0, 0.0));
}

fn case_rect_array_textured(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let texture = fixture_texture();
    let mut array = gfx::RectArray::new();
    let white = gfx::Color::new(255, 255, 255, 255);
    let tex_rect = gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(8.0, 8.0));
    for i in 0..3 {
        array.add_rect(
            &gfx::Rect::new(gfx::FloatPos(20.0 + i as f32 * 90.0, 60.0), gfx::FloatSize(64.0, 64.0)),
            &[white, white, white, white],
            &tex_rect,
        );
    }
    array.update();
    array.render(graphics, Some(&texture), gfx::FloatPos(0.0, 0.0));
}

/// `RectArray::render` offsets everything by its `pos` argument.
fn case_rect_array_translated(graphics: &mut gfx::GraphicsContext) {
    let mut array = gfx::RectArray::new();
    let color = gfx::Color::new(200, 120, 240, 255);
    let empty = gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
    array.add_rect(&gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(80.0, 60.0)), &[color, color, color, color], &empty);
    array.update();
    array.render(graphics, None, gfx::FloatPos(120.0, 90.0));
}

// --- cases: texture -------------------------------------------------------------------

fn case_texture_unscaled(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    fixture_texture().render(graphics, 1.0, gfx::FloatPos(40.0, 40.0), None, false, None);
}

fn case_texture_scaled(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    fixture_texture().render(graphics, 12.0, gfx::FloatPos(40.0, 40.0), None, false, None);
}

fn case_texture_flipped(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let texture = fixture_texture();
    texture.render(graphics, 10.0, gfx::FloatPos(20.0, 60.0), None, false, None);
    texture.render(graphics, 10.0, gfx::FloatPos(160.0, 60.0), None, true, None);
}

fn case_texture_src_rect(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    // just the bottom right quadrant of the fixture, which is the translucent yellow one
    let src = gfx::Rect::new(gfx::FloatPos(4.0, 4.0), gfx::FloatSize(4.0, 4.0));
    fixture_texture().render(graphics, 20.0, gfx::FloatPos(60.0, 60.0), Some(src), false, None);
}

fn case_texture_tinted(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let texture = fixture_texture();
    texture.render(graphics, 10.0, gfx::FloatPos(20.0, 60.0), None, false, Some(gfx::Color::new(255, 128, 128, 255)));
    texture.render(graphics, 10.0, gfx::FloatPos(160.0, 60.0), None, false, Some(gfx::Color::new(128, 128, 255, 128)));
}

/// A zero sized source rectangle takes the early return and draws nothing.
fn case_texture_empty_src_rect(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let src = gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
    fixture_texture().render(graphics, 10.0, gfx::FloatPos(60.0, 60.0), Some(src), false, None);
}

// --- cases: blend modes ---------------------------------------------------------------

fn case_blend_mode_multiply(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    gfx::Rect::new(gfx::FloatPos(20.0, 20.0), gfx::FloatSize(160.0, 100.0)).render(graphics, gfx::Color::new(255, 200, 100, 255));

    graphics.set_blend_mode(gfx::BlendMode::Multiply);
    gfx::Rect::new(gfx::FloatPos(80.0, 70.0), gfx::FloatSize(160.0, 100.0)).render(graphics, gfx::Color::new(120, 255, 200, 255));
    graphics.set_blend_mode(gfx::BlendMode::Alpha);
}

// --- cases: text ----------------------------------------------------------------------

fn case_text_basic(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    graphics.font.render_text(graphics, "Terralistic", gfx::FloatPos(20.0, 40.0), 1.0);
}

fn case_text_scaled(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    graphics.font.render_text(graphics, "Scale 2", gfx::FloatPos(20.0, 40.0), 2.0);
    graphics.font.render_text(graphics, "Scale 3", gfx::FloatPos(20.0, 100.0), 3.0);
}

fn case_text_mono(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    if let Some(font_mono) = &graphics.font_mono {
        font_mono.render_text(graphics, "iiii MMMM", gfx::FloatPos(20.0, 40.0), 2.0);
    }
    graphics.font.render_text(graphics, "iiii MMMM", gfx::FloatPos(20.0, 100.0), 2.0);
}

/// The CPU side of text: `create_text_surface` rasterises into a `Surface`, which is then
/// uploaded and drawn as one texture rather than glyph by glyph.
fn case_text_surface(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let surface = graphics.font.create_text_surface("wrapped text\nsecond line", None);
    let texture = gfx::Texture::load_from_surface(&surface);
    texture.render(graphics, 2.0, gfx::FloatPos(20.0, 40.0), None, false, Some(gfx::Color::new(255, 220, 120, 255)));
}

fn case_text_width_limit(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let surface = graphics.font.create_text_surface("a long line that gets wrapped", Some(80));
    let texture = gfx::Texture::load_from_surface(&surface);
    texture.render(graphics, 2.0, gfx::FloatPos(20.0, 20.0), None, false, None);
}

// --- cases: containers and orientation --------------------------------------------------

/// All nine orientations at once, each flush against the edge it names.
fn case_container_orientations(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let size = gfx::FloatSize(60.0, 40.0);
    for (orientation, color) in [
        (gfx::TOP_LEFT, gfx::Color::new(255, 0, 0, 255)),
        (gfx::TOP, gfx::Color::new(255, 128, 0, 255)),
        (gfx::TOP_RIGHT, gfx::Color::new(255, 255, 0, 255)),
        (gfx::LEFT, gfx::Color::new(0, 255, 0, 255)),
        (gfx::CENTER, gfx::Color::new(255, 255, 255, 255)),
        (gfx::RIGHT, gfx::Color::new(0, 255, 255, 255)),
        (gfx::BOTTOM_LEFT, gfx::Color::new(0, 0, 255, 255)),
        (gfx::BOTTOM, gfx::Color::new(128, 0, 255, 255)),
        (gfx::BOTTOM_RIGHT, gfx::Color::new(255, 0, 255, 255)),
    ] {
        let container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), size, orientation, Some(&parent));
        container.get_absolute_rect().render(graphics, color);
    }
}

/// A container nested inside another resolves against its parent's absolute rect, not the
/// window.
fn case_container_nested(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let root = parent_of(graphics);
    let outer = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(160.0, 120.0), gfx::CENTER, Some(&root));
    outer.get_absolute_rect().render(graphics, gfx::Color::new(80, 80, 110, 255));

    for (orientation, color) in [
        (gfx::TOP_LEFT, gfx::Color::new(255, 80, 80, 255)),
        (gfx::CENTER, gfx::Color::new(255, 255, 255, 255)),
        (gfx::BOTTOM_RIGHT, gfx::Color::new(80, 160, 255, 255)),
    ] {
        let inner = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(40.0, 30.0), orientation, Some(&outer));
        inner.get_absolute_rect().render(graphics, color);
    }
}

// --- cases: render rect -----------------------------------------------------------------

fn case_render_rect_fill_and_border(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut rect = gfx::RenderRect::new(gfx::FloatPos(30.0, 30.0), gfx::FloatSize(180.0, 120.0));
    rect.fill_color = gfx::Color::new(60, 120, 200, 255);
    rect.border_color = gfx::Color::new(255, 255, 255, 255);
    rect.jump_to_target();
    rect.render(graphics, &parent);
}

fn case_render_rect_translucent(graphics: &mut gfx::GraphicsContext) {
    striped_background(graphics);
    let parent = parent_of(graphics);
    let mut rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(180.0, 120.0));
    rect.orientation = gfx::CENTER;
    rect.fill_color = gfx::Color::new(0, 0, 0, 150);
    rect.jump_to_target();
    rect.render(graphics, &parent);
}

fn case_render_rect_shadow(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(140.0, 100.0));
    rect.orientation = gfx::CENTER;
    rect.fill_color = gfx::Color::new(220, 220, 220, 255);
    rect.shadow_intensity = 255;
    rect.jump_to_target();
    rect.render(graphics, &parent);
}

fn case_render_rect_blur(graphics: &mut gfx::GraphicsContext) {
    striped_background(graphics);
    let parent = parent_of(graphics);
    let mut rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(200.0, 140.0));
    rect.orientation = gfx::CENTER;
    rect.fill_color = gfx::Color::new(0, 0, 0, 60);
    rect.blur_radius = 30;
    rect.jump_to_target();
    rect.render(graphics, &parent);
}

/// `render_pos` is what gets drawn, and it lags `pos` by `smooth_factor`. Rendering without
/// jumping to the target must therefore still draw at the old position.
fn case_render_rect_lags_behind_target(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut rect = gfx::RenderRect::new(gfx::FloatPos(20.0, 20.0), gfx::FloatSize(80.0, 60.0));
    rect.fill_color = gfx::Color::new(240, 160, 40, 255);
    rect.smooth_factor = 10.0;
    // moving the target does not move what is drawn until the animation runs
    rect.pos = gfx::FloatPos(200.0, 150.0);
    rect.render(graphics, &parent);
}

// --- cases: sprite and atlas --------------------------------------------------------------

fn case_sprite_basic(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut sprite = gfx::Sprite::new();
    sprite.set_texture(fixture_texture());
    sprite.scale = 10.0;
    sprite.pos = gfx::FloatPos(20.0, 20.0);
    sprite.render(graphics, &parent);
}

fn case_sprite_flipped_tinted_centered(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut sprite = gfx::Sprite::new();
    sprite.set_texture(fixture_texture());
    sprite.scale = 12.0;
    sprite.flip = true;
    sprite.orientation = gfx::CENTER;
    sprite.color = gfx::Color::new(120, 255, 180, 255);
    sprite.render(graphics, &parent);
}

/// Atlas construction, `get_rect` and rendering a region back out of the packed texture.
///
/// A single entry, so the region fills the whole atlas: this pins the simple case where
/// there is no neighbour to bleed from, and the square must be a full 64 pixels wide rather
/// than the 63 the old `size + 0.1` mapping produced.
fn case_texture_atlas_single_region(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let mut surfaces = HashMap::new();
    surfaces.insert(0_u32, solid_surface(8, gfx::Color::new(230, 60, 60, 255)));
    let atlas = gfx::TextureAtlas::new(&surfaces);

    if let Some(rect) = atlas.get_rect(&0) {
        atlas.get_texture().render(graphics, 8.0, gfx::FloatPos(40.0, 80.0), Some(*rect), false, None);
    }
}

/// Three regions packed into one atlas, each drawn back out by key.
///
/// This is the case that used to be nondeterministic, before `TextureAtlas::new` started
/// packing in key order. Each square must be a flat colour: a column of the neighbouring
/// region's colour along an edge means the source rectangle is being sampled too wide.
fn case_texture_atlas_multiple_regions(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let mut surfaces = HashMap::new();
    surfaces.insert(0_u32, solid_surface(8, gfx::Color::new(230, 60, 60, 255)));
    surfaces.insert(1_u32, solid_surface(8, gfx::Color::new(60, 230, 60, 255)));
    surfaces.insert(2_u32, solid_surface(8, gfx::Color::new(60, 60, 230, 255)));
    let atlas = gfx::TextureAtlas::new(&surfaces);

    for key in 0..3_u32 {
        if let Some(rect) = atlas.get_rect(&key) {
            atlas.get_texture().render(graphics, 8.0, gfx::FloatPos(20.0 + key as f32 * 80.0, 80.0), Some(*rect), false, None);
        }
    }
}

/// An empty atlas leaves an unloaded texture behind, which must not draw anything.
fn case_texture_atlas_empty(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let atlas: gfx::TextureAtlas<u32> = gfx::TextureAtlas::new(&HashMap::new());
    atlas.get_texture().render(graphics, 8.0, gfx::FloatPos(40.0, 80.0), None, false, None);
}

// --- cases: widgets -----------------------------------------------------------------------

fn button_with_label(graphics: &gfx::GraphicsContext, label: &str) -> gfx::Button {
    let mut button = gfx::Button::new(|| {});
    button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(label, None));
    button.scale = 2.0;
    button.orientation = gfx::CENTER;
    button
}

fn case_button_idle(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut button = button_with_label(graphics, "Play");
    button.settle_hover(0.0);
    button.render(graphics, &parent);
}

fn case_button_hovered(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut button = button_with_label(graphics, "Play");
    button.settle_hover(1.0);
    button.render(graphics, &parent);
}

fn case_button_half_hovered(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut button = button_with_label(graphics, "Play");
    button.settle_hover(0.5);
    button.render(graphics, &parent);
}

fn case_button_disabled_darkened(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut button = button_with_label(graphics, "Play");
    button.disabled = true;
    button.darken_on_disabled = true;
    button.settle_hover(0.0);
    button.render(graphics, &parent);
}

fn toggle_at(orientation: gfx::Orientation, toggled: bool, progress: f32) -> gfx::Toggle {
    let mut toggle = gfx::Toggle::new();
    toggle.orientation = orientation;
    toggle.toggled = toggled;
    toggle.settle_animation(progress, 0.0);
    toggle
}

fn case_toggle_off(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    toggle_at(gfx::CENTER, false, 0.0).render(graphics, &parent);
}

fn case_toggle_on(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    toggle_at(gfx::CENTER, true, 1.0).render(graphics, &parent);
}

fn case_toggle_mid_travel(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    toggle_at(gfx::CENTER, true, 0.5).render(graphics, &parent);
}

fn case_text_input_with_text(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut input = gfx::TextInput::new(graphics);
    input.orientation = gfx::CENTER;
    input.set_text("Terralistic".to_owned());
    input.settle_animation();
    input.render(graphics, &parent);
}

/// With no text the hint is what shows, and the text texture is skipped entirely.
fn case_text_input_hint(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut input = gfx::TextInput::new(graphics);
    input.orientation = gfx::CENTER;
    input.set_hint(graphics, "type here");
    input.settle_animation();
    input.render(graphics, &parent);
}

/// Text wider than the box is clipped to its tail, so the end stays visible.
fn case_text_input_overflowing_text(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let parent = parent_of(graphics);
    let mut input = gfx::TextInput::new(graphics);
    input.orientation = gfx::CENTER;
    input.set_text("a very long value that does not fit in the box".to_owned());
    input.settle_animation();
    input.render(graphics, &parent);
}

const CASES: &[Case] = &[
    Case {
        name: "rect_solid",
        tolerance: Tolerance::EXACT,
        draw: case_rect_solid,
    },
    Case {
        name: "rect_outline",
        tolerance: Tolerance::EXACT,
        draw: case_rect_outline,
    },
    Case {
        name: "rect_alpha_blend",
        tolerance: Tolerance::EXACT,
        draw: case_rect_alpha_blend,
    },
    Case {
        name: "rect_offscreen_culled",
        tolerance: Tolerance::EXACT,
        draw: case_rect_offscreen_culled,
    },
    Case {
        name: "rect_partially_offscreen",
        tolerance: Tolerance::EXACT,
        draw: case_rect_partially_offscreen,
    },
    Case {
        name: "rect_zero_alpha_is_skipped",
        tolerance: Tolerance::EXACT,
        draw: case_rect_zero_alpha_is_skipped,
    },
    Case {
        name: "rect_array_gradient",
        tolerance: Tolerance::EXACT,
        draw: case_rect_array_gradient,
    },
    Case {
        name: "rect_array_textured",
        tolerance: Tolerance::EXACT,
        draw: case_rect_array_textured,
    },
    Case {
        name: "rect_array_translated",
        tolerance: Tolerance::EXACT,
        draw: case_rect_array_translated,
    },
    Case {
        name: "texture_unscaled",
        tolerance: Tolerance::EXACT,
        draw: case_texture_unscaled,
    },
    Case {
        name: "texture_scaled",
        tolerance: Tolerance::EXACT,
        draw: case_texture_scaled,
    },
    Case {
        name: "texture_flipped",
        tolerance: Tolerance::EXACT,
        draw: case_texture_flipped,
    },
    Case {
        name: "texture_src_rect",
        tolerance: Tolerance::EXACT,
        draw: case_texture_src_rect,
    },
    Case {
        name: "texture_tinted",
        tolerance: Tolerance::EXACT,
        draw: case_texture_tinted,
    },
    Case {
        name: "texture_empty_src_rect",
        tolerance: Tolerance::EXACT,
        draw: case_texture_empty_src_rect,
    },
    Case {
        name: "blend_mode_multiply",
        tolerance: Tolerance::EXACT,
        draw: case_blend_mode_multiply,
    },
    Case {
        name: "text_basic",
        tolerance: Tolerance::EXACT,
        draw: case_text_basic,
    },
    Case {
        name: "text_scaled",
        tolerance: Tolerance::EXACT,
        draw: case_text_scaled,
    },
    Case {
        name: "text_mono",
        tolerance: Tolerance::EXACT,
        draw: case_text_mono,
    },
    Case {
        name: "text_surface",
        tolerance: Tolerance::EXACT,
        draw: case_text_surface,
    },
    Case {
        name: "text_width_limit",
        tolerance: Tolerance::EXACT,
        draw: case_text_width_limit,
    },
    Case {
        name: "container_orientations",
        tolerance: Tolerance::EXACT,
        draw: case_container_orientations,
    },
    Case {
        name: "container_nested",
        tolerance: Tolerance::EXACT,
        draw: case_container_nested,
    },
    Case {
        name: "render_rect_fill_and_border",
        tolerance: Tolerance::EXACT,
        draw: case_render_rect_fill_and_border,
    },
    Case {
        name: "render_rect_translucent",
        tolerance: Tolerance::EXACT,
        draw: case_render_rect_translucent,
    },
    Case {
        name: "render_rect_shadow",
        tolerance: Tolerance::EXACT,
        draw: case_render_rect_shadow,
    },
    Case {
        name: "render_rect_blur",
        tolerance: Tolerance::BLURRY,
        draw: case_render_rect_blur,
    },
    Case {
        name: "render_rect_lags_behind_target",
        tolerance: Tolerance::EXACT,
        draw: case_render_rect_lags_behind_target,
    },
    Case {
        name: "sprite_basic",
        tolerance: Tolerance::EXACT,
        draw: case_sprite_basic,
    },
    Case {
        name: "sprite_flipped_tinted_centered",
        tolerance: Tolerance::EXACT,
        draw: case_sprite_flipped_tinted_centered,
    },
    Case {
        name: "texture_atlas_single_region",
        tolerance: Tolerance::EXACT,
        draw: case_texture_atlas_single_region,
    },
    Case {
        name: "texture_atlas_multiple_regions",
        tolerance: Tolerance::EXACT,
        draw: case_texture_atlas_multiple_regions,
    },
    Case {
        name: "texture_atlas_empty",
        tolerance: Tolerance::EXACT,
        draw: case_texture_atlas_empty,
    },
    Case {
        name: "button_idle",
        tolerance: Tolerance::EXACT,
        draw: case_button_idle,
    },
    Case {
        name: "button_hovered",
        tolerance: Tolerance::EXACT,
        draw: case_button_hovered,
    },
    Case {
        name: "button_half_hovered",
        tolerance: Tolerance::EXACT,
        draw: case_button_half_hovered,
    },
    Case {
        name: "button_disabled_darkened",
        tolerance: Tolerance::EXACT,
        draw: case_button_disabled_darkened,
    },
    Case {
        name: "toggle_off",
        tolerance: Tolerance::EXACT,
        draw: case_toggle_off,
    },
    Case {
        name: "toggle_on",
        tolerance: Tolerance::EXACT,
        draw: case_toggle_on,
    },
    Case {
        name: "toggle_mid_travel",
        tolerance: Tolerance::EXACT,
        draw: case_toggle_mid_travel,
    },
    Case {
        name: "text_input_with_text",
        tolerance: Tolerance::EXACT,
        draw: case_text_input_with_text,
    },
    Case {
        name: "text_input_hint",
        tolerance: Tolerance::EXACT,
        draw: case_text_input_hint,
    },
    Case {
        name: "text_input_overflowing_text",
        tolerance: Tolerance::EXACT,
        draw: case_text_input_overflowing_text,
    },
];

// --- comparison -------------------------------------------------------------------------

struct Diff {
    differing_pixels: u32,
    total_pixels: u32,
    max_channel_delta: u8,
    first_difference: Option<(gfx::IntPos, gfx::Color, gfx::Color)>,
}

impl Diff {
    fn fraction(&self) -> f32 {
        if self.total_pixels == 0 {
            0.0
        } else {
            self.differing_pixels as f32 / self.total_pixels as f32
        }
    }

    /// A case passes if no pixel drifted further than the allowed channel delta, or if the
    /// pixels that did drift are a small enough share of the frame.
    fn within(&self, tolerance: Tolerance) -> bool {
        self.max_channel_delta <= tolerance.max_channel_delta || self.fraction() <= tolerance.max_differing_fraction
    }
}

fn channel_delta(a: gfx::Color, b: gfx::Color) -> u8 {
    let deltas = [a.r.abs_diff(b.r), a.g.abs_diff(b.g), a.b.abs_diff(b.b), a.a.abs_diff(b.a)];
    deltas.into_iter().max().unwrap_or(0)
}

fn compare(actual: &gfx::Surface, expected: &gfx::Surface) -> Result<Diff> {
    if actual.get_size() != expected.get_size() {
        bail!("size mismatch: captured {:?}, golden {:?}", actual.get_size(), expected.get_size());
    }

    let mut diff = Diff {
        differing_pixels: 0,
        total_pixels: actual.get_size().0 * actual.get_size().1,
        max_channel_delta: 0,
        first_difference: None,
    };

    for (pos, actual_pixel) in actual.iter() {
        let expected_pixel = expected.get_pixel(pos)?;
        let delta = channel_delta(*actual_pixel, *expected_pixel);
        if delta > 0 {
            diff.differing_pixels += 1;
            diff.max_channel_delta = diff.max_channel_delta.max(delta);
            if diff.first_difference.is_none() {
                diff.first_difference = Some((pos, *actual_pixel, *expected_pixel));
            }
        }
    }

    Ok(diff)
}

// --- inspectable output -------------------------------------------------------------------

/// Composites onto a checkerboard and writes a binary PPM.
///
/// PPM has no alpha and no compression, but it needs no dependency and macOS `sips` and
/// most viewers open it, which is enough for eyeballing a golden or a failure.
fn write_ppm(path: &Path, surface: &gfx::Surface) -> Result<()> {
    let size = surface.get_size();
    let mut out = format!("P6\n{} {}\n255\n", size.0, size.1).into_bytes();

    for y in 0..size.1 {
        for x in 0..size.0 {
            let pixel = surface.get_pixel(gfx::IntPos(x as i32, y as i32))?;
            let checker = if (x / 8 + y / 8) % 2 == 0 { 153.0 } else { 102.0 };
            let alpha = pixel.a as f32 / 255.0;
            for channel in [pixel.r, pixel.g, pixel.b] {
                out.push((channel as f32 * alpha + checker * (1.0 - alpha)) as u8);
            }
        }
    }

    std::fs::write(path, out).with_context(|| format!("writing {}", path.display()))
}

/// Marks every differing pixel magenta over a dimmed copy of the golden.
fn difference_surface(actual: &gfx::Surface, expected: &gfx::Surface) -> gfx::Surface {
    let mut result = gfx::Surface::new(expected.get_size());
    for (pos, pixel) in result.iter_mut() {
        let expected_pixel = expected.get_pixel(pos).copied().unwrap_or_else(|_| gfx::Color::new(0, 0, 0, 255));
        let actual_pixel = actual.get_pixel(pos).copied().unwrap_or_else(|_| gfx::Color::new(0, 0, 0, 255));
        *pixel = if channel_delta(actual_pixel, expected_pixel) > 0 {
            gfx::Color::new(255, 0, 255, 255)
        } else {
            gfx::Color::new(expected_pixel.r / 3, expected_pixel.g / 3, expected_pixel.b / 3, 255)
        };
    }
    result
}

fn goldens_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("libraries/graphics/goldens")
}

fn output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("render_test_output")
}

// --- runner ---------------------------------------------------------------------------------

/// Draws one case into a freshly cleared offscreen frame and reads it back.
fn capture_case(graphics: &mut gfx::GraphicsContext, case: &Case) -> gfx::Surface {
    graphics.begin_capture_frame();
    graphics.settle_animations();
    // Cases are isolated from each other by the draw list itself: `begin_capture_frame`
    // drops anything left recorded, and executing a list resets the blend mode, so a case
    // that switches to multiply cannot leak into the next one.
    (case.draw)(graphics);
    graphics.capture_frame()
}

/// Runs every case. Returns true if they all passed.
///
/// With `regenerate` the goldens are rewritten from what the renderer currently produces
/// instead of being checked, and with `dump` a viewable PPM is written for every case
/// rather than only for failures.
pub fn run(regenerate: bool, dump: bool, font: &[u8], font_mono: &[u8]) -> Result<bool> {
    let goldens = goldens_dir();
    std::fs::create_dir_all(&goldens).with_context(|| format!("creating {}", goldens.display()))?;
    let output = output_dir();
    if dump {
        std::fs::create_dir_all(&output).with_context(|| format!("creating {}", output.display()))?;
    }

    let mut graphics = gfx::GraphicsContext::new_hidden(WINDOW_WIDTH, WINDOW_HEIGHT, font, Some(font_mono))?;

    let mut passed = 0_u32;
    let mut failed = 0_u32;
    let mut regenerated = 0_u32;

    for case in CASES {
        let actual = capture_case(&mut graphics, case);
        let golden_path = goldens.join(format!("{}.opa", case.name));

        if dump {
            write_ppm(&output.join(format!("{}.ppm", case.name)), &actual)?;
        }

        if regenerate {
            std::fs::write(&golden_path, actual.serialize_to_bytes()?).with_context(|| format!("writing {}", golden_path.display()))?;
            regenerated += 1;
            continue;
        }

        if !golden_path.exists() {
            println!("MISSING  {} (no golden at {})", case.name, golden_path.display());
            failed += 1;
            continue;
        }

        let expected = gfx::Surface::deserialize_from_bytes(&std::fs::read(&golden_path).with_context(|| format!("reading {}", golden_path.display()))?)?;

        match compare(&actual, &expected) {
            Ok(diff) if diff.within(case.tolerance) => {
                passed += 1;
            }
            Ok(diff) => {
                failed += 1;
                println!(
                    "FAIL     {}: {} of {} pixels differ ({:.3}%), largest channel delta {} (allowed: delta {} or {:.3}% of pixels)",
                    case.name,
                    diff.differing_pixels,
                    diff.total_pixels,
                    diff.fraction() * 100.0,
                    diff.max_channel_delta,
                    case.tolerance.max_channel_delta,
                    case.tolerance.max_differing_fraction * 100.0,
                );
                if let Some((pos, actual_pixel, expected_pixel)) = diff.first_difference {
                    println!("         first at ({}, {}): got {actual_pixel:?}, want {expected_pixel:?}", pos.0, pos.1);
                }
                std::fs::create_dir_all(&output).with_context(|| format!("creating {}", output.display()))?;
                write_ppm(&output.join(format!("{}.actual.ppm", case.name)), &actual)?;
                write_ppm(&output.join(format!("{}.expected.ppm", case.name)), &expected)?;
                write_ppm(&output.join(format!("{}.diff.ppm", case.name)), &difference_surface(&actual, &expected))?;
                println!("         wrote actual/expected/diff to {}", output.display());
            }
            Err(error) => {
                failed += 1;
                println!("FAIL     {}: {error}", case.name);
            }
        }
    }

    if regenerate {
        println!("\nregenerated {regenerated} goldens in {}", goldens.display());
        println!("look at them before committing: cargo run --features render-tests -- rendertest dump");
        return Ok(true);
    }

    println!("\n{passed} passed, {failed} failed, {} total", CASES.len());
    Ok(failed == 0)
}
