mod color;
pub use color::Color;

mod transformation;

mod vertex_buffer_impl;

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
    pub events: events::EventManager,
    pub renderer: renderer::Renderer,
}

pub fn init(window_width: i32, window_height: i32, window_title: String) -> GraphicsContext {
    let glfw_context = renderer::init_glfw(window_width, window_height, window_title);
    let events = events::EventManager::new(glfw_context.window_events);
    let mut renderer = renderer::Renderer::new(glfw_context.glfw, glfw_context.glfw_window);

    renderer.init();

    GraphicsContext{
        events,
        renderer,
    }

}