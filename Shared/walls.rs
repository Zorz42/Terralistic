
use serde_derive::{Serialize, Deserialize};
use bincode;
use snap;
use crate::blocks::{Blocks, ToolId, UNBREAKABLE};
use crate::blocks::Tool;
use crate::world_map::WorldMap;
use anyhow::{anyhow, Result};
use events::EventManager;

/**
WallId stores id to a type of wall.
 */
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WallId {
    pub id: i8,
}

impl WallId {
    pub fn new() -> Self {
        Self {
            id: -1
        }
    }
}

/**
Wall holds all information about a type of a wall.
*/
#[derive(Clone)]
pub struct Wall {
    pub id: WallId,
    pub break_time: i32,
    pub name: String,
}

impl Wall {
    pub fn new(name: String) -> Self {
        Self {
            id: WallId::new(),
            break_time: 0,
            name
        }
    }
}

/**
Stores the info about a breaking progress about a wall.
*/
pub struct BreakingWall {
    pub break_progress: i32,
    pub is_breaking: bool,
    coord: (i32, i32),
}

impl BreakingWall {
    pub fn new() -> Self {
        BreakingWall {
            break_progress: 0,
            is_breaking: false,
            coord: (0, 0),
        }
    }

    pub fn get_coord(&self) -> (i32, i32) {
        self.coord
    }
}

#[derive(Deserialize, Serialize)]
struct WallsData {
    walls: Vec<WallId>,
    map: WorldMap,
}

impl WallsData {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            map: WorldMap::new_empty(),
        }
    }
}

struct Walls {
    walls_data: WallsData,

    breaking_walls: Vec<BreakingWall>,
    wall_types: Vec<Wall>,

    curr_id: i32,

    pub clear: WallId,
    pub hammer: ToolId,
}

impl Walls{
    pub fn new(blocks: &mut Blocks) -> Self {
        let mut walls = Walls {
            walls_data: WallsData::new(),

            breaking_walls: Vec::new(),
            wall_types: Vec::new(),

            curr_id: 0,

            clear: WallId::new(),
            hammer: ToolId::new(),
        };

        let clear_type = Wall::new("clear".to_string());
        walls.clear = walls.register_wall_type(clear_type);

        let hammer = Tool::new("Hammer".to_string());
        walls.hammer = blocks.register_new_tool_type(hammer);
        walls
    }

    /**
    Creates an empty map with the given dimensions.
     */
    pub fn create(&mut self, width: i32, height: i32) -> Result<()> {
        self.walls_data.map = WorldMap::new(width, height)?;
        self.walls_data.walls = vec![WallId::new(); (width * height) as usize];
        Ok(())
    }

    /**
    Returns the wall id at the given position.
     */
    fn get_wall(&self, x: i32, y: i32) -> Result<WallId> {
        Ok(*self.walls_data.walls.get(self.walls_data.map.translate_coords(x, y)?).ok_or(anyhow!("Wall is accessed out of the bounds! ({}, {})", x, y))?)
    }

    /**
    Returns the world width in blocks
     */
    pub fn get_width(&self) -> i32 {
        self.walls_data.map.get_width()
    }

    /**
    Returns the world height in blocks
     */
    pub fn get_height(&self) -> i32 {
        self.walls_data.map.get_height()
    }

    /**
    Returns the wall type of the wall at given x and y
     */
    pub fn get_wall_type_at(&self, x: i32, y: i32) -> Result<&Wall> {
        self.get_wall_type(self.get_wall(x, y)?)
    }

    /**
    Returns the wall type with the given id
     */
    pub fn get_wall_type(&self, id: WallId) -> Result<&Wall> {
        self.wall_types.get(id.id as usize).ok_or(anyhow!("Wall type with id {} does not exist!", id.id))
    }

    /**
    This function sets the wall type on x and y and sends the WallChangeEvent.
     */
    pub fn set_wall_type(&mut self, x: i32, y: i32, wall_id: WallId) -> Result<()> {
        let wall = self.get_wall(x, y)?;
        if wall == wall_id {
            return Ok(());
        }

        *self.walls_data.walls.get_mut(self.walls_data.map.translate_coords(x, y)?).ok_or(anyhow!("Wall is accessed out of the bounds! ({}, {})", x, y))? = wall_id;

        //self.wall_change_event.send(WallChangeEvent::new(x, y));

        Ok(())
    }

