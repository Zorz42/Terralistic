use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

/*
This struct packs a mutable object which can be shared.
It doesn't require compile time knowledge of the size of the object.
*/
pub struct SharedMut<Type: ?Sized> {
    value: Rc<RefCell<Type>>,
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
    Creates a new shared mutable object from a Rc<RefCell<Type>>.
    */
    pub fn from_rc(value: Rc<RefCell<Type>>) -> Self {
        SharedMut {
            value,
        }
    }

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

    /*
    Returns cloned value.
    */
    pub fn get_cloned(&self) -> Rc<RefCell<Type>> {
        self.value.clone()
    }
}