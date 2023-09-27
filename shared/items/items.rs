use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::BlockId;
use crate::shared::entities::{Entities, EntityId, PhysicsComponent, PositionComponent};
use crate::shared::items::Item;
use crate::shared::walls::WallId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId {
    id: i32,
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecipeId {
    id: i32,
}

impl RecipeId {
    #[must_use]
    pub const fn new() -> Self {
        Self { id: -1 }
    }
}

#[derive(Clone)]
pub struct Recipe {
    pub result: ItemStack,
    pub ingredients: HashMap<ItemId, i32>,
    id: RecipeId,
}

impl Recipe {
    #[must_use]
    pub fn new() -> Self {
        Self {
            result: ItemStack::new(ItemId::new(), 0),
            ingredients: HashMap::new(),
            id: RecipeId::new(),
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> RecipeId {
        self.id
    }
}

pub struct Items {
    pub(super) item_types: Vec<Item>,
    pub(super) block_drops: HashMap<BlockId, TileDrop>,
    recipes: Vec<Recipe>,
    wall_drops: HashMap<WallId, TileDrop>,
}

impl Items {
    #[must_use]
    pub fn new() -> Self {
        Self {
            item_types: Vec::new(),
            block_drops: HashMap::new(),
            wall_drops: HashMap::new(),
            recipes: Vec::new(),
        }
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
        Ok(self
            .item_types
            .get(id.id as usize)
            .ok_or_else(|| anyhow!("item type not found"))?
            .clone())
    }

    /// this function returns the item type with the given name
    /// # Errors
    /// if the item type is not found
    pub fn get_item_type_by_name(&self, name: &str) -> Result<Item> {
        for item_type in &self.item_types {
            if item_type.name == name {
                return Ok(item_type.clone());
            }
        }
        bail!("item type not found")
    }

    /// this function returns the number of item types
    #[must_use]
    pub fn get_num_item_types(&self) -> usize {
        self.item_types.len()
    }

    /// this function sets the block drop for the given block type
    pub fn set_block_drop(&mut self, block_type: BlockId, drop: TileDrop) {
        self.block_drops.insert(block_type, drop);
    }

    /// this function returns the block drop for the given block type
    /// # Errors
    /// if the block drop is not found
    pub fn get_block_drop(&self, block_type: BlockId) -> Result<TileDrop> {
        Ok(self
            .block_drops
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

    #[must_use]
    pub fn get_all_item_type_ids(&self) -> Vec<ItemId> {
        let mut ids = Vec::new();
        for item in &self.item_types {
            ids.push(item.id);
        }
        ids
    }

    pub fn add_recipe(&mut self, mut recipe: Recipe) {
        recipe.id = RecipeId {
            id: self.recipes.len() as i32,
        };
        self.recipes.push(recipe);
    }

    #[must_use]
    pub const fn get_recipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }

    /// this function returns the recipe with the given id
    /// # Errors
    /// if the recipe is not found
    pub fn get_recipe(&self, id: RecipeId) -> Result<&Recipe> {
        self.recipes
            .get(id.id as usize)
            .ok_or_else(|| anyhow!("recipe not found"))
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

pub struct ItemCreationEvent {
    pub item_id: u32,
}
