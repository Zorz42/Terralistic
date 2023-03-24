mod tests;

extern crate alloc;
use alloc::collections::VecDeque;
use core::any::Any;

pub struct Event {
    event: Box<dyn Any + Send>,
}

impl Event {
    pub fn new<T: Any + Send>(event: T) -> Self {
        Self {
            event: Box::new(event),
        }
    }

    #[must_use]
    pub fn downcast<T: Any + Send>(&self) -> Option<&T> {
        self.event.downcast_ref::<T>()
    }
}

// SAFETY: Event is Send because it is only a wrapper around a Box<dyn Any + Send>
unsafe impl Send for Event {}

/**
Event manager can be used to push and pop events to and from a queue.
 */
pub struct EventManager {
    event_queue: VecDeque<Event>,
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

impl EventManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
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
