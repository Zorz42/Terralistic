use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::shared::blocks::{Block, BlockId, Blocks, Tool, ToolId};
use crate::shared::mod_manager::ModManager;

// make BlockId lua compatible
impl rlua::FromLua<'_> for BlockId {
    fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
        match value {
            rlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}

impl rlua::UserData for BlockId {
    // implement equals comparison for BlockId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: Self| Ok(this.id == other.id));
    }
}

/// initialize the mod interface for the blocks module
#[allow(clippy::too_many_lines)]
pub fn init_blocks_mod_interface(blocks: &Arc<Mutex<Blocks>>, mods: &mut ModManager) -> Result<()> {
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
            block_type.light_emission = (light_emission_r, light_emission_g, light_emission_b);
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
        let blocks = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);

        blocks.get_block_id_by_name(&name).map_err(|e| rlua::Error::RuntimeError(e.to_string()))
    })?;

    // a method to connect two blocks
    let blocks_clone = blocks.clone();
    mods.add_global_function("connect_blocks", move |_lua, (block_id1, block_id2): (BlockId, BlockId)| {
        let block_types = &mut blocks_clone.lock().unwrap_or_else(PoisonError::into_inner).block_types;
        block_types.get_mut(block_id1).map_err(|e| rlua::Error::RuntimeError(e.to_string()))?.connects_to.push(block_id2);
        block_types.get_mut(block_id2).map_err(|e| rlua::Error::RuntimeError(e.to_string()))?.connects_to.push(block_id1);
        Ok(())
    })?;

    // a method to get block id by position
    let blocks_clone = blocks.clone();
    mods.add_global_function("get_block", move |_lua, (x, y): (i32, i32)| {
        let block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let block_id = block_types.get_block(x, y).ok().ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;
        Ok(block_id)
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

    // a method to get block inventory items by position
    let blocks_clone = blocks.clone();
    mods.add_global_function("get_block_inventory_items", move |_lua, (x, y): (i32, i32)| {
        let block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let inventory = block_types
            .get_block_inventory_data(x, y)
            .ok()
            .ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;
        let mut res = Vec::new();
        for item in inventory {
            if let Some(item) = item {
                res.push(Some(item.item));
            } else {
                res.push(None);
            }
        }

        Ok(res)
    })?;

    // a method to get block inventory item counts by position
    let blocks_clone = blocks.clone();
    mods.add_global_function("get_block_inventory_item_counts", move |_lua, (x, y): (i32, i32)| {
        let block_types = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let inventory = block_types
            .get_block_inventory_data(x, y)
            .ok()
            .ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;
        let mut res = Vec::new();
        for item in inventory {
            if let Some(item) = item {
                res.push(Some(item.count));
            } else {
                res.push(None);
            }
        }

        Ok(res)
    })?;

    Ok(())
}

/// make `ToolId` Lua compatible
impl rlua::UserData for ToolId {}

impl rlua::FromLua<'_> for ToolId {
    fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
        match value {
            rlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}
