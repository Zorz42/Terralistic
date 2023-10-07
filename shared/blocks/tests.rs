#![cfg(test)]
#![allow(clippy::unwrap_used)]
mod tests {
    use anyhow::Result;

    use crate::libraries::events::EventManager;
    use crate::shared::blocks::Block;
    use crate::shared::blocks::BlockChangeEvent;
    use crate::shared::blocks::Blocks;

    #[test]
    fn test_blocks_new() {
        let blocks = Blocks::new();
        assert_eq!(blocks.get_width(), 0);
        assert_eq!(blocks.get_height(), 0);
    }

    #[test]
    fn test_blocks_create_dimensions() {
        let mut blocks = Blocks::new();
        blocks.create(42, 50);
        assert_eq!(blocks.get_width(), 42);
        assert_eq!(blocks.get_height(), 50);
    }

    #[test]
    fn test_blocks_create_dimensions_twice() {
        let mut blocks = Blocks::new();
        blocks.create(42, 50);
        blocks.create(10, 10);
        assert_eq!(blocks.get_width(), 10);
        assert_eq!(blocks.get_height(), 10);
    }

    fn assert_ok_and_eq<T: PartialEq>(result: Result<T>, expected: &T) {
        assert!(result.unwrap() == *expected);
    }

    #[test]
    fn test_blocks_set_get() {
        let mut blocks = Blocks::new();
        blocks.create(50, 50);
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
        blocks.create(50, 50);
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

        assert_eq!(blocks.get_width(), 4);
        assert_eq!(blocks.get_height(), 3);
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
        blocks.create(50, 50);
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
}
