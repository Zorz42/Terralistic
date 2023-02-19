use crate::shared::blocks::Blocks;
use serde_derive::{Deserialize, Serialize};

pub const DEFAULT_GRAVITY: f32 = 9.8 * 2.0;

#[must_use]
pub fn collides_with_blocks(
    position: &PositionComponent,
    physics: &PhysicsComponent,
    blocks: &Blocks,
) -> bool {
    let block_x = position.x as i32;
    let block_y = position.y as i32;

    let block_x2 = (position.x + physics.collision_width).ceil() as i32;
    let block_y2 = (position.y + physics.collision_height).ceil() as i32;

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

pub struct Entities {
    pub ecs: hecs::World,
    current_id: u32,
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
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
            physics.velocity_x += physics.acceleration_x / 1000.0;
            physics.velocity_y += physics.acceleration_y / 1000.0;

            let target_x = position.x + physics.velocity_x / 1000.0;
            let target_y = position.y + physics.velocity_y / 1000.0;
            let direction_size = 0.01;

            let direction_x = if physics.velocity_x > 0.0 { 1.0 } else { -1.0 } * direction_size;
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
                    physics.velocity_x = 0.0;
                    break;
                }
            }

            let direction_y = if physics.velocity_y > 0.0 { 1.0 } else { -1.0 } * direction_size;
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
                    physics.velocity_y = 0.0;
                    break;
                }
            }
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
}

#[derive(Serialize, Deserialize)]
pub struct EntityPositionPacket {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

pub struct IdComponent {
    id: u32,
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
    velocity_x: f32,
    velocity_y: f32,
    acceleration_x: f32,
    acceleration_y: f32,
    collision_width: f32,
    collision_height: f32,
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

    #[must_use]
    pub const fn get_velocity_x(&self) -> f32 {
        self.velocity_x
    }

    #[must_use]
    pub const fn get_velocity_y(&self) -> f32 {
        self.velocity_y
    }

    #[must_use]
    pub const fn get_acceleration_x(&self) -> f32 {
        self.acceleration_x
    }

    #[must_use]
    pub const fn get_acceleration_y(&self) -> f32 {
        self.acceleration_y
    }

    pub fn set_velocity_x(&mut self, x: f32) {
        self.velocity_x = x;
    }

    pub fn increase_velocity_x(&mut self, x: f32) {
        self.set_velocity_x(self.get_velocity_x() + x);
    }

    pub fn set_velocity_y(&mut self, y: f32) {
        self.velocity_y = y;
    }

    pub fn increase_velocity_y(&mut self, y: f32) {
        self.set_velocity_y(self.get_velocity_y() + y);
    }

    pub fn set_acceleration_x(&mut self, x: f32) {
        self.acceleration_x = x;
    }

    pub fn increase_acceleration_x(&mut self, x: f32) {
        self.set_acceleration_x(self.get_acceleration_x() + x);
    }

    pub fn set_acceleration_y(&mut self, y: f32) {
        self.acceleration_y = y;
    }

    pub fn increase_acceleration_y(&mut self, y: f32) {
        self.set_acceleration_y(self.get_acceleration_y() + y);
    }

    #[must_use]
    pub const fn get_collision_width(&self) -> f32 {
        self.collision_width
    }

    #[must_use]
    pub const fn get_collision_height(&self) -> f32 {
        self.collision_height
    }
}
