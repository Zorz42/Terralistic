use std::{collections::hash_map::HashMap, rc::Rc};
use std::mem::MaybeUninit;
use super::{items::*, blocks::*, player::*, entities::*};
use {deprecated_events::*};
use shared_mut::SharedMut;

//TODO: iKramp do debug assertions for all panics

const CRAFTING_BLOCK_RANGE: i32 = 3;
const INVENTORY_SIZE: usize = 10;

/**struct that describes a single crafting recipe*/
pub struct Recipe {
    //first is item type id, then item count
    pub ingredients: HashMap<i32, i32>,
    pub result: ItemStack,
    //if the recipe can only be crafted near a block (crafting table), it's listed here
    pub crafting_block: Option<Rc<BlockType>>,
}
impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.result.item_type.get_id() == other.result.item_type.get_id() &&
            self.result.stack == other.result.stack &&
            self.ingredients == other.ingredients
    }
}

/**event that fires when an inventory slot is changed*/
pub struct InventoryItemChangeEvent {
    pub item_pos: i32
}
impl InventoryItemChangeEvent {
    pub fn new(item_pos: i32) -> Self {
        InventoryItemChangeEvent{ item_pos }
    }
}
impl Event for InventoryItemChangeEvent {}

/**struct that contains all recipes*/
struct Recipes {
    recipes: Vec<Rc<Recipe>>,
}
impl Recipes {
    pub fn new() -> Self {
        Recipes{ recipes: Vec::new() }
    }
    pub fn register_a_recipe(&mut self, recipe: Rc<Recipe>) {
        self.recipes.push(recipe);
    }
    pub fn get_all_recipes(&self) -> &Vec<Rc<Recipe>> {
        &self.recipes
    }
}

