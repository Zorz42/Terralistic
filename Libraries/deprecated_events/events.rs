/*
This trait is used to define an event.
*/
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
pub struct Sender<Type: Event> {
    listeners: Vec<SharedMut<dyn Listener<Type>>>,
}

impl<Type: Event> Sender<Type> {
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
        self.listeners.push(SharedMut::from_rc(listener.get_cloned()));
    }

    /*
    Sends an event to all listeners.
    */
    pub fn send(&mut self, event: Type) {
        for listener in self.listeners.iter_mut() {
            listener.borrow().on_event(&event);
        }
    }
}