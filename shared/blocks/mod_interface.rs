use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::{Block, BlockBreakEvent, BlockId, Blocks, Tool, ToolId};
use crate::shared::mod_manager::ModManager;

// make BlockId lua compatible
impl rlua::UserData for BlockId {
    // implement equals comparison for BlockId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: Self| Ok(this.id == other.id));
    }
}

/// initialize the mod interface for the blocks module
#[allow(clippy::too_many_lines)]
pub fn init_blocks_mod_interface(blocks: &Arc<Mutex<Blocks>>, mods: &mut ModManager) -> Result<Receiver<Event>> {
    let (sender, receiver) = std::sync::mpsc::channel();

    mods.add_global_function("new_block_type", move |_lua, ()| Ok(Block::new()))?;

    let mut blocks2 = blocks.clone();
    mods.add_global_function("register_block_type", move |_lua, block_type: Block| {
        let result = blocks2.lock().unwrap_or_else(PoisonError::into_inner).register_new_block_type(block_type);

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
    mods.add_global_function("connect_blocks", move |_lua, (block_id1, block_id2): (BlockId, BlockId)| {
        let block_types = &mut blocks2.lock().unwrap_or_else(PoisonError::into_inner).block_types;
        block_types
            .get_mut(block_id1.id as usize)
            .ok_or(rlua::Error::RuntimeError("block type id is invalid".to_owned()))?
            .connects_to
            .push(block_id2);
        block_types
            .get_mut(block_id2.id as usize)
            .ok_or(rlua::Error::RuntimeError("block type id is invalid".to_owned()))?
            .connects_to
            .push(block_id1);
        Ok(())
    })?;

    // a method to break a block
    blocks2 = blocks.clone();
    let sender2 = sender.clone();
    mods.add_global_function("break_block", move |_lua, (x, y): (i32, i32)| {
        let mut block_types = blocks2.lock().unwrap_or_else(PoisonError::into_inner);
        let mut events = EventManager::new();
        block_types
            .break_block(&mut events, x, y)
            .ok()
            .ok_or(rlua::Error::RuntimeError("block type id is invalid".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender2.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }
        Ok(())
    })?;

    // a method to get block id by position
    blocks2 = blocks.clone();
    mods.add_global_function("get_block", move |_lua, (x, y): (i32, i32)| {
        let block_types = blocks2.lock().unwrap_or_else(PoisonError::into_inner);
        let block_id = block_types.get_block(x, y).ok().ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;
        Ok(block_id)
    })?;

    // a method to set block id by position
    blocks2 = blocks.clone();
    let sender2 = sender;
    mods.add_global_function("set_block", move |_lua, (x, y, block_id): (i32, i32, BlockId)| {
        let mut events = EventManager::new();

        let mut blocks = blocks2.lock().unwrap_or_else(PoisonError::into_inner);
        blocks
            .set_block(&mut events, x, y, block_id)
            .ok()
            .ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender2.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }

        Ok(())
    })?;

    // a method to register a new tool
    blocks2 = blocks.clone();
    mods.add_global_function("register_tool", move |_lua, name: String| {
        let mut block_types = blocks2.lock().unwrap_or_else(PoisonError::into_inner);
        let mut tool = Tool::new();
        tool.name = name;
        let tool_id = block_types.register_new_tool_type(tool);
        Ok(tool_id)
    })?;

    Ok(receiver)
}

pub fn handle_event_for_blocks_interface(mods: &mut ModManager, event: &Event) -> Result<()> {
    if let Some(event) = event.downcast::<BlockBreakEvent>() {
        for game_mod in mods.mods_iter_mut() {
            if game_mod.is_symbol_defined("on_block_break")? {
                game_mod.call_function("on_block_break", (event.x, event.y, event.prev_block_id))?;
            }
        }
    }
    Ok(())
}

/// make `BlockType` Lua compatible, implement getter and setter for every field except id
impl rlua::UserData for Block {
    #[allow(clippy::too_many_lines)]
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to set fields, id and image are not accessible
        methods.add_meta_method_mut(rlua::MetaMethod::NewIndex, |_lua_ctx, this, (key, value): (String, rlua::Value)| match value {
            rlua::Value::Integer(value) => {
                match key.as_str() {
                    "required_tool_power" => this.required_tool_power = value as i32,
                    "break_time" => this.break_time = Some(value as i32),
                    "light_emission_r" => this.light_emission_r = value as u8,
                    "light_emission_g" => this.light_emission_g = value as u8,
                    "light_emission_b" => this.light_emission_b = value as u8,
                    "width" => this.width = value as i32,
                    "height" => this.height = value as i32,
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of BlockType for integer value")));
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
                    "clickable" => this.clickable = value,
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of BlockType for boolean value")));
                    }
                };
                Ok(())
            }
            rlua::Value::String(value) => {
                match key.as_str() {
                    "name" => this.name = value.to_str().unwrap_or("undefined").to_owned(),
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of BlockType for string value")));
                    }
                };
                Ok(())
            }
            rlua::Value::Table(value) => {
                match key.as_str() {
                    "inventory_slots" => {
                        let len = value.len()?;
                        for curr_val in 1..=len {
                            let element = value.get::<_, rlua::Value>(curr_val)?;
                            if let rlua::Value::Table(element) = element {
                                let mut slot = (0, 0);
                                let len2 = element.len()?;
                                if len2 != 2 {
                                    return Err(rlua::Error::RuntimeError("Invalid table value for inventory_slots".to_owned()));
                                }

                                let element1 = element.get::<_, rlua::Value>(1)?;
                                let element2 = element.get::<_, rlua::Value>(2)?;

                                if let rlua::Value::Integer(element1) = element1 {
                                    slot.0 = element1 as i32;
                                } else {
                                    return Err(rlua::Error::RuntimeError("Invalid table value for inventory_slots".to_owned()));
                                }

                                if let rlua::Value::Integer(element2) = element2 {
                                    slot.1 = element2 as i32;
                                } else {
                                    return Err(rlua::Error::RuntimeError("Invalid table value for inventory_slots".to_owned()));
                                }

                                this.inventory_slots.push(slot);
                            } else {
                                return Err(rlua::Error::RuntimeError("Invalid table value for inventory_slots".to_owned()));
                            }
                        }
                    }
                    _ => {
                        return Err(rlua::Error::RuntimeError("{key} is not a valid field of BlockType for table value".to_owned()));
                    }
                };
                Ok(())
            }
            rlua::Value::UserData(value) => {
                match key.as_str() {
                    "effective_tool" => {
                        this.effective_tool = Some(*value.borrow::<ToolId>()?);
                    }
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of Block for userdata value")));
                    }
                };
                Ok(())
            }
            _ => Err(rlua::Error::RuntimeError("Not a valid value type of BlockType".to_owned())),
        });
    }
}

/// make `ToolId` Lua compatible
impl rlua::UserData for ToolId {}
