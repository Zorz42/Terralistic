extern crate alloc;

use alloc::sync::Arc;
use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};

use anyhow::{anyhow, bail, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::BlockId;
use crate::shared::entities::{Entities, EntityId, PhysicsComponent, PositionComponent};
use crate::shared::items::Item;
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::WallId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: i32,
}

impl ItemStack {
    #[must_use]
    pub const fn new(item_type: ItemId, stack: i32) -> Self {
        Self {
            item: item_type,
            count: stack,
        }
    }
}

pub struct ItemCreationEvent {
    pub item_id: u32,
}

#[derive(Clone)]
pub struct TileDrop {
    pub item: ItemId,
    pub chance: f32,
}

impl TileDrop {
    #[must_use]
    pub const fn new(drop: ItemId, chance: f32) -> Self {
        Self { item: drop, chance }
    }
}

pub struct Items {
    item_types: Arc<Mutex<Vec<Item>>>,
    block_drops: Arc<Mutex<HashMap<BlockId, TileDrop>>>,
    wall_drops: HashMap<WallId, TileDrop>,
}

impl Items {
    #[must_use]
    pub fn new() -> Self {
        Self {
            item_types: Arc::new(Mutex::new(Vec::new())),
            block_drops: Arc::new(Mutex::new(HashMap::new())),
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

        let block_drops = self.block_drops.clone();
        mods.add_global_function(
            "set_block_drop",
            move |_lua, (block_id, item_id, chance): (BlockId, ItemId, f32)| {
                block_drops
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .insert(block_id, TileDrop::new(item_id, chance));
                Ok(())
            },
        )?;
        Ok(())
    }

    /// this function spawns an item into the world
    /// # Errors
    /// if the item could not be spawned
    pub fn spawn_item(
        &mut self,
        events: &mut EventManager,
        entities: &mut Entities,
        item_id: ItemId,
        x: f32,
        y: f32,
        id: EntityId,
    ) -> Result<Entity> {
        let entity = entities.ecs.spawn((
            PositionComponent::new(x, y),
            PhysicsComponent::new(1.0, 1.0),
            ItemComponent::new(item_id),
        ));

        entities.assign_id(entity, id)?;

        let event = ItemSpawnEvent { entity };
        events.push_event(Event::new(event));

        Ok(entity)
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
        self.block_drops
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(block_type, drop);
    }

    /// this function returns the block drop for the given block type
    /// # Errors
    /// if the block drop is not found
    pub fn get_block_drop(&self, block_type: BlockId) -> Result<TileDrop> {
        Ok(self
            .block_drops
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(&block_type)
            .ok_or_else(|| anyhow!("block drop not found"))?
            .clone())
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

    pub fn get_all_item_type_ids(&self) -> Vec<ItemId> {
        let mut ids = Vec::new();
        let item_types = self
            .item_types
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let iter = item_types.iter();
        for item in iter {
            ids.push(item.id);
        }
        ids
    }
}

pub struct ItemSpawnEvent {
    pub entity: Entity,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemSpawnPacket {
    pub item_type: ItemId,
    pub x: f32,
    pub y: f32,
    pub id: EntityId,
}

pub struct ItemComponent {
    item_type: ItemId,
}

impl ItemComponent {
    #[must_use]
    pub const fn new(item_type: ItemId) -> Self {
        Self { item_type }
    }

    #[must_use]
    pub const fn get_item_type(&self) -> ItemId {
        self.item_type
    }
}
