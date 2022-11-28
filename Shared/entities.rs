use super::blocks::*;

enum EntityType { ITEM, PLAYER }
enum Direction { LEFT, RIGHT, UP, DOWN }
static mut CURR_ENTITY_ID: u32 = 1;

struct Entity {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    pub gravity: bool,
    pub friction: bool,
    pub has_moved_x: bool,
    pub ignore_server_updates: bool,
    pub has_moved: bool,
    pub id: u32,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(entity_type: EntityType, x: i32, y: i32, id: u32) -> Entity {
        let mut entity = Entity {
            x: x as f32,
            y: y as f32,
            velocity_x: 0.0,
            velocity_y: 0.0,
            gravity: true,
            friction: true,
            has_moved_x: false,
            ignore_server_updates: false,
            has_moved: false,
            id,
            entity_type,
        };
        if entity.id == 0 {
            entity.id = unsafe { CURR_ENTITY_ID };
            unsafe { CURR_ENTITY_ID += 1 };
        }
        entity
    }

    pub fn get_width(&self) -> i32 {
        0
    }
    pub fn get_height(&self) -> i32 {
        0
    }

    pub fn is_colliding(&self, blocks: &mut Blocks, direction: Direction, colliding_x: f32, colliding_y: f32) -> bool {
        self.is_colliding_with_block(blocks, direction, colliding_x, colliding_y)
    }
    pub fn is_colliding_with_block(&self, blocks: &mut Blocks, direction: Direction, colliding_x: f32, colliding_y: f32) -> bool {
        if colliding_x < 0.0 || colliding_y < 0.0 ||
            colliding_y >= (blocks.get_height() * BLOCK_WIDTH * 2 - self.get_height())  as f32 ||
            colliding_x >= (blocks.get_width() * BLOCK_WIDTH * 2 - self.get_width()) as f32 {
            return true
        }
        let starting_x = (colliding_x / (BLOCK_WIDTH * 2) as f32) as i32;
        let starting_y = (colliding_y / (BLOCK_WIDTH * 2) as f32) as i32;
        let ending_x = ((colliding_x + self.get_width() as f32 - 1.0) / (BLOCK_WIDTH * 2) as f32) as i32;
        let ending_y = ((colliding_y + self.get_height() as f32 - 1.0) / (BLOCK_WIDTH * 2) as f32) as i32;

        for x in starting_x..ending_x + 1 {
            for y in starting_y..ending_y + 1 {
                if !blocks.get_block_type(x, y).ghost && !blocks.get_block_type(x, y).feet_collidable {
                    return true
                }
            }
        }
        false
    }

    pub fn update_entity(&mut self, blocks: &mut Blocks) {
        if self.friction {
            self.velocity_y *= 0.995;
            self.velocity_x *= if self.is_touching_ground(blocks) { 0.99 } else { 0.9995 };
        }

        if self.is_touching_ground(blocks) {
            self.velocity_y = 0.0;
        }else {
            self.velocity_y += 0.2;
        }

        let prev_y = self.y;
        let y_to_be = self.y + self.velocity_y / 100.0;
        let move_y = y_to_be - self.y;
        let y_factor = if move_y > 0.0 { 1.0 } else { -1.0 };

        for _ in 0..(move_y.abs() as i32) {
            self.y += y_factor;
            if self.is_colliding(blocks, if y_factor == 1.0 { Direction::DOWN } else { Direction::UP }, self.x, self.y) {
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
            if self.is_colliding(blocks, if x_factor == 1.0 { Direction::RIGHT } else { Direction::LEFT }, self.x, self.y) {
                self.x -= x_factor;
                has_collided_x = true;
                break;
            }
        }
        if !has_collided_x {
            self.x = x_to_be;
        }
        self.has_moved_x = prev_x != self.x;
        self.has_moved = self.has_moved_x || prev_y != self.y;
    }

    pub fn is_touching_ground(&self, blocks: &mut Blocks) -> bool {
        self.is_colliding(blocks, Direction::DOWN, self.x, self.y + 1.0) && self.velocity_y == 0.0
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_velocity_x(&self) -> f32 {
        self.velocity_x
    }
    pub fn get_velocity_y(&self) -> f32 {
        self.velocity_y
    }
}
