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

    let blocks_clone = blocks.clone();
    mods.add_global_function(
        "register_block_type",
        move |_lua,
              (
            effective_tool,
            required_tool_power,
            ghost,
            transparent,
            name,
            connects_to,
            break_time,
            light_emission_r,
            light_emission_g,
            light_emission_b,
            width,
            height,
            can_update_states,
            feet_collidable,
            clickable,
            inventory_slots,
        ): (
            Option<ToolId>,
            i32,
            bool,
            bool,
            String,
            Vec<BlockId>,
            Option<i32>,
            u8,
            u8,
            u8,
            i32,
            i32,
            bool,
            bool,
            bool,
            Vec<Vec<i32>>,
        )| {
            let mut block_type = Block::new();

            // turn Vec<Vec<i32>> into Vec<(i32, i32)>
            let inventory_slots = {
                let mut inventory_slots2 = Vec::new();
                for slot in inventory_slots {
                    let val1 = slot.first().ok_or(rlua::Error::RuntimeError("invalid inventory slot".to_owned()))?;
                    let val2 = slot.get(1).ok_or(rlua::Error::RuntimeError("invalid inventory slot".to_owned()))?;

                    inventory_slots2.push((*val1, *val2));
                }
                inventory_slots2
            };

            block_type.effective_tool = effective_tool;
            block_type.required_tool_power = required_tool_power;
            block_type.ghost = ghost;
            block_type.transparent = transparent;
            block_type.name = name;
            block_type.connects_to = connects_to;
            block_type.break_time = break_time;
            block_type.light_emission_r = light_emission_r;
            block_type.light_emission_g = light_emission_g;
            block_type.light_emission_b = light_emission_b;
            block_type.width = width;
            block_type.height = height;
            block_type.can_update_states = can_update_states;
            block_type.feet_collidable = feet_collidable;
            block_type.clickable = clickable;
            block_type.inventory_slots = inventory_slots;

            let result = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner).register_new_block_type(block_type);

            Ok(result)
        },
    )?;

    let blocks_clone = blocks.clone();
    mods.add_global_function("get_block_id_by_name", move |_lua, name: String| {
        let block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);

        let iter = block_types.block_types.iter();
        for block_type in iter {
            if block_type.name == name {
                return Ok(block_type.get_id());
            }
        }
        Err(rlua::Error::RuntimeError("Block type not found".to_owned()))
    })?;

    // a method to connect two blocks
    let blocks_clone = blocks.clone();
    mods.add_global_function("connect_blocks", move |_lua, (block_id1, block_id2): (BlockId, BlockId)| {
        let block_types = &mut blocks_clone.lock().unwrap_or_else(PoisonError::into_inner).block_types;
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
    let blocks_clone = blocks.clone();
    let sender_clone = sender.clone();
    mods.add_global_function("break_block", move |_lua, (x, y): (i32, i32)| {
        let mut block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let mut events = EventManager::new();
        block_types
            .break_block(&mut events, x, y)
            .ok()
            .ok_or(rlua::Error::RuntimeError("block type id is invalid".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender_clone.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }
        Ok(())
    })?;

    // a method to get block id by position
    let blocks_clone = blocks.clone();
    mods.add_global_function("get_block", move |_lua, (x, y): (i32, i32)| {
        let block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let block_id = block_types.get_block(x, y).ok().ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;
        Ok(block_id)
    })?;

    // a method to set block id by position
    let blocks_clone = blocks.clone();
    let sender2 = sender;
    mods.add_global_function("set_block", move |_lua, (x, y, block_id): (i32, i32, BlockId)| {
        let mut events = EventManager::new();

        let mut blocks = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
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
    let blocks_clone = blocks.clone();
    mods.add_global_function("register_tool", move |_lua, name: String| {
        let mut block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
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

/// make `ToolId` Lua compatible
impl rlua::UserData for ToolId {}
