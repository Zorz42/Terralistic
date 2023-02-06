use super::{entities::*, items::*, player::*};
use crate::shared::blocks::Block;
use crate::shared::blocks::{Blocks, BLOCK_WIDTH};
use std::{collections::hash_map::HashMap, rc::Rc};

//TODO: iKramp do debug assertions for all panics

const CRAFTING_BLOCK_RANGE: i32 = 3;
const INVENTORY_SIZE: usize = 10;

/**struct that describes a single crafting recipe*/
pub struct Recipe {
    //first is item type id, then item count
    pub ingredients: HashMap<i32, i32>,
    pub result: ItemStack,
    //if the recipe can only be crafted near a block (crafting table), it's listed here
    pub crafting_block: Option<Rc<Block>>,
}
impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.result.item_type.get_id() == other.result.item_type.get_id()
            && self.result.stack == other.result.stack
            && self.ingredients == other.ingredients
    }
}

/**event that fires when an inventory slot is changed*/
pub struct InventoryItemChangeEvent {
    pub item_pos: i32,
}
impl InventoryItemChangeEvent {
    pub fn new(item_pos: i32) -> Self {
        InventoryItemChangeEvent { item_pos }
    }
}
//impl Event for InventoryItemChangeEvent {}

/**struct that contains all recipes*/
pub struct Recipes {
    recipes: Vec<Rc<Recipe>>,
}

impl Default for Recipes {
    fn default() -> Self {
        Self::new()
    }
}

impl Recipes {
    pub fn new() -> Self {
        Recipes {
            recipes: Vec::new(),
        }
    }
    pub fn register_a_recipe(&mut self, recipe: Rc<Recipe>) {
        self.recipes.push(recipe);
    }
    pub fn get_all_recipes(&self) -> &Vec<Rc<Recipe>> {
        &self.recipes
    }
}
impl Clone for Recipes {
    fn clone(&self) -> Self {
        let mut new_r = Recipes {
            recipes: Vec::new(),
        };
        for recipe in &self.recipes {
            new_r.recipes.push(recipe.clone());
        }
        new_r
    }
}

/**struct that defines the inventory and everything related to it*/
pub struct Inventory {
    recipes: Recipes,
    player_id: u32,
    mouse_item: ItemStack,
    item_counts: Vec<i32>,
    available_recipes: Vec<Rc<Recipe>>,
    inventory_arr: Vec<ItemStack>,
    pub selected_slot: i32,
}
impl Inventory {
    pub fn new(items: &Items) -> Self {
        let mut inventory_arr = Vec::new();
        for _ in 0..INVENTORY_SIZE {
            inventory_arr.push(ItemStack::new(items.nothing.clone(), 0));
        }

        let mouse_item = ItemStack::new(Rc::clone(&items.nothing), 0);
        let item_counts = vec![items.get_num_item_types() as i32; 0];

        Inventory {
            recipes: Recipes::new(),
            player_id: 0,
            mouse_item,
            item_counts,
            available_recipes: Vec::new(),
            inventory_arr,
            selected_slot: 0,
            //item_change_event: Sender::new(),
        }
    }
    pub fn from_existing(inventory: &Inventory) -> Self {
        Inventory {
            recipes: inventory.recipes.clone(),
            player_id: inventory.player_id,
            mouse_item: ItemStack::new(Rc::clone(&inventory.mouse_item.item_type.clone()), 0),
            item_counts: inventory.item_counts.clone(),
            available_recipes: inventory.available_recipes.clone(),
            inventory_arr: inventory.inventory_arr.clone(),
            selected_slot: inventory.selected_slot,
            //item_change_event: Sender::new(),
        }
    }

