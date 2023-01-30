/**this test tests whether the block setting and reading works corretly.
For now it only really cares about the correct block id and not any other data */
#[test]
fn test_block_types(){
    let mut blocks_obj = blocks::Blocks::new();
    blocks_obj.create(128, 128);
    for x in 0..128 {
        for y in 0..128 {
            assert_eq!(blocks_obj.get_block_type(x, y).id, 0, "not all blocks are initialized to air");
        }
    }
    let block_type_rand = BlockType::new("rand".to_string());
    blocks_obj.register_new_block_type(block_type_rand);
    let block_type_rand = blocks_obj.get_block_type_by_name("rand".to_string()).unwrap();
    blocks_obj.set_block(0, 0, block_type_rand, 0, 0);
    blocks_obj.set_block(1, 0, blocks_obj.get_block_type_by_id(1), 0, 0);
    blocks_obj.set_block(2, 0, blocks_obj.get_block_type_by_id(1), 0, 0);
    blocks_obj.set_block(1, 0, blocks_obj.get_block_type_by_id(0), 0, 0);
    assert_eq!(blocks_obj.get_block_type(0, 0).id, 1, "block type not set correctly");
    assert_eq!(blocks_obj.get_block_type(1, 0).id, 0, "block type not set correctly");
    assert_eq!(blocks_obj.get_block_type(2, 0).id, 1, "block type not set correctly");
}

#[test]
fn test_save_load_of_blocks(){//TODO: test saving and loading of custom data
    let mut blocks_obj = blocks::Blocks::new();
    blocks_obj.create(128, 128);

    let mut block_types: Vec<BlockType> = Vec::new();
    let mut copied_blocks: Vec<blocks::Block> = Vec::new();

    for i in 0..10 {
        let block_t = blocks::BlockType::new(format!("block{}", i));
        block_types.push(block_t);
        blocks_obj.register_new_block_type(block_types[block_types.len() - 1].clone());
    }

    for i in 0..128 {
        for j in 0..128 {
            let num = (rand::random::<u32>() % blocks_obj.get_number_block_types() as u32) as i32;
            blocks_obj.set_block(i, j, blocks_obj.get_block_type_by_id(num), 0, 0);
            copied_blocks.push(blocks_obj.get_block(i, j).deref().clone());
        }
    }

    let serial = blocks_obj.to_serial();
    let mut blocks_obj = blocks::Blocks::new();
    blocks_obj.create(128, 128);
    for i in 0..10 {
        blocks_obj.register_new_block_type(block_types[i].clone());
    }
    blocks_obj.from_serial(serial);

    assert_eq!(block_types.len() as i32 + 1, blocks_obj.get_number_block_types(), "block types not loaded correctly");

    for i in 0..copied_blocks.len() as i32 {
        assert_eq!(blocks_obj.get_block(i / 128, i % 128).deref().id, copied_blocks[i as usize].id, "{}",  format!("block {} not loaded correctly", i));
    }
}