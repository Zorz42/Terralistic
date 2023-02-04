use std::any::Any;
use std::collections::VecDeque;

pub struct Event {
    event: Box<dyn Any>,
}

impl Event {
    pub fn new<T: Any>(event: T) -> Self {
        Self {
            event: Box::new(event),
        }
    }

    pub fn downcast<T: Any>(&self) -> Option<&T> {
        self.event.downcast_ref::<T>()
    }
}

unsafe impl Send for Event {}

/**
Event manager can be used to push and pop events to and from a queue.
 */
pub struct EventManager {
    event_queue: VecDeque<Event>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            event_queue: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }
}
