use anyhow::Result;

pub use blend_mode::{set_blend_mode, BlendMode};
pub use button::Button;
pub use color::Color;
pub use container::{
    Container, Orientation, BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTER, LEFT, RIGHT, TOP, TOP_LEFT,
    TOP_RIGHT,
};
pub use events::{Event, Key};
pub use position::{FloatPos, FloatSize, IntPos, IntSize};
pub use rect::Rect;
pub use rect_array::RectArray;
pub use render_rect::RenderRect;
use renderer::Renderer;
pub use scrollable::Scrollable;
pub use sprite::Sprite;
pub use surface::Surface;
pub use text::Font;
pub use text_input::TextInput;
pub use texture::Texture;
pub use texture_atlas::TextureAtlas;
pub use theme::{
    BLACK, BLUR, BORDER_COLOR, DARK_GREY, GREY, LIGHT_GREY, SHADOW_INTENSITY, SPACING,
    TEXT_INPUT_WIDTH, TRANSPARENCY, TRANSPARENT, WHITE,
};

mod position;

mod color;

mod theme;

mod transformation;

mod vertex_buffer;

mod blend_mode;

mod shaders;

mod surface;

mod blur;

mod shadow;

mod passthrough_shader;

mod renderer;

mod events;

mod rect;

mod texture;

mod text;

mod container;

mod render_rect;

mod button;

mod text_input;

mod sprite;

mod texture_atlas;

mod rect_array;

mod scrollable;

/// A struct that will be passed all
/// around the functions that need drawing
pub struct GraphicsContext {
    pub renderer: Renderer,
    pub font: Font,
    pub font_mono: Option<Font>,
}

/// Initializes the graphics context.
/// # Errors
/// - If the renderer fails to initialize.
/// - If the font fails to initialize.
pub fn init(
    window_width: u32,
    window_height: u32,
    window_title: &str,
    default_font_data: &[u8],
    default_mono_font_data: Option<&[u8]>,
) -> Result<GraphicsContext> {
    Ok(GraphicsContext {
        renderer: Renderer::new(window_width, window_height, window_title)?,
        font: Font::new(default_font_data, false)?,
        font_mono: if let Some(data) = default_mono_font_data {
            Some(Font::new(data, true)?)
        } else {
            None
        },
    })
}
