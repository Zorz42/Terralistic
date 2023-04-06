extern crate alloc;
use crate::shared::blocks::{Block, BlockId, Blocks};
use crate::shared::mod_manager::ModManager;
use alloc::sync::Arc;
use anyhow::Result;
use rlua::UserDataMethods;
use std::sync::{Mutex, PoisonError};

/// initialize the mod interface for the blocks module
/// # Errors
/// if the lua context is not available
pub fn init_blocks_mod_interface(blocks: &Arc<Mutex<Blocks>>, mods: &mut ModManager) -> Result<()> {
    mods.add_global_function("new_block_type", move |_lua, _: ()| Ok(Block::new()))?;

    let mut blocks2 = blocks.clone();
    mods.add_global_function("register_block_type", move |_lua, block_type: Block| {
        let result = blocks2
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .register_new_block_type(block_type);

        Ok(result)
    })?;

    blocks2 = blocks.clone();
    mods.add_global_function("get_block_id_by_name", move |_lua, name: String| {
        let block_types = blocks2.lock().unwrap_or_else(PoisonError::into_inner);

        let iter = block_types.block_types.iter();
        for block_type in iter {
            if block_type.name == name {
                return Ok(block_type.get_id());
            }
        }
        Err(rlua::Error::RuntimeError("Block type not found".to_owned()))
    })?;

    // a method to connect two blocks
    blocks2 = blocks.clone();
    mods.add_global_function(
        "connect_blocks",
        move |_lua, (block_id1, block_id2): (BlockId, BlockId)| {
            let block_types = &mut blocks2
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .block_types;
            block_types
                .get_mut(block_id1.id as usize)
                .ok_or(rlua::Error::RuntimeError(
                    "block type id is invalid".to_owned(),
                ))?
                .connects_to
                .push(block_id2);
            block_types
                .get_mut(block_id2.id as usize)
                .ok_or(rlua::Error::RuntimeError(
                    "block type id is invalid".to_owned(),
                ))?
                .connects_to
                .push(block_id1);
            Ok(())
        },
    )?;

    Ok(())
}

/// make `BlockType` Lua compatible, implement getter and setter for every field except id
impl rlua::UserData for Block {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to set fields, id and image are not accessible
        methods.add_meta_method_mut(
            rlua::MetaMethod::NewIndex,
            |_lua_ctx, this, (key, value): (String, rlua::Value)| match value {
                rlua::Value::Integer(value) => {
                    match key.as_str() {
                        "required_tool_power" => this.required_tool_power = value as i32,
                        "break_time" => this.break_time = value as i32,
                        "light_emission_r" => this.light_emission_r = value as u8,
                        "light_emission_g" => this.light_emission_g = value as u8,
                        "light_emission_b" => this.light_emission_b = value as u8,
                        "width" => this.width = value as i32,
                        "height" => this.height = value as i32,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for integer value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::Boolean(value) => {
                    match key.as_str() {
                        "ghost" => this.ghost = value,
                        "transparent" => this.transparent = value,
                        "can_update_states" => this.can_update_states = value,
                        "feet_collidable" => this.feet_collidable = value,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for boolean value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::String(value) => {
                    match key.as_str() {
                        "name" => this.name = value.to_str().unwrap_or("undefined").to_owned(),
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for string value"
                            )))
                        }
                    };
                    Ok(())
                }
                _ => Err(rlua::Error::RuntimeError(
                    "Not a valid value type of BlockType".to_owned(),
                )),
            },
        );
    }
}