    /**
    Returns the break progress of the wall at x and y
     */
    pub fn get_break_progress(&self, x: i32, y: i32) -> i32 {
        for wall in &self.breaking_walls {
            if wall.coord == (x, y) {
                return wall.break_progress;
            }
        }
        0
    }

    /**
    Returns the break stage (for example to be used as a break texture stage) of the wall at x and y
     */
    pub fn get_break_stage(&self, x: i32, y: i32) -> Result<i32> {
        Ok((self.get_break_progress(x, y) * 9 / self.get_wall_type_at(x, y)?.break_time))
    }

    /**
    Includes the necessary steps to start breaking a wall, such as adding it to the breaking_walls list, setting is_breaking to true and sending the WallStartedBreakingEvent
     */
    pub fn start_breaking_wall(&mut self, x: i32, y: i32) -> Result<()> {
        if self.get_wall_type_at(x, y)?.break_time == UNBREAKABLE {
            return Ok(());
        }

        let mut breaking_wall: Option<&mut BreakingWall> = None;
        for wall in &mut self.breaking_walls {
            if wall.coord == (x, y) {
                breaking_wall = Some(wall);
                break;
            }
        }

        let breaking_wall = {
            if let Some(breaking_wall) = breaking_wall {
                breaking_wall
            } else {
                let mut new_breaking_wall = BreakingWall::new();
                new_breaking_wall.coord = (x, y);
                self.breaking_walls.push(new_breaking_wall);
                self.breaking_walls.last_mut().ok_or(anyhow!("Could not get last breaking wall!"))?
            }
        };


        breaking_wall.is_breaking = true;
        Ok(())

        //self.wall_started_breaking_event.send(WallStartedBreakingEvent::new(x, y));
    }

    /**
    Includes the necessary steps to stop breaking a wall,
    such as removing it from the breaking_walls list,
    setting is_breaking to false and sending the
    WallStoppedBreakingEvent
     */
    pub fn stop_breaking_wall(&mut self, x: i32, y: i32) {
        for wall in self.breaking_walls.iter_mut() {
            if wall.coord == (x, y) {
                wall.is_breaking = false;
                //self.wall_stopped_breaking_event.send(WallStoppedBreakingEvent::new(x, y));
                break;
            }
        }
    }

    /**
    Updates breaking walls by increasing break
    progress and breaking walls if necessary
     */
    pub fn update_breaking_walls(&mut self, frame_length: f32, _events: &mut EventManager) -> Result<()> {
        for breaking_wall in self.breaking_walls.iter_mut() {
            if breaking_wall.is_breaking {
                breaking_wall.break_progress += frame_length as i32;
            }
        }

        let mut broken_walls = Vec::new();
        for breaking_wall in self.breaking_walls.iter() {
            if breaking_wall.break_progress > self.get_wall_type_at(breaking_wall.get_coord().0, breaking_wall.get_coord().1)?.break_time {
                broken_walls.push(breaking_wall.get_coord());
            }
        }

        for broken_wall in broken_walls.iter() {
            let (x, y) = *broken_wall;

            //let _event = WallBreakEvent::new(x, y);
            //self.wall_break_event.send(event);

            self.set_wall_type(x, y, self.clear)?;

            self.breaking_walls.retain(|breaking_wall| breaking_wall.get_coord() != *broken_wall);
        }

        Ok(())
    }

    /**
    Serializes walls for saving
     */
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&bincode::serialize(&self.walls_data)?)?)
    }

    /**
    Deserializes walls from u8 vector
     */
    pub fn deserialize(&mut self, data: Vec<u8>) -> Result<()> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(&data)?;
        self.walls_data = bincode::deserialize(&decompressed)?;
        Ok(())
    }

    /**
    Registers a new wall type
     */
    pub fn register_wall_type(&mut self, mut wall: Wall) -> WallId {
        wall.id.id = self.wall_types.len() as i8;
        let result = wall.id;
        self.wall_types.push(wall);
        result
    }

    /**
    Returns a wall id type with the given name
     */
    pub fn get_wall_id_by_name(&self, name: &str) -> Result<WallId> {
        for wall_type in &self.wall_types {
            if wall_type.name == name {
                return Ok(wall_type.id);
            }
        }
        Err(anyhow!("No wall type with name {} found", name))
    }
}

struct WallChangeEvent {
    pub x: i32,
    pub y: i32
}

struct WallBreakEvent {
    pub x: i32,
    pub y: i32
}

struct WallStartedBreakingEvent {
    pub x: i32,
    pub y: i32
}

struct WallStoppedBreakingEvent {
    pub x: i32,
    pub y: i32
}