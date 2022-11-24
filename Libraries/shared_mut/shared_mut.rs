use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::Deref;

/*
This struct packs a mutable object which can be shared.
It doesn't require compile time knowledge of the size of the object.
*/
pub struct SharedMut<Type: ?Sized> {
    pub value: Rc<RefCell<Type>>,
}

impl<Type> SharedMut<Type> {
    /*
    Creates a new shared mutable object.
    */
    pub fn new(value: Type) -> Self {
        SharedMut {
            value: Rc::new(RefCell::new(value)),
        }
    }
}

impl<Type: ?Sized> SharedMut<Type> {
    /*
    Creates a duplicate of the shared mutable object.
    */
    pub fn duplicate(&self) -> Self {
        SharedMut {
            value: Rc::clone(&self.value),
        }
    }

    /*
    Returns a reference to the shared mutable object.
    */
    pub fn get(&self) -> Ref<Type> {
        self.value.borrow()
    }

    /*
    Returns a mutable reference to the shared mutable object.
    */
    pub fn get_mut(&self) -> RefMut<Type> {
        self.value.borrow_mut()
    }
}