/*use super::{
    blocks::Block,
    entities::{
        Direction, Entity, EntityDeletionEvent, EntityObject, EntityPositionChangeEvent,
        EntityStructTrait, EntityType, EntityVelocityChangeEvent,
    },
    walls,
};
use crate::shared::blocks::Tool;
use crate::shared::blocks::{BlockId, Blocks};
use crate::shared::walls::WallId;
use std::collections::HashMap;

use std::rc::Rc;

const ITEM_WIDTH: i32 = 8;

pub struct ItemType {
    pub name: String,
    pub display_name: String,
    pub max_stack: i32,
    pub places_block: Option<Rc<Block>>,
    pub places_wall: Option<Rc<walls::WallId>>,
    pub tool_powers: HashMap<Rc<Tool>, i32>,
    id: i32,
}

impl ItemType {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: String::new(),
            max_stack: 0,
            places_block: None,
            places_wall: None,
            tool_powers: HashMap::new(),
            id: 0,
        }
    }
    #[must_use]
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub struct Item {
    item_type: Rc<ItemType>,
    pub entity: Entity,
    pub entity_item_count: u32,
}

impl Item {
    /**creates a new item*/
    #[must_use]
    pub fn new(item_type: Rc<ItemType>, x: i32, y: i32, entity_item_count: u32, id: u32) -> Self {
        Self {
            item_type,
            entity_item_count,
            entity: Entity::new(EntityType::ITEM, x, y, id),
        }
    }
    /**returns item type*/
    #[must_use]
    pub fn get_type(&self) -> &ItemType {
        &self.item_type
    }
}

impl EntityObject for Item {
    fn get_width(&self) -> i32 {
        ITEM_WIDTH * 2
    }
    fn get_height(&self) -> i32 {
        ITEM_WIDTH * 2
    }
    fn is_colliding(
        &self,
        blocks: &Blocks,
        direction: Direction,
        colliding_x: f32,
        colliding_y: f32,
    ) -> bool {
        self.entity
            .is_colliding(blocks, direction, colliding_x, colliding_y)
    }
    fn is_colliding_with_block(
        &self,
        blocks: &Blocks,
        direction: Direction,
        colliding_x: f32,
        colliding_y: f32,
    ) -> bool {
        self.entity
            .is_colliding_with_block(blocks, direction, colliding_x, colliding_y)
    }
    fn update_entity(&mut self, blocks: &Blocks) {
        self.entity.update_entity(blocks);
    }
    fn is_touching_ground(&self, blocks: &Blocks) -> bool {
        self.entity.is_touching_ground(blocks)
    }
    fn get_x(&self) -> f32 {
        self.entity.get_x()
    }
    fn get_y(&self) -> f32 {
        self.entity.get_y()
    }
    fn get_velocity_x(&self) -> f32 {
        self.entity.get_velocity_x()
    }
    fn get_velocity_y(&self) -> f32 {
        self.entity.get_velocity_y()
    }
}

#[derive(Clone)]
pub struct ItemStack {
    pub item_type: Rc<ItemType>,
    pub stack: i32,
}

impl ItemStack {
    #[must_use]
    pub fn new(item_type: Rc<ItemType>, stack: i32) -> Self {
        Self { item_type, stack }
    }
}

pub struct ItemCreationEvent {
    pub item_id: u32,
}

pub struct TileDrop {
    pub drop: Rc<ItemType>,
    pub chance: f32,
}

impl TileDrop {
    #[must_use]
    pub fn new(drop: Rc<ItemType>, chance: f32) -> Self {
        Self { drop, chance }
    }
}

pub struct Items {
    items: Vec<Item>,
    item_types: Vec<Rc<ItemType>>,
    block_drops: HashMap<BlockId, TileDrop>,
    wall_drops: HashMap<WallId, TileDrop>,

    pub nothing: Rc<ItemType>,
    //item_creation_event: Sender<ItemCreationEvent>,
    //item_position_change_event: Sender<EntityPositionChangeEvent>,
    //item_velocity_change_event: Sender<EntityVelocityChangeEvent>,
    //item_deletion_event: Sender<EntityDeletionEvent>
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}

impl Items {
    #[must_use]
    pub fn new() -> Self {
        let nothing = Rc::new(ItemType::new("nothing".to_string()));

        let item_types = vec![nothing.clone()];

        Self {
            items: Vec::new(),
            item_types,
            block_drops: HashMap::new(),
            wall_drops: HashMap::new(),
            nothing,
            //item_creation_event: Sender::new(),
            //item_position_change_event: Sender::new(),
            //item_velocity_change_event: Sender::new(),
            //item_deletion_event: Sender::new()
        }
    }

