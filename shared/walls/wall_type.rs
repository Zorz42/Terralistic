use crate::libraries::registry::{NamedEntry, RegistryEntry};
use crate::shared::walls::WallId;

/// Wall holds all information about a type of a wall.
#[derive(Clone)]
pub struct Wall {
    pub(super) id: WallId,
    pub break_time: Option<i32>,
    pub name: String,
}

impl Wall {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: WallId::undefined(),
            break_time: None,
            name: String::new(),
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> WallId {
        self.id
    }
}

impl RegistryEntry<WallId> for Wall {
    fn set_id(&mut self, id: WallId) {
        self.id = id;
    }
}

impl NamedEntry for Wall {
    fn get_name(&self) -> &str {
        &self.name
    }
}
