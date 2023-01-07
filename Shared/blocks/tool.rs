/**
Struct that contains all the information about a tool
 */
pub struct Tool {
    pub name: String
}

impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}