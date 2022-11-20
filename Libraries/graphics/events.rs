use glfw;

/*
A collection of all supported events
*/
pub enum Event {

}

/*
Translates glfw type events to our event type
*/
pub fn glfw_event_to_gfx_event(glfw_event: glfw::WindowEvent) -> Option<Event> {
    match glfw_event {
        _ => None
    }
}