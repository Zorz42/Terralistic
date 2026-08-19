use anyhow::Result;
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::EventManager;
use crate::libraries::fixed::Fixed;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH};
use crate::shared::entities::{is_touching_ground, liquid_submersion, reduce_by, Entities, EntityId, HealthComponent, PhysicsComponent, PositionComponent};
use crate::shared::inventory::Inventory;
use crate::shared::items::{ItemComponent, ItemStack, Items};
use crate::shared::liquids::Liquids;
use crate::shared::TICKS_PER_SECOND;

pub const PLAYER_HEIGHT: Fixed = Fixed::from_num(24, BLOCK_WIDTH as i32);
pub const PLAYER_WIDTH: Fixed = Fixed::from_num(16, BLOCK_WIDTH as i32);
pub const PLAYER_MAX_HEALTH: i32 = 100;
pub const PLAYER_ACCELERATION: Fixed = Fixed::from_int(30);
pub const PLAYER_INITIAL_SPEED: Fixed = Fixed::from_int(5);
pub const PLAYER_JUMP_SPEED: Fixed = Fixed::from_int(30);
/// How hard a swimming player pushes upwards, and how fast that can get them going.
pub const PLAYER_SWIM_ACCELERATION: Fixed = Fixed::from_int(90);
pub const PLAYER_SWIM_SPEED: Fixed = Fixed::from_int(8);
/// How full a cell has to be before the jump key swims rather than doing nothing, which is
/// what keeps a puddle from being climbable.
const SWIMMABLE_SUBMERSION: Fixed = Fixed::from_num(1, 2);
pub const PLAYER_PICKUP_RADIUS: Fixed = Fixed::from_int(6);
pub const PLAYER_PICKUP_COEFFICIENT: Fixed = Fixed::from_num(1, 200);
pub const PLAYER_PICKUP_MIN_SPEED: Fixed = Fixed::from_num(4, 5);
/// How close an item has to get before it is taken, squared.
const PICKUP_REACH_SQUARED: Fixed = Fixed::from_num(3, 10);
/// An item is drawn from the middle of its cell, so its centre is half a block along each axis.
const ITEM_HALF_SIZE: Fixed = Fixed::from_num(1, 2);
pub const PLAYER_INVENTORY_SIZE: usize = 20;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default, Serialize, Deserialize)]
pub enum MovingType {
    #[default]
    Standing,
    MovingLeft,
    MovingRight,
}

pub fn spawn_player(entities: &mut Entities, x: Fixed, y: Fixed, name: &str, id: EntityId, health_component: HealthComponent) -> Result<Entity> {
    let entity = entities.ecs.spawn((
        PositionComponent::new(x, y),
        PhysicsComponent::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        Inventory::new(PLAYER_INVENTORY_SIZE),
        PlayerComponent::new(name),
        health_component,
    ));

    entities.assign_id(entity, id)?;

    Ok(entity)
}

/// Advances every player by one tick: jumping, swimming and the walk animation.
///
/// Runs on both sides. Like `step_entity` it touches nothing outside the player it is
/// looking at, so the client can replay it - which is why pulling items towards a player,
/// the one part that reads every *other* entity, is `attract_items_to_players` instead.
pub fn update_players_ms(entities: &mut Entities, blocks: &Blocks, liquids: &Liquids) {
    for (position, physics, player) in entities.ecs.query_mut::<(&PositionComponent, &mut PhysicsComponent, &mut PlayerComponent)>() {
        step_player(position, physics, player, blocks, liquids);
    }
}