    /**this function spawns an item into the world*/
    pub fn spawn_item(
        &mut self,
        item_type: Rc<ItemType>,
        x: i32,
        y: i32,
        entity_item_count: u32,
    ) -> u32 {
        let item = Item::new(item_type, x, y, entity_item_count, 0);
        self.register_entity(item);
        let item = self.items.last().unwrap();
        //self.item_creation_event.send(ItemCreationEvent::new(item.entity.id));
        item.entity.id
    }
    /**this function registers an item type*/
    pub fn register_new_item_type(&mut self, mut item_type: ItemType) {
        item_type.id = self.item_types.len() as i32;
        self.item_types.push(Rc::from(item_type));
    }
    /**this function returns the item type with the given id*/
    #[must_use]
    pub fn get_item_type(&self, id: i32) -> Rc<ItemType> {
        assert!(
            !(id < 0 || id >= self.item_types.len() as i32),
            "Item type id does not exist."
        );
        self.item_types[id as usize].clone()
    }
    /**this function returns the item type with the given name*/
    #[must_use]
    pub fn get_item_type_by_name(&self, name: &str) -> Option<Rc<ItemType>> {
        for item_type in &self.item_types {
            if item_type.name == name {
                return Option::Some(item_type.clone());
            }
        }
        Option::None
    }
    /**this function returns the number of item types*/
    #[must_use]
    pub fn get_num_item_types(&self) -> usize {
        self.item_types.len()
    }
    /**this function sets the block drop for the given block type*/
    pub fn set_block_drop(&mut self, block_type: Rc<Block>, drop: TileDrop) {
        self.block_drops.insert(block_type.get_id(), drop);
    }
    /**this function returns the block drop for the given block type*/
    #[must_use]
    pub fn get_block_drop(&self, block_type: Rc<Block>) -> Option<&TileDrop> {
        self.block_drops.get(&block_type.get_id())
    }
    /**this function sets the wall drop for the given wall type*/
    pub fn set_wall_drop(&mut self, wall_type: walls::WallId, drop: TileDrop) {
        self.wall_drops.insert(wall_type, drop);
    }
    /**this function returns the wall drop for the given wall type*/
    #[must_use]
    pub fn get_wall_drop(&self, wall_type: walls::WallId) -> Option<&TileDrop> {
        self.wall_drops.get(&wall_type)
    }
}

impl EntityStructTrait<Item> for Items {
    fn update_all_entities(&mut self, blocks: &Blocks) {
        for entity in &mut self.items {
            let _old_vel_x = entity.get_velocity_x();
            let _old_vel_y = entity.get_velocity_y();
            entity.update_entity(blocks);
        }
    }
    fn register_entity(&mut self, entity: Item) {
        self.items.push(entity);
    }
    fn remove_entity(&mut self, entity_id: u32) {
        let pos = self
            .items
            .iter()
            .position(|entity| entity.entity.id == entity_id);
        if pos.is_none() {
            return;
        }
        let _event = EntityDeletionEvent::new(entity_id);
        //self.item_deletion_event.send(event);
        self.items.remove(pos.unwrap());
    }
    fn get_entity_by_id(&self, entity_id: u32) -> Option<&Item> {
        self.items
            .iter()
            .find(|entity| entity.entity.id == entity_id)
    }
    fn get_entity_by_id_mut(&self, entity_id: u32) -> Option<&Item> {
        self.items
            .iter()
            .find(|entity| entity.entity.id == entity_id)
    }
    fn get_entities(&self) -> &Vec<Item> {
        &self.items
    }
    fn set_velocity_x(&mut self, entity: &mut Item, velocity_x: f32) {
        if entity.entity.get_velocity_x() != velocity_x {
            entity.entity.velocity_x = velocity_x;
            let _event = EntityVelocityChangeEvent::new(entity.entity.id);
            //self.item_velocity_change_event.send(event);
        }
    }
    fn set_velocity_y(&mut self, entity: &mut Item, velocity_y: f32) {
        if entity.entity.get_velocity_y() != velocity_y {
            entity.entity.velocity_y = velocity_y;
            let _event = EntityVelocityChangeEvent::new(entity.entity.id);
            //self.item_velocity_change_event.send(event);
        }
    }
    fn add_velocity_x(&mut self, entity: &mut Item, velocity_x: f32) {
        self.set_velocity_x(entity, entity.entity.get_velocity_x() + velocity_x);
    }
    fn add_velocity_y(&mut self, entity: &mut Item, velocity_y: f32) {
        self.set_velocity_y(entity, entity.entity.get_velocity_y() + velocity_y);
    }
    fn set_x(&mut self, entity: &mut Item, x: f32, send_to_everyone: bool) {
        if entity.entity.get_x() != x {
            entity.entity.x = x;
            if send_to_everyone {
                let _event = EntityPositionChangeEvent::new(entity.entity.id);
                //self.item_position_change_event.send(event);
            }
        }
    }
    fn set_y(&mut self, entity: &mut Item, y: f32, send_to_everyone: bool) {
        if entity.entity.get_y() != y {
            entity.entity.y = y;
            if send_to_everyone {
                let _event = EntityPositionChangeEvent::new(entity.entity.id);
                //self.item_position_change_event.send(event);
            }
        }
    }
}*/
