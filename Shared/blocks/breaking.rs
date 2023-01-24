use crate::blocks::{Blocks, CHUNK_SIZE};

/**
Breaking block struct represents a block that is currently being broken.
 */
pub(super) struct BreakingBlock {
    pub(super) break_progress: i32,
    pub(super) is_breaking: bool,
    pub(super) x: i32,
    pub(super) y: i32,
}

impl BreakingBlock {
    pub fn new() -> Self {
        Self {
            break_progress: 0,
            is_breaking: true,
            x: 0,
            y: 0
        }
    }
}

impl Blocks {
    /**
    Gets the breaking progress of a block.
     */
    pub fn get_break_progress(&mut self, x: i32, y: i32) -> i32 {
        for breaking_block in self.breaking_blocks.iter() {
            if breaking_block.x == x && breaking_block.y == y {
                return breaking_block.break_progress;
            }
        }
        0
    }

    /**
    Sets the break stage of a block, which is usually rendered.
     */
    pub fn get_break_stage(&mut self, x: i32, y: i32) -> i32 {
        (self.get_break_progress(x, y) as f64 / self.get_block_type_at(x, y).break_time as f64 * 9.0) as i32
    }

    /**
    Adds a block to the breaking list, which means that the block is being broken.
     */
    pub fn start_breaking_block(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x >= self.get_width() || y >= self.get_height() {
            panic!("Block is accessed out of bounds! x: {}, y: {}", x, y);
        }

        let mut breaking_block: Option<&mut BreakingBlock> = None;
        for i in self.breaking_blocks.iter_mut() {
            if i.x == x && i.y == y {
                breaking_block = Some(i);
                break;
            }
        }

        if breaking_block.is_none() {
            let mut new_breaking_block = BreakingBlock::new();
            new_breaking_block.x = x;
            new_breaking_block.y = y;
            self.breaking_blocks.push(new_breaking_block);
            breaking_block = Some(self.breaking_blocks.last_mut().unwrap());
        }

        breaking_block.unwrap().is_breaking = true;

        self.get_chunk(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count += 1;

        //let event = BlockStartedBreakingEvent::new(x, y);
        //self.block_started_breaking_event.send(event);
    }

    /**
    Stops breaking a block.
     */
    pub fn stop_breaking_block(&mut self, x: i32, y: i32) {
        assert!(x >= 0 && y >= 0 && x < self.get_width() && y < self.get_height(), "Block is accessed out of bounds! x: {}, y: {}", x, y);

        for breaking_block in self.breaking_blocks.iter_mut() {
            if breaking_block.x == x && breaking_block.y == y {
                breaking_block.is_breaking = false;
                self.get_chunk(x / CHUNK_SIZE, y / CHUNK_SIZE).breaking_blocks_count -= 1;
                //let event = BlockStoppedBreakingEvent::new(x, y);
                //self.block_stopped_breaking_event.send(event);
                break;
            }
        }
    }

    /**
    Updates the breaking progress of all blocks that are being broken.
     */
    pub fn update_breaking_blocks(&mut self, frame_length: f64) {
        for i in 0..self.breaking_blocks.len() {
            if self.breaking_blocks[i].is_breaking {
                self.breaking_blocks[i].break_progress += frame_length as i32;
                if self.breaking_blocks[i].break_progress > self.get_block_type_at(self.breaking_blocks[i].x, self.breaking_blocks[i].y).break_time {
                    self.break_block(self.breaking_blocks[i].x, self.breaking_blocks[i].y);
                }
            }
        }
    }

    /**
    Breaks a block and triggers the block break event which can be used to drop items.
     */
    pub fn break_block(&mut self, x: i32, y: i32){
        let transformed_x = x - self.get_block_from_main(x, y).0 as i32;
        let transformed_y = y - self.get_block_from_main(x, y).1 as i32;

        let _event = BlockBreakEvent::new(transformed_x, transformed_y);
        //self.block_break_event.send(event);
        //self.set_block_type(transformed_x, transformed_y, Rc::clone(&self.get_block_type(x, y)), 0, 0);
    }

    /**
    It only gets the breaking block count of a chunk, used to skip
    updating chunks that don't have any breaking blocks.
     */
    pub fn get_chunk_breaking_blocks_count(&mut self, x: i32, y: i32) -> i32 {
        self.get_chunk(x, y).breaking_blocks_count.into()
    }
}

/**
Event that is fired when a block is broken
 */
pub struct BlockBreakEvent {
    x: i32, y: i32
}
impl BlockBreakEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockBreakEvent{x, y} }
}
//impl Event for BlockBreakEvent {}

/**
Event that is fired when a block has started breaking
 */
pub struct BlockStartedBreakingEvent {
    x: i32, y: i32
}
impl BlockStartedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStartedBreakingEvent{x, y} }
}
//impl Event for BlockStartedBreakingEvent {}

/**
Event that is fired when a block has stopped breaking
 */
pub struct BlockStoppedBreakingEvent {
    x: i32, y: i32
}
impl BlockStoppedBreakingEvent {
    pub fn new(x: i32, y: i32) -> Self { BlockStoppedBreakingEvent{x, y} }
}
//impl Event for BlockStoppedBreakingEvent {}