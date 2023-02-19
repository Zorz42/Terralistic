use crate::shared::blocks::Blocks;
use serde_derive::{Deserialize, Serialize};

pub const DEFAULT_GRAVITY: f32 = 9.8 * 2.0;

#[must_use]
pub fn collides_with_blocks(
    position: &PositionComponent,
    velocity: &VelocityCollisionComponent,
    blocks: &Blocks,
) -> bool {
    let block_x = position.x as i32;
    let block_y = position.y as i32;

    let block_x2 = (position.x + velocity.collision_width).ceil() as i32;
    let block_y2 = (position.y + velocity.collision_height).ceil() as i32;

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
        for (_entity, (position, velocity)) in self
            .ecs
            .query_mut::<(&mut PositionComponent, &mut VelocityCollisionComponent)>()
        {
            let target_x = position.x + velocity.x / 1000.0;
            let target_y = position.y + velocity.y / 1000.0;
            let direction_size = 0.01;

            let direction_x = if velocity.x > 0.0 { 1.0 } else { -1.0 } * direction_size;
            loop {
                if (direction_x > 0.0 && position.x > target_x + direction_x)
                    || (direction_x < 0.0 && position.x < target_x + direction_x)
                {
                    position.x = target_x;
                    break;
                }

                position.x += direction_x;

                if collides_with_blocks(position, velocity, blocks) {
                    position.x -= direction_x;
                    velocity.x = 0.0;
                    break;
                }
            }

            let direction_y = if velocity.y > 0.0 { 1.0 } else { -1.0 } * direction_size;
            loop {
                if (direction_y > 0.0 && position.y > target_y + direction_y)
                    || (direction_y < 0.0 && position.y < target_y + direction_y)
                {
                    position.y = target_y;
                    break;
                }

                position.y += direction_y;

                if collides_with_blocks(position, velocity, blocks) {
                    position.y -= direction_y;
                    velocity.y = 0.0;
                    break;
                }
            }
        }

        for (_entity, (velocity, gravity)) in self
            .ecs
            .query_mut::<(&mut VelocityCollisionComponent, &GravityComponent)>()
        {
            velocity.y += gravity.gravity / 1000.0;
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

pub struct VelocityCollisionComponent {
    x: f32,
    y: f32,
    collision_width: f32,
    collision_height: f32,
}

impl VelocityCollisionComponent {
    #[must_use]
    pub const fn new(collision_width: f32, collision_height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            collision_width,
            collision_height,
        }
    }

    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    pub fn increase_x(&mut self, x: f32) {
        self.x += x;
    }

    pub fn increase_y(&mut self, y: f32) {
        self.y += y;
    }

    #[must_use]
    pub const fn collision_width(&self) -> f32 {
        self.collision_width
    }

    #[must_use]
    pub const fn collision_height(&self) -> f32 {
        self.collision_height
    }
}

pub struct GravityComponent {
    pub gravity: f32,
}

impl GravityComponent {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gravity: DEFAULT_GRAVITY,
        }
    }
}

impl Default for GravityComponent {
    fn default() -> Self {
        Self::new()
    }
}
