use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, PoisonError};

use crate::libraries::events::{Event, EventManager};
use anyhow::Result;

use crate::shared::blocks::{BlockId, ToolId};
use crate::shared::entities::{Entities, EntityId, PositionComponent};
use crate::shared::inventory::Inventory;
use crate::shared::items::{Item, ItemId, ItemStack, Items, Recipe, TileDrop};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::WallId;

// make ItemId lua compatible
impl rlua::UserData for ItemId {}

/// this function initializes the items mod interface
/// it adds lua functions to the lua context
pub fn init_items_mod_interface(items: &Arc<Mutex<Items>>, entities: &Arc<Mutex<Entities>>, mods: &mut ModManager) -> Result<Receiver<Event>> {
    let items_clone = items.clone();
    mods.add_global_function(
        "register_item_type",
        move |_lua, (name, display_name, max_stack, places_block, places_wall, tool, tool_power): (String, String, i32, Option<BlockId>, Option<WallId>, Option<ToolId>, i32)| {
            let mut item_type = Item::new();

            item_type.name = name;
            item_type.display_name = display_name;
            item_type.max_stack = max_stack;
            item_type.places_block = places_block;
            item_type.places_wall = places_wall;
            item_type.tool = tool;
            item_type.tool_power = tool_power;

            let result = Items::register_new_item_type(&mut items_clone.lock().unwrap_or_else(PoisonError::into_inner).item_types, item_type);
            Ok(result)
        },
    )?;

    let items_clone = items.clone();
    mods.add_global_function("get_item_id_by_name", move |_lua, name: String| {
        let item_types = &items_clone.lock().unwrap_or_else(PoisonError::into_inner).item_types;
        let iter = item_types.iter();
        for item_type in iter {
            if item_type.name == name {
                return Ok(item_type.get_id());
            }
        }
        Err(rlua::Error::RuntimeError("Item type not found".to_owned()))
    })?;

    let items_clone = items.clone();
    mods.add_global_function("set_block_drop", move |_lua, (block_id, item_id, chance): (BlockId, ItemId, f32)| {
        items_clone.lock().unwrap_or_else(PoisonError::into_inner).block_drops.insert(block_id, TileDrop::new(item_id, chance));
        Ok(())
    })?;

    let items_clone = items.clone();
    mods.add_global_function(
        "register_recipe",
        move |_lua, (result, result_count, ingredients, ingredients_count): (ItemId, i32, Vec<ItemId>, Vec<i32>)| {
            let mut recipe = Recipe::new();
            recipe.result = ItemStack::new(result, result_count);

            for (item, count) in ingredients.iter().zip(ingredients_count.iter()) {
                recipe.ingredients.insert(*item, *count);
            }

            items_clone.lock().unwrap_or_else(PoisonError::into_inner).add_recipe(recipe);

            Ok(())
        },
    )?;

    let (sender, receiver) = std::sync::mpsc::channel();

    let items_clone = items.clone();
    let entities_clone = entities.clone();
    mods.add_global_function("give_item", move |_lua, (entity_id, item_id, count): (EntityId, ItemId, i32)| {
        let entity = entities_clone.lock().unwrap_or_else(PoisonError::into_inner).get_entity_from_id(entity_id);
        let entity = match entity {
            Ok(entity) => entity,
            Err(err) => return Err(rlua::Error::RuntimeError(err.to_string())),
        };

        let ecs = &mut entities_clone.lock().unwrap_or_else(PoisonError::into_inner).ecs;
        let player_pos = {
            let player_pos = ecs.query_one_mut::<&PositionComponent>(entity);
            let player_pos = match player_pos {
                Ok(player_pos) => player_pos,
                Err(err) => return Err(rlua::Error::RuntimeError(err.to_string())),
            };
            player_pos.clone()
        };
        let mut inventory = {
            let inventory = ecs.query_one_mut::<&mut Inventory>(entity);
            let inventory = match inventory {
                Ok(inventory) => inventory,
                Err(err) => return Err(rlua::Error::RuntimeError(err.to_string())),
            };
            inventory.clone()
        };
        let mut events = EventManager::new();
        let res = inventory.give_item(
            ItemStack::new(item_id, count),
            (player_pos.x(), player_pos.y()),
            &mut items_clone.lock().unwrap_or_else(PoisonError::into_inner),
            &mut entities_clone.lock().unwrap_or_else(PoisonError::into_inner),
            &mut events,
        );
        if let Err(e) = res {
            return Err(rlua::Error::RuntimeError(e.to_string()));
        }

        while let Some(event) = events.pop_event() {
            let res = sender.send(event);
            if let Err(e) = res {
                return Err(rlua::Error::RuntimeError(e.to_string()));
            }
        }

        Ok(())
    })?;

    Ok(receiver)
}
