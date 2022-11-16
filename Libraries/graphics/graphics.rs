mod color;
pub use color::Color;

mod transformation;

mod surface;
pub use surface::Surface;

mod glfw_abstraction;

mod events;
pub use events::Event;

/*
A struct that will be passed all
around the functions that need drawing
*/
pub struct GraphicsContext {
    pub events: events::EventManager,
    pub renderer: glfw_abstraction::Renderer,
}

pub fn init(window_width: i32, window_height: i32, window_title: String) -> GraphicsContext {
    let glfw_context = glfw_abstraction::init_glfw(window_width, window_height, window_title);
    GraphicsContext{
        events: events::EventManager::new(glfw_context.window_events),
        renderer: glfw_abstraction::Renderer{
            glfw: glfw_context.glfw,
            glfw_window: glfw_context.glfw_window,
        }
    }

}