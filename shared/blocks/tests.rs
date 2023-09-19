#[allow(unused_imports)]
use anyhow::Result;

#[allow(unused_imports)]
use crate::libraries::events::EventManager;
#[allow(unused_imports)]
use crate::shared::blocks::Block;
#[allow(unused_imports)]
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

#[allow(dead_code)]
fn assert_ok_and_eq<T: PartialEq>(result: Result<T>, expected: T) {
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert!(value == expected);
    }
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

    assert!(blocks.set_block(&mut events, 0, 0, block_id1).is_ok());
    assert!(blocks.set_block(&mut events, 1, 0, block_id2).is_ok());
    assert!(blocks.set_block(&mut events, 0, 1, block_id1).is_ok());
    assert!(blocks.set_block(&mut events, 1, 1, block_id2).is_ok());
    assert_ok_and_eq(blocks.get_block(0, 0), block_id1);
    assert_ok_and_eq(blocks.get_block(1, 0), block_id2);
    assert_ok_and_eq(blocks.get_block(0, 1), block_id1);
    assert_ok_and_eq(blocks.get_block(1, 1), block_id2);
    assert_ok_and_eq(blocks.get_block(2, 2), blocks.air());
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
