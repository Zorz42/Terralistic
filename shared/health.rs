use crate::shared::entities::IdComponent;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthChangePacket {
    pub health: i32,
}

pub struct HealthComponent {
    health: i32,
    max_health: i32,
}

pub struct HealthChangeEvent {
    pub health: i32,
    pub entity: IdComponent,
}

impl HealthComponent {
    #[must_use]
    pub const fn new(health: i32, max_health: i32) -> Self {
        Self { health, max_health }
    }
}
