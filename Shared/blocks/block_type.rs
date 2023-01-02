use std::borrow::BorrowMut;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use snap;
use graphics as gfx;
use shared_mut::SharedMut;
use crate::blocks::blocks::Blocks;
use crate::blocks::tool::Tool;
use crate::mod_manager::ModManager;

/**
Includes properties for each block type
 */
pub struct BlockType{
    // tool that can break the block, none means it can be broken by hand or any tool
    pub effective_tool: Option<*const Tool>,
    // how powerful the tool needs to be
    pub required_tool_power: i32,
    // ghost blocks are blocks that are not solid and can be walked through
    pub ghost: bool,
    // transparent blocks are blocks that can be seen through and let light through
    pub transparent: bool,
    // name of the block\
    pub name: String,
    // to which blocks it visually connects
    pub connects_to: Vec<i32>,
    // how much time it takes to break the block
    pub break_time: i32,
    // what light color the block emits
    pub light_emission_r: u8,
    pub light_emission_g: u8,
    pub light_emission_b: u8,
    // block id, used for saving and loading and for networking
    pub id: i32,
    // if the block is larger than 1x1 it connects with other blocks of the same type
    // and those blocks break and place together, for example: canopies
    pub width: i32,
    pub height: i32,
    // if the block has any different states for connecting to other blocks
    pub can_update_states: bool,
    // if the block is only collidable by feet, for example: platforms, they have special collision
    pub feet_collidable: bool,
    // the image of the block
    pub image: gfx::Surface,
}

/**
make BlockType Lua compatible
 */
impl rlua::UserData for BlockType {

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
            id: 0,
            width: 0,
            height: 0,
            can_update_states: false,
            feet_collidable: false,
            image: gfx::Surface::new(0, 0),
        }
    }

    /**
    This function returns the block id
     */
    pub fn get_id(&self) -> i32 {
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