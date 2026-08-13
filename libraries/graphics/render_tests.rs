//! Golden-image tests for the graphics library.
//!
//! Every case draws into the offscreen texture the renderer already uses, reads it back with
//! `GraphicsContext::capture_frame`, and compares it against a committed `Surface` in
//! `goldens/`. That makes the pixels the renderer produces a checked-in specification rather
//! than something only a human eye ever verifies.
//!
//! # Why these are not `#[test]`s
//!
//! They need a real window to hang a GPU surface off. On macOS that has to be the main thread,
//! and libtest always runs a test body on a spawned worker - even under `--test-threads=1`. So
//! the suite gets its own main-thread entry point:
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
//! A golden is only useful if the same code always produces the same pixels. Three things in
//! this toolkit fight that, and each has a `#[cfg(feature = "render-tests")]` hook to pin it:
//! animations driven by `Instant::elapsed` (`AnimationTimer::freeze`, `Button::settle_hover`,
//! `Toggle::settle_animation`, `TextInput::settle_animation`), the blur and scale fades on the
//! context itself (`GraphicsContext::settle_animations`), and hover states that read the real
//! mouse position - which the settle hooks also neutralise, because they stop the animation
//! advancing towards whatever target hover reports.
//!
//! **Run a new case five times before committing its golden.** Iteration order and sampling
//! error are the two things that make a case pass once and fail later.
//!
//! # What these cover that the draw-list tests cannot
//!
//! Drawing records a `DrawCommand` and the backend replays it, so `cargo test` can assert on
//! what a primitive *asks* for without a window - see the draw list tests in `tests.rs`. These
//! cases are the other half: the only check that the backend turns those commands into the
//! right pixels, and the only coverage of anything that has to own a GPU object before it can
//! draw at all (`RectArray`, `TextureAtlas`, `ShadowContext`, fonts).
//!
//! They are also a hard test of deferred resource release. `fixture_texture()` returns a
//! temporary, so `fixture_texture().render(graphics, ..)` drops the texture at the end of the
//! statement, long before the frame is executed in `capture_frame`. These cases only match
//! their goldens because `gpu_device` parks the resource until the frame has run.

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
///
/// The two knobs compose: a pixel is *over tolerance* when a channel moved further than
/// `max_channel_delta`, and the case fails when more than `max_differing_fraction` of the frame
/// is over tolerance. Small drift everywhere is allowed, and so is a handful of pixels that
/// moved a lot - but not a whole region that did.
#[derive(Clone, Copy)]
struct Tolerance {
    /// Largest difference on any single channel that still counts as the same pixel.
    max_channel_delta: u8,
    /// Largest allowed fraction of pixels that may exceed `max_channel_delta`.
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
    /// Use this sparingly - a loose tolerance hides real changes, and this one once absorbed a
    /// genuine one pixel shift in the text input cases. The shadow looks like it belongs here
    /// and does not: `ShadowContext` bakes its gaussian into a CPU `Surface` and draws it as an
    /// ordinary `NEAREST` texture, so it is exactly reproducible. Everything except
    /// `render_rect_blur` holds at `EXACT`.
    const BLURRY: Self = Self {
        max_channel_delta: 4,
        max_differing_fraction: 0.02,
    };
}

struct Case {
    /// The drawing function's own name, `case_` included. Use `name()` for the golden's.
    function: &'static str,
    tolerance: Tolerance,
    draw: fn(&mut gfx::GraphicsContext),
}

impl Case {
    /// What the golden is filed under, and what a failure is reported as.
    fn name(&self) -> &'static str {
        self.function.strip_prefix("case_").unwrap_or(self.function)
    }
}

