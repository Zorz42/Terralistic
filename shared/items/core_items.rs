use crate::shared::blocks::{BlockId, Blocks};
use crate::shared::entities::Entity;
use crate::shared::items::Item;
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::WallId;
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};
extern crate alloc;
use alloc::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ItemId {
    id: i32,
}

// make ItemId lua compatible
impl rlua::UserData for ItemId {}

impl ItemId {
    #[must_use]
    pub const fn new() -> Self {
        Self { id: -1 }
    }
}

pub struct MapItem {
    pub item_id: ItemId,
    pub entity: Entity,
}

impl MapItem {
    #[must_use]
    pub fn new(item_type: &Item, x: f32, y: f32, id: Option<u32>) -> Self {
        Self {
            item_id: item_type.id,
            entity: Entity::new(x, y, item_type.width, item_type.height, id),
        }
    }
}

impl MapItem {}

#[derive(Clone)]
pub struct ItemStack {
    pub item_type: ItemId,
    pub stack: i32,
}

impl ItemStack {
    #[must_use]
    pub const fn new(item_type: ItemId, stack: i32) -> Self {
        Self { item_type, stack }
    }
}

pub struct ItemCreationEvent {
    pub item_id: u32,
}

pub struct TileDrop {
    pub drop: ItemId,
    pub chance: f32,
}

impl TileDrop {
    #[must_use]
    pub const fn new(drop: ItemId, chance: f32) -> Self {
        Self { drop, chance }
    }
}

pub struct Items {
    map_items: Vec<MapItem>,
    item_types: Arc<Mutex<Vec<Item>>>,
    block_drops: HashMap<BlockId, TileDrop>,
    wall_drops: HashMap<WallId, TileDrop>,
}

impl Default for Items {
    fn default() -> Self {
        Self::new()
    }
}

impl Items {
    #[must_use]
    pub fn new() -> Self {
        Self {
            map_items: Vec::new(),
            item_types: Arc::new(Mutex::new(Vec::new())),
            block_drops: HashMap::new(),
            wall_drops: HashMap::new(),
        }
    }

    /// this function initializes the items
    /// it adds lua functions to the lua context
    /// # Errors
    /// if the function fails to add the lua functions
    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        mods.add_global_function("new_item_type", move |_lua, _: ()| Ok(Item::new()))?;

        let mut item_types = self.item_types.clone();
        mods.add_global_function("register_item_type", move |_lua, item_type: Item| {
            let result = Self::register_new_item_type(
                &mut item_types.lock().unwrap_or_else(PoisonError::into_inner),
                item_type,
            );
            Ok(result)
        })?;

        item_types = self.item_types.clone();
        mods.add_global_function("get_item_id_by_name", move |_lua, name: String| {
            let item_types = item_types.lock().unwrap_or_else(PoisonError::into_inner);
            let iter = item_types.iter();
            for item_type in iter {
                if item_type.name == name {
                    return Ok(item_type.get_id());
                }
            }
            Err(rlua::Error::RuntimeError("Item type not found".to_owned()))
        })?;
        Ok(())
    }

    /// this function spawns an item into the world
    pub fn spawn_item(&mut self, item_type: &Item, x: f32, y: f32, id: Option<u32>) -> u32 {
        let item = MapItem::new(item_type, x, y, id);
        let item_id = item.entity.id;
        self.map_items.push(item);

        item_id
    }

    /// this function registers an item type
    pub fn register_new_item_type(item_types: &mut Vec<Item>, mut item_type: Item) -> ItemId {
        item_type.id = ItemId::new();
        item_type.id.id = item_types.len() as i32;
        let id = item_type.id;
        item_types.push(item_type);
        id
    }

    /// this function returns the item type with the given id
    /// # Errors
    /// if the item type is not found
    pub fn get_item_type(&self, id: ItemId) -> Result<Item> {
        let item_types = self
            .item_types
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        Ok(item_types
            .get(id.id as usize)
            .ok_or_else(|| anyhow!("item type not found"))?
            .clone())
    }

    /// this function returns the item type with the given name
    /// # Errors
    /// if the item type is not found
    pub fn get_item_type_by_name(&self, name: &str) -> Result<Item> {
        let item_types = self
            .item_types
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let iter = item_types.iter();
        for item_type in iter {
            if item_type.name == name {
                return Ok(item_type.clone());
            }
        }
        bail!("item type not found")
    }

    /// this function returns the number of item types
    #[must_use]
    pub fn get_num_item_types(&self) -> usize {
        self.item_types
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .len()
    }

    /// this function sets the block drop for the given block type
    pub fn set_block_drop(&mut self, block_type: BlockId, drop: TileDrop) {
        self.block_drops.insert(block_type, drop);
    }

    /// this function returns the block drop for the given block type
    /// # Errors
    /// if the block drop is not found
    pub fn get_block_drop(&self, block_type: BlockId) -> Result<&TileDrop> {
        self.block_drops
            .get(&block_type)
            .ok_or_else(|| anyhow!("block drop not found"))
    }

    /// this function sets the wall drop for the given wall type
    pub fn set_wall_drop(&mut self, wall_type: WallId, drop: TileDrop) {
        self.wall_drops.insert(wall_type, drop);
    }

    /// this function returns the wall drop for the given wall type
    /// # Errors
    /// if the wall drop is not found
    pub fn get_wall_drop(&self, wall_type: WallId) -> Result<&TileDrop> {
        self.wall_drops
            .get(&wall_type)
            .ok_or_else(|| anyhow!("wall drop not found"))
    }

    pub fn update_all_items(&mut self, blocks: &Blocks) {
        for item in &mut self.map_items {
            item.entity.update(blocks);
        }
    }

    /// # Errors
    /// if the item is not found
    pub fn remove_item(&mut self, entity_id: u32) -> Result<()> {
        let pos = self
            .map_items
            .iter()
            .position(|entity| entity.entity.id == entity_id)
            .ok_or_else(|| anyhow!("entity not found"));

        self.map_items.remove(pos?);
        Ok(())
    }

    /// # Errors
    /// if the item is not found
    pub fn get_item_by_id(&self, entity_id: u32) -> Result<&MapItem> {
        self.map_items
            .iter()
            .find(|entity| entity.entity.id == entity_id)
            .ok_or_else(|| anyhow!("entity not found"))
    }

    #[must_use]
    pub const fn get_entities(&self) -> &Vec<MapItem> {
        &self.map_items
    }
}
