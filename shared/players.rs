use crate::shared::blocks::{Blocks, BLOCK_WIDTH};
use crate::shared::entities::{
    is_touching_ground, reduce_by, Entities, IdComponent, PhysicsComponent, PositionComponent,
};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

pub const PLAYER_HEIGHT: f32 = 28.0 / BLOCK_WIDTH;
pub const PLAYER_WIDTH: f32 = 18.0 / BLOCK_WIDTH;
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

fn update_player_ms(
    position: &PositionComponent,
    physics: &mut PhysicsComponent,
    player: &mut PlayerComponent,
    blocks: &Blocks,
) {
    if player.jumping && is_touching_ground(position, physics, blocks) {
        physics.velocity_y += -PLAYER_JUMP_SPEED;
    }

    // animation frame for being in air is 0
    // animation frames from 1 to 9 inclusive are for walking
    // animation frame 1 is for standing
    // animation frame changes every n calls to this function
    let n = 15;

    match player.moving_type {
        MovingType::Standing => {
            player.animation_frame = 1;
        }
        MovingType::MovingRight | MovingType::MovingLeft => {
            if player.moving_type == MovingType::MovingRight {
                player.direction = Direction::Right;
            } else {
                player.direction = Direction::Left;
            }

            player.frame_progress += 1;
            if player.frame_progress >= n {
                player.frame_progress = 0;
                player.animation_frame += 1;
            }

            if player.animation_frame < 1 || player.animation_frame > 9 {
                player.animation_frame = 1;
                player.frame_progress = 0;
            }
        }
    }

    if !is_touching_ground(position, physics, blocks) {
        player.animation_frame = 0;
    }

    if physics.velocity_x.abs() < 0.01
        && (player.moving_type == MovingType::MovingRight
            || player.moving_type == MovingType::MovingLeft)
    {
        player.animation_frame = 1;
    }
}

pub fn update_players_ms(entities: &mut Entities, blocks: &Blocks) {
    for (_, (position, physics, player)) in entities.ecs.query_mut::<(
        &PositionComponent,
        &mut PhysicsComponent,
        &mut PlayerComponent,
    )>() {
        update_player_ms(position, physics, player, blocks);
    }
}

pub enum Direction {
    Left,
    Right,
}

pub struct PlayerComponent {
    moving_type: MovingType,
    pub jumping: bool,
    pub animation_frame: i32,
    pub frame_progress: i32,
    pub direction: Direction,
    name: String,
}

impl PlayerComponent {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            moving_type: MovingType::Standing,
            jumping: false,
            animation_frame: 0,
            frame_progress: 0,
            direction: Direction::Right,
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
                physics.acceleration_x += PLAYER_ACCELERATION;
                reduce_by(&mut physics.velocity_x, PLAYER_INITIAL_SPEED);
            }
            MovingType::MovingRight => {
                physics.acceleration_x -= PLAYER_ACCELERATION;
                reduce_by(&mut physics.velocity_x, PLAYER_INITIAL_SPEED);
            }
        }

        // apply effects of the new state
        match moving_type {
            MovingType::Standing => {}
            MovingType::MovingLeft => {
                physics.acceleration_x -= PLAYER_ACCELERATION;
                physics.velocity_x -= PLAYER_INITIAL_SPEED;
            }
            MovingType::MovingRight => {
                physics.acceleration_x += PLAYER_ACCELERATION;
                physics.velocity_x += PLAYER_INITIAL_SPEED;
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
