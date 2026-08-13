use anyhow::Result;

pub use animation_timer::{approach, AnimationTimer};
pub use button::Button;
pub use color::{interpolate_colors, Color};
pub use container::{Container, Orientation, BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTER, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};
#[cfg(test)]
pub use draw_list::DrawRecorder;
pub use draw_list::{BlendMode, DrawCommand, DrawList, DrawTarget};
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
#[cfg(test)]
pub use ui_context::HeadlessContext;
pub use ui_context::UiContext;
pub use ui_element::{BaseUiElement, ClickTracker, UiElement};

mod animation_timer;
mod button;
mod color;
mod container;
/// What to draw, as backend-agnostic data. The seam the renderer is built around.
pub mod draw_list;
mod events;
/// The GPU device and the registry a `DrawCommand`'s handles resolve against.
pub(crate) mod gpu_device;
mod position;
mod rect;
mod rect_array;
mod render_rect;
/// Golden-image tests. Behind a feature because they need a real GPU surface on the main
/// thread, which `cargo test` cannot provide - see the module docs.
#[cfg(feature = "render-tests")]
pub mod render_tests;
mod renderer;
mod scrollable;
mod shadow;
mod sprite;
mod surface;
mod tests;
mod text;
mod text_input;
mod texture;
mod texture_atlas;
mod theme;
mod toggle;
mod transformation;
mod ui_context;
mod ui_element;
mod vertex_buffer;
/// How a draw list becomes pixels. The only module that talks to wgpu, apart from the
/// resource types that own GPU objects.
mod wgpu_backend;
/// The window and the input. The only module that talks to winit.
mod window;

/// Initializes the graphics context.
pub fn init(window_width: u32, window_height: u32, window_title: &str, default_font_data: &[u8], default_mono_font_data: Option<&[u8]>) -> Result<GraphicsContext> {
    GraphicsContext::new(window_width, window_height, window_title, default_font_data, default_mono_font_data)
}
