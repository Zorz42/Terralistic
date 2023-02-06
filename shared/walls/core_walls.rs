use crate::shared::blocks::Tool;
use crate::shared::blocks::{Blocks, ToolId};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{BreakingWall, Wall};
use crate::shared::world_map::WorldMap;
use anyhow::{anyhow, Result};
use bincode;

use serde_derive::{Deserialize, Serialize};
use snap;
use std::sync::{Arc, Mutex};

/**
WallId stores id to a type of wall.
 */
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WallId {
    pub id: i8,
}

impl Default for WallId {
    fn default() -> Self {
        Self::new()
    }
}

impl WallId {
    pub fn new() -> Self {
        Self { id: -1 }
    }
}

// make WallId lua compatible
impl rlua::UserData for WallId {
    // implement equals comparison for BlockId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: WallId| {
            Ok(this.id == other.id)
        });
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

pub struct Walls {
    walls_data: WallsData,

    pub(super) breaking_walls: Vec<BreakingWall>,
    wall_types: Arc<Mutex<Vec<Wall>>>,

    pub clear: WallId,
    pub hammer: ToolId,
}

impl Walls {
    pub fn new(blocks: &mut Blocks) -> Self {
        let mut result = Self {
            walls_data: WallsData::new(),

            breaking_walls: Vec::new(),
            wall_types: Arc::new(Mutex::new(Vec::new())),

            clear: WallId::new(),
            hammer: ToolId::new(),
        };

        let mut clear = Wall::new();
        clear.name = "clear".to_string();
        result.clear = Self::register_new_wall_type(
            &mut result.wall_types.lock().unwrap_or_else(|e| e.into_inner()),
            clear,
        );

        let mut hammer = Tool::new();
        hammer.name = "hammer".to_string();
        result.hammer = blocks.register_new_tool_type(hammer);

        result
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
    Initializes
     */
    pub fn init(&mut self, mods: &mut ModManager) {
        mods.add_global_function("new_wall_type", move |_lua, _: ()| Ok(Wall::new()));

        let wall_types = self.wall_types.clone();
        mods.add_global_function("register_wall_type", move |_lua, wall_type: Wall| {
            let result = Self::register_new_wall_type(
                &mut wall_types.lock().unwrap_or_else(|e| e.into_inner()),
                wall_type,
            );
            Ok(result)
        });

        let wall_types = self.wall_types.clone();
        mods.add_global_function("get_wall_id_by_name", move |_lua, name: String| {
            let wall_types = wall_types.lock().unwrap_or_else(|e| e.into_inner());
            for wall_type in wall_types.iter() {
                if wall_type.name == name {
                    return Ok(wall_type.get_id());
                }
            }
            Err(rlua::Error::RuntimeError("Wall type not found".to_string()))
        });
    }

    /**
    Returns the wall id at the given position.
     */
    fn get_wall(&self, x: i32, y: i32) -> Result<WallId> {
        Ok(*self
            .walls_data
            .walls
            .get(self.walls_data.map.translate_coords(x, y)?)
            .ok_or(anyhow!(
                "Wall is accessed out of the bounds! ({}, {})",
                x,
                y
            ))?)
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
    pub fn get_wall_type_at(&self, x: i32, y: i32) -> Result<Wall> {
        self.get_wall_type(self.get_wall(x, y)?)
    }

    /**
    Returns the wall type with the given id
     */
    pub fn get_wall_type(&self, id: WallId) -> Result<Wall> {
        let walls = self.wall_types.lock().unwrap_or_else(|e| e.into_inner());
        Ok(walls
            .get(id.id as usize)
            .ok_or(anyhow!("Wall type not found"))?
            .clone())
    }

    /**
    This function sets the wall type on x and y and sends the WallChangeEvent.
     */
    pub fn set_wall_type(&mut self, x: i32, y: i32, wall_id: WallId) -> Result<()> {
        let wall = self.get_wall(x, y)?;
        if wall == wall_id {
            return Ok(());
        }

        *self
            .walls_data
            .walls
            .get_mut(self.walls_data.map.translate_coords(x, y)?)
            .ok_or(anyhow!(
                "Wall is accessed out of the bounds! ({}, {})",
                x,
                y
            ))? = wall_id;

        //self.wall_change_event.send(WallChangeEvent::new(x, y));

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
    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(data)?;
        self.walls_data = bincode::deserialize(&decompressed)?;

        Ok(())
    }

    /**
    This function adds a new wall type, but is used internally by mods.
     */
    fn register_new_wall_type(wall_types: &mut Vec<Wall>, mut wall_type: Wall) -> WallId {
        let id = wall_types.len() as i8;
        let result = WallId { id };
        wall_type.id = result;
        wall_types.push(wall_type);
        result
    }

    /**
    Returns a wall id type with the given name
     */
    pub fn get_wall_id_by_name(&self, name: &str) -> Result<WallId> {
        for wall_type in self
            .wall_types
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            if wall_type.name == name {
                return Ok(wall_type.id);
            }
        }
        Err(anyhow!("No wall type with name {} found", name))
    }

    /**
    This function creates a world from a 2d vector of wall type ids
     */
    pub fn create_from_wall_ids(&mut self, wall_ids: &Vec<Vec<WallId>>) -> Result<()> {
        let width = wall_ids.len() as i32;
        let height;
        if let Some(row) = wall_ids.get(0) {
            height = row.len() as i32;
        } else {
            return Err(anyhow!("Wall ids must not be empty"));
        }

        // check that all the rows have the same length
        for row in wall_ids {
            if row.len() as i32 != height {
                return Err(anyhow!("All rows must have the same length"));
            }
        }

        self.create(width, height)?;
        self.walls_data.walls.clear();
        for row in wall_ids {
            for wall_id in row {
                self.walls_data.walls.push(*wall_id);
            }
        }

        Ok(())
    }

    /**
    Returns all wall ids.
     */
    pub fn get_all_wall_ids(&mut self) -> Vec<WallId> {
        let mut result = Vec::new();
        for wall_type in self
            .wall_types
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
        {
            result.push(wall_type.id);
        }
        result
    }

    /**
    Returns all breaking walls
     */
    pub fn get_breaking_walls(&self) -> &Vec<BreakingWall> {
        &self.breaking_walls
    }
}

pub struct WallChangeEvent {
    pub x: i32,
    pub y: i32,
}

pub struct WallBreakEvent {
    pub x: i32,
    pub y: i32,
}

pub struct WallStartedBreakingEvent {
    pub x: i32,
    pub y: i32,
}

pub struct WallStoppedBreakingEvent {
    pub x: i32,
    pub y: i32,
}

/**
A welcome packet that carries all the information about the world walls
 */
#[derive(Serialize, Deserialize)]
pub struct WallsWelcomePacket {
    pub data: Vec<u8>,
}
