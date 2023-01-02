use std::sync::{Arc, Mutex, MutexGuard};

/**
This struct packs a mutable object which can be shared.
It doesn't require compile time knowledge of the size of the object.
 */
pub struct SharedMut<Type: ?Sized> {
    value: Arc<Mutex<Type>>,
}

impl<Type> SharedMut<Type> {
    /**
    Creates a new shared mutable object.
     */
    pub fn new(value: Type) -> Self {
        SharedMut {
            value: Arc::new(Mutex::new(value)),
        }
    }
}

impl<Type: ?Sized> SharedMut<Type> {
    /**
    Creates a new shared mutable object from a Rc<RefCell<Type>>.
     */
    pub fn from_rc(value: Arc<Mutex<Type>>) -> Self {
        SharedMut {
            value,
        }
    }

    /**
    Returns a reference to the shared mutable object.
     */
    pub fn borrow(&self) -> MutexGuard<Type> {
        loop {
            match self.value.lock() {
                Ok(value) => return value,
                Err(_) => {}
            }
            println!("Waiting for lock...");
        }
    }
}

/**
Cloning a shared mutable object creates a duplicate of the shared mutable object.
 */
impl<Type: ?Sized> Clone for SharedMut<Type> {
    fn clone(&self) -> Self {
        SharedMut {
            value: Arc::clone(&self.value),
        }
    }
}