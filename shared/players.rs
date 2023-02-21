use crate::shared::entities::{Entities, IdComponent, PhysicsComponent, PositionComponent};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

pub const PLAYER_HEIGHT: f32 = 3.0;
pub const PLAYER_WIDTH: f32 = 2.0;
pub const PLAYER_MAX_HEALTH: i32 = 80;
pub const PLAYER_ACCELERATION: f32 = 30.0;
pub const PLAYER_INITIAL_SPEED: f32 = 5.0;
pub const PLAYER_JUMP_SPEED: f32 = 30.0;

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum MovingType {
    Standing,
    Walking,
    Sneaking,
    SneakWalking,
    Running,
}

pub fn spawn_player(entities: &mut Entities, x: f32, y: f32, id: Option<u32>) -> Entity {
    let id = entities.unwrap_id(id);
    entities.ecs.spawn((
        IdComponent::new(id),
        PositionComponent::new(x, y),
        PhysicsComponent::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        PlayerComponent::new(),
    ))
}

pub struct PlayerComponent;

impl PlayerComponent {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
