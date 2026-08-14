use std::collections::HashMap;

use anyhow::{anyhow, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::libraries::registry::{Registry, RegistryEntry, RegistryId};
use crate::shared::blocks::BlockId;
use crate::shared::entities::{Entities, EntityId, PhysicsComponent, PositionComponent};
use crate::shared::items::Item;
use crate::shared::walls::WallId;

const VELOCITY_RANGE: f32 = 5.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ItemId {
    pub(super) id: i32,
}

impl ItemId {
    #[must_use]
    pub const fn new() -> Self {
        Self { id: -1 }
    }
}

impl RegistryId for ItemId {
    const KIND: &'static str = "item type";

    fn from_index(index: usize) -> Self {
        Self { id: index as i32 }
    }

    fn index(self) -> Option<usize> {
        usize::try_from(self.id).ok()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ItemStack {
    pub item: ItemId,
    pub count: i32,
}

impl ItemStack {
    #[must_use]
    pub const fn new(item_type: ItemId, stack: i32) -> Self {
        Self { item: item_type, count: stack }
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

impl RegistryId for RecipeId {
    const KIND: &'static str = "recipe";

    fn from_index(index: usize) -> Self {
        Self { id: index as i32 }
    }

    fn index(self) -> Option<usize> {
        usize::try_from(self.id).ok()
    }
}

/// A recipe has no name to look up by, so it implements only the half of the registry
/// contract that gives it an id.
impl RegistryEntry<RecipeId> for Recipe {
    fn set_id(&mut self, id: RecipeId) {
        self.id = id;
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
    pub(super) item_types: Registry<ItemId, Item>,
    pub(super) block_drops: HashMap<BlockId, TileDrop>,
    recipes: Registry<RecipeId, Recipe>,
    wall_drops: HashMap<WallId, TileDrop>,
}

impl Items {
    #[must_use]
    pub fn new() -> Self {
        Self {
            item_types: Registry::new(),
            block_drops: HashMap::new(),
            wall_drops: HashMap::new(),
            recipes: Registry::new(),
        }
    }

    /// this function spawns an item into the world
    pub fn spawn_item(&self, events: &mut EventManager, entities: &mut Entities, item_id: ItemId, x: f32, y: f32, id: EntityId) -> Result<Entity> {
        let entity = entities.ecs.spawn((PositionComponent::new(x, y), PhysicsComponent::new(1.0, 1.0), ItemComponent::new(item_id)));

        entities.assign_id(entity, id)?;

        let event = ItemSpawnEvent { entity };
        events.push_event(Event::new(event));

        Ok(entity)
    }

    /// spawns an item with random velocity
    pub fn drop_item(&self, events: &mut EventManager, entities: &mut Entities, item: ItemId, x: f32, y: f32) -> Result<()> {
        let id = entities.new_id();
        let entity = self.spawn_item(events, entities, item, x, y, id)?;
        let velocity_x = rand::random::<f32>() * 2.0 * VELOCITY_RANGE - VELOCITY_RANGE;
        let velocity_y = -rand::random::<f32>() * 4.0 * VELOCITY_RANGE;

        let mut physics = entities.ecs.get::<&mut PhysicsComponent>(entity)?;
        physics.velocity_x = velocity_x;
        physics.velocity_y = velocity_y;

        Ok(())
    }

    /// Registers an item type on this `Items` and returns its id.
    ///
    /// The mod interface has to use `register_new_item_type` directly because it only
    /// holds the item type vector, not the whole `Items`. Everything else should use this.
    pub fn register_item_type(&mut self, item_type: Item) -> ItemId {
        Self::register_new_item_type(&mut self.item_types, item_type)
    }

    /// this function registers an item type
    pub fn register_new_item_type(item_types: &mut Registry<ItemId, Item>, item_type: Item) -> ItemId {
        item_types.register(item_type)
    }

    /// this function returns the item type with the given id
    pub fn get_item_type(&self, id: ItemId) -> Result<Item> {
        Ok(self.item_types.get(id)?.clone())
    }

    /// this function returns the item type with the given name
    pub fn get_item_type_by_name(&self, name: &str) -> Result<Item> {
        Ok(self.item_types.get_by_name(name)?.clone())
    }

    /// this function returns the number of item types
    #[must_use]
    pub const fn get_num_item_types(&self) -> usize {
        self.item_types.len()
    }

    /// this function sets the block drop for the given block type
    pub fn set_block_drop(&mut self, block_type: BlockId, drop: TileDrop) {
        self.block_drops.insert(block_type, drop);
    }

    /// this function returns the block drop for the given block type
    pub fn get_block_drop(&self, block_type: BlockId) -> Result<TileDrop> {
        Ok(self.block_drops.get(&block_type).ok_or_else(|| anyhow!("block drop not found"))?.clone())
    }

    /// this function sets the wall drop for the given wall type
    pub fn set_wall_drop(&mut self, wall_type: WallId, drop: TileDrop) {
        self.wall_drops.insert(wall_type, drop);
    }

    /// this function returns the wall drop for the given wall type
    pub fn get_wall_drop(&self, wall_type: WallId) -> Result<&TileDrop> {
        self.wall_drops.get(&wall_type).ok_or_else(|| anyhow!("wall drop not found"))
    }

    #[must_use]
    pub fn get_all_item_type_ids(&self) -> Vec<ItemId> {
        self.item_types.ids()
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.register(recipe);
    }

    #[must_use]
    pub const fn get_recipes(&self) -> &Registry<RecipeId, Recipe> {
        &self.recipes
    }

    /// this function returns the recipe with the given id
    pub fn get_recipe(&self, id: RecipeId) -> Result<&Recipe> {
        self.recipes.get(id)
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
    pub velocity_x: f32,
    pub velocity_y: f32,
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
