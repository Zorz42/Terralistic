/*
This trait is used to define an event.
*/
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

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
    listeners: Vec<Rc<RefCell<dyn Listener<Type>>>>,
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
    pub fn add_listener(&mut self, listener: &Rc<RefCell<dyn Listener<Type>>>) {
        self.listeners.push(Rc::clone(listener));
    }

    /*
    Sends an event to all listeners.
    */
    pub fn send(&mut self, event: &Type) {
        for i in (0..self.listeners.len()).rev() {
            //self.listeners[i].get_mut().on_event(event);
            //self.listeners[i].borrow_mut().on_event(event);
            self.listeners[i].deref().borrow_mut().on_event(event);
        }
    }
}