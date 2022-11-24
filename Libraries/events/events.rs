/*
This trait is used to define an event.
*/
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use shared_mut::SharedMut;

pub trait Event {}

/*
This trait is used to define a event listener.
*/
pub trait Listener<Type> {
    fn on_event(&mut self, event: &Type);
}

/*
This struct is an event sender.
*/
pub struct Sender<Type> {
    listeners: Vec<SharedMut<dyn Listener<Type>>>,
}

impl<Type> Sender<Type> {
    /*
    Creates a new event sender.
    */
    pub fn new() -> Self {
        Sender {
            listeners: vec![],
        }
    }

    /*
    Adds a listener to the event sender.
    */
    pub fn add_listener<ListenerType: Listener<Type> + 'static>(&mut self, listener: &SharedMut<ListenerType>) {
        self.listeners.push(SharedMut{value: listener.value.clone()});
    }

    /*
    Sends an event to all listeners.
    */
    pub fn send(&mut self, event: &Type) {
        for listener in self.listeners.iter_mut() {
            listener.get_mut().on_event(event);
        }
    }
}