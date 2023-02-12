use crate::shared::blocks::{Blocks, BLOCK_WIDTH};
use core::sync::atomic::{AtomicU32, Ordering};
use once_cell::sync::Lazy;

static CURR_ENTITY_ID: Lazy<AtomicU32> = Lazy::new(|| AtomicU32::new(0));

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub gravity: bool,
    pub friction: bool,
    pub has_moved: bool,
    pub has_moved_x: bool,
    pub id: u32,
}

impl Entity {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32, id: Option<u32>) -> Self {
        Self {
            x,
            y,
            width,
            height,
            velocity_x: 0.0,
            velocity_y: 0.0,
            gravity: true,
            friction: true,
            has_moved: false,
            has_moved_x: false,
            id: id.unwrap_or_else(|| {
                CURR_ENTITY_ID.fetch_add(1, Ordering::Relaxed);
                CURR_ENTITY_ID.load(Ordering::Relaxed)
            }),
        }
    }

    #[must_use]
    pub fn is_colliding(&self, blocks: &Blocks) -> bool {
        self.is_colliding_at(blocks, self.x, self.y)
    }

    #[must_use]
    pub fn is_colliding_at(&self, blocks: &Blocks, colliding_x: f32, colliding_y: f32) -> bool {
        if colliding_x < 0.0
            || colliding_y < 0.0
            || colliding_y >= (blocks.get_height() as i32 * BLOCK_WIDTH * 2) as f32 - self.height
            || colliding_x >= (blocks.get_width() as i32 * BLOCK_WIDTH * 2) as f32 - self.width
        {
            return true;
        }
        let starting_x = (colliding_x / (BLOCK_WIDTH * 2) as f32) as i32;
        let starting_y = (colliding_y / (BLOCK_WIDTH * 2) as f32) as i32;
        let ending_x = ((colliding_x + self.width - 1.0) / (BLOCK_WIDTH * 2) as f32) as i32;
        let ending_y = ((colliding_y + self.width - 1.0) / (BLOCK_WIDTH * 2) as f32) as i32;

        for x in starting_x..=ending_x {
            for y in starting_y..=ending_y {
                let block_type = blocks.get_block_type_at(x, y);
                if let Ok(block_type) = block_type {
                    if !block_type.ghost && !block_type.feet_collidable {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[must_use]
    pub fn is_touching_ground(&self, blocks: &Blocks) -> bool {
        self.is_colliding_at(blocks, self.x, self.y + 1.0) && self.velocity_y == 0.0
    }

    pub fn update(&mut self, blocks: &Blocks) {
        if self.friction {
            self.velocity_y *= 0.995;
            self.velocity_x *= if self.is_touching_ground(blocks) {
                0.99
            } else {
                0.9995
            };
        }

        if self.is_touching_ground(blocks) {
            self.velocity_y = 0.0;
        } else {
            self.velocity_y += 0.2;
        }

        let prev_y = self.y;
        let y_to_be = self.y + self.velocity_y / 100.0;
        let move_y = y_to_be - self.y;
        let y_factor = if move_y > 0.0 { 1.0 } else { -1.0 };

        for _ in 0..(move_y.abs() as i32) {
            self.y += y_factor;
            if self.is_colliding(blocks) {
                self.y -= y_factor;
                self.velocity_y = 0.0;
                break;
            }
        }
        if self.velocity_y != 0.0 {
            self.y = y_to_be;
        }

        let prev_x = self.x;
        let x_to_be = self.x + self.velocity_x / 100.0;
        let move_x = x_to_be - self.x;
        let x_factor = if move_x > 0.0 { 1.0 } else { -1.0 };
        let mut has_collided_x = false;
        for _ in 0..(move_x.abs() as i32) {
            self.x += x_factor;
            if self.is_colliding(blocks) {
                self.x -= x_factor;
                has_collided_x = true;
                break;
            }
        }
        if !has_collided_x {
            self.x = x_to_be;
        }
        self.has_moved_x = (prev_x - self.x).abs() > f32::EPSILON;
        self.has_moved = self.has_moved_x || (prev_y - self.y).abs() > f32::EPSILON;
    }
}

pub struct EntitySpawnEvent {
    pub entity_id: u32,
}

pub struct EntityVelocityChangeEvent {
    pub entity_id: u32,
}
