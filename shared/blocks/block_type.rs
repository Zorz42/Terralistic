use super::tool::Tool;
use super::BlockId;
use rlua::UserDataMethods;
extern crate alloc;
use alloc::sync::Arc;

/**
Includes properties for each block type
 */
#[derive(Clone)]
pub struct Block {
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

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

/**
make `BlockType` Lua compatible, implement getter and setter for every field except id and image
 */
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

impl Block {
    /// Creates a new block type with default values
    #[must_use]
    pub const fn new() -> Self {
        Self {
            effective_tool: None,
            required_tool_power: 0,
            ghost: false,
            transparent: false,
            name: String::new(),
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

    /// This function returns the block id
    #[must_use]
    pub const fn get_id(&self) -> BlockId {
        self.id
    }
}

/// Block types are equal if they have the same id
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
