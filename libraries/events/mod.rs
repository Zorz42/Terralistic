use std::any::Any;
use std::collections::VecDeque;

mod tests;

pub struct Event {
    event: Box<dyn Any + Send>,
}

impl Event {
    pub fn new<T: Any + Send>(event: T) -> Self {
        Self { event: Box::new(event) }
    }

    #[must_use]
    pub fn downcast<T: Any + Send>(&self) -> Option<&T> {
        self.event.downcast_ref::<T>()
    }
}

unsafe impl Send for Event {}

/// A queue of type-erased events, pushed at one end and drained at the other.
pub struct EventManager {
    event_queue: VecDeque<Event>,
}

impl EventManager {
    #[must_use]
    pub const fn new() -> Self {
        Self { event_queue: VecDeque::new() }
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }

    /// How many events are waiting. Only tests ask - the game drains rather than measures -
    /// but how big the queue gets is worth pinning; see `integration_tests/server_lifecycle.rs`.
    #[cfg(test)]
    #[must_use]
    pub fn queued_count(&self) -> usize {
        self.event_queue.len()
    }
}
