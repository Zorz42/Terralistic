use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use std::sync::{Arc, Mutex};

use serde_derive::{Deserialize, Serialize};
use snap;

use crate::blocks::{BlockType, BreakingBlock, Tool};
use crate::mod_manager::ModManager;
use anyhow::{Result};

pub const BLOCK_WIDTH: i32 = 8;
pub const RENDER_SCALE: f32 = 2.0;
pub const RENDER_BLOCK_WIDTH: i32 = (BLOCK_WIDTH as f32 * RENDER_SCALE) as i32;
pub const UNBREAKABLE: i32 = -1;
pub const CHUNK_SIZE: i32 = 16;
pub const RANDOM_TICK_SPEED: i32 = 10;

/**
A welcome packet that carries all the information about the world blocks
 */
#[derive(Serialize, Deserialize)]
pub struct BlocksWelcomePacket {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

/**
A chunk is a 16x16 area of blocks
 */
#[derive(Clone)]
pub(super) struct BlockChunk {
    pub(super) breaking_blocks_count: i8,
}

impl BlockChunk {
    pub fn new() -> Self { BlockChunk{breaking_blocks_count: 0} }
}

#[derive(Serialize, Deserialize)]
pub(super) struct BlocksData {
    pub blocks: Vec<Vec<BlockId>>,
    pub width: i32,
    pub height: i32,
    // tells how much blocks a block in a big block is from the main block, it is mostly 0, 0 so it is stored in a hashmap
    pub block_from_main: HashMap<(i32, i32), (i32, i32)>,
    // saves the block data, it is mostly empty so it is stored in a hashmap
    pub block_data: HashMap<(i32, i32), Vec<u8>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId {
    pub(super) id: i8,
}

impl BlockId {
    pub fn new() -> Self {
        Self {
            id: -1
        }
    }
}

// make BlockId lua compatible
impl rlua::UserData for BlockId {
    // implement equals comparison for BlockId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: BlockId| {
            Ok(this.id == other.id)
        });
    }
}

/**
A world is a 2d array of blocks and chunks.
 */
pub struct Blocks {
    pub(super) block_data: BlocksData,
    pub(super) chunks: Vec<Vec<BlockChunk>>,
    pub(super) breaking_blocks: Vec<BreakingBlock>,
    pub(super) block_types: Arc<Mutex<Vec<BlockType>>>,
    pub(super) tool_types: Vec<Tool>,
    pub air: BlockId,
}

impl Blocks{
    pub fn new() -> Self {
        let mut result = Self{
            block_data: BlocksData {
                blocks: Vec::new(),
                block_from_main: HashMap::new(),
                block_data: HashMap::new(),
                width: 0,
                height: 0,
            },
            chunks: vec![],
            breaking_blocks: vec![],
            block_types: Arc::new(Mutex::new(vec![])),
            tool_types: vec![],
            air: BlockId::new(),
        };

        let mut air = BlockType::new();
        air.name = "air".to_string();
        air.ghost = true;
        air.transparent = true;
        air.break_time = UNBREAKABLE;
        result.air = Self::register_new_block_type(&mut result.block_types.lock().unwrap_or_else(|e| e.into_inner()) , air);

        result
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        mods.add_global_function("new_block_type", move |_lua, _: ()| {
            Ok(BlockType::new())
        });

        let block_types = self.block_types.clone();
        mods.add_global_function("register_block_type", move |_lua, block_type: BlockType| {
            let result = Self::register_new_block_type(&mut block_types.lock().unwrap_or_else(|e| e.into_inner()), block_type);
            Ok(result)
        });

        let block_types = self.block_types.clone();
        mods.add_global_function("get_block_id_by_name", move |_lua, name: String| {
            let block_types = block_types.lock().unwrap_or_else(|e| e.into_inner());
            for block_type in block_types.iter() {
                if block_type.name == name {
                    return Ok(block_type.get_id());
                }
            }
            Err(rlua::Error::RuntimeError("Block type not found".to_string()))
        });

        // a method to connect two blocks
        let block_types = self.block_types.clone();
        mods.add_global_function("connect_blocks", move |_lua, (block_id1, block_id2): (BlockId, BlockId)| {
            let mut block_types = block_types.lock().unwrap_or_else(|e| e.into_inner());
            block_types[block_id1.id as usize].connects_to.push(block_id2);
            block_types[block_id2.id as usize].connects_to.push(block_id1);
            Ok(())
        });
    }

