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

pub struct EventManager {
    glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl EventManager {
    pub fn new(glfw_events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>) -> Self {
        EventManager {
            glfw_events
        }
    }

    /*
    Returns an array of events, such as key presses
    */
    pub fn get_events(&mut self) -> Vec<Event> {
        let mut events = vec![];

        for (_, glfw_event) in glfw::flush_messages(&self.glfw_events) {
            match glfw_event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height)
                    }
                }
                _ => {}
            }

            if let Some(event) = glfw_event_to_gfx_event(glfw_event) {
                events.push(event);
            }
        }

        events
    }
}