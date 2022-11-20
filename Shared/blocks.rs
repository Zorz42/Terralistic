use std::collections::LinkedList;

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

struct Tool {
    name: String
}
impl Tool {
    pub fn new(name: String) -> Self { Tool{name} }
}

mod DefaultData {
    //idk what tf this is, will leave for now
    //also this is now a mod, not a struct because it doesn't need variables i think
}


trait Link {}



/*
includes properties for each block type
B has to be a generic for connects_to list to work, it's just some rust black magic
https://stackoverflow.com/questions/48288640/generic-struct-with-a-reference-to-the-same-type-but-with-any-concrete-type
this is how it's supposed to work^^
*/
struct BlockType <'tool_lifetime, 'block_type_lifetime, B>
where B: 'block_type_lifetime{
    effective_tool: Option<&'tool_lifetime Tool>,
    required_tool_power: i32,
    ghost: bool, transparent: bool,
    name: String,
    connects_to: Vec<&'block_type_lifetime B>,
    break_time: i32,
    light_emission_r: i32, light_emission_g: i32, light_emission_b: i32,
    id: i32,
    width: i32, height: i32,
    block_data_index: i32,//future me change this to a reference if possible
    can_update_states: bool,
    feet_collidable: bool
}

impl<'tool_lifetime, 'block_type_lifetime, B> BlockType<'tool_lifetime, 'block_type_lifetime, B> {
    pub fn new(name: String) -> Self {
        BlockType {
            effective_tool: None,
            required_tool_power: 0,
            ghost: false, transparent: false,
            name: String::from(""),
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
    pub fn update_state(/*blocks: &Blocks, */x: i32, y: i32){}
}