use std::borrow::BorrowMut;
use std::ops::Deref;
use std::rc::Rc;
use events::*;
use serde_derive::{Serialize, Deserialize};
use super::block_data::*;
use snap;

pub const BLOCK_WIDTH: i32 = 8;
pub const UNBREAKABLE: i32 = -1;
pub const CHUNK_SIZE: i32 = 16;
pub const RANDOM_TICK_SPEED: i32 = 10;

//TODO: write tests for block changes, block data, and block updates

/*
structs for events that modify blocks
*/
struct BlockChangeEvent {
    x: i32, y: i32
}
impl BlockChangeEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockChangeEvent{x, y} }
}
impl Event for BlockChangeEvent {}

struct BlockRandomTickEvent {
    x: i32, y: i32
}
impl BlockRandomTickEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockRandomTickEvent{x, y} }
}
impl Event for BlockRandomTickEvent {}

struct BlockBreakEvent {
    x: i32, y: i32
}
impl BlockBreakEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockBreakEvent{x, y} }
}
impl Event for BlockBreakEvent {}

struct BlockStartedBreakingEvent {
    x: i32, y: i32
}
impl BlockStartedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStartedBreakingEvent{x, y} }
}
impl Event for BlockStartedBreakingEvent {}

struct BlockStoppedBreakingEvent {
    x: i32, y: i32
}
impl BlockStoppedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStoppedBreakingEvent{x, y} }
}
impl Event for BlockStoppedBreakingEvent {}

struct BlockUpdateEvent {
    x: i32, y: i32
}
impl BlockUpdateEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockUpdateEvent{x, y} }
}
impl Event for BlockUpdateEvent {}

pub struct Tool {
    name: String
}

impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}

/*
includes properties for each block type
*/
pub struct BlockType{
    pub effective_tool: Option<*const Tool>,
    pub required_tool_power: i32,
    pub ghost: bool, transparent: bool,
    pub name: String,
    pub connects_to: Vec<i32>,
    pub break_time: i32,
    pub light_emission_r: i32, light_emission_g: i32, light_emission_b: i32,
    pub id: i32,
    pub width: i32, height: i32,
    pub block_data_index: i32,//TODO: future me change this to a reference if possible
    pub can_update_states: bool,
    pub feet_collidable: bool,
    pub custom_data_type: Option<CustomData>
}

impl BlockType {
    pub fn new(name: String) -> Self {
        BlockType {
            effective_tool: None,
            required_tool_power: 0,
            ghost: false, transparent: false,
            name,
            connects_to: vec![],
            break_time: 0,
            light_emission_r: 0, light_emission_g: 0, light_emission_b: 0,
            id: 0,
            width: 0, height: 0,
            block_data_index: 0,
            can_update_states: false,
            feet_collidable: false,
            custom_data_type: Option::None
        }
    }
    pub fn update_states(blocks: &mut Blocks, x: i32, y: i32) -> i32 {
        if blocks.get_block_type(x, y).can_update_states {
            let mut state = 0;
            if blocks.update_state_side(x, y,  0, -1){
                state += 1 << 0;
            }
            if blocks.update_state_side(x, y,  1,  0){
                state += 1 << 1;
            }
            if blocks.update_state_side(x, y,  0,  1){
                state += 1 << 2;
            }
            if blocks.update_state_side(x, y, -1,  0){
                state += 1 << 3;
            }
            state
        }else {
            0
        }
    }
}


struct Block{
    id: i32,
    x_from_main: i8, y_from_main: i8,
    block_data: Option<CustomData>
}

impl Block{
    pub fn new() -> Self { Block{id: 0, x_from_main: 0, y_from_main: 0, block_data: Option::None} }
}

struct BreakingBlock{
    break_progress: i32,
    is_breaking: bool,
    x: i32, y: i32,
}

impl BreakingBlock{
    pub fn new() -> Self { BreakingBlock{break_progress: 0, is_breaking: true, x: 0, y: 0} }
}


