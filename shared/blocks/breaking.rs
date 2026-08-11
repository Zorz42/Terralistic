use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::{BlockId, ToolId};

use super::Blocks;

/// How many frames the breaking overlay texture (`misc:breaking.opa`, 8x64) has.
/// A break stage is an index into it, so the valid range is `0..BREAK_STAGES`.
pub const BREAK_STAGES: i32 = 8;

/// Breaking block struct represents a block that is currently being broken.
#[derive(Clone)]
pub struct BreakingBlock {
    pub break_progress: i32,
    pub is_breaking: bool,
    pub coord: (i32, i32),
}

impl BreakingBlock {
    #[must_use]
    pub const fn new(coord: (i32, i32)) -> Self {
        Self {
            break_progress: 0,
            is_breaking: true,
            coord,
        }
    }
}

impl Blocks {
    /// Gets the breaking progress of a block.
    pub fn get_break_progress(&self, x: i32, y: i32) -> Result<i32> {
        self.block_data.map.translate_coords(x, y)?;

        for breaking_block in &self.breaking_blocks {
            if breaking_block.coord == (x, y) {
                return Ok(breaking_block.break_progress);
            }
        }
        Ok(0)
    }

    /// Sets the breaking progress of a block.
    pub fn set_break_progress(&mut self, x: i32, y: i32, progress: i32) -> Result<()> {
        // check if coordinates are out of bounds
        self.block_data.map.translate_coords(x, y)?;

        // an unbreakable block must never enter the breaking list, or update_breaking_blocks
        // would go on to break it - start_breaking_block already refuses these
        if self.get_block_type_at(x, y)?.break_time.is_none() {
            return Ok(());
        }

        for breaking_block in &mut self.breaking_blocks {
            if breaking_block.coord == (x, y) {
                breaking_block.break_progress = progress;
                return Ok(());
            }
        }

        let mut breaking_block = BreakingBlock::new((x, y));
        breaking_block.break_progress = progress;
        breaking_block.is_breaking = false;
        self.breaking_blocks.push(breaking_block);
        Ok(())
    }

    /// Gets the break stage of a block, which is rendered.
    /// An unbreakable block never shows breaking progress, and the result is always a
    /// valid index into the breaking texture.
    pub fn get_break_stage(&self, x: i32, y: i32) -> Result<i32> {
        let Some(break_time) = self.get_block_type_at(x, y)?.break_time else {
            return Ok(0);
        };

        if break_time <= 0 {
            return Ok(0);
        }

        Ok((self.get_break_progress(x, y)? * BREAK_STAGES / break_time).clamp(0, BREAK_STAGES - 1))
    }

    /// Adds a block to the breaking list, which means that the block is being broken.
    pub fn start_breaking_block(&mut self, events: &mut EventManager, x: i32, y: i32, tool: Option<ToolId>, tool_power: i32) -> Result<()> {
        if self.get_block_type_at(x, y)?.break_time.is_none() {
            return Ok(());
        }

        if let Some(effective_tool) = self.get_block_type_at(x, y)?.effective_tool {
            if Some(effective_tool) != tool || self.get_block_type_at(x, y)?.required_tool_power > tool_power {
                return Ok(());
            }
        }

        let mut breaking_block: Option<&mut BreakingBlock> = None;
        for i in &mut self.breaking_blocks {
            if i.coord == (x, y) {
                breaking_block = Some(i);
                break;
            }
        }

        if let Some(breaking_block) = breaking_block {
            breaking_block.is_breaking = true;
        } else {
            self.breaking_blocks.push(BreakingBlock::new((x, y)));
        }

        let event = BlockStartedBreakingEvent { x, y, tool, tool_power };
        events.push_event(Event::new(event));

        Ok(())
    }

    /// Stops breaking a block.
    pub fn stop_breaking_block(&mut self, events: &mut EventManager, x: i32, y: i32) -> Result<()> {
        self.block_data.map.translate_coords(x, y)?;

        for breaking_block in &mut self.breaking_blocks {
            if breaking_block.coord == (x, y) {
                breaking_block.is_breaking = false;
                let event = BlockStoppedBreakingEvent { x, y };
                events.push_event(Event::new(event));
                break;
            }
        }
        Ok(())
    }

    /// Updates the breaking progress of all blocks that are being broken.
    pub fn update_breaking_blocks(&mut self, events: &mut EventManager, frame_length: f32) -> Result<()> {
        for breaking_block in &mut self.breaking_blocks {
            if breaking_block.is_breaking {
                breaking_block.break_progress += frame_length as i32;
            }
        }

        let mut broken_blocks = Vec::new();
        for breaking_block in &self.breaking_blocks {
            // break_time of None means unbreakable, so such a block never completes.
            // It used to fall back to 1, which broke it on the very next update.
            let Some(break_time) = self.get_block_type_at(breaking_block.coord.0, breaking_block.coord.1)?.break_time else {
                continue;
            };

            if breaking_block.break_progress > break_time {
                broken_blocks.push(breaking_block.coord);
            }
        }

        for broken_block in &broken_blocks {
            self.break_block(events, broken_block.0, broken_block.1)?;

            self.breaking_blocks.retain(|breaking_block| breaking_block.coord != *broken_block);
        }

        Ok(())
    }

    /// Returns and immutable reference to the breaking blocks
    #[must_use]
    pub const fn get_breaking_blocks(&self) -> &Vec<BreakingBlock> {
        &self.breaking_blocks
    }
}

/// A packet that is sent to the server,
/// when the server should start to
/// break the block.
#[derive(Serialize, Deserialize)]
pub struct BlockBreakStartPacket {
    pub x: i32,
    pub y: i32,
    pub tool: Option<ToolId>,
    pub tool_power: i32,
}

/// A packet that is sent to the server, when
/// client starts to break a block
#[derive(Serialize, Deserialize)]
pub struct ClientBlockBreakStartPacket {
    pub x: i32,
    pub y: i32,
}

/// A packet that is sent to the server, when client stops
/// breaking a block and when the server should stop
/// breaking the block.
#[derive(Serialize, Deserialize)]
pub struct BlockBreakStopPacket {
    pub x: i32,
    pub y: i32,
    pub break_time: i32,
}

/// Event that is fired when a block is broken
pub struct BlockBreakEvent {
    pub prev_block_id: BlockId,
    pub x: i32,
    pub y: i32,
}

/// Event that is fired when a block has started breaking
pub struct BlockStartedBreakingEvent {
    pub x: i32,
    pub y: i32,
    pub tool: Option<ToolId>,
    pub tool_power: i32,
}

/// Event that is fired when a block has stopped breaking
pub struct BlockStoppedBreakingEvent {
    pub x: i32,
    pub y: i32,
}
