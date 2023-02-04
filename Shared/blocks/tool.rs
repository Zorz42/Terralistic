use crate::blocks::Blocks;

use serde_derive::{Deserialize, Serialize};

/**
Struct that contains all the information about a tool
 */
pub struct Tool {
    pub name: String,
    pub id: i32,
}

impl Tool {
    pub fn new() -> Self {
        Tool {
            name: String::new(),
            id: -1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId {
    pub(super) id: i32,
}

impl ToolId {
    pub fn new() -> Self {
        Self { id: -1 }
    }
}

impl Blocks {
    /**
    Adds a new tool type to the world.
     */
    pub fn register_new_tool_type(&mut self, mut tool: Tool) -> ToolId {
        let id = self.tool_types.len() as i32;
        tool.id = id;
        self.tool_types.push(tool);
        ToolId { id }
    }

    /**
    Returns the tool type that has the specified name
     */
    pub fn get_tool_id_by_name(&mut self, name: String) -> Option<ToolId> {
        for tool_type in self.tool_types.iter() {
            if tool_type.name == name {
                return Some(ToolId { id: tool_type.id });
            }
        }
        None
    }

    /**
    Returns the reference to the Tool with the specified id.
     */
    pub fn get_tool_by_id(&mut self, id: ToolId) -> Option<&Tool> {
        self.tool_types
            .iter()
            .find(|&tool_type| tool_type.id == id.id)
    }
}