#[derive(Clone)]
struct BlockChunk{
    breaking_blocks_count: i8,
}

impl BlockChunk{
    pub fn new() -> Self { BlockChunk{breaking_blocks_count: 0} }
}


pub struct Blocks{
    blocks: Vec<Block>,
    chunks: Vec<BlockChunk>,
    width: i32, height: i32,
    breaking_blocks: Vec<BreakingBlock>,
    block_types: Vec<Rc<BlockType>>,
    tool_types: Vec<Rc<Tool>>,
    air: Rc<BlockType>,
    hand: Rc<Tool>,
    block_change_event: Sender<BlockChangeEvent>,
    block_break_event: Sender<BlockBreakEvent>,
    block_started_breaking_event: Sender<BlockStartedBreakingEvent>,
    block_stopped_breaking_event: Sender<BlockStoppedBreakingEvent>,
    block_update_event: Sender<BlockUpdateEvent>,
}

impl Blocks{
    pub fn new() -> Self {
        let mut b = Blocks{
            blocks: vec![],
            chunks: vec![],
            width: 0, height: 0,
            breaking_blocks: vec![],
            block_types: vec![],
            tool_types: vec![],
            air: Rc::new(BlockType::new("_".to_string())),
            hand: Rc::new(Tool::new("hand".to_string())),
            block_change_event: Sender::new(),
            block_break_event: Sender::new(),
            block_started_breaking_event: Sender::new(),
            block_stopped_breaking_event: Sender::new(),
            block_update_event: Sender::new(),

        };

        let mut air = BlockType::new(String::from("air"));
        air.ghost = true;
        air.transparent = true;
        air.break_time = UNBREAKABLE;
        air.light_emission_r = 0;
        air.light_emission_g = 0;
        air.light_emission_b = 0;
        air.can_update_states = false;
        b.register_new_block_type(air);
        b.air = Rc::clone(&b.block_types[0]);

        let hand = Rc::new(Tool::new(String::from("hand")));
        b.register_new_tool_type(Rc::clone(&hand));
        b
    }

