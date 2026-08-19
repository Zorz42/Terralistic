//! The renderer: what to draw, and how it becomes pixels.
//!
//! Drawing calls do not touch the graphics API - they record a `DrawCommand` into the frame's
//! `DrawList`, and `GraphicsContext::update_window` hands the list to the backend. That seam is
//! what makes `libraries::ui` testable without a GPU and the golden images possible at all.
//!
//! **Not in scope**: widgets, layout and input routing, which are `libraries::ui`. This knows
//! about rectangles, textures, glyphs and a window; it has no notion of a button.

use anyhow::Result;

pub use color::{interpolate_colors, Color};
#[cfg(test)]
pub use draw_list::DrawRecorder;
pub use draw_list::{BlendMode, DrawCommand, DrawList, DrawTarget};
pub use events::{Event, Key};
pub use position::{FloatPos, FloatSize, IntPos, IntSize};
pub use rect::Rect;
pub use rect_array::RectArray;
pub use renderer::GraphicsContext;
pub use surface::Surface;
pub use text::Font;
pub use texture::Texture;
pub use texture_atlas::TextureAtlas;
pub use window::PIXELS_PER_SCROLL_LINE;

mod color;
/// What to draw, as backend-agnostic data. The seam the renderer is built around.
pub mod draw_list;
mod events;
/// The GPU device and the registry a `DrawCommand`'s handles resolve against.
pub(crate) mod gpu_device;
mod position;
mod rect;
mod rect_array;
/// Golden-image tests. Behind a feature because they need a real GPU surface on the main
/// thread, which `cargo test` cannot provide - see the module docs.
#[cfg(feature = "render-tests")]
pub mod render_tests;
mod renderer;
mod shadow;
mod surface;
mod tests;
mod text;
mod texture;
mod texture_atlas;
mod transformation;
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
