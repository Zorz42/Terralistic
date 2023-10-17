use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};
use snap;

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::{Block, BreakingBlock, Tool};
use crate::shared::world_map::WorldMap;

pub const BLOCK_WIDTH: f32 = 8.0;
pub const RENDER_SCALE: f32 = 2.0;
pub const RENDER_BLOCK_WIDTH: f32 = BLOCK_WIDTH * RENDER_SCALE;
pub const RANDOM_TICK_SPEED: i32 = 10;

#[derive(Serialize, Deserialize)]
pub(super) struct BlocksData {
    pub map: WorldMap,
    pub blocks: Vec<BlockId>,
    // tells how much blocks a block in a big block is from the main block, it is mostly 0, 0 so it is stored in a hashmap
    pub block_from_main: HashMap<usize, (i32, i32)>,
    // saves the extra block data, it is mostly empty so it is stored in a hashmap
    pub block_data: HashMap<usize, Vec<u8>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockId {
    pub(super) id: i8,
}

impl BlockId {
    #[must_use]
    pub const fn undefined() -> Self {
        Self { id: -1 }
    }
}

/// A world is a 2d array of blocks.
pub struct Blocks {
    pub(super) block_data: BlocksData,
    pub(super) breaking_blocks: Vec<BreakingBlock>,
    pub(super) block_types: Vec<Block>,
    pub(super) tool_types: Vec<Tool>,
    air: BlockId,
}

impl Blocks {
    #[must_use]
    pub fn new() -> Self {
        let mut result = Self {
            block_data: BlocksData {
                blocks: Vec::new(),
                block_from_main: HashMap::new(),
                block_data: HashMap::new(),
                map: WorldMap::new_empty(),
            },
            breaking_blocks: vec![],
            block_types: vec![],
            tool_types: vec![],
            air: BlockId::undefined(),
        };

        let mut air = Block::new();
        air.name = "air".to_owned();
        air.ghost = true;
        air.transparent = true;
        result.air = result.register_new_block_type(air);

        result
    }

    #[must_use]
    pub const fn get_width(&self) -> u32 {
        self.block_data.map.get_width()
    }

    #[must_use]
    pub const fn air(&self) -> BlockId {
        self.air
    }

    #[must_use]
    pub const fn get_height(&self) -> u32 {
        self.block_data.map.get_height()
    }

    /// Creates an empty world with given width and height
    pub fn create(&mut self, width: u32, height: u32) {
        self.block_data.map = WorldMap::new(width, height);
        self.block_data.blocks = vec![self.air; (height * height) as usize];
    }

    /// This function creates a world from a 2d vector of block type ids
    pub fn create_from_block_ids(&mut self, block_ids: &Vec<Vec<BlockId>>) -> Result<()> {
        let width = block_ids.len() as u32;
        let height;
        if let Some(row) = block_ids.get(0) {
            height = row.len() as u32;
        } else {
            bail!("Block ids must not be empty");
        }

        // check that all the rows have the same length
        for row in block_ids {
            if row.len() as u32 != height {
                bail!("All rows must have the same length");
            }
        }

        self.create(width, height);
        self.block_data.blocks.clear();
        for row in block_ids {
            self.block_data.blocks.extend_from_slice(row);
        }
        Ok(())
    }

