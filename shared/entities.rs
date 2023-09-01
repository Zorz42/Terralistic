use crate::libraries::events::{Event, EventManager};
use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};

use crate::shared::blocks::Blocks;

pub const DEFAULT_GRAVITY: f32 = 80.0;
pub const FRICTION_COEFFICIENT: f32 = 0.2;
pub const AIR_RESISTANCE_COEFFICIENT: f32 = 0.005;
const DIRECTION_SIZE: f32 = 0.01;

#[must_use]
pub fn collides_with_blocks(
    position: &PositionComponent,
    physics: &PhysicsComponent,
    blocks: &Blocks,
) -> bool {
    let block_x = position.x as i32;
    let block_y = position.y as i32;

    let block_x2 = (position.x + physics.collision_width - 2.0 * DIRECTION_SIZE).ceil() as i32;
    let block_y2 = (position.y + physics.collision_height - 2.0 * DIRECTION_SIZE).ceil() as i32;

    for x in block_x..block_x2 {
        for y in block_y..block_y2 {
            let block = blocks.get_block_type_at(x, y);
            if let Ok(block) = block {
                if !block.ghost {
                    return true;
                }
            }
        }
    }

    false
}

#[must_use]
pub fn is_touching_ground(
    position: &PositionComponent,
    physics: &PhysicsComponent,
    blocks: &Blocks,
) -> bool {
    collides_with_blocks(
        &PositionComponent {
            x: position.x,
            y: position.y + DIRECTION_SIZE * 2.0,
        },
        physics,
        blocks,
    ) && physics.velocity_y.abs() <= 0.01
}

pub struct Entities {
    pub ecs: hecs::World,
    current_id: u32,
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}

/// Reduce a by b, but never go below 0. if a is negative, increase it by b but never go above 0.
pub fn reduce_by(a: &mut f32, b: f32) {
    let b = b.abs();
    if *a > 0.0 {
        *a -= b;
        if *a < 0.0 {
            *a = 0.0;
        }
    } else {
        *a += b;
        if *a > 0.0 {
            *a = 0.0;
        }
    }
}

impl Entities {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ecs: hecs::World::new(),
            current_id: 0,
        }
    }

    pub fn update_entities_ms(&mut self, blocks: &Blocks) {
        for (_entity, (position, physics)) in self
            .ecs
            .query_mut::<(&mut PositionComponent, &mut PhysicsComponent)>()
        {
            physics.velocity_x += physics.acceleration_x / 200.0;
            physics.velocity_y += physics.acceleration_y / 200.0;

            let target_x = position.x + physics.velocity_x / 200.0;
            let target_y = position.y + physics.velocity_y / 200.0;

            let direction_x = if physics.velocity_x > 0.0 { 1.0 } else { -1.0 } * DIRECTION_SIZE;
            loop {
                if (direction_x > 0.0 && position.x > target_x + direction_x)
                    || (direction_x < 0.0 && position.x < target_x + direction_x)
                {
                    position.x = target_x;
                    break;
                }

                position.x += direction_x;

                if collides_with_blocks(position, physics, blocks) {
                    position.x -= direction_x;
                    reduce_by(
                        &mut physics.velocity_y,
                        physics.velocity_x * FRICTION_COEFFICIENT,
                    );
                    physics.velocity_x = 0.0;
                    break;
                }
            }

            let direction_y = if physics.velocity_y > 0.0 { 1.0 } else { -1.0 } * DIRECTION_SIZE;
            loop {
                if (direction_y > 0.0 && position.y > target_y + direction_y)
                    || (direction_y < 0.0 && position.y < target_y + direction_y)
                {
                    position.y = target_y;
                    break;
                }

                position.y += direction_y;

                if collides_with_blocks(position, physics, blocks) {
                    position.y -= direction_y;
                    reduce_by(
                        &mut physics.velocity_x,
                        physics.velocity_y * FRICTION_COEFFICIENT,
                    );
                    physics.velocity_y = 0.0;
                    break;
                }
            }

            physics.velocity_x *= 1.0 - AIR_RESISTANCE_COEFFICIENT;
            physics.velocity_y *= 1.0 - AIR_RESISTANCE_COEFFICIENT;
        }
    }

    pub fn unwrap_id(&mut self, id: Option<u32>) -> u32 {
        if let Some(id) = id {
            id
        } else {
            self.current_id += 1;
            self.current_id
        }
    }

    /// # Errors
    /// If the entity could not be despawned
    pub fn despawn_entity(&mut self, id: u32, events: &mut EventManager) -> Result<()> {
        let mut entity_to_despawn = None;

        for (entity, id_component) in &mut self.ecs.query::<&IdComponent>() {
            if id_component.id() == id {
                entity_to_despawn = Some(entity);
                break;
            }
        }

        if let Some(entity) = entity_to_despawn {
            self.ecs.despawn(entity)?;
        } else {
            bail!("Could not find entity with id");
        }

        events.push_event(Event::new(EntityDespawnEvent { id }));

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct EntityPositionVelocityPacket {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDespawnPacket {
    pub id: u32,
}

pub struct IdComponent {
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDespawnEvent {
    pub id: u32,
}

impl IdComponent {
    #[must_use]
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionComponent {
    x: f32,
    y: f32,
}

impl PositionComponent {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

pub struct PhysicsComponent {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub collision_width: f32,
    pub collision_height: f32,
}

impl PhysicsComponent {
    #[must_use]
    pub const fn new(collision_width: f32, collision_height: f32) -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: DEFAULT_GRAVITY,
            collision_width,
            collision_height,
        }
    }
}
