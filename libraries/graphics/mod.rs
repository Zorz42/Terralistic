use anyhow::Result;

pub use animation_timer::AnimationTimer;
pub use blend_mode::{set_blend_mode, BlendMode};
pub use button::Button;
pub use color::{interpolate_colors, Color};
pub use container::{Container, Orientation, BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTER, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};
pub use events::{Event, Key};
pub use position::{FloatPos, FloatSize, IntPos, IntSize};
pub use rect::Rect;
pub use rect_array::RectArray;
pub use render_rect::RenderRect;
pub use renderer::GraphicsContext;
pub use scrollable::Scrollable;
pub use sprite::Sprite;
pub use surface::Surface;
pub use text::Font;
pub use text_input::TextInput;
pub use texture::Texture;
pub use texture_atlas::TextureAtlas;
pub use theme::{BLACK, BLUR, BORDER_COLOR, DARK_GREY, GREY, LIGHT_GREY, SHADOW_INTENSITY, SPACING, TEXT_INPUT_WIDTH, TRANSPARENCY, TRANSPARENT, WHITE};
pub use toggle::Toggle;
pub use ui_element::UiElement;

mod animation_timer;
mod blend_mode;
mod blur;
mod button;
mod color;
mod container;
mod events;
mod passthrough_shader;
mod position;
mod rect;
mod rect_array;
mod render_rect;
mod renderer;
mod scrollable;
mod shaders;
mod shadow;
mod sprite;
mod surface;
mod text;
mod text_input;
mod texture;
mod texture_atlas;
mod theme;
mod toggle;
mod transformation;
mod ui_element;
mod vertex_buffer;

/// Initializes the graphics context.
pub fn init(window_width: u32, window_height: u32, window_title: &str, default_font_data: &[u8], default_mono_font_data: Option<&[u8]>) -> Result<GraphicsContext> {
    GraphicsContext::new(window_width, window_height, window_title, default_font_data, default_mono_font_data)
}