/// One player's own tick: what the jump key does, and which walk frame to draw.
///
/// Split out of the loop so a single player can be advanced on its own, which is what a
/// replay does - it re-simulates one player over the ticks whose answers have to change,
/// and knows nothing about the others.
pub fn step_player(position: &PositionComponent, physics: &mut PhysicsComponent, player: &mut PlayerComponent, blocks: &Blocks, liquids: &Liquids) {
    if player.jumping {
        if is_touching_ground(position, physics, blocks) {
            physics.velocity_y += -PLAYER_JUMP_SPEED;
        } else {
            // Swimming is the jump key held down in a liquid: an upward push every tick
            // rather than one impulse, capped, so a player rises steadily to the surface
            // instead of leaping out of it.
            let (submersion, _speed_multiplier) = liquid_submersion(position, physics, liquids);
            if submersion > SWIMMABLE_SUBMERSION {
                physics.velocity_y = (physics.velocity_y - PLAYER_SWIM_ACCELERATION / TICKS_PER_SECOND).max(-PLAYER_SWIM_SPEED);
            }
        }
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

    if physics.velocity_x.abs() < Fixed::from_num(1, 100) && (player.moving_type == MovingType::MovingRight || player.moving_type == MovingType::MovingLeft) {
        player.animation_frame = 1;
    }
}

/// Pulls nearby items towards each player, the step before they are picked up.
///
/// **Server only, deliberately.** The force is strong, uncapped and aimed at a player's exact
/// position, so two sides holding slightly different positions send an item off in visibly
/// different directions - and the client's copy of a *remote* player is only as fresh as the
/// last packet. Predicting this cannot come out right, and the item is about to be taken and
/// despawned anyway, so the client simply watches the server's answer arrive.
pub fn attract_items_to_players(entities: &mut Entities) {
    let mut positions = Vec::new();
    for (position, _player) in entities.ecs.query_mut::<(&PositionComponent, &PlayerComponent)>() {
        positions.push((position.x() + PLAYER_WIDTH / 2, position.y() + PLAYER_HEIGHT / 2));
    }

    for player_position in positions {
        // the speed change is c * (r^2 - d^2), where r is the pickup range, d the distance
        // and c a constant, applied along the line from the item to the player
        for (item_position, item_physics, _item) in entities.ecs.query_mut::<(&PositionComponent, &mut PhysicsComponent, &ItemComponent)>() {
            let dx = player_position.0 - item_position.x() - ITEM_HALF_SIZE;
            let dy = player_position.1 - item_position.y() - ITEM_HALF_SIZE;

            // squaring a world-sized distance would saturate, so anything obviously out of
            // range is dropped before the multiply rather than after it
            if dx.abs() > PLAYER_PICKUP_RADIUS || dy.abs() > PLAYER_PICKUP_RADIUS {
                continue;
            }

            let d2 = dx * dx + dy * dy;
            let size = PLAYER_PICKUP_COEFFICIENT * (PLAYER_PICKUP_RADIUS * PLAYER_PICKUP_RADIUS - d2) + PLAYER_PICKUP_MIN_SPEED;
            let d2s = d2.sqrt();
            if size > PLAYER_PICKUP_MIN_SPEED && d2s > Fixed::ZERO {
                item_physics.velocity_x += size * dx / d2s;
                item_physics.velocity_y += size * dy / d2s;
            }
        }
    }
}

pub fn remove_all_picked_items(entities: &mut Entities, events: &mut EventManager, items: &Items) -> Result<()> {
    let mut positions = Vec::new();
    for (entity, position, _player) in entities.ecs.query_mut::<(Entity, &PositionComponent, &PlayerComponent)>() {
        positions.push(((position.x() + PLAYER_WIDTH / 2, position.y() + PLAYER_HEIGHT / 2), entity));
    }

    for (player_position, player_entity) in positions {
        let mut items_to_remove = Vec::new();
        for (entity, item_position, _item) in entities.ecs.query_mut::<(Entity, &PositionComponent, &ItemComponent)>() {
            let dx = player_position.0 - item_position.x() - ITEM_HALF_SIZE;
            let dy = player_position.1 - item_position.y() - ITEM_HALF_SIZE;

            if dx.abs() < Fixed::ONE && dy.abs() < Fixed::ONE && dx * dx + dy * dy < PICKUP_REACH_SQUARED {
                items_to_remove.push(entity);
            }
        }

        for entity in items_to_remove {
            let item_type = entities.ecs.get::<&ItemComponent>(entity)?.get_item_type();
            let mut inventory = (*entities.ecs.get::<&Inventory>(player_entity)?).clone();
            inventory.give_item(ItemStack::new(item_type, 1), player_position, items, entities, events)?;
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

    #[must_use]
    pub const fn get_input(&self) -> PlayerInput {
        PlayerInput {
            moving_type: self.moving_type,
            jumping: self.jumping,
        }
    }

    /// Puts an input in force. The one way a player's controls reach its physics, so the
    /// client applying its own and the server applying the one it was sent run the same code.
    pub fn apply_input(&mut self, input: PlayerInput, physics: &mut PhysicsComponent) {
        self.set_moving_type(input.moving_type, physics);
        self.jumping = input.jumping;
    }

    /// Puts an input back in force *without* its effect on physics, for rewinding to a
    /// remembered tick.
    ///
    /// `apply_input` is a transition - changing direction is an impulse and a change of
    /// acceleration - so replaying a run of inputs only comes out right if the component
    /// starts from what it held at the checkpoint. The velocity that goes with it is
    /// restored separately, from the same frame.
    pub const fn restore_input(&mut self, input: PlayerInput) {
        self.moving_type = input.moving_type;
        self.jumping = input.jumping;
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

/// Everything a client controls about its own player on one tick.
///
/// This is the *whole* of what a client is allowed to say about itself. It used to send a
/// position, which the server compared against its own and either adopted or overruled - but
/// a position sampled on the client is always from the past by the time the server reads it,
/// so the comparison was really measuring latency and the tolerance was a speed limit.
/// An input has no such problem: it is true whenever it arrives, and the position follows
/// from simulating it.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default, Serialize, Deserialize)]
pub struct PlayerInput {
    pub moving_type: MovingType,
    pub jumping: bool,
}

/// A client saying what it did and which tick it did it on.
///
/// Sent only when the input changes: it is a held state, so the server keeps the last one in
/// force until told otherwise, and the transport is ordered and reliable.
#[derive(Serialize, Deserialize)]
pub struct PlayerInputPacket {
    pub tick: u64,
    pub input: PlayerInput,
}

/// A client's checksum of its own player at a tick.
///
/// The two sides simulate the same inputs through the same deterministic step, so these
/// should match exactly, forever. When one does not, the tick it names is where the two
/// simulations first parted company - which turns "it sometimes rubber-bands" into something
/// that can be looked at.
#[derive(Serialize, Deserialize)]
pub struct PlayerStateHashPacket {
    pub tick: u64,
    pub hash: u64,
}

/// The same, relayed to everyone else, so their copy of that player simulates what it did.
#[derive(Serialize, Deserialize)]
pub struct PlayerInputPacketToClient {
    pub tick: u64,
    pub player_id: EntityId,
    pub input: PlayerInput,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSpawnPacket {
    pub id: EntityId,
    pub x: Fixed,
    pub y: Fixed,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RespawnPacket;

#[derive(Serialize, Deserialize)]
pub struct NamePacket {
    pub name: String,
}
