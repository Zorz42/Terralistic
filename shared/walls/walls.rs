use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};
use snap;

use crate::libraries::serialization;
use crate::shared::blocks::Tool;
use crate::shared::blocks::{Blocks, ToolId};
use crate::shared::walls::{BreakingWall, Wall};
use crate::shared::world_map::WorldMap;

/// `WallId` stores id to a type of wall.
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct WallId {
    pub id: i8,
}

impl WallId {
    #[must_use]
    pub const fn undefined() -> Self {
        Self { id: -1 }
    }
}

#[derive(Deserialize, Serialize)]
pub(super) struct WallsData {
    walls: Vec<WallId>,
    pub(super) map: WorldMap,
}

impl WallsData {
    pub const fn new() -> Self {
        Self {
            walls: Vec::new(),
            map: WorldMap::new_empty(),
        }
    }
}

pub struct Walls {
    pub(super) walls_data: WallsData,

    pub(super) breaking_walls: Vec<BreakingWall>,
    pub(super) wall_types: Vec<Wall>,

    pub clear: WallId,
    pub hammer: ToolId,
}

impl Walls {
    pub fn new(blocks: &mut Blocks) -> Self {
        let mut result = Self {
            walls_data: WallsData::new(),

            breaking_walls: Vec::new(),
            wall_types: Vec::new(),

            clear: WallId::undefined(),
            hammer: ToolId::new(),
        };

        let mut clear = Wall::new();
        "clear".clone_into(&mut clear.name);
        result.clear = Self::register_new_wall_type(&mut result.wall_types, clear);

        let mut hammer = Tool::new();
        "hammer".clone_into(&mut hammer.name);
        result.hammer = blocks.register_new_tool_type(hammer);

        result
    }

    /// Creates an empty map with the given dimensions.
    pub fn create(&mut self, size: (u32, u32)) {
        self.walls_data.map = WorldMap::new(size);
        self.walls_data.walls = vec![WallId::undefined(); (size.0 * size.1) as usize];
    }

    /// Returns the wall id at the given position.
    fn get_wall(&self, x: i32, y: i32) -> Result<WallId> {
        Ok(*self
            .walls_data
            .walls
            .get(self.walls_data.map.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Wall is accessed out of the bounds! ({x}, {y})"))?)
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.walls_data.map.get_size()
    }

    /// Returns the wall type of the wall at given x and y
    pub fn get_wall_type_at(&self, x: i32, y: i32) -> Result<Wall> {
        self.get_wall_type(self.get_wall(x, y)?)
    }

    /// Returns the wall type with the given id
    pub fn get_wall_type(&self, id: WallId) -> Result<Wall> {
        Ok(self.wall_types.get(id.id as usize).ok_or_else(|| anyhow!("Wall type not found"))?.clone())
    }

    /// This function sets the wall type on x and y and sends the `WallChangeEvent`.
    pub fn set_wall_type(&mut self, x: i32, y: i32, wall_id: WallId) -> Result<()> {
        let wall = self.get_wall(x, y)?;
        if wall == wall_id {
            return Ok(());
        }

        *self
            .walls_data
            .walls
            .get_mut(self.walls_data.map.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Wall is accessed out of the bounds! ({x}, {y})"))? = wall_id;

        Ok(())
    }

    /// Serializes walls for saving
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&serialization::serialize(&self.walls_data)?)?)
    }

    /// Deserializes walls from u8 vector
    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(data)?;
        self.walls_data = serialization::deserialize(&decompressed)?;

        Ok(())
    }

    /// This function adds a new wall type, but is used internally by mods.
    pub(super) fn register_new_wall_type(wall_types: &mut Vec<Wall>, mut wall_type: Wall) -> WallId {
        let id = wall_types.len() as i8;
        let result = WallId { id };
        wall_type.id = result;
        wall_types.push(wall_type);
        result
    }

    /// Returns a wall id type with the given name
    pub fn get_wall_id_by_name(&self, name: &str) -> Result<WallId> {
        let iter = self.wall_types.iter();
        for wall_type in iter {
            if wall_type.name == name {
                return Ok(wall_type.id);
            }
        }
        bail!("No wall type with name {name} found")
    }

    /// This function creates a world from a 2d vector of wall type ids
    pub fn create_from_wall_ids(&mut self, wall_ids: &Vec<Vec<WallId>>) -> Result<()> {
        let width = wall_ids.len() as u32;
        let height;
        if let Some(row) = wall_ids.first() {
            height = row.len() as u32;
        } else {
            bail!("Wall ids must not be empty");
        }

        // check that all the rows have the same length
        for row in wall_ids {
            if row.len() as u32 != height {
                bail!("All rows must have the same length");
            }
        }

        self.create((width, height));
        self.walls_data.walls.clear();
        for row in wall_ids {
            for wall_id in row {
                self.walls_data.walls.push(*wall_id);
            }
        }

        Ok(())
    }

    /// Returns all wall ids.
    #[must_use]
    pub fn get_all_wall_ids(&self) -> Vec<WallId> {
        let mut result = Vec::new();
        for wall_type in &self.wall_types {
            result.push(wall_type.id);
        }
        result
    }

    /// Returns all breaking walls
    #[must_use]
    pub const fn get_breaking_walls(&self) -> &Vec<BreakingWall> {
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

/// A welcome packet that carries all the information about the world walls
#[derive(Serialize, Deserialize)]
pub struct WallsWelcomePacket {
    pub data: Vec<u8>,
}
