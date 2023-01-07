use std::borrow::BorrowMut;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use snap;
use graphics as gfx;
use shared_mut::SharedMut;
use crate::mod_manager::ModManager;

/**
Struct that contains all the information about a tool
 */
pub struct Tool {
    pub name: String
}

impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}