use super::blocks::*;
use std::rc::Rc;
use serde_derive::{Serialize, Deserialize};
use bincode;
use snap;
use crate::blocks::blocks::{Blocks, CHUNK_SIZE, UNBREAKABLE};
use crate::blocks::tool::Tool;

struct WallChangeEvent {
    pub x: i32,
    pub y: i32
}
impl WallChangeEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallChangeEvent{ x, y } }
}
//impl Event for WallChangeEvent {}

struct WallBreakEvent {
    pub x: i32,
    pub y: i32
}
impl WallBreakEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallBreakEvent{ x, y } }
}
//impl Event for WallBreakEvent {}

struct WallStartedBreakingEvent {
    pub x: i32,
    pub y: i32
}
impl WallStartedBreakingEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallStartedBreakingEvent{ x, y } }
}
//impl Event for WallStartedBreakingEvent {}

struct WallStoppedBreakingEvent {
    pub x: i32,
    pub y: i32
}
impl WallStoppedBreakingEvent{
    pub fn new(x: i32, y: i32) -> Self{ WallStoppedBreakingEvent{ x, y } }
}
//impl Event for WallStoppedBreakingEvent {}

pub struct WallType {
    pub id: i32,
    pub break_time: i32,
    pub name: String,
}
impl WallType {
    pub fn new(id: i32, break_time: i32, name: String) -> Self {
        WallType {
            id,
            break_time,
            name
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct Wall{
    pub id: i32,
}
impl Wall {
    pub fn new() -> Self {
        Wall {
            id: 0
        }
    }
}

struct BreakingWall {
    break_progress: i32,
    is_breaking: bool,
    x: i32,
    y: i32,
}
impl BreakingWall {
    pub fn new() -> Self {
        BreakingWall {
            break_progress: 0,
            is_breaking: false,
            x: 0,
            y: 0
        }
    }
}

#[derive(Clone)]
struct WallChunk {
    breaking_wall_count: u8,
}
impl WallChunk {
    pub fn new() -> Self {
        WallChunk {
            breaking_wall_count: 0
        }
    }
}

struct Walls{
    walls: Vec<Wall>,
    chunks: Vec<WallChunk>,

    breaking_walls: Vec<BreakingWall>,
    wall_types: Vec<Rc<WallType>>,

    curr_id: i32,

    pub clear: Rc<WallType>,
    pub hammer: Rc<Tool>,

    width: i32,
    height: i32,

    //pub wall_change_event: Sender<WallChangeEvent>,
    //pub wall_break_event: Sender<WallBreakEvent>,
    //pub wall_started_breaking_event: Sender<WallStartedBreakingEvent>,
    //pub wall_stopped_breaking_event: Sender<WallStoppedBreakingEvent>,
}

impl Walls{
    pub fn new(blocks: &mut Blocks) -> Self {
        let mut walls = Walls {
            walls: Vec::new(),
            chunks: Vec::new(),

            breaking_walls: Vec::new(),
            wall_types: Vec::new(),

            curr_id: 0,

            clear: Rc::new(WallType::new(0, UNBREAKABLE, "clear".to_string())),
            hammer: Rc::new(Tool::new("Hammer".to_string())),

            width: blocks.get_width(),
            height: blocks.get_height(),

            //wall_change_event: Sender::new(),
            //wall_break_event: Sender::new(),
            //wall_started_breaking_event: Sender::new(),
            //wall_stopped_breaking_event: Sender::new(),
        };
        walls.register_wall_type(walls.clear.clone());//TODO: @zorz42 make this work
        blocks.register_new_tool_type(walls.hammer.clone());
        walls
    }

    /**creates wall and chunk vectors*/
    pub fn create(&mut self, blocks: &Blocks){
        self.width = blocks.get_width();
        self.height = blocks.get_height();
        self.walls = vec![Wall::new(); (self.get_width() * self.get_height()) as usize];
        self.chunks = vec![WallChunk::new(); (self.get_width() / CHUNK_SIZE * self.get_height() / CHUNK_SIZE) as usize];
    }

    /**returns the wall at the given position*/
    fn get_wall(&self, x: i32, y: i32) -> &Wall {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        &self.walls[(x + y * self.get_width()) as usize]
    }

    /**returns the mutable wall at the given position*/
    fn get_wall_mut(&mut self, x: i32, y: i32) -> &mut Wall {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        let width = self.get_width();
        &mut self.walls[(x + y * width) as usize]
    }

    /**returns the chunk of the wall at the given position*/
    fn get_chunk(&self, x: i32, y: i32) -> &WallChunk {
        if x < 0 || y < 0 || x >= self.get_width() / CHUNK_SIZE || y >= self.get_height() / CHUNK_SIZE {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        &self.chunks[(x + y * self.get_width() / 16) as usize]
    }

    /**returns the mutable chunk of the wall at the given position*/
    fn get_chunk_mut(&mut self, x: i32, y: i32) -> &mut WallChunk {
        if x < 0 || y < 0 || x >= self.get_width() / CHUNK_SIZE || y >= self.get_height() / CHUNK_SIZE {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        let width = self.get_width() / CHUNK_SIZE;
        &mut self.chunks[(x + y * width) as usize]
    }

    /**returns world width in blocks*/
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /**returns world height in blocks*/
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /**returns the wall type of the wall at given x and y*/
    pub fn get_wall_type(&self, x: i32, y: i32) -> &WallType {
        &self.get_wall_type_by_id(self.get_wall(x, y).id)
    }

    /**returns the wall type with the given id*/
    pub fn get_wall_type_by_id(&self, id: i32) -> &WallType {
        if id < 0 || id >= self.wall_types.len() as i32 {
            panic!("Wall type with id {} does not exist!", id);
        }
        &self.wall_types[id as usize]
    }

    /**this function sets the wall type on x and y and sends the WallChangeEvent.
    It also removes the wall on x and y from breaking walls if it is in that list*/
    pub fn set_wall_type(&mut self, x: i32, y: i32, wall_type: Rc<WallType>) {
        let wall = self.get_wall(x, y);
        if wall.id == wall_type.id {
            return;
        }
        self.set_wall_type_silently(x, y, wall_type);

        for i in 0..self.breaking_walls.len() {
            if self.breaking_walls[i].x == x && self.breaking_walls[i].y == y {
                self.breaking_walls.remove(i);
                break;
            }
        }
        //self.wall_change_event.send(WallChangeEvent::new(x, y));
    }

    /**this function sets the wall type on x and y without triggering an event (without updating the world)*/
    pub fn set_wall_type_silently(&mut self, x: i32, y: i32, wall_type: Rc<WallType>) {
        self.get_wall_mut(x, y).id = wall_type.id;
    }

    /**returns break progress of the wall at x and y*/
    pub fn get_break_progress(&self, x: i32, y: i32) -> i32 {
        for wall in &self.breaking_walls {
            if wall.x == x && wall.y == y {
                return wall.break_progress;
            }
        }
        0
    }

    /**returns break stage (for example to be used as a break texture stage) of the wall at x and y*/
    pub fn get_break_stage(&self, x: i32, y: i32) -> i32 {
        (self.get_break_progress(x, y) as f64 / self.get_wall_type(x, y).break_time as f64 * 9.0) as i32
    }

    /**includes the necessary steps to start breaking a wall, such as adding it to the breaking_walls list, setting is_breaking to true and sending the WallStartedBreakingEvent*/
    pub fn start_breaking_wall(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        if self.get_wall_type(x, y).break_time == UNBREAKABLE {
            return;
        }

        let mut breaking_wall: Option<&mut BreakingWall> = None;
        for wall in &mut self.breaking_walls {
            if wall.x == x && wall.y == y {
                breaking_wall = Some(wall);
                break;
            }
        }

        if breaking_wall.is_none() {
            let mut new_breaking_wall = BreakingWall::new();
            new_breaking_wall.x = x;
            new_breaking_wall.y = y;
            self.breaking_walls.push(new_breaking_wall);
            breaking_wall = Some(self.breaking_walls.last_mut().unwrap());
        }

        breaking_wall.unwrap().is_breaking = true;

        self.get_chunk_mut(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_wall_count += 1;

        //self.wall_started_breaking_event.send(WallStartedBreakingEvent::new(x, y));
    }

    /**includes the necessary steps to stop breaking a wall, such as removing it from the breaking_walls list, setting is_breaking to false and sending the WallStoppedBreakingEvent*/
    pub fn stop_breaking_wall(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }

        for wall in self.breaking_walls.iter_mut() {
            if wall.x == x && wall.y == y {
                wall.is_breaking = false;
                self.get_chunk_mut(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_wall_count -= 1;
                //self.wall_stopped_breaking_event.send(WallStoppedBreakingEvent::new(x, y));
                break;
            }
        }
    }

    /**updates breaking walls by increasing break progress and pbreaking walls if necessary*/
    pub fn update_breaking_walls(&mut self, frame_length: i32) {
        for i in 0..self.breaking_walls.len() {
            if self.breaking_walls[i].is_breaking {
                self.breaking_walls[i].break_progress += frame_length;
                if self.breaking_walls[i].break_progress > self.get_wall_type(self.breaking_walls[i].x, self.breaking_walls[i].y).break_time {
                    self.break_wall(self.breaking_walls[i].x, self.breaking_walls[i].y);
                }
            }
        }
    }

    /**breaks the wall at x and y and sends the WallBrokenEvent*/
    pub fn break_wall(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Wall is accessed out of the bounds! ({}, {})", x, y);
        }
        //self.wall_break_event.send(WallBreakEvent::new(x, y));

        self.set_wall_type(x, y, Rc::clone(&self.clear));
    }

    /**returns the count of breaking walls in the given chunk*/
    pub fn get_breaking_wall_count(&self, chunk_x: i32, chunk_y: i32) -> u8 {
        self.get_chunk(chunk_x, chunk_y).breaking_wall_count
    }

    /**serializes walls for saving*/
    pub fn to_serial(&self) -> Vec<u8> {
        snap::raw::Encoder::new().
            compress_vec(&bincode::
            serialize(&self.walls).unwrap()
            ).unwrap()
    }

    /**deserializes walls from u8 vector*/
    pub fn from_serial(&mut self, data: Vec<u8>, blocks: &Blocks) {
        let decompressed = snap::raw::Decoder::new().decompress_vec(&data).unwrap();
        self.create(blocks);
        self.walls = bincode::deserialize(&decompressed).unwrap();
    }

    /**registers new wall type*/
    pub fn register_wall_type(&mut self, wall_type: Rc<WallType>) {//TODO assign id automatically?
        self.wall_types.push(wall_type);
    }

    /**returns wall type with the given name*/
    pub fn get_wall_type_by_name(&self, name: &str) -> Option<Rc<WallType>> {
        for wall_type in &self.wall_types {
            if wall_type.name == name {
                return Some(Rc::clone(wall_type));
            }
        }
        None
    }
}