    /**
    Creates an empty world with given width and height
     */
    pub fn create(&mut self, width: i32, height: i32) -> Result<()> {
        if width < 0 || height < 0 {
            return Err(NegativeDimensionError{}.into());
        }

        self.block_data.width = width;
        self.block_data.height = height;

        self.block_data.blocks = vec![vec![BlockId::new(); width as usize]; height as usize];
        self.chunks = vec![vec![BlockChunk::new(); (height / CHUNK_SIZE) as usize]; (width / CHUNK_SIZE) as usize];
        Ok(())
    }

    /**
    This function creates a world from a 2d vector of block type ids
     */
    pub fn create_from_block_ids(&mut self, block_ids: Vec<Vec<BlockId>>) -> Result<()> {
        let width = block_ids.len() as i32;
        let height;
        if let Some(row) = block_ids.get(0) {
            height = row.len() as i32;
        } else {
            return Err(RowMismatchError{}.into());
        }

        // check that all the rows have the same length
        for row in &block_ids {
            if row.len() as i32 != height {
                return Err(RowMismatchError{}.into());
            }
        }

        self.create(width, height)?;
        self.block_data.blocks = block_ids;
        Ok(())
    }

    /**
    This function returns the block id at given position
     */
    pub fn get_block(&self, x: i32, y: i32) -> Result<BlockId> {
        if let Some(row) = self.block_data.blocks.get(x as usize) {
            if let Some(block_id) = row.get(y as usize) {
                return Ok(*block_id);
            }
        }
        Err(CoordinateOutOfBoundsError{}.into())
    }

    /**
    This sets the type of a block from a coordinate.
     */
    pub fn set_big_block(&mut self, x: i32, y: i32, block_id: BlockId, from_main: (i32, i32)) -> Result<()> {
        if block_id != self.get_block(x, y)? || from_main != self.get_block_from_main(x, y) {
            self.set_block_data(x, y, vec![]);
            self.block_data.blocks[x as usize][y as usize] = block_id;

            for i in 0..self.breaking_blocks.len() {
                if self.breaking_blocks[i].x == x && self.breaking_blocks[i].y == y {
                    self.breaking_blocks.remove(i);
                    break;
                }
            }
            self.set_block_from_main(x, y, from_main);
            //let event = BlockChangeEvent::new(x, y);
            //self.block_change_event.send(event);
        }
        Ok(())
    }

    /**
    This sets the type of a block from a coordinate.
     */
    pub fn set_block(&mut self, x: i32, y: i32, block_id: BlockId) {
        self.set_big_block(x, y, block_id, (0, 0));
    }

    /**
    This function returns the chunk at given position
     */
    pub(super) fn get_chunk(&mut self, x: i32, y: i32) -> &mut BlockChunk {
        &mut self.chunks[x as usize][y as usize]
    }

    /**
    This is used to get a block type from its id, it is used for serialization.
     */
    pub fn get_block_type_by_id(&self, id: BlockId) -> BlockType {
        self.block_types.lock().unwrap_or_else(|e| e.into_inner())[id.id as usize].clone()
    }

    /**
    This function sets x and y from main for a block. If it is 0, 0 the value is removed from the hashmap.
     */
    pub(super) fn set_block_from_main(&mut self, x: i32, y: i32, from_main: (i32, i32)) {
        if from_main.0 == 0 && from_main.1 == 0 {
            self.block_data.block_from_main.remove(&(x, y));
        } else {
            self.block_data.block_from_main.insert((x, y), from_main);
        }
    }

    /**
    This function gets the block from main for a block. If the value is not found, it returns 0, 0.
     */
    pub(super) fn get_block_from_main(&self, x: i32, y: i32) -> (i32, i32) {
        *self.block_data.block_from_main.get(&(x, y)).unwrap_or(&(0, 0))
    }

    /**
    This function sets the block data for a block. If it is empty the value is removed from the hashmap.
     */
    pub(super) fn set_block_data(&mut self, x: i32, y: i32, data: Vec<u8>) {
        if data.is_empty() {
            self.block_data.block_data.remove(&(x, y));
        } else {
            self.block_data.block_data.insert((x, y), data);
        }
    }

