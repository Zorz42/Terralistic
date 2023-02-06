use super::Blocks;
use crate::libraries::events::{Event, EventManager};
use anyhow::{anyhow, Result};

/**
Breaking block struct represents a block that is currently being broken.
 */
pub struct BreakingBlock {
    pub break_progress: i32,
    pub is_breaking: bool,
    pub coord: (i32, i32),
}

impl BreakingBlock {
    pub fn new(coord: (i32, i32)) -> Self {
        Self {
            break_progress: 0,
            is_breaking: true,
            coord,
        }
    }

    pub fn get_coord(&self) -> (i32, i32) {
        self.coord
    }
}

impl Blocks {
    /**
    Gets the breaking progress of a block.
     */
    pub fn get_break_progress(&self, x: i32, y: i32) -> Result<i32> {
        self.block_data.map.translate_coords(x, y)?;

        for breaking_block in self.breaking_blocks.iter() {
            if breaking_block.coord == (x, y) {
                return Ok(breaking_block.break_progress);
            }
        }
        Ok(0)
    }

    /**
    Sets the breaking progress of a block.
     */
    pub fn set_break_progress(&mut self, x: i32, y: i32, progress: i32) -> Result<()> {
        self.block_data.map.translate_coords(x, y)?;

        for breaking_block in self.breaking_blocks.iter_mut() {
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

    /**
    Gets the break stage of a block, which is usually rendered.
     */
    pub fn get_break_stage(&self, x: i32, y: i32) -> Result<i32> {
        Ok(
            (self.get_break_progress(x, y)? as f32
                / self.get_block_type_at(x, y)?.break_time as f32
                * 9.0) as i32,
        )
    }

    /**
    Adds a block to the breaking list, which means that the block is being broken.
     */
    pub fn start_breaking_block(
        &mut self,
        events: &mut EventManager,
        x: i32,
        y: i32,
    ) -> Result<()> {
        self.block_data.map.translate_coords(x, y)?;

        let mut breaking_block: Option<&mut BreakingBlock> = None;
        for i in self.breaking_blocks.iter_mut() {
            if i.coord == (x, y) {
                breaking_block = Some(i);
                break;
            }
        }

        let breaking_block = {
            if let Some(breaking_block) = breaking_block {
                breaking_block
            } else {
                let new_breaking_block = BreakingBlock::new((x, y));
                self.breaking_blocks.push(new_breaking_block);
                self.breaking_blocks
                    .last_mut()
                    .ok_or(anyhow!("Failed to get last breaking block!"))?
            }
        };

        breaking_block.is_breaking = true;

        let event = BlockStartedBreakingEvent { x, y };
        events.push_event(Event::new(event));

        Ok(())
    }

    /**
    Stops breaking a block.
     */
    pub fn stop_breaking_block(&mut self, events: &mut EventManager, x: i32, y: i32) -> Result<()> {
        self.block_data.map.translate_coords(x, y)?;

        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.get_coord() == (x, y) {
                breaking_block.is_breaking = false;
                let event = BlockStoppedBreakingEvent { x, y };
                events.push_event(Event::new(event));
                break;
            }
        }
        Ok(())
    }

    /**
    Updates the breaking progress of all blocks that are being broken.
     */
    pub fn update_breaking_blocks(
        &mut self,
        events: &mut EventManager,
        frame_length: f32,
    ) -> Result<()> {
        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.is_breaking {
                breaking_block.break_progress += frame_length as i32;
            }
        }

        let mut broken_blocks = Vec::new();
        for breaking_block in self.breaking_blocks.iter() {
            if breaking_block.break_progress
                > self
                    .get_block_type_at(breaking_block.get_coord().0, breaking_block.get_coord().1)?
                    .break_time
            {
                broken_blocks.push(breaking_block.get_coord());
            }
        }

        for broken_block in broken_blocks.iter() {
            let (x, y) = *broken_block;
            let transformed_x = x - self.get_block_from_main(x, y)?.0;
            let transformed_y = y - self.get_block_from_main(x, y)?.1;

            //let _event = BlockBreakEvent::new(transformed_x, transformed_y);
            //self.block_break_event.send(event);

            self.set_block(events, transformed_x, transformed_y, self.air)?;

            self.breaking_blocks
                .retain(|breaking_block| breaking_block.get_coord() != *broken_block);
        }

        Ok(())
    }

    /**
    returns and immutable reference to the breaking blocks
     */
    pub fn get_breaking_blocks(&self) -> &Vec<BreakingBlock> {
        &self.breaking_blocks
    }
}

/**
Event that is fired when a block is broken
 */
pub struct BlockBreakEvent {
    pub x: i32,
    pub y: i32,
}

/**
Event that is fired when a block has started breaking
 */
pub struct BlockStartedBreakingEvent {
    pub x: i32,
    pub y: i32,
}

/**
Event that is fired when a block has stopped breaking
 */
pub struct BlockStoppedBreakingEvent {
    pub x: i32,
    pub y: i32,
}
