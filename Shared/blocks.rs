use std::borrow::BorrowMut;

const BLOCK_WIDTH: i32 = 8;
const UNBREAKABLE: i32 = -1;
const CHUNK_SIZE: i32 = 16;
const RANDOM_TICK_SPEED: i32 = 10;

/*
structs for events that modify blocks
*/
struct BlockChangeEvent {
    x: i32, y: i32
}
impl BlockChangeEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockChangeEvent{x, y} }
}

struct BlockRandomTickEvent {
    x: i32, y: i32
}
impl BlockRandomTickEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockRandomTickEvent{x, y} }
}

struct BlockBreakEvent {
    x: i32, y: i32
}
impl BlockBreakEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockBreakEvent{x, y} }
}

struct BlockStartedBreakingEvent {
    x: i32, y: i32
}
impl BlockStartedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStartedBreakingEvent{x, y} }
}

struct BlockStoppedBreakingEvent {
    x: i32, y: i32
}
impl BlockStoppedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStoppedBreakingEvent{x, y} }
}

struct BlockUpdateEvent {
    x: i32, y: i32
}
impl BlockUpdateEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockUpdateEvent{x, y} }
}

pub struct Tool {
    name: String
}

impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}

mod DefaultData {
    //idk what tf this is, will leave for now
    //also this is now a mod, not a struct because it doesn't need variables i think
}

/*
includes properties for each block type
B has to be a generic for connects_to list to work, it's just some rust black magic
https://stackoverflow.com/questions/48288640/generic-struct-with-a-reference-to-the-same-type-but-with-any-concrete-type
this is how it's supposed to work^^
*/
#[derive(PartialEq)]
pub struct BlockType{
    effective_tool: Option<*const Tool>,
    required_tool_power: i32,
    ghost: bool, transparent: bool,
    name: String,
    connects_to: Vec<i32>,
    break_time: i32,
    light_emission_r: i32, light_emission_g: i32, light_emission_b: i32,
    id: i32,
    width: i32, height: i32,
    block_data_index: i32,//future me change this to a reference if possible
    can_update_states: bool,
    feet_collidable: bool
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
            feet_collidable: false
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

#[derive(Clone)]
struct Block{
    id: i32,
    x_from_main: i8, y_from_main: i8,
    //default data, will leave out for now
}

impl Block{
    pub fn new() -> Self { Block{id: 0, x_from_main: 0, y_from_main: 0} }
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
    //data deliverer, will change custom data tho
    width: i32, height: i32,
    breaking_blocks: Vec<BreakingBlock>,
    block_types: Vec<BlockType>,
    tool_types: Vec<Tool>,

    //event senders
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
        };

