use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::container_file;
use crate::libraries::grid::Grid;
use crate::libraries::registry::{Registry, RegistryId};
use crate::shared::blocks::Tool;
use crate::shared::blocks::{Blocks, ToolId};
use crate::shared::walls::{BreakingWall, Wall};

/// `WallId` stores id to a type of wall.
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct WallId {
    pub id: i8,
}

impl WallId {
    #[must_use]
    pub const fn undefined() -> Self {
        Self { id: -1 }
    }
}

impl RegistryId for WallId {
    const KIND: &'static str = "wall type";

    fn from_index(index: usize) -> Self {
        Self { id: index as i8 }
    }

    fn index(self) -> Option<usize> {
        usize::try_from(self.id).ok()
    }
}

#[derive(Deserialize, Serialize)]
pub(super) struct WallsData {
    pub(super) walls: Grid<WallId>,
}

impl WallsData {
    pub const fn new() -> Self {
        Self { walls: Grid::new_empty() }
    }
}

pub struct Walls {
    pub(super) walls_data: WallsData,

    pub(super) breaking_walls: Vec<BreakingWall>,
    pub(super) wall_types: Registry<WallId, Wall>,

    pub clear: WallId,
    pub hammer: ToolId,
}

impl Walls {
    pub fn new(blocks: &mut Blocks) -> Self {
        let mut result = Self {
            walls_data: WallsData::new(),

            breaking_walls: Vec::new(),
            wall_types: Registry::new(),

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

    /// Creates a map with the given dimensions, every cell holding `WallId::undefined()`.
    ///
    /// **That fill is a trap for anyone who calls this directly**: reading a wall before
    /// setting one gets an id no wall type answers to, and `get_wall_type` errors on it.
    /// Only `create_from_wall_ids` calls this, and it overwrites every cell immediately.
    pub fn create(&mut self, size: (u32, u32)) {
        self.walls_data.walls = Grid::filled(size, WallId::undefined());
    }

    /// Returns the wall id at the given position.
    fn get_wall(&self, x: i32, y: i32) -> Result<WallId> {
        Ok(*self.walls_data.walls.get(x, y)?)
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.walls_data.walls.get_size()
    }

    /// Returns the wall type of the wall at given x and y
    pub fn get_wall_type_at(&self, x: i32, y: i32) -> Result<Wall> {
        self.get_wall_type(self.get_wall(x, y)?)
    }

    /// Returns the wall type with the given id.
    ///
    /// A clone rather than a reference, unlike `Blocks::get_block_type` - the callers hold
    /// a lock while they use it, and handing back a borrow of the guard's contents is a
    /// separate change from this one.
    pub fn get_wall_type(&self, id: WallId) -> Result<Wall> {
        Ok(self.wall_types.get(id)?.clone())
    }

    /// This function sets the wall type on x and y and sends the `WallChangeEvent`.
    pub fn set_wall_type(&mut self, x: i32, y: i32, wall_id: WallId) -> Result<()> {
        let wall = self.get_wall(x, y)?;
        if wall == wall_id {
            return Ok(());
        }

        self.walls_data.walls.set(x, y, wall_id)?;

        Ok(())
    }

    /// Serializes walls for saving
    pub fn serialize(&self) -> Result<Vec<u8>> {
        container_file::pack(&self.walls_data)
    }

    /// Deserializes walls from u8 vector
    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        self.walls_data = container_file::unpack(data)?;

        Ok(())
    }

    /// This function adds a new wall type, but is used internally by mods.
    pub(super) fn register_new_wall_type(wall_types: &mut Registry<WallId, Wall>, wall_type: Wall) -> WallId {
        wall_types.register(wall_type)
    }

    /// Returns a wall id type with the given name
    pub fn get_wall_id_by_name(&self, name: &str) -> Result<WallId> {
        self.wall_types.get_id_by_name(name)
    }

    /// This function creates a world from a 2d vector of wall type ids
    pub fn create_from_wall_ids(&mut self, wall_ids: &[Vec<WallId>]) -> Result<()> {
        self.walls_data.walls = Grid::from_columns(wall_ids)?;
        Ok(())
    }

    /// Returns all wall ids.
    #[must_use]
    pub fn get_all_wall_ids(&self) -> Vec<WallId> {
        self.wall_types.ids()
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
