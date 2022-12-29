use std::borrow::BorrowMut;
use std::rc::Rc;
use deprecated_events::*;
use serde_derive::{Serialize, Deserialize};
use snap;
use graphics as gfx;

pub const BLOCK_WIDTH: i32 = 8;
pub const RENDER_SCALE: f32 = 2.0;
pub const RENDER_BLOCK_WIDTH: i32 = (BLOCK_WIDTH as f32 * RENDER_SCALE) as i32;
pub const UNBREAKABLE: i32 = -1;
pub const CHUNK_SIZE: i32 = 16;
pub const RANDOM_TICK_SPEED: i32 = 10;

//TODO: write tests for block changes, block data, and block updates

/**
Event that is fired when a block is changed
 */
pub struct BlockChangeEvent {
    pub x: i32, pub y: i32
}
impl BlockChangeEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockChangeEvent{x, y} }
}
impl Event for BlockChangeEvent {}

/**
Event that is fired when a random tick is fired for a block
 */
struct BlockRandomTickEvent {
    x: i32, y: i32
}
impl BlockRandomTickEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockRandomTickEvent{x, y} }
}
impl Event for BlockRandomTickEvent {}

/**
Event that is fired when a block is broken
 */
pub struct BlockBreakEvent {
    x: i32, y: i32
}
impl BlockBreakEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockBreakEvent{x, y} }
}
impl Event for BlockBreakEvent {}

/**
Event that is fired when a block has started breaking
 */
pub struct BlockStartedBreakingEvent {
    x: i32, y: i32
}
impl BlockStartedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStartedBreakingEvent{x, y} }
}
impl Event for BlockStartedBreakingEvent {}

/**
Event that is fired when a block has stopped breaking
 */
pub struct BlockStoppedBreakingEvent {
    x: i32, y: i32
}
impl BlockStoppedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStoppedBreakingEvent{x, y} }
}
impl Event for BlockStoppedBreakingEvent {}

/**
Event that is fired when a block is updated
 */
pub struct BlockUpdateEvent {
    x: i32, y: i32
}
impl BlockUpdateEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockUpdateEvent{x, y} }
}
impl Event for BlockUpdateEvent {}

/**
Struct that contains all the information about a tool
 */
pub struct Tool {
    name: String
}

impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}

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
    // name of the block displayed in inventory
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
    id: i32,
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

impl BlockType {
    /**
    Creates a new block type with default values
     */
    pub fn new(name: String) -> Self {
        BlockType {
            effective_tool: None,
            required_tool_power: 0,
            ghost: false, transparent: false,
            name,
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

/**
Block struct represents a state of a block in a world.
 */
#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Block {
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
    block_types: Vec<Rc<BlockType>>,
    tool_types: Vec<Rc<Tool>>,
    pub air: Rc<BlockType>,
    pub test_block: Rc<BlockType>,
}

impl Blocks{
    pub fn new() -> Self {
        let mut result = Self{
            blocks: vec![],
            chunks: vec![],
            width: 0, height: 0,
            breaking_blocks: vec![],
            block_types: vec![],
            tool_types: vec![],
            air: Rc::new(BlockType::new("".to_string())),
            test_block: Rc::new(BlockType::new("".to_string())),
        };

        let mut air = BlockType::new(String::from("air"));
        air.ghost = true;
        air.transparent = true;
        air.break_time = UNBREAKABLE;
        result.air = result.register_new_block_type(air);

        let mut test_block = BlockType::new(String::from("test_block"));
        test_block.break_time = UNBREAKABLE;
        // test_block image is 8x8 of black pixels
        test_block.image = gfx::Surface::new(BLOCK_WIDTH, BLOCK_WIDTH);
        for x in 0..BLOCK_WIDTH {
            for y in 0..BLOCK_WIDTH {
                test_block.image.set_pixel(x, y, gfx::Color::new(0, 0, 0, 255));
            }
        }

        result.test_block = result.register_new_block_type(test_block);

        result
    }

    /**
    This gets the block from x and y coordinates
     */
    pub(crate) fn get_block(&self, x: i32, y: i32) -> &Block {
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
        for i in 0..width * height {
            self.blocks.push(Block::new());
        }
        self.chunks = vec![];
        for _ in 0..(width * height / CHUNK_SIZE / CHUNK_SIZE) as usize {
            self.chunks.push(BlockChunk::new());
        }
    }

    /**
    This is used to get a block type from its id, it is used for serialization.
     */
    pub fn get_block_type_by_id(&self, id: i32) -> Rc<BlockType> {
        if id < 0 || id >= self.block_types.len() as i32 {
            panic!("Block type id is out of bounds! id: {}", id);
        }
        Rc::clone(&self.block_types[id as usize])
    }

    /**
    This is used to get a block type from from a coordinate.
     */
    pub fn get_block_type(&self, x: i32, y: i32) -> Rc<BlockType> {
        let id = self.get_block(x, y).id.into();
        self.get_block_type_by_id(id)
    }

    /**
    This sets the type of a block from a coordinate.
     */
    pub fn set_big_block(&mut self, x: i32, y: i32, block_type: Rc<BlockType>, x_from_main: i8, y_from_main: i8) {
        if block_type.id != self.get_block(x, y).id {
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
    pub fn set_block(&mut self, x: i32, y: i32, block_type: Rc<BlockType>) {
        self.set_big_block(x, y, block_type, 0, 0);
    }

    /**
    This sets the type of a block from a coordinate without sending a block change event
    and other stuff, just pure block type change. This is potentially dangerous, use with caution.
    It can be used for stuff like loading a world, generating a world, etc.
     */
    pub fn set_block_silently(&mut self, x: i32, y: i32, block_type: Rc<BlockType>) {
        self.get_block_mut(x, y).block_data.clear();
        self.get_block_mut(x, y).id = block_type.id;
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
    pub fn start_breaking_block(&mut self, x: i32, y: i32){
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

        if breaking_block.is_none(){
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
    pub fn stop_breaking_block(&mut self, x: i32, y: i32){
        if x < 0 || y < 0 || x >= self.width || y >= self.height{
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
    pub fn update_breaking_blocks(&mut self, frame_length: f64){
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
    Deserializes the world, used for loading the world and receiving it from the client.
     */
    pub fn deserialize(&mut self, serial: Vec<u8>){
        self.blocks = bincode::
            deserialize(&snap::raw::Decoder::new().
            decompress_vec(&serial).unwrap()).unwrap();
    }

    /**
    Adds a new block type to the world.
     */
    pub fn register_new_block_type(&mut self, mut block_type: BlockType) -> Rc<BlockType> {
        block_type.id = self.block_types.len() as i32;
        self.block_types.push(Rc::new(block_type));
        Rc::clone(self.block_types.last_mut().unwrap())
    }

    /**
    Returns the block type that has the specified name, used
    with commands to get the block type from the name.
     */
    pub fn get_block_type_by_name(&mut self, name: String) -> Option<Rc<BlockType>> {
        for block_type in self.block_types.iter() {
            if block_type.name == name {
                return Some(Rc::clone(block_type));
            }
        }
        None
    }

    /**
    Returns the number of block types that are registered.
     */
    pub fn get_number_block_types(&mut self) -> i32 {
        self.block_types.len() as i32
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
            self.get_block_type(x + side_x, y + side_y).id == this_block_id ||
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

