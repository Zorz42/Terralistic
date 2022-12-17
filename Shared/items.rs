use super::{walls, entities::*, blocks, blocks::Blocks};
use std::{rc::Rc, collections::hash_map};
use std::collections::hash_map::Entry;
use std::ops::Deref;
use events::*;

const ITEM_WIDTH: i32 = 8;

struct ItemType{
    pub name: String, pub display_name: String,
    pub max_stack: i32,
    pub places_block: Option<Rc<blocks::BlockType>>,
    pub places_wall: Option<Rc<walls::WallType>>,
    pub tool_powers: hash_map::HashMap<Rc<blocks::Tool>, i32>,
    id: i32
}

impl ItemType {
    pub fn new(name: String) -> Self {
        ItemType{
            name,
            display_name: "".to_string(),
            max_stack: 0,
            places_block: None,
            places_wall: None,
            tool_powers: hash_map::HashMap::new(),
            id: 0
        }
    }
}

struct Item {
    item_type: Rc<ItemType>,
    pub entity: Entity,
    pub entity_item_count: u32,
}

impl Item {

    /**creates a new item*/
    pub fn new(item_type: Rc<ItemType>, x: i32, y: i32, entity_item_count: u32, id: u32) -> Self {
        Item{
            item_type,
            entity_item_count,
            entity: Entity::new(EntityType::ITEM, x, y, id)
        }
    }

    /**returns item type*/
    pub fn getType(&self) -> &ItemType { self.item_type.deref() }
}

impl entity_object for Item {
    fn get_width(&self) -> i32 { ITEM_WIDTH * 2 }
    fn get_height(&self) -> i32 { ITEM_WIDTH * 2 }
    fn is_colliding(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool{
        self.entity.is_colliding(blocks, direction, colliding_x, colliding_y)
    }
    fn is_colliding_with_block(&self, blocks: &Blocks, direction: Direction, colliding_x: f64, colliding_y: f64) -> bool{
        self.entity.is_colliding_with_block(blocks, direction, colliding_x, colliding_y)
    }
    fn update_entity(&mut self, blocks: &Blocks){
        self.entity.update_entity(blocks);
    }
    fn is_touching_ground(&self, blocks: &Blocks) -> bool{
        self.entity.is_touching_ground(blocks)
    }
    fn get_x(&self) -> f64{
        self.entity.get_x()
    }
    fn get_y(&self) -> f64{
        self.entity.get_y()
    }
    fn get_velocity_x(&self) -> f64{
        self.entity.get_velocity_x()
    }
    fn get_velocity_y(&self) -> f64{
        self.entity.get_velocity_y()
    }
}

struct ItemStack {
    pub item_type: Rc<ItemType>,
    pub stack: i32
}

impl ItemStack {
    pub fn new(item_type: Rc<ItemType>, stack: i32) -> Self {
        ItemStack{ item_type, stack }
    }
}

struct ItemCreationEvent {
    pub item_id: u32,
}

impl ItemCreationEvent {
    pub fn new(item_id: u32) -> Self { ItemCreationEvent{ item_id } }
}
impl Event for ItemCreationEvent {}

struct TileDrop {
    drop: Rc<ItemType>,
    chance: f64
}

impl TileDrop {
    pub fn new(drop: Rc<ItemType>, chance: f64) -> Self {
        TileDrop{ drop, chance }
    }
}

struct Items<'blocks, 'entities> {
    entities: &'entities Entities<'blocks>,
    blocks: &'blocks Blocks,

    item_types: Vec<Rc<ItemType>>,
    block_drops: Vec<TileDrop>,
    wall_drops: Vec<TileDrop>,

    pub nothing: Rc<ItemType>,

    item_creation_event: Sender<ItemCreationEvent>,
}

impl<'blocks, 'entities> Items<'blocks, 'entities> {
    pub fn new(entities: &'entities Entities<'blocks>, blocks: &'blocks Blocks) -> Self {
        let mut item_types = Vec::new();
        let mut block_drops = Vec::new();
        let mut wall_drops = Vec::new();

        let nothing = Rc::new(ItemType::new("nothing".to_string()));

        item_types.push(nothing.clone());

        Items{
            entities,
            blocks,
            item_types,
            block_drops,
            wall_drops,
            nothing,
            item_creation_event: Sender::new()
        }
    }//TODO: finish implementing functions
}
