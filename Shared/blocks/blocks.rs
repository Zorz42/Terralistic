use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::Arc;
use serde_derive::{Serialize, Deserialize};
use snap;
use graphics as gfx;
use shared_mut::SharedMut;
use crate::blocks::block_type::BlockType;
use crate::blocks::events::BlockBreakEvent;
use crate::blocks::tool::Tool;
use crate::mod_manager::ModManager;

pub const BLOCK_WIDTH: i32 = 8;
pub const RENDER_SCALE: f32 = 0.5;
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
Block struct represents a state of a block in a world.
 */
#[derive(Clone, Deserialize, Serialize)]
pub struct Block {
    pub id: i32,
    pub x_from_main: i8,
    pub y_from_main: i8,
    pub block_data: Vec<u8>
}

impl Block {
    pub fn new() -> Self {
        Self {
            id: 0,
            x_from_main: 0,
            y_from_main: 0,
            block_data: vec![]
        }
    }
}

/**
Breaking block struct represents a block that is currently being broken.
 */
struct BreakingBlock {
    break_progress: i32,
    is_breaking: bool,
    x: i32,
    y: i32,
}

impl BreakingBlock {
    pub fn new() -> Self {
        Self {
            break_progress: 0,
            is_breaking: true,
            x: 0,
            y: 0
        }
    }
}


/**
A chunk is a 16x16 area of blocks
 */
struct BlockChunk {
    breaking_blocks_count: i8,
}

impl BlockChunk {
    pub fn new() -> Self { BlockChunk{breaking_blocks_count: 0} }
}


/**
A world is a 2d array of blocks and chunks.
 */
pub struct Blocks {
    blocks: Vec<Block>,
    chunks: Vec<BlockChunk>,
    width: i32,
    height: i32,
    breaking_blocks: Vec<BreakingBlock>,
    block_types: SharedMut<Vec<Arc<BlockType>>>,
    curr_block_id: i32,
    tool_types: Vec<Rc<Tool>>,
    pub air: Arc<BlockType>,
}

impl Blocks{
    pub fn new() -> Self {
        let mut result = Self{
            blocks: vec![],
            chunks: vec![],
            width: 0, height: 0,
            breaking_blocks: vec![],
            block_types: SharedMut::new(vec![]),
            tool_types: vec![],
            air: Arc::new(BlockType::new()),
            curr_block_id: 2,
        };

        let mut air = BlockType::new();
        air.name = "air".to_string();
        air.ghost = true;
        air.transparent = true;
        air.break_time = UNBREAKABLE;
        result.air = result.register_new_block_type(air);

        result
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        let block_types = self.block_types.clone();
        mods.add_global_function("new_block_type", move |lua, _: ()| {
            Ok(BlockType::new())
        });

        mods.add_global_function("register_block_type", move |lua, block_type: BlockType| {
            let result = Self::_register_new_block_type(block_types.clone(), block_type);
            Ok(result.id)
        });
    }

