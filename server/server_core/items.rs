use crate::shared::items::Items;
use crate::shared::mod_manager::ModManager;
use anyhow::Result;

pub struct ServerItems {
    pub items: Items,
}

impl ServerItems {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.items.init(mods)
    }
}