    fn get_block(&self, x: i32, y: i32) -> &Block {
        if x < 0 || y < 0 || x >= self.width || y >= self.height || self.blocks.is_empty() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }
        &self.blocks[(y * self.width + x) as usize]
    }

    fn get_block_mut(&mut self, x: i32, y: i32) -> &mut Block {
        if x < 0 || y < 0 || x >= self.width || y >= self.height || self.blocks.is_empty() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }
        &mut self.blocks[(y * self.width + x) as usize]
    }

    fn get_chunk_mut(&mut self, x: i32, y: i32) -> &mut BlockChunk {
        if x < 0 || y < 0 || x >= self.width / CHUNK_SIZE || y >= self.height / CHUNK_SIZE || self.chunks.is_empty() {
            panic!("Chunk is accessed out of bounds! x: {}, y: {}", x, y);
        }
        self.chunks[(y * self.width / CHUNK_SIZE + x) as usize].borrow_mut()
    }

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
        self.chunks = vec![BlockChunk::new(); (width * height / CHUNK_SIZE / CHUNK_SIZE) as usize];
    }

    pub fn get_block_type_by_id(&self, id: i32) -> Rc<BlockType> {
        if id < 0 || id >= self.block_types.len() as i32 {
            panic!("Block type id is out of bounds! id: {}", id);
        }
        Rc::clone(&self.block_types[id as usize])
    }

    pub fn get_block_type(&self, x: i32, y: i32) -> Rc<BlockType> {
        let id = self.get_block(x, y).id.into();
        self.get_block_type_by_id(id)
    }

    pub fn set_block_type(&mut self, x: i32, y: i32, block_type: Rc<BlockType>, x_from_main: i8, y_from_main: i8) {
        if block_type.id != self.get_block(x, y).id {
            self.set_block_type_silently(x, y, block_type);
            for i in 0..self.breaking_blocks.len() {
                if self.breaking_blocks[i].x == x && self.breaking_blocks[i].y == y {
                    self.breaking_blocks.remove(i);
                    break;
                }
            }
            self.get_block_mut(x, y).x_from_main = x_from_main;
            self.get_block_mut(x, y).y_from_main = y_from_main;
            let event = BlockChangeEvent::new(x, y);
            self.block_change_event.send(event);
        }
    }

    pub fn set_block_type_silently(&mut self, x: i32, y: i32, block_type: Rc<BlockType>) {
        self.get_block_mut(x, y).block_data = Option::None;
        self.get_block_mut(x, y).id = block_type.id;
        let data = block_type.custom_data_type.clone();
        self.get_block_mut(x, y).block_data = data;
    }

    pub fn get_block_x_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).x_from_main
    }

    pub fn get_block_y_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).y_from_main
    }

    pub fn get_block_data(&mut self, x: i32, y: i32) -> &Option<CustomData> {
        &self.get_block(x, y).block_data
    }

    pub fn get_break_progress(&mut self, x: i32, y: i32) -> i32 {
        for breaking_block in self.breaking_blocks.iter() {
            if breaking_block.x == x && breaking_block.y == y {
                return breaking_block.break_progress;
            }
        }
        0
    }

    pub fn get_break_stage(&mut self, x: i32, y: i32) -> i32 {
        (self.get_break_progress(x, y) as f64 / self.get_block_type(x, y).break_time as f64 * 9.0) as i32
    }

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

        let event = BlockStartedBreakingEvent::new(x, y);
        self.block_started_breaking_event.send(event);
    }

    pub fn stop_breaking_block(&mut self, x: i32, y: i32){
        if x < 0 || y < 0 || x >= self.width || y >= self.height{
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }

        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.x == x && breaking_block.y == y {
                breaking_block.is_breaking = false;
                self.get_chunk_mut(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count -= 1;
                let event = BlockStoppedBreakingEvent::new(x, y);
                self.block_stopped_breaking_event.send(event);
                break;
            }
        }
    }

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
    pub fn break_block(&mut self, x: i32, y: i32){
        let transformed_x = x - self.get_block_x_from_main(x, y) as i32;
        let transformed_y = y - self.get_block_y_from_main(x, y) as i32;

        let event = BlockBreakEvent::new(transformed_x, transformed_y);
        self.block_break_event.send(event);
        self.set_block_type(transformed_x, transformed_y, Rc::clone(&self.get_block_type(x, y)), 0, 0);
    }

    pub fn get_chunk_breaking_blocks_count(&mut self, x: i32, y: i32) -> i32 {
        self.get_chunk_mut(x, y).breaking_blocks_count.into()
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn to_serial(&mut self) -> Vec<u8> {
        let mut serial: Vec<u8> = Vec::new();
        let mut iter: u32 = 0;
        serial.resize(serial.len() + (self.width * self.height * 6 + 8) as usize, 0);//TODO: use serialize
        serial[(iter    ) as usize] = (self.width >> 24) as u8;
        serial[(iter + 1) as usize] = (self.width >> 16) as u8;
        serial[(iter + 2) as usize] = (self.width >>  8) as u8;
        serial[(iter + 3) as usize] = (self.width      ) as u8;
        iter += 4;
        serial[(iter    ) as usize] = (self.height >> 24) as u8;
        serial[(iter + 1) as usize] = (self.height >> 16) as u8;
        serial[(iter + 2) as usize] = (self.height >>  8) as u8;
        serial[(iter + 3) as usize] = (self.height      ) as u8;
        iter += 4;
        for(_i, block) in self.blocks.iter().enumerate() {
            serial[(iter    ) as usize] = (block.id >> 24) as u8;
            serial[(iter + 1) as usize] = (block.id >> 16) as u8;
            serial[(iter + 2) as usize] = (block.id >>  8) as u8;
            serial[(iter + 3) as usize] = (block.id      ) as u8;
            serial[(iter + 4) as usize] = block.x_from_main as u8;
            serial[(iter + 5) as usize] = block.y_from_main as u8;
            iter += 6;
            if block.block_data.is_some() {
                serial.resize(serial.len() + block.block_data.as_ref().unwrap().serial_length as usize, 0);
                for i in block.block_data.as_ref().unwrap().data.iter() {
                    serial[iter as usize] = *i;
                    iter += 1;
                }
            }

        }
        snap::raw::Encoder::new().
            compress_vec(&serial).unwrap()
    }

    pub fn from_serial(&mut self, serial: Vec<u8>){//TODO: implement serialize/deserialize on whole blocks?
        let mut iter: u32 = 0;
        let decompressed = snap::raw::Decoder::new().decompress_vec(&serial).unwrap();
        let width  = (decompressed[(iter    ) as usize] as i32) << 24 | (decompressed[(iter + 1) as usize] as i32) << 16 | (decompressed[(iter + 2) as usize] as i32) << 8 | decompressed[(iter + 3) as usize] as i32;
        let height = (decompressed[(iter + 4) as usize] as i32) << 24 | (decompressed[(iter + 5) as usize] as i32) << 16 | (decompressed[(iter + 6) as usize] as i32) << 8 | decompressed[(iter + 7) as usize] as i32;
        iter += 8;
        self.create(width, height);

        for i in 0..self.blocks.len() {
            self.blocks[i].id = (decompressed[iter as usize] as i32) << 24 | (decompressed[(iter + 1) as usize] as i32) << 16 | (decompressed[(iter + 2) as usize] as i32) << 8 | decompressed[(iter + 3) as usize] as i32;
            self.blocks[i].x_from_main = decompressed[(iter + 1) as usize] as i8;
            self.blocks[i].y_from_main = decompressed[(iter + 2) as usize] as i8;
            iter += 3;


            self.blocks[i].block_data = self.get_block_type_by_id(self.blocks[i].id).custom_data_type.clone();
            if self.blocks[i].block_data.is_some() {
                self.blocks[i].block_data.as_mut().unwrap().data = decompressed[iter as usize..(iter + self.blocks[i].block_data.as_ref().unwrap().serial_length) as usize].to_vec();
            }
        }
    }

    pub fn register_new_block_type(&mut self, mut block_type: BlockType) -> Rc<BlockType>{
        block_type.id = self.block_types.len() as i32;
        self.block_types.push(Rc::new(block_type));
        Rc::clone(self.block_types.last_mut().unwrap())
    }

    pub fn get_block_type_by_name(&mut self, name: String) -> Option<Rc<BlockType>> {
        for block_type in self.block_types.iter() {
            if block_type.name == name {
                return Some(Rc::clone(block_type));
            }
        }
        None
    }

    pub fn get_number_block_types(&mut self) -> i32 {
        self.block_types.len() as i32
    }

    pub fn register_new_tool_type(&mut self, tool_type: Rc<Tool>){
        self.tool_types.push(tool_type);
    }

    pub fn get_tool_type_by_name(&mut self, name: String) -> Option<Rc<Tool>> {
        for tool_type in self.tool_types.iter() {
            if tool_type.name == name {
                return Some(Rc::clone(tool_type));
            }
        }
        None
    }

    pub fn update_state_side(&mut self, x: i32, y: i32, side_x: i32, side_y: i32) -> bool {
        let this_block_id = self.get_block(x, y).id;
        let side_block_id = self.get_block(x + side_x, y + side_y).id;
        x + side_x >= self.width || x + side_x < 0 || y + side_y >= self.height || y + side_y < 0 ||
            self.get_block_type(x + side_x, y + side_y).id == this_block_id ||
            self.get_block_type(x, y).connects_to.contains(&side_block_id)
    }
}

