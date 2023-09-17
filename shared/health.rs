use crate::shared::entities::EntityId;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthChangePacket {
    pub health: i32,
    pub max_health: i32,
}

pub struct HealthComponent {
    health: i32,
    max_health: i32,
}

pub struct HealthChangeEvent {
    pub health: i32,
    pub entity: EntityId,
}

impl HealthComponent {
    #[must_use]
    pub const fn new(health: i32, max_health: i32) -> Self {
        Self { health, max_health }
    }

    #[must_use]
    pub const fn health(&self) -> i32 {
        self.health
    }

    #[must_use]
    pub const fn max_health(&self) -> i32 {
        self.max_health
    }
}
