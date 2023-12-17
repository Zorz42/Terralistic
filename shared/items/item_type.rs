use crate::shared::blocks::{BlockId, ToolId};
use crate::shared::items::ItemId;
use crate::shared::walls::WallId;

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub display_name: String,
    pub max_stack: i32,
    pub places_block: Option<BlockId>,
    pub places_wall: Option<WallId>,
    pub tool: Option<ToolId>,
    pub tool_power: i32,
    pub(super) id: ItemId,
}

impl Item {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            max_stack: 0,
            places_block: None,
            places_wall: None,
            tool: None,
            tool_power: 0,
            id: ItemId::new(),
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> ItemId {
        self.id
    }
}