/**struct that defines the inventory and everything related to it*/
pub struct Inventory {
    items: SharedMut<Items>,
    recipes: Recipes,
    players: SharedMut<Players>,
    player_id: u32,
    blocks: SharedMut<Blocks>,
    mouse_item: ItemStack,
    item_counts: Vec<i32>,
    available_recipes: Vec<Rc<Recipe>>,
    inventory_arr: [ItemStack; INVENTORY_SIZE],
    pub selected_slot: i32,
    pub item_change_event: Sender<InventoryItemChangeEvent>,
}
impl Inventory {
    pub fn new(items: SharedMut<Items>, players: SharedMut<Players>, blocks: SharedMut<Blocks>) -> Self {

        //some shenanigans because an array can't be directly initialized with non copy types
        let mut temp_arr: [MaybeUninit<ItemStack>; INVENTORY_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..INVENTORY_SIZE {
            temp_arr[i] = MaybeUninit::new(ItemStack::new(items.borrow().nothing.clone(), 0));
        }
        let inventory_arr: [ItemStack; INVENTORY_SIZE] = unsafe { std::mem::transmute(temp_arr) };
        let mouse_item = ItemStack::new(Rc::clone(&items.borrow().nothing), 0);
        let item_counts = vec![items.borrow().get_num_item_types() as i32; 0];

        let inventory = Inventory{
            items,
            recipes: Recipes::new(),
            players,
            player_id: 0,
            blocks,
            mouse_item,
            item_counts,
            available_recipes: Vec::new(),
            inventory_arr,
            selected_slot: 0,
            item_change_event: Sender::new(),
        };
        inventory
    }

    /**returns whether the player has enough items to craft the recipe*/
    fn can_craft_recipe(&self, recipe: &Recipe) -> bool {
        let items = self.items.borrow();
        let players = self.players.borrow();
        for i in 0..items.get_num_item_types() {
            if recipe.ingredients.get(&(i as i32)).unwrap_or(&0) > self.item_counts.get(i).unwrap_or(&0) {
                return false;
            }
        }
        if recipe.crafting_block.is_none() {
            return true;
        } else {
            let player_coords = [players.get_entity_by_id(self.player_id).unwrap().get_x() as i32, players.get_entity_by_id(self.player_id).unwrap().get_y() as i32];
            let blocks = self.blocks.borrow();
            for x in -CRAFTING_BLOCK_RANGE..CRAFTING_BLOCK_RANGE {
                for y in -CRAFTING_BLOCK_RANGE..CRAFTING_BLOCK_RANGE {
                    let block = blocks.get_block(std::cmp::min(blocks.get_width(), std::cmp::max(0, player_coords[0] / (BLOCK_WIDTH * 2) + x)),
                                                 std::cmp::min(blocks.get_height(), std::cmp::max(0, player_coords[1] / (BLOCK_WIDTH * 2) + y)));
                    if block.id == recipe.crafting_block.as_ref().unwrap().get_id() {
                        return true;
                    }
                }
            }
            return false;
        }
    }
    /**returns available recipes*/
    pub fn get_available_recipes(&self) -> &Vec<Rc<Recipe>> {
        &self.available_recipes
    }
    /**updates the list of available recipes*/
    pub fn update_available_recipes(&mut self) {
        for i in 0..self.available_recipes.len() {
            if !self.can_craft_recipe(&self.recipes.get_all_recipes()[i]) {
                self.available_recipes.remove(i);
            }
        }
        for recipe in self.recipes.get_all_recipes() {
            if self.can_craft_recipe(recipe) && !self.available_recipes.contains(recipe) {
                self.available_recipes.push(Rc::clone(recipe));
            }
        }
    }
    /**adds an item to the inventory*/
    pub fn add_item(&mut self, item: Rc<ItemType>, mut count: i32) -> i32 {
        if count <= 0 {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("item count cannot be negative");
            return -1;
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == item.get_id() {
                count -= self.increase_stack(i, count);
                if count <= 0 {
                    return i;
                }
            }
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == self.items.borrow().nothing.get_id() {
                self.set_item(i, ItemStack::new(Rc::clone(&item), count));
                count -= self.increase_stack(i, count);
                if count <= 0 {
                    return i;
                }
            }
        }
        -1
    }
    /**removes an item from the inventory*/
    pub fn remove_item(&mut self, item: Rc<ItemType>, mut count: i32) -> i32 {
        if count <= 0 {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("item count cannot be negative");
            return -1;
        }
        for i in 0..INVENTORY_SIZE as i32 {
            if self.get_item(i).item_type.get_id() == item.get_id() {
                count -= self.decrease_stack(i, count);
                if count <= 0 {
                    return i;
                }
            }
        }
        if self.mouse_item.item_type.get_id() == item.get_id() {
            count -= self.decrease_stack(-1, count);
            if count <= 0 {
                return -1;
            }
        }
        -1
    }
    /**sets an inventory slot to an item*/
    pub fn set_item(&mut self, slot: i32, item: ItemStack) {
        if slot < 0 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("slot out of bounds");
            return;
        }
        if self.item_counts.is_empty() {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("item counts is empty");
            self.item_counts = vec![self.items.borrow().get_num_item_types() as i32; 0];
        }
        let old_item_id = self.get_item(slot).item_type.get_id() as usize;
        self.item_counts[old_item_id] -= self.get_item(slot).stack;
        self.item_counts[item.item_type.get_id() as usize] += item.stack;
        let item_stack = if slot == -1 {&mut self.mouse_item} else {&mut self.inventory_arr[slot as usize]};
        *item_stack = item;
        self.update_available_recipes();

        self.item_change_event.send(InventoryItemChangeEvent::new(slot));
    }
    /**gets an item from an inventory slot*/
    pub fn get_item(&self, slot: i32) -> &ItemStack {
        if slot < -1 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("slot out of bounds");
            return &self.inventory_arr[0];
        }
        if slot == -1 {
            return &self.mouse_item;
        }
        &self.inventory_arr[slot as usize]
    }
    /**gets a mutable reference to an item from an inventory slot*/
    pub fn get_item_mut(&mut self, slot: i32) -> &mut ItemStack {
        if slot < -1 || slot >= INVENTORY_SIZE as i32 {
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("slot out of bounds");
            return &mut self.inventory_arr[0];
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
            #[cfg(debug_assertions)]//only panic in debug mode
            panic!("slot out of bounds");
            return;
        }
        std::mem::swap(&mut self.inventory_arr[slot as usize], &mut self.mouse_item);
    }
    /**increases the stack count of an item in an inventory slot*/
    pub fn increase_stack(&mut self, slot: i32, stack: i32) -> i32 {
        let mut stack_to_be = self.get_item(slot).stack + stack;
        let result;
        if stack_to_be > self.get_item(slot).item_type.max_stack {
            stack_to_be = self.get_item(slot).item_type.max_stack;
        }
        result = stack_to_be - self.get_item(slot).stack;
        self.set_item(slot, ItemStack::new(Rc::clone(&self.get_item(slot).item_type), stack_to_be));
        result
    }
    /**decreases the stack count of an item in an inventory slot*/
    pub fn decrease_stack(&mut self, slot: i32, stack: i32) -> i32 {
        if stack > self.get_item(slot).stack {
            let prev_stack = self.get_item(slot).stack;
            let nothing_type = Rc::clone(&self.items.borrow().nothing);
            self.set_item(slot, ItemStack::new(nothing_type, 0));
            prev_stack
        } else {
            self.set_item(slot, ItemStack::new(Rc::clone(&self.get_item(slot).item_type), self.get_item(slot).stack - stack));
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
    pub fn deserialize(&mut self, data: Vec<u8>) {
        let mut i = 0;
        for item in &mut self.inventory_arr {
            let item_id = i32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
            let stack = i32::from_le_bytes([data[i + 4], data[i + 5], data[i + 6], data[i + 7]]);
            *item = ItemStack::new(Rc::clone(&self.items.borrow().get_item_type(item_id)), stack);
            i += 8;
        }
    }

    /**sets the inventory's player*/
    pub fn set_player(&mut self, player: &mut Player) {
        self.player_id = player.get_id();
    }
    /**sets the inventory's blocks reference.
    This will soon be deprecated, do not use. it is here just as a temporary placeholder for testing!*/
    pub fn set_blocks(&mut self, blocks: SharedMut<Blocks>) {//TODO: remove
        self.blocks = blocks;
    }
}