    /**
    This gets the block from x and y coordinates
     */
    pub fn get_block(&self, x: i32, y: i32) -> &Block {
        if x < 0 || y < 0 || x >= self.width || y >= self.height || self.blocks.is_empty() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }
        &self.blocks[(y * self.width + x) as usize]
    }

    /**
    This gets the mutable reference of a block from x and y coordinates
     */
    fn get_block_mut(&mut self, x: i32, y: i32) -> &mut Block {
        if x < 0 || y < 0 || x >= self.width || y >= self.height || self.blocks.is_empty() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }
        &mut self.blocks[(y * self.width + x) as usize]
    }

    /**
    This gets the mutable reference of a chunk from x and y coordinates
     */
    fn get_chunk_mut(&mut self, x: i32, y: i32) -> &mut BlockChunk {
        if x < 0 || y < 0 || x >= self.width / CHUNK_SIZE || y >= self.height / CHUNK_SIZE || self.chunks.is_empty() {
            panic!("Chunk is accessed out of bounds! x: {}, y: {}", x, y);
        }
        self.chunks[(y * self.width / CHUNK_SIZE + x) as usize].borrow_mut()
    }

    /**
    Creates an empty world with given width and height
     */
    pub fn create(&mut self, width: i32, height: i32) {
        if width < 0 || height < 0 {
            panic!("Width and height must be positive!");
        }
        self.width = width;
        self.height = height;
        self.blocks = Vec::new();
        self.blocks.resize((width * height) as usize, Block::new());

        self.chunks = vec![];
        self.chunks.reserve((width * height / CHUNK_SIZE / CHUNK_SIZE) as usize);
        for _ in 0..(width * height / CHUNK_SIZE / CHUNK_SIZE) as usize {
            self.chunks.push(BlockChunk::new());
        }
    }

    /**
    This is used to get a block type from its id, it is used for serialization.
     */
    pub fn get_block_type_by_id(&self, id: i32) -> Arc<BlockType> {
        if id < 0 || id >= self.block_types.borrow().len() as i32 {
            panic!("Block type id is out of bounds! id: {}", id);
        }
        self.block_types.borrow()[id as usize].clone()
    }

    /**
    This is used to get a block type from from a coordinate.
     */
    pub fn get_block_type(&self, x: i32, y: i32) -> Arc<BlockType> {
        let id = self.get_block(x, y).id.into();
        self.get_block_type_by_id(id)
    }

    /**
    This sets the type of a block from a coordinate.
     */
    pub fn set_big_block(&mut self, x: i32, y: i32, block_type: Arc<BlockType>, x_from_main: i8, y_from_main: i8) {
        if block_type.get_id() != self.get_block(x, y).id {
            self.set_block_silently(x, y, block_type);
            for i in 0..self.breaking_blocks.len() {
                if self.breaking_blocks[i].x == x && self.breaking_blocks[i].y == y {
                    self.breaking_blocks.remove(i);
                    break;
                }
            }
            self.get_block_mut(x, y).x_from_main = x_from_main;
            self.get_block_mut(x, y).y_from_main = y_from_main;
            //let event = BlockChangeEvent::new(x, y);
            //self.block_change_event.send(event);
        }
    }

    /**
    This sets the type of a block from a coordinate.
     */
    pub fn set_block(&mut self, x: i32, y: i32, block_type: Arc<BlockType>) {
        self.set_big_block(x, y, block_type, 0, 0);
    }

    /**
    This sets the type of a block from a coordinate without sending a block change event
    and other stuff, just pure block type change. This is potentially dangerous, use with caution.
    It can be used for stuff like loading a world, generating a world, etc.
     */
    pub fn set_block_silently(&mut self, x: i32, y: i32, block_type: Arc<BlockType>) {
        self.get_block_mut(x, y).block_data.clear();
        self.get_block_mut(x, y).id = block_type.get_id();
    }

    /**
    Gets the x from main property of a block used for big blocks.
     */
    pub fn get_block_x_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).x_from_main
    }

    /**
    Gets the y from main property of a block used for big blocks.
     */
    pub fn get_block_y_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).y_from_main
    }

    /**
    Gets the raw block data of a block. Used for blocks that have extra data,
    like chests, furnaces, etc.
     */
    pub fn get_block_data(&mut self, x: i32, y: i32) -> &Vec<u8> {
        &self.get_block(x, y).block_data
    }

    /**
    Gets the breaking progress of a block.
     */
    pub fn get_break_progress(&mut self, x: i32, y: i32) -> i32 {
        for breaking_block in self.breaking_blocks.iter() {
            if breaking_block.x == x && breaking_block.y == y {
                return breaking_block.break_progress;
            }
        }
        0
    }

    /**
    Sets the break stage of a block, which is usually rendered.
     */
    pub fn get_break_stage(&mut self, x: i32, y: i32) -> i32 {
        (self.get_break_progress(x, y) as f64 / self.get_block_type(x, y).break_time as f64 * 9.0) as i32
    }

    /**
    Adds a block to the breaking list, which means that the block is being broken.
     */
    pub fn start_breaking_block(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }

        let mut breaking_block: Option<&mut BreakingBlock> = None;
        for i in self.breaking_blocks.iter_mut() {
            if i.x == x && i.y == y {
                breaking_block = Some(i);
                break;
            }
        }

        if breaking_block.is_none() {
            let mut new_breaking_block = BreakingBlock::new();
            new_breaking_block.x = x;
            new_breaking_block.y = y;
            self.breaking_blocks.push(new_breaking_block);
            breaking_block = Some(self.breaking_blocks.last_mut().unwrap());
        }

        breaking_block.unwrap().is_breaking = true;

        self.get_chunk_mut(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count += 1;

        //let event = BlockStartedBreakingEvent::new(x, y);
        //self.block_started_breaking_event.send(event);
    }

    /**
    Stops breaking a block.
     */
    pub fn stop_breaking_block(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }

        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.x == x && breaking_block.y == y {
                breaking_block.is_breaking = false;
                self.get_chunk_mut(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count -= 1;
                //let event = BlockStoppedBreakingEvent::new(x, y);
                //self.block_stopped_breaking_event.send(event);
                break;
            }
        }
    }

    /**
    Updates the breaking progress of all blocks that are being broken.
     */
    pub fn update_breaking_blocks(&mut self, frame_length: f64) {
        for i in 0..self.breaking_blocks.len() {
            if self.breaking_blocks[i].is_breaking {
                self.breaking_blocks[i].break_progress += frame_length as i32;
                if self.breaking_blocks[i].break_progress > self.get_block_type(self.breaking_blocks[i].x, self.breaking_blocks[i].y).break_time {
                    self.break_block(self.breaking_blocks[i].x, self.breaking_blocks[i].y);
                }
            }
        }
    }

    /**
    Breaks a block and triggers the block break event which can be used to drop items.
     */
    pub fn break_block(&mut self, x: i32, y: i32){
        let transformed_x = x - self.get_block_x_from_main(x, y) as i32;
        let transformed_y = y - self.get_block_y_from_main(x, y) as i32;

        let event = BlockBreakEvent::new(transformed_x, transformed_y);
        //self.block_break_event.send(event);
        //self.set_block_type(transformed_x, transformed_y, Rc::clone(&self.get_block_type(x, y)), 0, 0);
    }

    /**
    It only gets the breaking block count of a chunk, used to skip
    updating chunks that don't have any breaking blocks.
     */
    pub fn get_chunk_breaking_blocks_count(&mut self, x: i32, y: i32) -> i32 {
        self.get_chunk_mut(x, y).breaking_blocks_count.into()
    }

    /**
    Returns the width of the world in blocks.
     */
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /**
    Returns the height of the world in blocks.
     */
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /**
    Serializes the world, used for saving the world and sending it to the client.
     */
    pub fn serialize(&mut self) -> Vec<u8> {
        snap::raw::Encoder::new().
            compress_vec(&bincode::
            serialize(&self.blocks).unwrap()).unwrap()
    }

    /**
    Deserializes the world, used for loading the world and receiving it from the server.
     */
    pub fn deserialize(&mut self, serial: Vec<u8>){
        self.blocks = bincode::
            deserialize(&snap::raw::Decoder::new().
            decompress_vec(&serial).unwrap()).unwrap();
    }

    /**
    This function adds a new block type, but is used internally by mods.
     */
    pub fn _register_new_block_type(block_types: SharedMut<Vec<Arc<BlockType>>>, mut block_type: BlockType) -> Arc<BlockType> {
        block_type.id = block_types.borrow().len() as i32;
        block_types.borrow().push(Arc::new(block_type));
        Arc::clone(block_types.borrow().last_mut().unwrap())
    }

    /**
    Adds a new block type to the world.
     */
    pub fn register_new_block_type(&mut self, mut block_type: BlockType) -> Arc<BlockType> {
        Self::_register_new_block_type(self.block_types.clone(), block_type)
    }

    /**
    Returns the block type that has the specified name, used
    with commands to get the block type from the name.
     */
    pub fn get_block_type_by_name(&mut self, name: String) -> Option<Arc<BlockType>> {
        for block_type in self.block_types.borrow().iter() {
            if block_type.name == name {
                return Some(block_type.clone());
            }
        }
        None
    }

    /**
    Returns the number of block types that are registered.
     */
    pub fn get_number_block_types(&mut self) -> i32 {
        self.block_types.borrow().len() as i32
    }

    /**
    Adds a new tool type to the world.
     */
    pub fn register_new_tool_type(&mut self, tool_type: Rc<Tool>){
        self.tool_types.push(tool_type);
    }

    /**
    Returns the tool type that has the specified name
     */
    pub fn get_tool_type_by_name(&mut self, name: String) -> Option<Rc<Tool>> {
        for tool_type in self.tool_types.iter() {
            if tool_type.name == name {
                return Some(Rc::clone(tool_type));
            }
        }
        None
    }

    /**
    Returns true, if the block and its neighbor are connected.
    Used for rendering.
     */
    fn update_state_side(&mut self, x: i32, y: i32, side_x: i32, side_y: i32) -> bool {
        let this_block_id = self.get_block(x, y).id;
        let side_block_id = self.get_block(x + side_x, y + side_y).id;
        x + side_x >= self.width || x + side_x < 0 || y + side_y >= self.height || y + side_y < 0 ||
            self.get_block_type(x + side_x, y + side_y).get_id() == this_block_id ||
            self.get_block_type(x, y).connects_to.contains(&side_block_id)
    }

    /**
    Updates the state of a block, used for rendering.
    The state of the block is used to determine which texture to use.
     */
    pub fn update_states(&mut self, x: i32, y: i32) -> i32 {
        if self.get_block_type(x, y).can_update_states {
            let mut state = 0;
            if self.update_state_side(x, y,  0, -1) {
                state += 1;
            }
            if self.update_state_side(x, y,  1,  0) {
                state += 2;
            }
            if self.update_state_side(x, y,  0,  1) {
                state += 4;
            }
            if self.update_state_side(x, y, -1,  0) {
                state += 8;
            }
            state
        } else {
            0
        }
    }
}