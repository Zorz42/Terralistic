use crate::libraries::events::EventManager;
use crate::shared::entities::Entities;
use crate::shared::items::{ItemId, ItemStack, Items};
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Recipe {
    pub result: ItemId,
    pub ingredients: HashMap<ItemId, i32>,
}

pub struct Recipes {
    recipes: Vec<Recipe>,
}

impl Recipes {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            recipes: Vec::new(),
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    #[must_use]
    pub const fn get_recipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }
}

#[derive(Clone)]
pub struct Inventory {
    items: Vec<Option<ItemStack>>,
    pub has_changed: bool,
}

impl Inventory {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            items: vec![None; size],
            has_changed: false,
        }
    }

    /// # Errors
    /// if the index is out of bounds
    pub fn get_item(&self, index: usize) -> Result<Option<ItemStack>> {
        Ok(self
            .items
            .get(index)
            .ok_or_else(|| anyhow!("no item at index"))?
            .clone())
    }

    /// # Errors
    /// if the index is out of bounds
    pub fn set_item(&mut self, index: usize, item: Option<ItemStack>) -> Result<()> {
        if self.get_item(index)? != item {
            self.has_changed = true;
            *self
                .items
                .get_mut(index)
                .ok_or_else(|| anyhow!("Out of bounds"))? = item;
        }
        Ok(())
    }

    #[must_use]
    pub fn get_item_count(&self, item: ItemId) -> i32 {
        let mut count = 0;
        for slot in self.items.iter().flatten() {
            if slot.item == item {
                count += slot.count;
            }
        }
        count
    }

    #[must_use]
    pub fn can_craft(&self, recipe: &Recipe) -> bool {
        for (item, count) in &recipe.ingredients {
            if self.get_item_count(*item) < *count {
                return false;
            }
        }
        true
    }

    /// # Errors
    /// if the recipe can't be crafted
    pub fn craft(&mut self, recipe: &Recipe) -> Result<()> {
        if !self.can_craft(recipe) {
            bail!("can't craft")
        }

        let mut counts_to_remove = recipe.ingredients.clone();
        for slot in &mut self.items {
            if let Some(item) = slot {
                if let Some(count) = counts_to_remove.get_mut(&item.item) {
                    if *count > 0 {
                        if item.count > *count {
                            item.count -= *count;
                            *count = 0;
                        } else {
                            *count -= item.count;
                            *slot = None;
                        }
                    }
                }
            }
        }

        let result = self
            .items
            .iter_mut()
            .find(|item| item.is_none())
            .ok_or_else(|| anyhow!("no empty slot"))?;
        *result = Some(ItemStack {
            item: recipe.result,
            count: 1,
        });
        Ok(())
    }

    /// This function adds an item to the
    /// inventory. If the item can't be added
    /// it is dropped in the world.
    /// # Errors
    /// item stack is invalid
    pub fn give_item(
        &mut self,
        item: &ItemStack,
        drop_pos: (f32, f32),
        items: &mut Items,
        entities: &mut Entities,
        events: &mut EventManager,
    ) -> Result<()> {
        let mut item = item.clone();
        for slot in self.items.iter_mut().flatten() {
            if slot.item == item.item {
                let max = items.get_item_type(slot.item)?.max_stack;
                if slot.count < max {
                    let count = core::cmp::min(max - slot.count, item.count);
                    slot.count += count;
                    item.count -= count;
                    if item.count == 0 {
                        return Ok(());
                    }
                }
            }
        }

        for slot in &mut self.items {
            if slot.is_none() {
                let max = items.get_item_type(item.item)?.max_stack;
                if item.count > max {
                    *slot = Some(ItemStack {
                        item: item.item,
                        count: max,
                    });
                    item.count -= max;
                } else {
                    *slot = Some(item);
                    return Ok(());
                }
            }
        }

        // drop item
        for _ in 0..item.count {
            items.spawn_item(events, entities, item.item, drop_pos.0, drop_pos.1, None);
        }

        Ok(())
    }
}

impl Serialize for Inventory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.items.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Inventory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let items = Vec::deserialize(deserializer)?;
        Ok(Self {
            items,
            has_changed: false,
        })
    }
}