        let mut air = BlockType::new(String::from("air"));
        air.ghost = true;
        air.transparent = true;
        air.break_time = UNBREAKABLE;
        air.light_emission_r = 0;
        air.light_emission_g = 0;
        air.light_emission_b = 0;
        air.can_update_states = false;
        let hand = Tool::new(String::from("hand"));
        b.register_new_block_type(air);
        b.register_new_tool_type(hand);
        b
    }

    fn get_block(&mut self, x: i32, y: i32) -> &mut Block {
        if x < 0 || y < 0 || x >= self.width || y >= self.height || self.blocks.is_empty() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }
        &mut self.blocks[(y * self.width + x) as usize]
    }

    fn get_chunk(&mut self, x: i32, y: i32) -> &mut BlockChunk {
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
        self.blocks = vec![Block::new(); (width * height) as usize];
        self.chunks = vec![BlockChunk::new(); (width * height / CHUNK_SIZE / CHUNK_SIZE) as usize];
    }

    pub fn get_block_type_by_id(&mut self, id: i32) -> &mut BlockType {
        if id < 0 || id >= self.block_types.len() as i32 {
            panic!("Block type id is out of bounds! id: {}", id);
        }
        &mut self.block_types[id as usize]
    }

    pub fn get_block_type(&mut self, x: i32, y: i32) -> &mut BlockType {
            let id = self.get_block(x, y).id.into();
            self.get_block_type_by_id(id)
    }

    pub fn set_block_type(&mut self, x: i32, y: i32, block_type: &BlockType, x_from_main: i8, y_from_main: i8) {
        if block_type.id != self.get_block(x, y).id {
            self.set_block_type_silently(x, y, block_type);
            for i in 0..self.breaking_blocks.len() {
                if self.breaking_blocks[i].x == x && self.breaking_blocks[i].y == y {
                    self.breaking_blocks.remove(i);
                    break;
                }
            }
                let block_ : &mut Block = self.get_block(x, y);
                block_.x_from_main = x_from_main;
                block_.y_from_main = y_from_main;
            let event = BlockChangeEvent::new(x, y);
            //call event
        }
    }

    pub fn set_block_type_silently(&mut self, x: i32, y: i32, block_type: &BlockType) {
        //delete custom data
        self.get_block(x, y).id = block_type.id;
        //add custom data
    }

    pub fn get_block_x_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).x_from_main
    }

    pub fn get_block_y_from_main(&mut self, x: i32, y: i32) -> i8 {
        self.get_block(x, y).y_from_main
    }

    //get block data function

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

        self.get_chunk(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count += 1;

        let event = BlockStartedBreakingEvent::new(x, y);
        //call event
    }

    pub fn stop_breaking_block(&mut self, x: i32, y: i32){
        if x < 0 || y < 0 || x >= self.width || y >= self.height{
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }

        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.x == x && breaking_block.y == y {
                breaking_block.is_breaking = false;
                self.get_chunk(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count -= 1;
                let event = BlockStoppedBreakingEvent::new(x, y);
                //call event
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
        //call event

        let block_type: *mut BlockType = &mut *self.get_block_type(x, y);
        unsafe { self.set_block_type(transformed_x, transformed_y, &(*block_type), 0, 0); }
    }

    pub fn get_chunk_breaking_blocks_count(&mut self, x: i32, y: i32) -> i32 {
        self.get_chunk(x, y).breaking_blocks_count.into()
    }

    pub fn get_width(&mut self) -> i32 {
        self.width
    }

    pub fn get_height(&mut self) -> i32 {
        self.height
    }

    pub fn to_serial(&mut self) -> Vec<i8> {
        let mut serial: Vec<i8> = Vec::new();
        let mut iter: u32 = 0;
        let size = 0;
        //save custom block data
        serial.resize(serial.len() + (self.width * self.height * 6 + 8 + size) as usize, 0);
        //save width in first 4 bytes
        serial[(iter    ) as usize] = (self.width >> 24) as i8;
        serial[(iter + 1) as usize] = (self.width >> 16) as i8;
        serial[(iter + 2) as usize] = (self.width >>  8) as i8;
        serial[(iter + 3) as usize] = (self.width      ) as i8;
        iter += 4;
        //save height in next 4 bytes
        serial[(iter    ) as usize] = (self.height >> 24) as i8;
        serial[(iter + 1) as usize] = (self.height >> 16) as i8;
        serial[(iter + 2) as usize] = (self.height >>  8) as i8;
        serial[(iter + 3) as usize] = (self.height      ) as i8;
        iter += 4;
        for(_i, block) in self.blocks.iter().enumerate() {
            serial[(iter    ) as usize] = (block.id >> 24) as i8;
            serial[(iter + 1) as usize] = (block.id >> 16) as i8;
            serial[(iter + 2) as usize] = (block.id >>  8) as i8;
            serial[(iter + 3) as usize] = (block.id      ) as i8;
            serial[(iter + 4) as usize] = block.x_from_main;
            serial[(iter + 5) as usize] = block.y_from_main;
            iter += 6;
            //save custom data when implemented
        }
        serial
        //return compressed serial
    }

    pub fn from_serial(&mut self, serial: Vec<i8>){
        let mut iter: u32 = 0;
        let decompressed = serial;//decompress serial whan implemented
        let width  = (decompressed[(iter    ) as usize] as i32) << 24 | (decompressed[(iter + 1) as usize] as i32) << 16 | (decompressed[(iter + 2) as usize] as i32) << 8 | decompressed[(iter + 3) as usize] as i32;
        let height = (decompressed[(iter + 4) as usize] as i32) << 24 | (decompressed[(iter + 5) as usize] as i32) << 16 | (decompressed[(iter + 6) as usize] as i32) << 8 | decompressed[(iter + 7) as usize] as i32;
        iter += 8;
        self.create(width, height);
        for(_i, block) in self.blocks.iter_mut().enumerate() {
            block.id = (decompressed[iter as usize] as i32) << 24 | (decompressed[(iter + 1) as usize] as i32) << 16 | (decompressed[(iter + 2) as usize] as i32) << 8 | decompressed[(iter + 3) as usize] as i32;
            block.x_from_main = decompressed[(iter + 1) as usize];
            block.y_from_main = decompressed[(iter + 2) as usize];
            iter += 3;
            //load custom data when implemented
        }
    }

    pub fn register_new_block_type(&mut self, mut block_type: BlockType){
        block_type.id = self.block_types.len() as i32;
        self.block_types.push(block_type);
    }

    pub fn get_block_type_by_name(&mut self, name: String) -> &BlockType {
        for block_type in self.block_types.iter() {
            if block_type.name == name {
                return &block_type;
            }
        }
        panic!("Block type with name {} not found!", name);
    }

    pub fn get_number_block_types(&mut self) -> i32 {
        self.block_types.len() as i32
    }

    //data deliverer, will probably be rewritten
    pub fn register_new_tool_type(&mut self, tool_type: Tool){
        self.tool_types.push(tool_type);
    }

    pub fn get_tool_type_by_name(&mut self, name: String) -> &Tool {
        for tool_type in self.tool_types.iter() {
            if tool_type.name == name {
                return &tool_type;
            }
        }
        panic!("Tool type with name {} not found!", name);
    }

    pub fn update_state_side(&mut self, x: i32, y: i32, side_x: i32, side_y: i32) -> bool {
        let this_block_id = self.get_block(x, y).id;
        let side_block_id = self.get_block(x + side_x, y + side_y).id;
        x + side_x >= self.width || x + side_x < 0 || y + side_y >= self.height || y + side_y < 0 ||
            self.get_block_type(x + side_x, y + side_y).id == this_block_id ||
            self.get_block_type(x, y).connects_to.contains(&side_block_id)
    }
}

