mod color;
pub use color::Color;

mod transformation;

mod vertex_buffer_impl;

mod blend_mode;
pub use blend_mode::{BlendMode, set_blend_mode};

mod shaders;
pub use shaders::compile_shader;

mod surface;
pub use surface::Surface;

mod renderer;

mod events;
pub use events::Event;

mod rect_shape;
pub use rect_shape::RectShape;

mod texture;
pub use texture::Texture;

mod read_opa;
pub use read_opa::read_opa;

/*
A struct that will be passed all
around the functions that need drawing
*/
pub struct GraphicsContext {
    pub renderer: renderer::Renderer,
}

pub fn init(window_width: i32, window_height: i32, window_title: String) -> GraphicsContext {
    GraphicsContext{
        renderer: renderer::Renderer::new(window_width, window_height, window_title),
    }

}