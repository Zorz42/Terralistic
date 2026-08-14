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

/// Event manager can be used to push and pop events to and from a queue.
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

    /// How many events are waiting. Only the tests ask: the game drains the queue rather than
    /// measuring it, but how *big* the queue gets is a property worth pinning - see the
    /// generation test in `integration_tests/server_lifecycle.rs`.
    #[cfg(test)]
    #[must_use]
    pub fn queued_count(&self) -> usize {
        self.event_queue.len()
    }
}