/// Builds the case table from a list of drawing functions, naming each case after its own
/// function. `case_foo` is the case `foo` at `Tolerance::EXACT`; `case_foo: BLURRY` names a
/// different tolerance.
macro_rules! cases {
    ($($function:ident $(: $tolerance:ident)?),* $(,)?) => {
        [$(Case {
            function: stringify!($function),
            tolerance: cases!(@tolerance $($tolerance)?),
            draw: $function,
        }),*]
    };
    (@tolerance) => { Tolerance::EXACT };
    (@tolerance $tolerance:ident) => { Tolerance::$tolerance };
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

/// The same text drawn four times, a quarter of a pixel further along each time.
///
/// All four rows must come out **identical**, which is the whole point: the backend snaps a
/// texture draw to a whole pixel, so where layout happened to put it between pixels cannot
/// change which texel a destination pixel takes. Without that snap the half pixel row
/// rendered its 3x glyph pixels 2 and 4 wide instead of 3 - uneven and faintly slanted - and
/// at scale 1 it swallowed the one pixel gaps between strokes outright.
///
/// This is what the world list looked like: every row sat on a fractional offset from the
/// scroll position, so every world name was drawn wrong in its own way.
fn case_text_on_fractional_offsets(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("World", None));
    for (step, offset) in [0.0_f32, 0.25, 0.5, 0.75].into_iter().enumerate() {
        texture.render(graphics, 3.0, gfx::FloatPos(20.0 + offset, 5.0 + step as f32 * 56.0), None, false, None);
    }
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

/// A frame whose *first* command is a blur, which is what a menu drawing a blurred panel over
/// nothing else records.
///
/// The blur pass writes the back offscreen texture and reads the front, so this is the case
/// that pins the frame's clear onto the front rather than onto whichever attachment the first
/// pass happens to use. Get that wrong and the blur reads back the previous frame - here, the
/// previous case's image - and everything outside the blurred region keeps it.
///
/// Blurring a cleared frame is exact despite the shader: every tap reads the same transparent
/// black, so there is nothing for a driver's float error to disagree about. The region comes
/// out opaque because the shader's accumulator starts its alpha at 255 rather than 0.
fn case_blur_over_a_cleared_frame(graphics: &mut gfx::GraphicsContext) {
    graphics.blur_rect(gfx::Rect::new(gfx::FloatPos(60.0, 50.0), gfx::FloatSize(200.0, 140.0)), 30);
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
/// The case that catches both halves of atlas sampling: each square must be a flat colour, so a
/// column of the neighbouring region's colour along an edge means either the source rectangle
/// is sampled too wide or the atlas did not pack in key order.
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

/// A toggle that is not centred, laid out against the right edge of a row the way the settings
/// menu does it.
///
/// The border has to show evenly on all four sides. It used not to: the bar was drawn by
/// shrinking the toggle's container and laying it out again, which also moves it by the
/// orientation, so at `RIGHT` the bar came out flush against the right edge with twice the
/// padding on the left. Every other toggle case is `CENTER`, where that happens to be correct,
/// which is why nothing caught it.
fn case_toggle_right_oriented(graphics: &mut gfx::GraphicsContext) {
    background(graphics);
    let root = parent_of(graphics);
    let row = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(240.0, 80.0), gfx::CENTER, Some(&root));
    row.get_absolute_rect().render(graphics, gfx::Color::new(70, 70, 90, 255));

    let mut toggle = toggle_at(gfx::RIGHT, false, 0.0);
    toggle.pos = gfx::FloatPos(-gfx::SPACING, 0.0);
    // Bright, unlike the theme's, so the frame the padding leaves is what the eye lands on when
    // this case is dumped - the default border is a grey nobody can measure by looking at it.
    toggle.border_color = gfx::Color::new(255, 255, 255, 255);
    toggle.render(graphics, &row);
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

/// Every case, in the order they run.
///
/// The golden's name is the drawing function's own with the `case_` prefix taken off, so a
/// case cannot end up compared against a different one's image - which a hand written table
/// of `name` / `draw` pairs is one careless copy-paste away from. `EXACT` is the default;
/// name a tolerance after a colon only where a case needs a looser one.
const CASES: &[Case] = &cases![
    case_rect_solid,
    case_rect_outline,
    case_rect_alpha_blend,
    case_rect_offscreen_culled,
    case_rect_partially_offscreen,
    case_rect_zero_alpha_is_skipped,
    case_rect_array_gradient,
    case_rect_array_textured,
    case_rect_array_translated,
    case_texture_unscaled,
    case_texture_scaled,
    case_texture_flipped,
    case_texture_src_rect,
    case_texture_tinted,
    case_texture_empty_src_rect,
    case_blend_mode_multiply,
    case_text_basic,
    case_text_scaled,
    case_text_on_fractional_offsets,
    case_text_mono,
    case_text_surface,
    case_text_width_limit,
    case_container_orientations,
    case_container_nested,
    case_render_rect_fill_and_border,
    case_render_rect_translucent,
    case_render_rect_shadow,
    case_render_rect_blur: BLURRY,
    case_blur_over_a_cleared_frame,
    case_render_rect_lags_behind_target,
    case_sprite_basic,
    case_sprite_flipped_tinted_centered,
    case_texture_atlas_single_region,
    case_texture_atlas_multiple_regions,
    case_texture_atlas_empty,
    case_button_idle,
    case_button_hovered,
    case_button_half_hovered,
    case_button_disabled_darkened,
    case_toggle_off,
    case_toggle_on,
    case_toggle_mid_travel,
    case_toggle_right_oriented,
    case_text_input_with_text,
    case_text_input_hint,
    case_text_input_overflowing_text,
];

// --- comparison -------------------------------------------------------------------------

struct Diff {
    differing_pixels: u32,
    /// Of those, the ones that moved further than the tolerance allows.
    pixels_over_tolerance: u32,
    total_pixels: u32,
    max_channel_delta: u8,
    first_difference: Option<(gfx::IntPos, gfx::Color, gfx::Color)>,
}

impl Diff {
    /// The share of the frame that moved further than a channel delta the tolerance forgives.
    fn fraction(&self) -> f32 {
        if self.total_pixels == 0 {
            0.0
        } else {
            self.pixels_over_tolerance as f32 / self.total_pixels as f32
        }
    }

    /// A case passes when the pixels that drifted past the allowed channel delta are a small
    /// enough share of the frame.
    ///
    /// Both halves have to hold. This used to be an `||`, which made the channel limit
    /// unreachable: at `BLURRY` any 2% of the frame could change by any amount at all - 1500
    /// pixels, where the blurred region is only 28000 - and the case still passed.
    fn within(&self, tolerance: Tolerance) -> bool {
        self.fraction() <= tolerance.max_differing_fraction
    }
}

fn channel_delta(a: gfx::Color, b: gfx::Color) -> u8 {
    let deltas = [a.r.abs_diff(b.r), a.g.abs_diff(b.g), a.b.abs_diff(b.b), a.a.abs_diff(b.a)];
    deltas.into_iter().max().unwrap_or(0)
}

fn compare(actual: &gfx::Surface, expected: &gfx::Surface, tolerance: Tolerance) -> Result<Diff> {
    if actual.get_size() != expected.get_size() {
        bail!("size mismatch: captured {:?}, golden {:?}", actual.get_size(), expected.get_size());
    }

    let mut diff = Diff {
        differing_pixels: 0,
        pixels_over_tolerance: 0,
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
        if delta > tolerance.max_channel_delta {
            diff.pixels_over_tolerance += 1;
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
fn capture_case(graphics: &mut gfx::GraphicsContext, case: &Case) -> Result<gfx::Surface> {
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
        let actual = capture_case(&mut graphics, case).with_context(|| format!("capturing {}", case.name()))?;
        let golden_path = goldens.join(format!("{}.opa", case.name()));

        if dump {
            write_ppm(&output.join(format!("{}.ppm", case.name())), &actual)?;
        }

        if regenerate {
            std::fs::write(&golden_path, actual.serialize_to_bytes()?).with_context(|| format!("writing {}", golden_path.display()))?;
            regenerated += 1;
            continue;
        }

        if !golden_path.exists() {
            println!("MISSING  {} (no golden at {})", case.name(), golden_path.display());
            failed += 1;
            continue;
        }

        let expected = gfx::Surface::deserialize_from_bytes(&std::fs::read(&golden_path).with_context(|| format!("reading {}", golden_path.display()))?)?;

        match compare(&actual, &expected, case.tolerance) {
            Ok(diff) if diff.within(case.tolerance) => {
                passed += 1;
            }
            Ok(diff) => {
                failed += 1;
                println!(
                    "FAIL     {}: {} of {} pixels differ, {} of them by more than {} ({:.3}%, allowed {:.3}%); largest channel delta {}",
                    case.name(),
                    diff.differing_pixels,
                    diff.total_pixels,
                    diff.pixels_over_tolerance,
                    case.tolerance.max_channel_delta,
                    diff.fraction() * 100.0,
                    case.tolerance.max_differing_fraction * 100.0,
                    diff.max_channel_delta,
                );
                if let Some((pos, actual_pixel, expected_pixel)) = diff.first_difference {
                    println!("         first at ({}, {}): got {actual_pixel:?}, want {expected_pixel:?}", pos.0, pos.1);
                }
                std::fs::create_dir_all(&output).with_context(|| format!("creating {}", output.display()))?;
                write_ppm(&output.join(format!("{}.actual.ppm", case.name())), &actual)?;
                write_ppm(&output.join(format!("{}.expected.ppm", case.name())), &expected)?;
                write_ppm(&output.join(format!("{}.diff.ppm", case.name())), &difference_surface(&actual, &expected))?;
                println!("         wrote actual/expected/diff to {}", output.display());
            }
            Err(error) => {
                failed += 1;
                println!("FAIL     {}: {error}", case.name());
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
