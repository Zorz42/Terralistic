use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};
use snap;

use crate::libraries::events::{Event, EventManager};
use crate::libraries::grid::Grid;
use crate::libraries::serialization;
use crate::shared::blocks::{Block, BlockBreakEvent, BreakingBlock, Tool};
use crate::shared::items::ItemStack;

// width of one block in pixels
pub const BLOCK_WIDTH: f32 = 8.0;
// how many window pixels does one block pixel take
pub const RENDER_SCALE: f32 = 2.0;
// how many window pixels does one block take
pub const RENDER_BLOCK_WIDTH: f32 = BLOCK_WIDTH * RENDER_SCALE;

#[derive(Serialize, Deserialize)]
pub(super) struct BlocksData {
    pub blocks: Grid<BlockId>,
    // tells how much blocks a block in a big block is from the main block, it is mostly (0, 0) so it is stored in a hashmap
    pub block_from_main: HashMap<usize, (i32, i32)>,
    // saves the extra block data, it is mostly empty so it is stored in a hashmap
    pub block_data: HashMap<usize, Vec<u8>>,
    // saves the block inventory slots data and it is also mostly empty
    pub block_inventory_data: HashMap<usize, Vec<Option<ItemStack>>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
                blocks: Grid::new_empty(),
                block_from_main: HashMap::new(),
                block_data: HashMap::new(),
                block_inventory_data: HashMap::new(),
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
    pub const fn get_size(&self) -> (u32, u32) {
        self.block_data.blocks.get_size()
    }

    #[must_use]
    pub const fn air(&self) -> BlockId {
        self.air
    }

    pub fn create(&mut self, size: (u32, u32)) {
        self.block_data.blocks = Grid::filled(size, self.air);
    }

    pub fn create_from_block_ids(&mut self, block_ids: &[Vec<BlockId>]) -> Result<()> {
        self.block_data.blocks = Grid::from_columns(block_ids)?;
        Ok(())
    }

    pub fn get_block(&self, x: i32, y: i32) -> Result<BlockId> {
        Ok(*self.block_data.blocks.get(x, y)?)
    }

    pub fn set_big_block(&mut self, events: &mut EventManager, x: i32, y: i32, block_id: BlockId, from_main: (i32, i32)) -> Result<()> {
        if block_id != self.get_block(x, y)? || from_main != self.get_block_from_main(x, y)? {
            let prev_block = self.get_block(x, y)?;

            self.set_block_data(x, y, vec![])?;
            self.block_data.blocks.set(x, y, block_id)?;

            self.breaking_blocks.retain(|b| b.coord != (x, y));
            self.set_block_from_main(x, y, from_main)?;

            let size = self.get_block_inventory_size(x, y)? as usize;
            self.set_block_inventory_data(x, y, vec![None; size], events)?;

            let event = BlockChangeEvent { x, y, prev_block };
            events.push_event(Event::new(event));
        }
        Ok(())
    }

    pub fn set_block(&mut self, events: &mut EventManager, x: i32, y: i32, block_id: BlockId) -> Result<()> {
        self.set_big_block(events, x, y, block_id, (0, 0))
    }

    fn set_block_from_main(&mut self, x: i32, y: i32, from_main: (i32, i32)) -> Result<()> {
        let index = self.block_data.blocks.translate_coords(x, y)?;

        if from_main.0 == 0 && from_main.1 == 0 {
            self.block_data.block_from_main.remove(&index);
        } else {
            self.block_data.block_from_main.insert(index, from_main);
        }
        Ok(())
    }

    pub fn get_block_from_main(&self, x: i32, y: i32) -> Result<(i32, i32)> {
        Ok(self.block_data.block_from_main.get(&self.block_data.blocks.translate_coords(x, y)?).copied().unwrap_or((0, 0)))
    }

    pub fn set_block_data(&mut self, x: i32, y: i32, data: Vec<u8>) -> Result<()> {
        let index = self.block_data.blocks.translate_coords(x, y)?;
        if data.is_empty() {
            self.block_data.block_data.remove(&index);
        } else {
            self.block_data.block_data.insert(index, data);
        }
        Ok(())
    }

