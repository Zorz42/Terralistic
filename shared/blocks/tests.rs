#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::libraries::events::EventManager;
    use crate::shared::blocks::Block;
    use crate::shared::blocks::BlockChangeEvent;
    use crate::shared::blocks::Blocks;
    use crate::shared::blocks::BREAK_STAGES;

    #[test]
    fn test_blocks_new() {
        let blocks = Blocks::new();
        assert_eq!(blocks.get_size(), (0, 0));
    }

    #[test]
    fn test_blocks_create_dimensions() {
        let mut blocks = Blocks::new();
        blocks.create((42, 50));
        assert_eq!(blocks.get_size(), (42, 50));
    }

    #[test]
    fn test_blocks_create_dimensions_twice() {
        let mut blocks = Blocks::new();
        blocks.create((42, 50));
        blocks.create((10, 11));
        assert_eq!(blocks.get_size(), (10, 11));
    }

    fn assert_ok_and_eq<T: PartialEq>(result: Result<T>, expected: &T) {
        assert!(result.unwrap() == *expected);
    }

    #[test]
    fn test_blocks_set_get() {
        let mut blocks = Blocks::new();
        blocks.create((50, 50));
        let block_type1 = Block::new();
        let block_type2 = Block::new();
        let block_id1 = blocks.register_new_block_type(block_type1);
        let block_id2 = blocks.register_new_block_type(block_type2);

        let mut events = EventManager::new();

        blocks.set_block(&mut events, 0, 0, block_id1).unwrap();
        blocks.set_block(&mut events, 1, 0, block_id2).unwrap();
        blocks.set_block(&mut events, 0, 1, block_id1).unwrap();
        blocks.set_block(&mut events, 1, 1, block_id2).unwrap();
        assert_ok_and_eq(blocks.get_block(0, 0), &block_id1);
        assert_ok_and_eq(blocks.get_block(1, 0), &block_id2);
        assert_ok_and_eq(blocks.get_block(0, 1), &block_id1);
        assert_ok_and_eq(blocks.get_block(1, 1), &block_id2);
        assert_ok_and_eq(blocks.get_block(2, 2), &blocks.air());
    }

    #[test]
    fn test_blocks_set_out_of_bound() {
        let mut blocks = Blocks::new();
        blocks.create((50, 50));
        let block_type1 = Block::new();
        let block_id1 = blocks.register_new_block_type(block_type1);

        let mut events = EventManager::new();

        assert!(blocks.set_block(&mut events, 50, 50, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 51, 51, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 52, 52, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 100, 100, block_id1).is_err());
        assert!(blocks.set_block(&mut events, -1, -1, block_id1).is_err());
        assert!(blocks.set_block(&mut events, -100, 5, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 5, -100, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 2, 1000, block_id1).is_err());
        assert!(blocks.set_block(&mut events, 1000, 2, block_id1).is_err());
    }

    #[test]
    fn test_blocks_create_from_block_ids() {
        let mut blocks = Blocks::new();

        let block_type1 = Block::new();
        let block_type2 = Block::new();
        let block_id1 = blocks.register_new_block_type(block_type1);
        let block_id2 = blocks.register_new_block_type(block_type2);

        let blocks_vector = vec![
            vec![block_id1, block_id1, block_id1],
            vec![block_id1, block_id2, block_id2],
            vec![block_id1, block_id2, block_id1],
            vec![block_id1, block_id2, block_id2],
        ];
        blocks.create_from_block_ids(&blocks_vector).unwrap();

        assert_eq!(blocks.get_size(), (4, 3));
        assert_ok_and_eq(blocks.get_block(0, 0), &block_id1);
        assert_ok_and_eq(blocks.get_block(0, 1), &block_id1);
        assert_ok_and_eq(blocks.get_block(0, 2), &block_id1);
        assert_ok_and_eq(blocks.get_block(1, 0), &block_id1);
        assert_ok_and_eq(blocks.get_block(1, 1), &block_id2);
        assert_ok_and_eq(blocks.get_block(1, 2), &block_id2);
        assert_ok_and_eq(blocks.get_block(2, 0), &block_id1);
        assert_ok_and_eq(blocks.get_block(2, 1), &block_id2);
        assert_ok_and_eq(blocks.get_block(2, 2), &block_id1);
        assert_ok_and_eq(blocks.get_block(3, 0), &block_id1);
        assert_ok_and_eq(blocks.get_block(3, 1), &block_id2);
        assert_ok_and_eq(blocks.get_block(3, 2), &block_id2);
    }

    #[test]
    fn test_set_spawns_event() {
        let mut blocks = Blocks::new();
        blocks.create((50, 50));
        let block_type1 = Block::new();
        let block_type2 = Block::new();
        let block_id1 = blocks.register_new_block_type(block_type1);
        let block_id2 = blocks.register_new_block_type(block_type2);

        let mut events = EventManager::new();

        blocks.set_block(&mut events, 0, 0, block_id1).unwrap();
        blocks.set_block(&mut events, 2, 1, block_id1).unwrap();
        blocks.set_block(&mut events, 3, 3, block_id1).unwrap();
        blocks.set_block(&mut events, 3, 3, block_id1).unwrap();
        blocks.set_block(&mut events, 3, 3, block_id2).unwrap();

        let event = events.pop_event().unwrap();
        let event = event.downcast::<BlockChangeEvent>().unwrap();
        assert_eq!(event.x, 0);
        assert_eq!(event.y, 0);
        assert!(event.prev_block == blocks.air());

        let event = events.pop_event().unwrap();
        let event = event.downcast::<BlockChangeEvent>().unwrap();
        assert_eq!(event.x, 2);
        assert_eq!(event.y, 1);
        assert!(event.prev_block == blocks.air());

        let event = events.pop_event().unwrap();
        let event = event.downcast::<BlockChangeEvent>().unwrap();
        assert_eq!(event.x, 3);
        assert_eq!(event.y, 3);
        assert!(event.prev_block == blocks.air());

        let event = events.pop_event().unwrap();
        let event = event.downcast::<BlockChangeEvent>().unwrap();
        assert_eq!(event.x, 3);
        assert_eq!(event.y, 3);
        assert!(event.prev_block == block_id1);

        let event = events.pop_event();
        assert!(event.is_none());
    }

    /// Builds a 5x5 world with a breakable and an unbreakable block type registered.
    fn blocks_with_break_types() -> (Blocks, crate::shared::blocks::BlockId, crate::shared::blocks::BlockId) {
        let mut blocks = Blocks::new();

        let mut breakable = Block::new();
        breakable.name = "breakable".to_owned();
        breakable.break_time = Some(1000);
        let breakable_id = blocks.register_new_block_type(breakable);

        let mut unbreakable = Block::new();
        unbreakable.name = "unbreakable".to_owned();
        unbreakable.break_time = None;
        let unbreakable_id = blocks.register_new_block_type(unbreakable);

        blocks.create((5, 5));

        (blocks, breakable_id, unbreakable_id)
    }

    /// `set_break_progress` is a second way into the breaking list, and unlike
    /// `start_breaking_block` it did not check whether the block can be broken at all.
    /// `update_breaking_blocks` then compared against `break_time.unwrap_or(1)`, so an
    /// unbreakable block was destroyed on the very next update.
    #[test]
    fn test_set_break_progress_cannot_destroy_an_unbreakable_block() {
        let (mut blocks, _breakable, unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_block(&mut events, 1, 1, unbreakable).unwrap();
        blocks.set_break_progress(1, 1, 5000).unwrap();
        blocks.update_breaking_blocks(&mut events, 0.0).unwrap();

        assert!(blocks.get_block(1, 1).unwrap() == unbreakable, "an unbreakable block was destroyed");
    }

    /// An unbreakable block has no break time to measure progress against, so it shows
    /// no breaking overlay. This used to divide by zero and saturate to i32::MAX.
    #[test]
    fn test_break_stage_of_unbreakable_block_is_zero() {
        let (mut blocks, _breakable, unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_block(&mut events, 1, 1, unbreakable).unwrap();
        blocks.set_break_progress(1, 1, 500).unwrap();

        assert_eq!(blocks.get_break_stage(1, 1).unwrap(), 0);
    }

    /// The break stage indexes the 8 frame breaking texture, so it has to stay in range
    /// even when progress has reached break_time.
    #[test]
    fn test_break_stage_stays_in_texture_range() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_block(&mut events, 1, 1, breakable).unwrap();

        for progress in [0, 1, 500, 999, 1000] {
            blocks.set_break_progress(1, 1, progress).unwrap();
            let stage = blocks.get_break_stage(1, 1).unwrap();
            assert!((0..BREAK_STAGES).contains(&stage), "progress {progress} gave break stage {stage}, outside 0..{BREAK_STAGES}");
        }
    }

    /// A breakable block still breaks normally.
    #[test]
    fn test_breakable_block_still_breaks() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_block(&mut events, 1, 1, breakable).unwrap();
        blocks.start_breaking_block(&mut events, 1, 1, None, 0).unwrap();

        for _ in 0..3 {
            blocks.update_breaking_blocks(&mut events, 1000.0).unwrap();
        }

        assert!(blocks.get_block(1, 1).unwrap() == blocks.air(), "a breakable block was not broken");
    }
}
