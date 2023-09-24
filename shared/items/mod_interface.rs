use crate::shared::blocks::{BlockId, ToolId};
use crate::shared::items::{Item, ItemId, ItemStack, Items, Recipe, TileDrop};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::WallId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use anyhow::Result;

// make ItemId lua compatible
impl rlua::UserData for ItemId {}

/// this function initializes the items mod interface
/// it adds lua functions to the lua context
/// # Errors
/// if the function fails to add the lua functions
pub fn init_items_mod_interface(items: &Arc<Mutex<Items>>, mods: &mut ModManager) -> Result<()> {
    mods.add_global_function("new_item_type", move |_lua, _: ()| Ok(Item::new()))?;

    let items_clone = items.clone();
    mods.add_global_function("register_item_type", move |_lua, item_type: Item| {
        let result = Items::register_new_item_type(
            &mut items_clone
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .item_types,
            item_type,
        );
        Ok(result)
    })?;

    let items_clone = items.clone();
    mods.add_global_function("get_item_id_by_name", move |_lua, name: String| {
        let item_types = &items_clone
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .item_types;
        let iter = item_types.iter();
        for item_type in iter {
            if item_type.name == name {
                return Ok(item_type.get_id());
            }
        }
        Err(rlua::Error::RuntimeError("Item type not found".to_owned()))
    })?;

    let items_clone = items.clone();
    mods.add_global_function(
        "set_block_drop",
        move |_lua, (block_id, item_id, chance): (BlockId, ItemId, f32)| {
            items_clone
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .block_drops
                .insert(block_id, TileDrop::new(item_id, chance));
            Ok(())
        },
    )?;

    let items_clone = items.clone();
    mods.add_global_function(
        "register_recipe",
        move |_lua,
              (result, result_count, ingredients, ingredients_count): (
            ItemId,
            i32,
            Vec<ItemId>,
            Vec<i32>,
        )| {
            let mut recipe = Recipe {
                result: ItemStack {
                    item: result,
                    count: result_count,
                },
                ingredients: HashMap::new(),
            };

            for (item, count) in ingredients.iter().zip(ingredients_count.iter()) {
                recipe.ingredients.insert(*item, *count);
            }

            items_clone
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .add_recipe(recipe);

            Ok(())
        },
    )?;

    Ok(())
}

// make item lua compatible
impl rlua::UserData for Item {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(
            rlua::MetaMethod::NewIndex,
            |_, this, (key, value): (String, rlua::Value)| match value {
                rlua::Value::Integer(value) => {
                    match key.as_str() {
                        "max_stack" => this.max_stack = value as i32,
                        "width" => this.width = value as f32,
                        "height" => this.height = value as f32,
                        "tool_power" => this.tool_power = value as i32,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for integer value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::String(value) => {
                    match key.as_str() {
                        "name" => this.name = value.to_str().unwrap_or("undefined").to_owned(),
                        "display_name" => {
                            this.display_name = value.to_str().unwrap_or("undefined").to_owned();
                        }
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for string value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::UserData(value) => {
                    match key.as_str() {
                        "places_block" => {
                            this.places_block = Some(*value.borrow::<BlockId>()?);
                        }
                        "places_wall" => {
                            this.places_wall = Some(*value.borrow::<WallId>()?);
                        }
                        "tool" => {
                            this.tool = Some(*value.borrow::<ToolId>()?);
                        }
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for userdata value"
                            )))
                        }
                    };
                    Ok(())
                }
                _ => Err(rlua::Error::RuntimeError(
                    "Not a valid value type of Item".to_owned(),
                )),
            },
        );
    }
}