    /// This function returns the block id at given position
    pub fn get_block(&self, x: i32, y: i32) -> Result<BlockId> {
        Ok(*self
            .block_data
            .blocks
            .get(self.block_data.map.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Coordinate out of bounds"))?)
    }

    /// This sets the type of a block from a coordinate.
    pub fn set_big_block(
        &mut self,
        events: &mut EventManager,
        x: i32,
        y: i32,
        block_id: BlockId,
        from_main: (i32, i32),
    ) -> Result<()> {
        if block_id != self.get_block(x, y)? || from_main != self.get_block_from_main(x, y)? {
            let prev_block = self.get_block(x, y)?;

            self.set_block_data(x, y, vec![])?;
            *self
                .block_data
                .blocks
                .get_mut(self.block_data.map.translate_coords(x, y)?)
                .ok_or_else(|| anyhow!("Coordinate out of bounds"))? = block_id;

            self.breaking_blocks.retain(|b| b.get_coord() != (x, y));

            self.set_block_from_main(x, y, from_main)?;
            let event = BlockChangeEvent { x, y, prev_block };
            events.push_event(Event::new(event));
        }
        Ok(())
    }

    /// This sets the type of a block from a coordinate.
    pub fn set_block(
        &mut self,
        events: &mut EventManager,
        x: i32,
        y: i32,
        block_id: BlockId,
    ) -> Result<()> {
        self.set_big_block(events, x, y, block_id, (0, 0))
    }

    /// This function sets x and y from main for a block. If it is 0, 0 the value is removed from the hashmap.
    pub(super) fn set_block_from_main(
        &mut self,
        x: i32,
        y: i32,
        from_main: (i32, i32),
    ) -> Result<()> {
        let index = self.block_data.map.translate_coords(x, y)?;

        if from_main.0 == 0 && from_main.1 == 0 {
            self.block_data.block_from_main.remove(&index);
        } else {
            self.block_data.block_from_main.insert(index, from_main);
        }
        Ok(())
    }

    /// This function gets the block from main for a block. If the value is not found, it returns 0, 0.
    pub fn get_block_from_main(&self, x: i32, y: i32) -> Result<(i32, i32)> {
        Ok(*self
            .block_data
            .block_from_main
            .get(&self.block_data.map.translate_coords(x, y)?)
            .unwrap_or(&(0, 0)))
    }

    /// This function sets the block data for a block. If it is empty the value is removed from the hashmap.
    pub(super) fn set_block_data(&mut self, x: i32, y: i32, data: Vec<u8>) -> Result<()> {
        let index = self.block_data.map.translate_coords(x, y)?;
        if data.is_empty() {
            self.block_data.block_data.remove(&index);
        } else {
            self.block_data.block_data.insert(index, data);
        }
        Ok(())
    }

    /// This function returns block data, if it is not found it returns an empty vector.
    pub fn get_block_data(&self, x: i32, y: i32) -> Result<Vec<u8>> {
        Ok(self
            .block_data
            .block_data
            .get(&self.block_data.map.translate_coords(x, y)?)
            .unwrap_or(&vec![])
            .clone())
    }

    /// Serializes the world, used for saving the world and sending it to the client.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&bincode::serialize(&self.block_data)?)?)
    }

    /// Deserializes the world, used for loading the world and receiving it from the server.
    pub fn deserialize(&mut self, serial: &[u8]) -> Result<()> {
        self.block_data = bincode::deserialize(&snap::raw::Decoder::new().decompress_vec(serial)?)?;
        Ok(())
    }

    /// This function adds a new block type
    pub fn register_new_block_type(&mut self, mut block_type: Block) -> BlockId {
        let id = self.block_types.len() as i8;
        let result = BlockId { id };
        block_type.id = result;
        self.block_types.push(block_type);
        result
    }

    /// Returns the block type that has the specified name, used
    /// with commands to get the block type from the name.
    pub fn get_block_id_by_name(&mut self, name: &str) -> Result<BlockId> {
        for block_type in &self.block_types {
            if block_type.name == name {
                return Ok(block_type.id);
            }
        }
        bail!("Block type not found")
    }

    /// Returns all block ids.
    #[must_use]
    pub fn get_all_block_ids(&self) -> Vec<BlockId> {
        let mut result = Vec::new();
        for block_type in &self.block_types {
            result.push(block_type.id);
        }
        result
    }

    /// Returns the block type that has the specified id.
    pub fn get_block_type(&self, id: BlockId) -> Result<Block> {
        Ok(self
            .block_types
            .get(id.id as usize)
            .ok_or_else(|| anyhow!("Block type not found"))?
            .clone())
    }

    /// Updates the block at the specified coordinates.
    pub fn update_block(&mut self, x: i32, y: i32, events: &mut EventManager) -> Result<()> {
        let block = self.get_block_type_at(x, y)?;
        if block.width != 0 || block.height != 0 {
            let from_main = self.get_block_from_main(x, y)?;

            if (from_main.0 > 0 && self.get_block_type_at(x - 1, y)?.get_id() != block.get_id())
                || (from_main.1 > 0 && self.get_block_type_at(x, y - 1)?.get_id() != block.get_id())
            {
                self.set_block(events, x, y, self.air)?;
                return Ok(());
            }

            if from_main.0 + 1 < block.width {
                self.set_big_block(
                    events,
                    x + 1,
                    y,
                    block.get_id(),
                    (from_main.0 + 1, from_main.1),
                )?;
            }

            if from_main.1 + 1 < block.height {
                self.set_big_block(
                    events,
                    x,
                    y + 1,
                    block.get_id(),
                    (from_main.0, from_main.1 + 1),
                )?;
            }
        }

        Ok(())
    }

    /// Returns the block type at specified coordinates.
    pub fn get_block_type_at(&self, x: i32, y: i32) -> Result<Block> {
        self.get_block_type(self.get_block(x, y)?)
    }
}

/// Event that is fired when a block is changed
pub struct BlockChangeEvent {
    pub x: i32,
    pub y: i32,
    pub prev_block: BlockId,
}

/// Event that is fired when a random tick is fired for a block
pub struct BlockRandomTickEvent {
    pub x: i32,
    pub y: i32,
}

/// Event that is fired when a block is updated
pub struct BlockUpdateEvent {
    pub x: i32,
    pub y: i32,
}

/// A welcome packet that carries all the information about the world blocks
#[derive(Serialize, Deserialize)]
pub struct BlocksWelcomePacket {
    pub data: Vec<u8>,
}

/// A packet that is sent to the client to update the block at the specified coordinates.
#[derive(Serialize, Deserialize)]
pub struct BlockChangePacket {
    pub x: i32,
    pub y: i32,
    pub from_main_x: i32,
    pub from_main_y: i32,
    pub block: BlockId,
}

/// A packet that is sent to the server, when client
/// right clicks a block.
#[derive(Serialize, Deserialize)]
pub struct BlockRightClickPacket {
    pub x: i32,
    pub y: i32,
}
