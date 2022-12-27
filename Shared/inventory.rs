use std::{iter::Map, rc::Rc};
use std::borrow::Borrow;
use std::mem::MaybeUninit;
use super::{items::*, blocks::*, player::*};
use {events::*};
use shared_mut::SharedMut;

const INVENTORY_SIZE: usize = 10;

/**struct that describes a single crafting recipe*/
struct Recipe {
    pub ingredients: Map<Rc<ItemType>, i32>,
    pub result: ItemStack,
    //if the recipe can only be crafted near a block (crafting table), it's listed here
    pub crafting_block: Option<Rc<BlockType>>,
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
    recipes: Vec<Recipe>,
}
impl Recipes {
    pub fn new() -> Self {
        Recipes{ recipes: Vec::new() }
    }
    pub fn register_a_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }
    pub fn get_all_recipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }
}

/**struct that defines the inventory and everything related to it*/
pub struct Inventory {
    items: SharedMut<Items>,
    recipes: Recipes,
    players: SharedMut<Players>,
    blocks: SharedMut<Blocks>,
    mouse_item: ItemStack,
    item_counts: Vec<i32>,
    available_recipes: Vec<Recipe>,
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

        let mut inventory = Inventory{
            items,
            recipes: Recipes::new(),
            players,
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
}