    pub fn get_block_inventory_size(&self, x: i32, y: i32) -> Result<i32> {
        if self.get_block_from_main(x, y)? != (0, 0) {
            return Ok(0);
        }
        Ok(self.get_block_type_at(x, y)?.inventory_slots.len() as i32)
    }

    pub fn get_block_inventory_data(&self, x: i32, y: i32) -> Result<Vec<Option<ItemStack>>> {
        Ok(self
            .block_data
            .block_inventory_data
            .get(&self.block_data.blocks.translate_coords(x, y)?)
            .cloned()
            .unwrap_or_else(Vec::new))
    }

    pub fn set_block_inventory_data(&mut self, x: i32, y: i32, data: Vec<Option<ItemStack>>, events: &mut EventManager) -> Result<()> {
        let index = self.block_data.blocks.translate_coords(x, y)?;
        let size = self.get_block_inventory_size(x, y)?;

        if size != data.len() as i32 {
            bail!("Invalid inventory size");
        }

        let prev_data = self.get_block_inventory_data(x, y)?;

        if prev_data != data {
            if data.is_empty() {
                self.block_data.block_inventory_data.remove(&index);
            } else {
                self.block_data.block_inventory_data.insert(index, data);
            }
            events.push_event(Event::new(BlockInventoryChangeEvent { x, y }));
        }
        Ok(())
    }

    pub fn get_block_data(&self, x: i32, y: i32) -> Result<Vec<u8>> {
        Ok(self.block_data.block_data.get(&self.block_data.blocks.translate_coords(x, y)?).cloned().unwrap_or_else(Vec::new))
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&serialization::serialize(&self.block_data)?)?)
    }

    pub fn deserialize(&mut self, serial: &[u8]) -> Result<()> {
        self.block_data = serialization::deserialize(&snap::raw::Decoder::new().decompress_vec(serial)?)?;
        Ok(())
    }

    pub fn register_new_block_type(&mut self, mut block_type: Block) -> BlockId {
        let id = self.block_types.len() as i8;
        let result = BlockId { id };
        block_type.id = result;
        self.block_types.push(block_type);
        result
    }

    pub fn get_block_id_by_name(&self, name: &str) -> Result<BlockId> {
        for block_type in &self.block_types {
            if block_type.name == name {
                return Ok(block_type.id);
            }
        }
        bail!("Block type not found")
    }

    #[must_use]
    pub fn get_all_block_ids(&self) -> Vec<BlockId> {
        let mut result = Vec::new();
        for block_type in &self.block_types {
            result.push(block_type.id);
        }
        result
    }

    pub fn get_block_type(&self, id: BlockId) -> Result<&Block> {
        self.block_types.get(id.id as usize).ok_or_else(|| anyhow!("Invalid block id"))
    }

    pub fn get_block_type_at(&self, x: i32, y: i32) -> Result<&Block> {
        self.get_block_type(self.get_block(x, y)?)
    }

    pub fn break_block(&mut self, events: &mut EventManager, x: i32, y: i32) -> Result<()> {
        let transformed_x = x - self.get_block_from_main(x, y)?.0;
        let transformed_y = y - self.get_block_from_main(x, y)?.1;

        let prev_block_id = self.get_block_type_at(transformed_x, transformed_y)?.id;

        let event = BlockBreakEvent {
            x: transformed_x,
            y: transformed_y,
            prev_block_id,
        };
        events.push_event(Event::new(event));

        self.set_block(events, transformed_x, transformed_y, self.air())?;

        Ok(())
    }
}
pub struct BlockChangeEvent {
    pub x: i32,
    pub y: i32,
    pub prev_block: BlockId,
}

pub struct BlockRandomTickEvent {
    pub x: i32,
    pub y: i32,
}

pub struct BlockUpdateEvent {
    pub x: i32,
    pub y: i32,
}

pub struct BlockInventoryChangeEvent {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct BlocksWelcomePacket {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockChangePacket {
    pub x: i32,
    pub y: i32,
    pub from_main_x: i32,
    pub from_main_y: i32,
    pub block: BlockId,
    pub inventory: Vec<Option<ItemStack>>,
}

#[derive(Serialize, Deserialize)]
pub struct BlockRightClickPacket {
    pub x: i32,
    pub y: i32,
}
