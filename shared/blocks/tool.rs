use serde_derive::{Deserialize, Serialize};

use crate::libraries::registry::{NamedEntry, RegistryEntry, RegistryId};
use crate::shared::blocks::Blocks;

/// Struct that contains all the information about a tool
pub struct Tool {
    pub name: String,
    pub id: ToolId,
}

impl Tool {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            id: ToolId::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId {
    pub(super) id: i32,
}

impl ToolId {
    #[must_use]
    pub const fn new() -> Self {
        Self { id: -1 }
    }
}

impl RegistryId for ToolId {
    const KIND: &'static str = "tool type";

    fn from_index(index: usize) -> Self {
        Self { id: index as i32 }
    }

    fn index(self) -> Option<usize> {
        usize::try_from(self.id).ok()
    }
}

impl RegistryEntry<ToolId> for Tool {
    fn set_id(&mut self, id: ToolId) {
        self.id = id;
    }
}

impl NamedEntry for Tool {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Blocks {
    /// Adds a new tool type to the world.
    pub fn register_new_tool_type(&mut self, tool: Tool) -> ToolId {
        self.tool_types.register(tool)
    }

    /// Returns the tool type that has the specified name
    #[must_use]
    pub fn get_tool_id_by_name(&self, name: &str) -> Option<ToolId> {
        self.tool_types.get_id_by_name(name).ok()
    }

    /// Returns the reference to the Tool with the specified id.
    #[must_use]
    pub fn get_tool_by_id(&self, id: ToolId) -> Option<&Tool> {
        self.tool_types.get(id).ok()
    }
}
