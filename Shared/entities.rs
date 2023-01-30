use std::sync::{Mutex};
use once_cell::sync::Lazy;
use crate::blocks::{BLOCK_WIDTH, Blocks};

#[derive(PartialEq)]
pub enum EntityType { ITEM, PLAYER }//TODO: remove this since they're now in separate objects?
#[derive(PartialEq, Copy, Clone)]
pub enum Direction { LEFT, RIGHT, UP, DOWN }
static CURR_ENTITY_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

pub trait EntityObject {
    /**return the width of the entity*/
    fn get_width(&self) -> i32;
    /**return the height of the entity*/
    fn get_height(&self) -> i32;
    /**returns whether or not the entity is colliding*/
    fn is_colliding(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool;
    /**returns whether or not the entity is colliding with a block*/
    fn is_colliding_with_block(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool;
    /**updates the entity*/
    fn update_entity(&mut self, blocks: &Blocks);
    /** returns whether or not the entity is touching ground*/
    fn is_touching_ground(&self, blocks: &Blocks) -> bool;
    /**returns the entity's x position*/
    fn get_x(&self) -> f64;
    /**returns the entity's y position*/
    fn get_y(&self) -> f64;
    /**returns the entity's x velocity*/
    fn get_velocity_x(&self) -> f64;
    /**returns the entity's y velocity*/
    fn get_velocity_y(&self) -> f64;
}

pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
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
            x: x as f64,
            y: y as f64,
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
            entity.id = *CURR_ENTITY_ID.lock().unwrap();
            *CURR_ENTITY_ID.lock().unwrap() += 1;
        }
        entity
    }
}

impl EntityObject for Entity{
    fn get_width(&self) -> i32 {
        0
    }
    fn get_height(&self) -> i32 {
        0
    }
    fn is_colliding(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool {
        self.is_colliding_with_block(blocks, direction, colliding_x, colliding_y)
    }
    fn is_colliding_with_block(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool {
        if colliding_x < 0.0 || colliding_y < 0.0 ||
            colliding_y >= (blocks.get_height() * BLOCK_WIDTH * 2 - self.get_height())  as f64 ||
            colliding_x >= (blocks.get_width() * BLOCK_WIDTH * 2 - self.get_width()) as f64 {
            return true
        }
        let starting_x = (colliding_x / (BLOCK_WIDTH * 2) as f64) as i32;
        let starting_y = (colliding_y / (BLOCK_WIDTH * 2) as f64) as i32;
        let ending_x = ((colliding_x + self.get_width() as f64 - 1.0) / (BLOCK_WIDTH * 2) as f64) as i32;
        let ending_y = ((colliding_y + self.get_height() as f64 - 1.0) / (BLOCK_WIDTH * 2) as f64) as i32;

        for x in starting_x..ending_x + 1 {
            for y in starting_y..ending_y + 1 {
                if !blocks.get_block_type_at(x, y).ghost && !blocks.get_block_type_at(x, y).feet_collidable {
                    return true
                }
            }
        }
        false
    }
    fn update_entity(&mut self, blocks: &Blocks) {
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
    fn is_touching_ground(&self, blocks: &Blocks) -> bool {
        self.is_colliding(blocks, Direction::DOWN, self.x, self.y + 1.0) && self.velocity_y == 0.0
    }
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_velocity_x(&self) -> f64 {
        self.velocity_x
    }
    fn get_velocity_y(&self) -> f64 {
        self.velocity_y
    }
}

pub struct EntityVelocityChangeEvent {
    pub entity_id: u32,
}
impl EntityVelocityChangeEvent{
    pub fn new(entity_id: u32) -> Self{
        EntityVelocityChangeEvent{ entity_id }
    }
}

pub struct EntityAbsoluteVelocityChangeEvent {
    pub entity_id: u32,
    pub old_vel_x: f64,
    pub old_vel_y: f64,
}
impl EntityAbsoluteVelocityChangeEvent{
    pub fn new(entity_id: u32, old_vel_x: f64, old_vel_y: f64) -> Self{
        EntityAbsoluteVelocityChangeEvent{
            entity_id,
            old_vel_x,
            old_vel_y
        }
    }
}
pub struct EntityPositionChangeEvent {
    pub entity_id: u32,
}
impl EntityPositionChangeEvent{
    pub fn new(entity_id: u32) -> Self{
        EntityPositionChangeEvent{
            entity_id
        }
    }
}
pub struct EntityDeletionEvent {
    pub entity_id: u32,
}
impl EntityDeletionEvent{
    pub fn new(entity_id: u32) -> Self{
        EntityDeletionEvent{
            entity_id
        }
    }
}

//impl Event for EntityVelocityChangeEvent {}
//impl Event for EntityAbsoluteVelocityChangeEvent {}
//impl Event for EntityPositionChangeEvent {}
//impl Event for EntityDeletionEvent {}

pub trait EntityStructTrait<EntityType: EntityObject> {
    /**updates all entities in that struct*/
    fn update_all_entities(&mut self, blocks: &Blocks);
    /**registers a new entity in that struct*/
    fn register_entity(&mut self, entity: EntityType);
    /**removes an entity in that struct*/
    fn remove_entity(&mut self, entity_id: u32);
    /**returns entity from that struct by id*/
    fn get_entity_by_id(&self, entity_id: u32) -> Option<&EntityType>;
    /**returns mutable entity from that struct by id*/
    fn get_entity_by_id_mut(&self, entity_id: u32) -> Option<&EntityType>;
    /**returns entity vector from that struct*/
    fn get_entities(&self) -> &Vec<EntityType>;
    /**sets the x velocity of an entity from that struct*/
    fn set_velocity_x(&mut self, entity: &mut EntityType, velocity_x: f64);
    /**sets the y velocity of an entity from that struct*/
    fn set_velocity_y(&mut self, entity: &mut EntityType, velocity_y: f64);
    /**adds to the x velocity of an entity from that struct*/
    fn add_velocity_x(&mut self, entity: &mut EntityType, velocity_x: f64);
    /**adds to the y velocity of an entity from that struct*/
    fn add_velocity_y(&mut self, entity: &mut EntityType, velocity_y: f64);
    /**sets the x position of an entity from that struct*/
    fn set_x(&mut self, entity: &mut EntityType, x: f64, send_to_everyone: bool);
    /**sets the y position of an entity from that struct*/
    fn set_y(&mut self, entity: &mut EntityType, y: f64, send_to_everyone: bool);
}