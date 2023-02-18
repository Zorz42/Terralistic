use crate::shared::blocks::{Blocks, BLOCK_WIDTH};

pub struct IdComponent {
    pub id: u32,
}

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

pub struct VelocityCollisionComponent {
    pub x: f32,
    pub y: f32,
    pub collision_width: f32,
    pub collision_height: f32,
}

#[must_use]
pub fn collides_with_blocks(
    position: &PositionComponent,
    velocity: &VelocityCollisionComponent,
    blocks: &Blocks,
) -> bool {
    let target_x = position.x + velocity.x;
    let target_y = position.y + velocity.y;

    let collision_width = velocity.collision_width;
    let collision_height = velocity.collision_height;

    let block_x = target_x / BLOCK_WIDTH as f32;
    let block_y = target_y / BLOCK_WIDTH as f32;

    let block_x2 = (target_x + collision_width) / BLOCK_WIDTH as f32;
    let block_y2 = (target_y + collision_height) / BLOCK_WIDTH as f32;

    let block_x = block_x.floor() as i32;
    let block_y = block_y.floor() as i32;

    let block_x2 = block_x2.floor() as i32;
    let block_y2 = block_y2.floor() as i32;

    for x in block_x..=block_x2 {
        for y in block_y..=block_y2 {
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
            .query_mut::<(&mut PositionComponent, &VelocityCollisionComponent)>()
        {
            let target_x = position.x + velocity.x / 1000.0;
            let target_y = position.y + velocity.y / 1000.0;

            let direction_x = if velocity.x > 0.0 { 1 } else { -1 };
            let mut collided_x = true;
            while !collides_with_blocks(position, velocity, blocks) {
                if (direction_x == 1 && position.x >= target_x)
                    || (direction_x == -1 && position.x <= target_x)
                {
                    collided_x = false;
                    break;
                }

                position.x += direction_x as f32;
            }
            if !collided_x {
                position.x = target_x;
            }

            let direction_y = if velocity.y > 0.0 { 1 } else { -1 };
            let mut collided_y = true;
            while !collides_with_blocks(position, velocity, blocks) {
                if (direction_y == 1 && position.y >= target_y)
                    || (direction_y == -1 && position.y <= target_y)
                {
                    collided_y = false;
                    break;
                }

                position.y += direction_y as f32;
            }
            if !collided_y {
                position.y = target_y;
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