    /**returns whether the player has enough items to craft the recipe*/
    fn can_craft_recipe(
        &self,
        recipe: &Recipe,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) -> bool {
        for i in 0..items.get_num_item_types() {
            if recipe.ingredients.get(&(i as i32)).unwrap_or(&0)
                > self.item_counts.get(i).unwrap_or(&0)
            {
                return false;
            }
        }
        if recipe.crafting_block.is_none() {
            true
        } else {
            let player_coords = [
                players.get_entity_by_id(self.player_id).unwrap().get_x() as i32,
                players.get_entity_by_id(self.player_id).unwrap().get_y() as i32,
            ];
            for x in -CRAFTING_BLOCK_RANGE..CRAFTING_BLOCK_RANGE {
                for y in -CRAFTING_BLOCK_RANGE..CRAFTING_BLOCK_RANGE {
                    let block_id = blocks
                        .get_block(
                            std::cmp::min(
                                blocks.get_width(),
                                std::cmp::max(0, player_coords[0] / (BLOCK_WIDTH * 2) + x),
                            ),
                            std::cmp::min(
                                blocks.get_height(),
                                std::cmp::max(0, player_coords[1] / (BLOCK_WIDTH * 2) + y),
                            ),
                        )
                        .unwrap();
                    if block_id == recipe.crafting_block.as_ref().unwrap().get_id() {
                        return true;
                    }
                }
            }
            false
        }
    }
    /**returns available recipes*/
    pub fn get_available_recipes(&self) -> &Vec<Rc<Recipe>> {
        &self.available_recipes
    }
    /**updates the list of available recipes*/
    pub fn update_available_recipes(&mut self, blocks: &Blocks, items: &Items, players: &Players) {
        for i in 0..self.available_recipes.len() {
            if !self.can_craft_recipe(&self.recipes.get_all_recipes()[i], blocks, items, players) {
                self.available_recipes.remove(i);
            }
        }
        for recipe in self.recipes.get_all_recipes() {
            if self.can_craft_recipe(recipe, blocks, items, players)
                && !self.available_recipes.contains(recipe)
            {
                self.available_recipes.push(Rc::clone(recipe));
            }
        }
    }
    /**adds an item to the inventory*/
    pub fn add_item(
        &mut self,
        item: Rc<ItemType>,
        mut count: i32,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) -> i32 {
        if count <= 0 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("item count cannot be negative");
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == item.get_id() {
                count -= self.increase_stack(i, count, blocks, items, players);
                if count <= 0 {
                    return i;
                }
            }
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == items.nothing.get_id() {
                self.set_item(
                    i,
                    ItemStack::new(Rc::clone(&item), count),
                    blocks,
                    items,
                    players,
                );
                count -= self.increase_stack(i, count, blocks, items, players);
                if count <= 0 {
                    return i;
                }
            }
        }
        -1
    }
    /**removes an item from the inventory*/
    pub fn remove_item(
        &mut self,
        item: Rc<ItemType>,
        mut count: i32,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) -> i32 {
        if count <= 0 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("item count cannot be negative");
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == item.get_id() {
                count -= self.decrease_stack(i, count, blocks, items, players);
                if count <= 0 {
                    return i;
                }
            }
        }
        if self.mouse_item.item_type.get_id() == item.get_id() {
            count -= self.decrease_stack(-1, count, blocks, items, players);
            if count <= 0 {
                return -1;
            }
        }
        -1
    }
    /**sets an inventory slot to an item*/
    pub fn set_item(
        &mut self,
        slot: i32,
        item: ItemStack,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) {
        if slot < 0 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("slot out of bounds");
        }
        if self.item_counts.is_empty() {
            self.item_counts = vec![items.get_num_item_types() as i32; 0];

            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("item counts is empty");
        }
        let old_item_id = self.get_item(slot).item_type.get_id() as usize;
        self.item_counts[old_item_id] -= self.get_item(slot).stack;
        self.item_counts[item.item_type.get_id() as usize] += item.stack;
        let item_stack = if slot == -1 {
            &mut self.mouse_item
        } else {
            &mut self.inventory_arr[slot as usize]
        };
        *item_stack = item;
        self.update_available_recipes(blocks, items, players);

        //self.item_change_event.send(InventoryItemChangeEvent::new(slot));
    }
    /**gets an item from an inventory slot*/
    pub fn get_item(&self, slot: i32) -> &ItemStack {
        if slot < -1 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("slot out of bounds");
        }
        if slot == -1 {
            return &self.mouse_item;
        }
        &self.inventory_arr[slot as usize]
    }
    /**gets a mutable reference to an item from an inventory slot*/
    pub fn get_item_mut(&mut self, slot: i32) -> &mut ItemStack {
        if slot < -1 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("slot out of bounds");
        }
        if slot == -1 {
            return &mut self.mouse_item;
        }
        &mut self.inventory_arr[slot as usize]
    }
    /**counts the items of 1 type in the inventory*/
    pub fn count_items(&self, item_id: i32) -> i32 {
        let mut result = 0;
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == item_id {
                result += self.get_item(i).stack;
            }
        }
        result
    }
    /**returns the item in the selected slot*/
    pub fn get_selected_slot(&self) -> &ItemStack {
        self.get_item(self.selected_slot)
    }
    /**swaps an item in an inventory slot with the item in the mouse slot*/
    pub fn swap_with_mouse_item(&mut self, slot: i32) {
        if slot < 0 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)] //only panic in debug mode
            panic!("slot out of bounds");
        }
        std::mem::swap(&mut self.inventory_arr[slot as usize], &mut self.mouse_item);
    }
    /**increases the stack count of an item in an inventory slot*/
    pub fn increase_stack(
        &mut self,
        slot: i32,
        stack: i32,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) -> i32 {
        let mut stack_to_be = self.get_item(slot).stack + stack;

        if stack_to_be > self.get_item(slot).item_type.max_stack {
            stack_to_be = self.get_item(slot).item_type.max_stack;
        }
        let result = stack_to_be - self.get_item(slot).stack;
        self.set_item(
            slot,
            ItemStack::new(Rc::clone(&self.get_item(slot).item_type), stack_to_be),
            blocks,
            items,
            players,
        );
        result
    }
    /**decreases the stack count of an item in an inventory slot*/
    pub fn decrease_stack(
        &mut self,
        slot: i32,
        stack: i32,
        blocks: &Blocks,
        items: &Items,
        players: &Players,
    ) -> i32 {
        if stack > self.get_item(slot).stack {
            let prev_stack = self.get_item(slot).stack;
            let nothing_type = items.nothing.clone();
            self.set_item(
                slot,
                ItemStack::new(nothing_type, 0),
                blocks,
                items,
                players,
            );
            prev_stack
        } else {
            self.set_item(
                slot,
                ItemStack::new(
                    Rc::clone(&self.get_item(slot).item_type),
                    self.get_item(slot).stack - stack,
                ),
                blocks,
                items,
                players,
            );
            stack
        }
    }
    /**serailizes the inventory, used when saving the world or sending it to the client*/
    pub fn serialize(&self) -> Vec<u8> {
        let mut serial: Vec<u8> = Vec::new();
        for item in &self.inventory_arr {
            serial.append(&mut item.item_type.get_id().to_le_bytes().to_vec());
            serial.append(&mut item.stack.to_le_bytes().to_vec());
        }
        serial
    }
    /**deserializes the inventory, used when loading the world or receiving it from the server*/
    pub fn deserialize(&mut self, data: Vec<u8>, items: &Items) {
        let mut i = 0;
        for item in &mut self.inventory_arr {
            let item_id = i32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            let stack = i32::from_le_bytes([data[i + 4], data[i + 5], data[i + 6], data[i + 7]]);
            *item = ItemStack::new(items.get_item_type(item_id).clone(), stack);
            i += 8;
        }
    }

    /**sets the inventory's player*/
    pub fn set_player(&mut self, player: &mut Player) {
        self.player_id = player.get_id();
    }
}
