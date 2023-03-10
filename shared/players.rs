use crate::shared::blocks::Blocks;
use crate::shared::entities::{
    is_touching_ground, reduce_by, Entities, IdComponent, PhysicsComponent, PositionComponent,
};
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
    MovingLeft,
    MovingRight,
}

pub fn spawn_player(
    entities: &mut Entities,
    x: f32,
    y: f32,
    name: &str,
    id: Option<u32>,
) -> Entity {
    let id = entities.unwrap_id(id);
    entities.ecs.spawn((
        IdComponent::new(id),
        PositionComponent::new(x, y),
        PhysicsComponent::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        PlayerComponent::new(name),
    ))
}

pub fn update_player(
    position: &PositionComponent,
    physics: &mut PhysicsComponent,
    player: &PlayerComponent,
    blocks: &Blocks,
) {
    if player.jumping && is_touching_ground(position, physics, blocks) {
        physics.increase_velocity_y(-PLAYER_JUMP_SPEED);
    }
}

pub fn update_players(entities: &mut Entities, blocks: &Blocks) {
    for (_, (position, physics, player)) in
        entities
            .ecs
            .query_mut::<(&PositionComponent, &mut PhysicsComponent, &PlayerComponent)>()
    {
        update_player(position, physics, player, blocks);
    }
}

pub struct PlayerComponent {
    moving_type: MovingType,
    pub jumping: bool,
    name: String,
}

impl PlayerComponent {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            moving_type: MovingType::Standing,
            jumping: false,
            name: name.to_owned(),
        }
    }

    #[must_use]
    pub const fn get_moving_type(&self) -> MovingType {
        self.moving_type
    }

    pub fn set_moving_type(&mut self, moving_type: MovingType, physics: &mut PhysicsComponent) {
        if self.moving_type == moving_type {
            return;
        }

        // revert effects of the current state
        match self.moving_type {
            MovingType::Standing => {}
            MovingType::MovingLeft => {
                physics.increase_acceleration_x(PLAYER_ACCELERATION);
                let mut velocity = physics.get_velocity_x();
                reduce_by(&mut velocity, PLAYER_INITIAL_SPEED);
                physics.set_velocity_x(velocity);
            }
            MovingType::MovingRight => {
                physics.increase_acceleration_x(-PLAYER_ACCELERATION);
                let mut velocity = physics.get_velocity_x();
                reduce_by(&mut velocity, PLAYER_INITIAL_SPEED);
                physics.set_velocity_x(velocity);
            }
        }

        // apply effects of the new state
        match moving_type {
            MovingType::Standing => {}
            MovingType::MovingLeft => {
                physics.increase_acceleration_x(-PLAYER_ACCELERATION);
                physics.increase_velocity_x(-PLAYER_INITIAL_SPEED);
            }
            MovingType::MovingRight => {
                physics.increase_acceleration_x(PLAYER_ACCELERATION);
                physics.increase_velocity_x(PLAYER_INITIAL_SPEED);
            }
        }

        self.moving_type = moving_type;
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerMovingPacket {
    pub moving_type: MovingType,
    pub jumping: bool,
    pub player_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSpawnPacket {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NamePacket {
    pub name: String,
}
