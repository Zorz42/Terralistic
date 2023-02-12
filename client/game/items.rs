use crate::shared::items::Items;
use crate::shared::mod_manager::ModManager;
use anyhow::Result;

pub struct ClientItems {
    pub items: Items,
}

impl ClientItems {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.items.init(mods)
    }
}
