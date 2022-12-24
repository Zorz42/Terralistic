use std::any::Any;
use std::collections::VecDeque;

/**
Event manager can be used to push and pop events to and from a queue.
 */
pub struct EventManager {
    event_queue: VecDeque<Box<dyn Any>>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            event_queue: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: Box<dyn Any>) {
        self.event_queue.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<Box<dyn Any>> {
        self.event_queue.pop_front()
    }
}