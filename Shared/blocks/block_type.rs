use std::sync::Arc;
use rlua::{ToLua, UserDataMethods};
use crate::blocks::BlockId;
use crate::blocks::tool::Tool;

/**
Includes properties for each block type
 */
#[derive(Clone)]
pub struct BlockType {
    // tool that can break the block, none means it can be broken by hand or any tool
    // TODO: implement for lua
    pub effective_tool: Option<Arc<Tool>>,
    // how powerful the tool needs to be
    pub required_tool_power: i32,
    // ghost blocks are blocks that are not solid and can be walked through
    pub ghost: bool,
    // transparent blocks are blocks that can be seen through and let light through
    pub transparent: bool,
    // name of the block
    pub name: String,
    // to which blocks it visually connects
    // TODO: implement for lua
    pub connects_to: Vec<BlockId>,
    // how much time it takes to break the block
    pub break_time: i32,
    // what light color the block emits
    pub light_emission_r: u8,
    pub light_emission_g: u8,
    pub light_emission_b: u8,
    // block id, used for saving and loading and for networking
    pub(super) id: BlockId,
    // if the block is larger than 1x1 it connects with other blocks of the same type
    // and those blocks break and place together, for example: canopies
    pub width: i32,
    pub height: i32,
    // if the block has any different states for connecting to other blocks
    pub can_update_states: bool,
    // if the block is only collidable by feet, for example: platforms, they have special collision
    pub feet_collidable: bool,
}

/**
make BlockType Lua compatible, implement getter and setter for every field except id and image
 */
impl rlua::UserData for BlockType {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to access fields, id and image are not accessible
        methods.add_meta_method(rlua::MetaMethod::Index, |lua_ctx, this, key: String| {
            match key.as_str() {
                "required_tool_power" => Ok(this.required_tool_power.to_lua(lua_ctx).unwrap()),
                "ghost" => Ok(this.ghost.to_lua(lua_ctx).unwrap()),
                "transparent" => Ok(this.transparent.to_lua(lua_ctx).unwrap()),
                "name" => Ok(this.name.clone().to_lua(lua_ctx).unwrap()),
                "break_time" => Ok(this.break_time.to_lua(lua_ctx).unwrap()),
                "light_emission_r" => Ok(this.light_emission_r.to_lua(lua_ctx).unwrap()),
                "light_emission_g" => Ok(this.light_emission_g.to_lua(lua_ctx).unwrap()),
                "light_emission_b" => Ok(this.light_emission_b.to_lua(lua_ctx).unwrap()),
                "width" => Ok(this.width.to_lua(lua_ctx).unwrap()),
                "height" => Ok(this.height.to_lua(lua_ctx).unwrap()),
                "can_update_states" => Ok(this.can_update_states.to_lua(lua_ctx).unwrap()),
                "feet_collidable" => Ok(this.feet_collidable.to_lua(lua_ctx).unwrap()),
                _ => Err(rlua::Error::RuntimeError(format!("{} is not a valid field of BlockType", key))),
            }
        });
        // add meta method to set fields, id and image are not accessible
        methods.add_meta_method_mut(rlua::MetaMethod::NewIndex, |_lua_ctx, this, (key, value): (String, rlua::Value)| {
            match key.as_str() {
                "required_tool_power" => {
                    match value {
                        rlua::Value::Integer(i) => this.required_tool_power = i as i32,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for required_tool_power")))
                    }
                    Ok(())
                },
                "ghost" => {
                    match value {
                        rlua::Value::Boolean(b) => this.ghost = b,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for ghost")))
                    }
                    Ok(())
                },
                "transparent" => {
                    match value {
                        rlua::Value::Boolean(b) => this.transparent = b,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for transparent")))
                    }
                    Ok(())
                },
                "name" => {
                    match value {
                        rlua::Value::String(s) => this.name = s.to_str().unwrap().to_string(),
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for name")))
                    }
                    Ok(())
                },
                "break_time" => {
                    match value {
                        rlua::Value::Integer(i) => this.break_time = i as i32,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for break_time")))
                    }
                    Ok(())
                },
                "light_emission_r" => {
                    match value {
                        rlua::Value::Integer(i) => this.light_emission_r = i as u8,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for light_emission_r")))
                    }
                    Ok(())
                },
                "light_emission_g" => {
                    match value {
                        rlua::Value::Integer(i) => this.light_emission_g = i as u8,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for light_emission_g")))
                    }
                    Ok(())
                },
                "light_emission_b" => {
                    match value {
                        rlua::Value::Integer(i) => this.light_emission_b = i as u8,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for light_emission_b")))
                    }
                    Ok(())
                },
                "width" => {
                    match value {
                        rlua::Value::Integer(i) => this.width = i as i32,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for width")))
                    }
                    Ok(())
                },
                "height" => {
                    match value {
                        rlua::Value::Integer(i) => this.height = i as i32,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for height")))
                    }
                    Ok(())
                },
                "can_update_states" => {
                    match value {
                        rlua::Value::Boolean(b) => this.can_update_states = b,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for can_update_states")))
                    }
                    Ok(())
                },
                "feet_collidable" => {
                    match value {
                        rlua::Value::Boolean(b) => this.feet_collidable = b,
                        _ => return Err(rlua::Error::RuntimeError(format!("value is not a valid value for feet_collidable")))
                    }
                    Ok(())
                },
                _ => Err(rlua::Error::RuntimeError(format!("{} is not a valid field of BlockType", key))),
            }
        });
    }
}
impl BlockType {
    /**
    Creates a new block type with default values
     */
    pub fn new() -> Self {
        BlockType {
            effective_tool: None,
            required_tool_power: 0,
            ghost: false, transparent: false,
            name: "".to_string(),
            connects_to: vec![],
            break_time: 0,
            light_emission_r: 0,
            light_emission_g: 0,
            light_emission_b: 0,
            id: BlockId::new(),
            width: 0,
            height: 0,
            can_update_states: false,
            feet_collidable: false,
        }
    }

    /**
    This function returns the block id
     */
    pub fn get_id(&self) -> BlockId {
        self.id
    }
}

/**
Block types are equal if they have the same id
 */
impl PartialEq for BlockType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}