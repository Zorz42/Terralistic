use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::EventManager;
use crate::shared::entities::Entities;
use crate::shared::items::{ItemId, ItemStack, Items, Recipe, RecipeId};

#[derive(Clone, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<Option<ItemStack>>,
    pub has_changed: bool,
    pub selected_slot: Option<usize>,
}

impl Inventory {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            items: vec![None; size],
            has_changed: false,
            selected_slot: None,
        }
    }

    pub fn transfer_items_from(&mut self, other: Self) {
        self.items = other.items;
    }

    pub fn get_item(&self, index: usize) -> Result<Option<ItemStack>> {
        Ok(self
            .items
            .get(index)
            .ok_or_else(|| anyhow!("no item at index"))?
            .clone())
    }

    pub fn set_item(&mut self, index: usize, mut item: Option<ItemStack>) -> Result<()> {
        if let Some(item_stack) = item.clone() {
            if item_stack.count <= 0 {
                item = None;
            }
        }

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

    pub fn craft(
        &mut self,
        recipe: &Recipe,
        drop_pos: (f32, f32),
        items: &mut Items,
        entities: &mut Entities,
        events: &mut EventManager,
    ) -> Result<()> {
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
                        self.has_changed = true;
                    }
                }
            }
        }

        self.give_item(recipe.result.clone(), drop_pos, items, entities, events)?;
        Ok(())
    }

    /// This function adds an item to the
    /// inventory. If the item can't be added
    /// it is dropped in the world.
    pub fn give_item(
        &mut self,
        mut item: ItemStack,
        drop_pos: (f32, f32),
        items: &mut Items,
        entities: &mut Entities,
        events: &mut EventManager,
    ) -> Result<()> {
        for slot in self.items.iter_mut().flatten() {
            if slot.item == item.item {
                let max = items.get_item_type(slot.item)?.max_stack;
                if slot.count < max {
                    let count = std::cmp::min(max - slot.count, item.count);
                    slot.count += count;
                    item.count -= count;
                    self.has_changed = true;
                    if item.count == 0 {
                        return Ok(());
                    }
                }
            }
        }

        for slot in &mut self.items {
            if item.count == 0 {
                break;
            }

            if slot.is_none() {
                let max = items.get_item_type(item.item)?.max_stack;
                if item.count > max {
                    *slot = Some(ItemStack {
                        item: item.item,
                        count: max,
                    });
                    item.count -= max;
                } else {
                    *slot = Some(item.clone());
                    item.count = 0;
                }
                self.has_changed = true;
            }
        }

        // drop item
        for _ in 0..item.count {
            let id = entities.new_id();
            items.spawn_item(events, entities, item.item, drop_pos.0, drop_pos.1, id)?;
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Option<ItemStack>> {
        self.items.iter()
    }

    pub fn reverse_iter(&self) -> impl Iterator<Item = &Option<ItemStack>> {
        self.items.iter().rev()
    }

    #[must_use]
    pub fn get_selected_item(&self) -> Option<ItemStack> {
        self.selected_slot
            .and_then(|slot| self.items.get(slot).and_then(Clone::clone))
    }

    pub fn swap_with_selected_item(&mut self, slot: usize) -> Result<()> {
        let selected_item = self.get_selected_item();
        let hovered_item = self.get_item(slot)?;

        if let Some(selected_slot) = self.selected_slot {
            self.set_item(selected_slot, hovered_item)?;
            self.set_item(slot, selected_item)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct InventoryPacket {
    pub inventory: Inventory,
}

#[derive(Serialize, Deserialize)]
pub struct InventorySelectPacket {
    pub slot: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct InventorySwapPacket {
    pub slot: usize,
}

#[derive(Serialize, Deserialize)]
pub struct InventoryCraftPacket {
    pub recipe: RecipeId,
}
