use anyhow::Result;
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::EventManager;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH};
use crate::shared::entities::{
    is_touching_ground, reduce_by, Entities, EntityId, HealthComponent, PhysicsComponent,
    PositionComponent,
};
use crate::shared::inventory::Inventory;
use crate::shared::items::{ItemComponent, ItemStack, Items};

pub const PLAYER_HEIGHT: f32 = 28.0 / BLOCK_WIDTH;
pub const PLAYER_WIDTH: f32 = 18.0 / BLOCK_WIDTH;
pub const PLAYER_MAX_HEALTH: i32 = 100;
pub const PLAYER_ACCELERATION: f32 = 30.0;
pub const PLAYER_INITIAL_SPEED: f32 = 5.0;
pub const PLAYER_JUMP_SPEED: f32 = 30.0;
pub const PLAYER_PICKUP_RADIUS: f32 = 6.0;
pub const PLAYER_PICKUP_COEFFICIENT: f32 = 0.005;
pub const PLAYER_PICKUP_MIN_SPEED: f32 = 0.8;

#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum MovingType {
    Standing,
    MovingLeft,
    MovingRight,
}

/// # Errors
/// Returns an error if the player could not be spawned
pub fn spawn_player(
    entities: &mut Entities,
    x: f32,
    y: f32,
    name: &str,
    id: EntityId,
) -> Result<Entity> {
    let entity = entities.ecs.spawn((
        PositionComponent::new(x, y),
        PhysicsComponent::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        Inventory::new(20),
        PlayerComponent::new(name),
        HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
    ));

    entities.assign_id(entity, id)?;

    Ok(entity)
}

pub fn update_players_ms(entities: &mut Entities, blocks: &Blocks) {
    for (_, (position, physics, player)) in entities.ecs.query_mut::<(
        &PositionComponent,
        &mut PhysicsComponent,
        &mut PlayerComponent,
    )>() {
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

    let mut positions = Vec::new();
    for (_, (position, _player)) in entities
        .ecs
        .query_mut::<(&PositionComponent, &PlayerComponent)>()
    {
        positions.push((
            position.x() + PLAYER_WIDTH / 2.0,
            position.y() + PLAYER_HEIGHT / 2.0,
        ));
    }

    for player_position in positions {
        // loop through all items in the world and
        // check if the player is near them, if so
        // the item should be accelerated towards the player
        // a square of the distance is calculated as d^2, and
        // size of the speed change is c * (r^2 - d^2), where r is the
        // pickup range of the player and c is a constant
        // the speed change is applied to the item's velocity
        // in the direction of the player

        for (_, (item_position, item_physics, _item)) in
            entities
                .ecs
                .query_mut::<(&PositionComponent, &mut PhysicsComponent, &ItemComponent)>()
        {
            let dx = player_position.0 - item_position.x() - 0.5;
            let dy = player_position.1 - item_position.y() - 0.5;
            let d2 = dx * dx + dy * dy;
            let r2 = PLAYER_PICKUP_RADIUS * PLAYER_PICKUP_RADIUS;
            let c = PLAYER_PICKUP_COEFFICIENT;
            let size = c * (r2 - d2) + PLAYER_PICKUP_MIN_SPEED;
            if size > PLAYER_PICKUP_MIN_SPEED {
                let d2s = d2.sqrt();
                let speed_change_x = size * dx / d2s;
                let speed_change_y = size * dy / d2s;
                item_physics.velocity_x += speed_change_x;
                item_physics.velocity_y += speed_change_y;
            }
        }
    }
}

/// # Errors
/// Returns an error if the item could not be removed
pub fn remove_all_picked_items(
    entities: &mut Entities,
    events: &mut EventManager,
    items: &mut Items,
) -> Result<()> {
    let mut positions = Vec::new();
    for (entity, (position, _player)) in entities
        .ecs
        .query_mut::<(&PositionComponent, &PlayerComponent)>()
    {
        positions.push((
            (
                position.x() + PLAYER_WIDTH / 2.0,
                position.y() + PLAYER_HEIGHT / 2.0,
            ),
            entity,
        ));
    }

    for (player_position, player_entity) in positions {
        let mut items_to_remove = Vec::new();
        for (entity, (item_position, _item)) in entities
            .ecs
            .query_mut::<(&PositionComponent, &ItemComponent)>()
        {
            let dx = player_position.0 - item_position.x() - 0.5;
            let dy = player_position.1 - item_position.y() - 0.5;
            let d2 = dx * dx + dy * dy;

            if d2 < 0.3 {
                items_to_remove.push(entity);
            }
        }

        for entity in items_to_remove {
            let item_type = entities.ecs.get::<&ItemComponent>(entity)?.get_item_type();
            let mut inventory = (*entities.ecs.get::<&Inventory>(player_entity)?).clone();
            inventory.give_item(
                ItemStack::new(item_type, 1),
                player_position,
                items,
                entities,
                events,
            )?;
            *entities.ecs.get::<&mut Inventory>(player_entity)? = inventory;

            let item_id = entities.get_id_from_entity(entity)?;
            entities.despawn_entity(item_id, events)?;
        }
    }

    Ok(())
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
pub struct PlayerMovingPacketToClient {
    pub moving_type: MovingType,
    pub jumping: bool,
    pub player_id: EntityId,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerMovingPacketToServer {
    pub moving_type: MovingType,
    pub jumping: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSpawnPacket {
    pub id: EntityId,
    pub x: f32,
    pub y: f32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NamePacket {
    pub name: String,
}
