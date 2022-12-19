use super::blocks::*;
use events::*;
use shared_mut::*;

#[derive(PartialEq)]
pub enum EntityType { ITEM, PLAYER }
#[derive(PartialEq)]
pub enum Direction { LEFT, RIGHT, UP, DOWN }
static mut CURR_ENTITY_ID: u32 = 1;

pub struct Entity {
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
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
            entity.id = unsafe { CURR_ENTITY_ID };
            unsafe { CURR_ENTITY_ID += 1 };
        }
        entity
    }
}

pub trait entity_object{
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
    fn is_colliding(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool;
    fn is_colliding_with_block(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool;
    fn update_entity(&mut self, blocks: &Blocks);
    fn is_touching_ground(&self, blocks: &Blocks) -> bool;
    fn get_x(&self) -> f64;
    fn get_y(&self) -> f64;
    fn get_velocity_x(&self) -> f64;
    fn get_velocity_y(&self) -> f64;
    }

impl entity_object for Entity{
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
                if !blocks.get_block_type(x, y).ghost && !blocks.get_block_type(x, y).feet_collidable {
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
    pub old_vel_x: f64, old_vel_y: f64
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

impl Event for EntityVelocityChangeEvent {}
impl Event for EntityAbsoluteVelocityChangeEvent {}
impl Event for EntityPositionChangeEvent {}
impl Event for EntityDeletionEvent {}

pub struct Entities{
    entities: Vec<Entity>,
    blocks: SharedMut<Blocks>,
    pub entity_position_change_event: Sender<EntityPositionChangeEvent>,
    pub entity_velocity_change_event: Sender<EntityVelocityChangeEvent>,
    pub entity_absolute_velocity_change_event: Sender<EntityAbsoluteVelocityChangeEvent>,
    pub entity_deletion_event: Sender<EntityDeletionEvent>
}

impl Entities{
    pub fn new(blocks: SharedMut<Blocks>) -> Self {
        Entities{
            entities: Vec::new(),
            blocks,
            entity_position_change_event: Sender::new(),
            entity_velocity_change_event: Sender::new(),
            entity_absolute_velocity_change_event: Sender::new(),
            entity_deletion_event: Sender::new()
        }
    }

    pub fn update_all_entities(&mut self) {
        for entity in &mut self.entities {
            let old_vel_x = entity.get_velocity_x();
            let old_vel_y = entity.get_velocity_y();
            entity.update_entity(&self.blocks.get());
            if entity.entity_type == EntityType::PLAYER && old_vel_x != entity.get_velocity_x() || old_vel_y != entity.get_velocity_y(){
                let event = EntityAbsoluteVelocityChangeEvent::new(entity.id, old_vel_x, old_vel_y);
                self.entity_absolute_velocity_change_event.send(event);
            }
        }
    }

    pub fn register_entity(&mut self, entity: Entity){
        self.entities.push(entity);
    }

    pub fn remove_entity(&mut self, entity_id: u32){
        let pos = self.entities.iter().position(|entity| entity.id == entity_id).expect("Removed non existing entity");
        let event = EntityDeletionEvent::new(entity_id);
        self.entity_deletion_event.send(event);
        self.entities.remove(pos);
    }

    pub fn get_entity_by_id(&self, entity_id: u32) -> Option<&Entity>{
        self.entities.iter().find(|entity| entity.id == entity_id)
    }

    pub fn get_entity_by_id_mut(&self, entity_id: u32) -> Option<&Entity>{
        self.entities.iter().find(|entity| entity.id == entity_id)
    }

    pub fn get_entities(&self) -> &Vec<Entity>{
        &self.entities
    }

    pub fn set_velocity_x(&mut self, entity: &mut Entity, velocity_x: f64){
        if entity.velocity_x != velocity_x {
            entity.velocity_x = velocity_x;
            let event = EntityVelocityChangeEvent::new(entity.id);
            self.entity_velocity_change_event.send(event);
        }
    }

    pub fn set_velocity_y(&mut self, entity: &mut Entity, velocity_y: f64){
        if entity.velocity_y != velocity_y {
            entity.velocity_y = velocity_y;
            let event = EntityVelocityChangeEvent::new(entity.id);
            self.entity_velocity_change_event.send(event);
        }
    }

    pub fn add_velocity_x(&mut self, entity: &mut Entity, velocity_x: f64){
        self.set_velocity_x(entity, entity.velocity_x + velocity_x);
    }

    pub fn add_velocity_y(&mut self, entity: &mut Entity, velocity_y: f64){
        self.set_velocity_y(entity, entity.velocity_y + velocity_y);
    }

    pub fn set_x(&mut self, entity: &mut Entity, x: f64, send_to_everyone: bool){
        if entity.x != x {
            entity.x = x;
            if send_to_everyone{
                let event = EntityPositionChangeEvent::new(entity.id);
                self.entity_position_change_event.send(event);
            }
        }
    }

    pub fn set_y(&mut self, entity: &mut Entity, y: f64, send_to_everyone: bool){
        if entity.y != y {
            entity.y = y;
            if send_to_everyone{
                let event = EntityPositionChangeEvent::new(entity.id);
                self.entity_position_change_event.send(event);
            }
        }
    }
}