    /**
    This function returns block data, if it is not found it returns an empty vector.
     */
    pub(super) fn get_block_data(&self, x: i32, y: i32) -> Vec<u8> {
        self.block_data.block_data.get(&(x, y)).unwrap_or(&vec![]).clone()
    }

    /**
    Returns the width of the world in blocks.
     */
    pub fn get_width(&self) -> i32 {
        self.block_data.width
    }

    /**
    Returns the height of the world in blocks.
     */
    pub fn get_height(&self) -> i32 {
        self.block_data.height
    }

    /**
    Serializes the world, used for saving the world and sending it to the client.
     */
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&bincode::serialize(&self.block_data)?)?)
    }

    /**
    Deserializes the world, used for loading the world and receiving it from the server.
     */
    pub fn deserialize(&mut self, serial: &Vec<u8>) -> Result<()> {
        self.block_data = bincode::deserialize(&snap::raw::Decoder::new().decompress_vec(serial)?)?;
        Ok(())
    }

    /**
    This function adds a new block type, but is used internally by mods.
     */
    fn register_new_block_type(block_types: &mut Vec<BlockType>, mut block_type: BlockType) -> BlockId {
        let id = block_types.len() as i8;
        let result = BlockId{ id };
        block_type.id = result;
        block_types.push(block_type);
        result
    }

    /**
    Returns the block type that has the specified name, used
    with commands to get the block type from the name.
     */
    pub fn get_block_id_by_name(&mut self, name: String) -> Result<BlockId> {
        for block_type in self.block_types.lock().unwrap_or_else(|e| e.into_inner()).iter() {
            if block_type.name == name {
                return Ok(block_type.id);
            }
        }
        Err(BlockNotFoundError{}.into())
    }

    /**
    Returns all block ids.
     */
    pub fn get_all_block_ids(&mut self) -> Vec<BlockId> {
        let mut result = Vec::new();
        for block_type in self.block_types.lock().unwrap_or_else(|e| e.into_inner()).iter() {
            result.push(block_type.id);
        }
        result
    }

    /**
    Returns the block type that has the specified id.
     */
    pub fn get_block_type(&self, id: BlockId) -> Result<BlockType> {
        let blocks = self.block_types.lock().unwrap_or_else(|e| e.into_inner());
        Ok(blocks.get(id.id as usize).ok_or(BlockNotFoundError{})?.clone())
    }

    /**
    Returns the block type at specified coordinates.
     */
    pub fn get_block_type_at(&self, x: i32, y: i32) -> Result<BlockType> {
        self.get_block_type(self.get_block(x, y)?)
    }
}

/**
Event that is fired when a block is changed
 */
pub struct BlockChangeEvent {
    pub x: i32, pub y: i32
}
impl BlockChangeEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockChangeEvent{x, y} }
}
//impl Event for BlockChangeEvent {}

/**
Event that is fired when a random tick is fired for a block
 */
struct BlockRandomTickEvent {
    x: i32, y: i32
}
impl BlockRandomTickEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockRandomTickEvent{x, y} }
}
//impl Event for BlockRandomTickEvent {}

/**
Event that is fired when a block is updated
 */
pub struct BlockUpdateEvent {
    x: i32, y: i32
}
impl BlockUpdateEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockUpdateEvent{x, y} }
}
//impl Event for BlockUpdateEvent {}

pub struct RowMismatchError {}

impl std::fmt::Display for RowMismatchError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "The number of rows in the block data is not equal to the height of the world")
    }
}

impl Debug for RowMismatchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The number of rows in the block data is not equal to the height of the world")
    }
}

impl std::error::Error for RowMismatchError {}

pub struct NegativeDimensionError {}

impl std::fmt::Display for NegativeDimensionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "The width or height of the world is negative")
    }
}

impl Debug for NegativeDimensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The width or height of the world is negative")
    }
}

impl std::error::Error for NegativeDimensionError {}

pub struct CoordinateOutOfBoundsError {}

impl std::fmt::Display for CoordinateOutOfBoundsError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "The coordinates are out of bounds")
    }
}

impl Debug for CoordinateOutOfBoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The coordinates are out of bounds")
    }
}

impl std::error::Error for CoordinateOutOfBoundsError {}

struct BlockNotFoundError {}

impl std::fmt::Display for BlockNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "The block was not found")
    }
}

impl Debug for BlockNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The block was not found")
    }
}

impl std::error::Error for BlockNotFoundError {}