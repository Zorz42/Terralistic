#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::libraries::events::EventManager;
    use crate::shared::blocks::Block;
    use crate::shared::blocks::BlockChangeEvent;
    use crate::shared::blocks::BlockId;
    use crate::shared::blocks::Blocks;
    use crate::shared::blocks::Tool;
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

        blocks.set_block(&mut events, 50, 50, block_id1).unwrap_err();
        blocks.set_block(&mut events, 51, 51, block_id1).unwrap_err();
        blocks.set_block(&mut events, 52, 52, block_id1).unwrap_err();
        blocks.set_block(&mut events, 100, 100, block_id1).unwrap_err();
        blocks.set_block(&mut events, -1, -1, block_id1).unwrap_err();
        blocks.set_block(&mut events, -100, 5, block_id1).unwrap_err();
        blocks.set_block(&mut events, 5, -100, block_id1).unwrap_err();
        blocks.set_block(&mut events, 2, 1000, block_id1).unwrap_err();
        blocks.set_block(&mut events, 1000, 2, block_id1).unwrap_err();
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
    /// no breaking overlay. This used to divide by zero and saturate to `i32::MAX`.
    #[test]
    fn test_break_stage_of_unbreakable_block_is_zero() {
        let (mut blocks, _breakable, unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_block(&mut events, 1, 1, unbreakable).unwrap();
        blocks.set_break_progress(1, 1, 500).unwrap();

        assert_eq!(blocks.get_break_stage(1, 1).unwrap(), 0);
    }

    /// The break stage indexes the 8 frame breaking texture, so it has to stay in range
    /// even when progress has reached `break_time`.
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

    // --- block types and tools ---

    #[test]
    fn test_air_is_registered_first_and_walkable() {
        let blocks = Blocks::new();
        let air = blocks.get_block_type(blocks.air()).unwrap();

        assert_eq!(air.name, "air");
        assert!(air.ghost, "air must be walk-through or entities cannot move");
        assert!(air.transparent, "air must let light through");
        assert!(air.break_time.is_none(), "air cannot be broken");
    }

    #[test]
    fn test_block_defaults() {
        let block = Block::new();
        assert_eq!(block.width, 1);
        assert_eq!(block.height, 1);
        assert!(!block.ghost);
        assert!(!block.transparent);
        assert!(block.break_time.is_none());
        assert!(block.inventory_slots.is_empty());
    }

    #[test]
    fn test_look_up_block_by_name() {
        let mut blocks = Blocks::new();
        let mut dirt = Block::new();
        dirt.name = "dirt".to_owned();
        let dirt_id = blocks.register_new_block_type(dirt);

        assert!(blocks.get_block_id_by_name("dirt").unwrap() == dirt_id);
        assert!(blocks.get_block_id_by_name("nothing").is_err());
    }

    #[test]
    fn test_get_all_block_ids_includes_air() {
        let mut blocks = Blocks::new();
        let mut dirt = Block::new();
        dirt.name = "dirt".to_owned();
        blocks.register_new_block_type(dirt);

        assert_eq!(blocks.get_all_block_ids().len(), 2, "air plus the one registered type");
    }

    #[test]
    fn test_unknown_block_id_is_an_error() {
        let blocks = Blocks::new();
        assert!(blocks.get_block_type(BlockId::undefined()).is_err());
    }

    #[test]
    fn test_tools() {
        let mut blocks = Blocks::new();

        let mut pickaxe = Tool::new();
        pickaxe.name = "pickaxe".to_owned();
        let pickaxe_id = blocks.register_new_tool_type(pickaxe);

        assert!(blocks.get_tool_id_by_name("pickaxe") == Some(pickaxe_id));
        assert!(blocks.get_tool_id_by_name("shovel").is_none());
        assert_eq!(blocks.get_tool_by_id(pickaxe_id).unwrap().name, "pickaxe");
    }

    // --- big blocks ---

    /// A block larger than 1x1 stores, for each of its tiles, the offset back to the main
    /// tile. That offset is what `break_block` follows to break the whole thing.
    #[test]
    fn test_big_block_offsets() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_big_block(&mut events, 2, 2, breakable, (0, 0)).unwrap();
        blocks.set_big_block(&mut events, 3, 2, breakable, (1, 0)).unwrap();

        assert_eq!(blocks.get_block_from_main(2, 2).unwrap(), (0, 0));
        assert_eq!(blocks.get_block_from_main(3, 2).unwrap(), (1, 0));
    }

    #[test]
    fn test_breaking_a_big_block_breaks_the_main_tile() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_big_block(&mut events, 2, 2, breakable, (0, 0)).unwrap();
        blocks.set_big_block(&mut events, 3, 2, breakable, (1, 0)).unwrap();

        // breaking the offset tile should clear the main one
        blocks.break_block(&mut events, 3, 2).unwrap();

        assert!(blocks.get_block(2, 2).unwrap() == blocks.air(), "the main tile should have been broken");
    }

    #[test]
    fn test_plain_set_block_clears_the_offset() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();

        blocks.set_big_block(&mut events, 3, 2, breakable, (1, 0)).unwrap();
        assert_eq!(blocks.get_block_from_main(3, 2).unwrap(), (1, 0));

        blocks.set_block(&mut events, 3, 2, blocks.air()).unwrap();
        assert_eq!(blocks.get_block_from_main(3, 2).unwrap(), (0, 0), "set_block should reset the offset");
    }

    // --- per block data ---

    #[test]
    fn test_block_data_defaults_to_empty() {
        let (blocks, _breakable, _unbreakable) = blocks_with_break_types();
        assert!(blocks.get_block_data(1, 1).unwrap().is_empty());
    }

    #[test]
    fn test_set_and_get_block_data() {
        let (mut blocks, _breakable, _unbreakable) = blocks_with_break_types();

        blocks.set_block_data(1, 1, vec![1, 2, 3]).unwrap();
        assert_eq!(blocks.get_block_data(1, 1).unwrap(), vec![1, 2, 3]);

        // setting it empty removes it again
        blocks.set_block_data(1, 1, vec![]).unwrap();
        assert!(blocks.get_block_data(1, 1).unwrap().is_empty());
    }

    #[test]
    fn test_block_data_out_of_bounds() {
        let (mut blocks, _breakable, _unbreakable) = blocks_with_break_types();
        blocks.set_block_data(99, 99, vec![1]).unwrap_err();
        blocks.get_block_data(99, 99).unwrap_err();
    }

    // --- block inventories ---

    #[test]
    fn test_block_without_slots_has_no_inventory() {
        let (blocks, _breakable, _unbreakable) = blocks_with_break_types();
        assert_eq!(blocks.get_block_inventory_size(1, 1).unwrap(), 0);
    }

    #[test]
    fn test_block_with_slots_reports_its_size() {
        let mut blocks = Blocks::new();

        let mut chest = Block::new();
        chest.name = "chest".to_owned();
        chest.inventory_slots = vec![(0, 0), (1, 0), (2, 0)];
        let chest_id = blocks.register_new_block_type(chest);

        blocks.create((5, 5));
        let mut events = EventManager::new();
        blocks.set_block(&mut events, 1, 1, chest_id).unwrap();

        assert_eq!(blocks.get_block_inventory_size(1, 1).unwrap(), 3);
        assert_eq!(blocks.get_block_inventory_data(1, 1).unwrap().len(), 3);
    }

    /// An inventory write of the wrong length is rejected rather than silently resized.
    #[test]
    fn test_block_inventory_size_is_enforced() {
        let mut blocks = Blocks::new();

        let mut chest = Block::new();
        chest.name = "chest".to_owned();
        chest.inventory_slots = vec![(0, 0), (1, 0)];
        let chest_id = blocks.register_new_block_type(chest);

        blocks.create((5, 5));
        let mut events = EventManager::new();
        blocks.set_block(&mut events, 1, 1, chest_id).unwrap();

        blocks.set_block_inventory_data(1, 1, vec![None], &mut events).unwrap_err();
        blocks.set_block_inventory_data(1, 1, vec![None, None], &mut events).unwrap();
    }

    // --- serialization ---

    /// As with walls, the grid travels but the type registry does not, so the same types
    /// have to be registered in the same order before loading.
    #[test]
    fn test_blocks_serialize_round_trip() {
        let (mut blocks, breakable, unbreakable) = blocks_with_break_types();
        let mut events = EventManager::new();
        blocks.set_block(&mut events, 1, 1, breakable).unwrap();
        blocks.set_block(&mut events, 4, 4, unbreakable).unwrap();
        blocks.set_block_data(2, 2, vec![7, 7]).unwrap();

        let bytes = blocks.serialize().unwrap();

        let (mut restored, _b, _u) = blocks_with_break_types();
        restored.deserialize(&bytes).unwrap();

        assert_eq!(restored.get_size(), (5, 5));
        assert!(restored.get_block(1, 1).unwrap() == breakable);
        assert!(restored.get_block(4, 4).unwrap() == unbreakable);
        assert!(restored.get_block(0, 0).unwrap() == restored.air());
        assert_eq!(restored.get_block_data(2, 2).unwrap(), vec![7, 7]);
    }

    #[test]
    fn test_blocks_deserialize_rejects_garbage() {
        let mut blocks = Blocks::new();
        blocks.deserialize(&[9, 9, 9, 9]).unwrap_err();
    }

    #[test]
    fn test_create_from_ragged_block_ids_fails() {
        let (mut blocks, breakable, _unbreakable) = blocks_with_break_types();
        let ragged = vec![vec![breakable, breakable], vec![breakable]];
        blocks.create_from_block_ids(&ragged).unwrap_err();
    }

    #[test]
    fn test_create_from_empty_block_ids_fails() {
        let (mut blocks, _breakable, _unbreakable) = blocks_with_break_types();
        blocks.create_from_block_ids(&[]).unwrap_err();
    }

    // --- the lua mod interface ---

    /// Drives the block half of the mod API the way `base_game` does: register a block
    /// type from lua, then read it back from Rust.
    #[test]
    fn test_lua_can_register_a_block_type() {
        use crate::libraries::scripting::{ScriptHost, ScriptModule};
        use crate::shared::blocks::init_blocks_mod_interface;
        use crate::shared::MOD_FUNCTION_PREFIX;
        use std::sync::{Arc, Mutex};

        let lua = r#"
            function init()
                dirt = terralistic_register_block_type(
                    nil,    -- effective_tool
                    10,     -- required_tool_power
                    false,  -- ghost
                    false,  -- transparent
                    "dirt", -- name
                    {},     -- connects_to
                    700,    -- break_time
                    1, 2, 3,-- light emission
                    1, 1,   -- size
                    true,   -- can_update_states
                    false,  -- feet_collidable
                    false,  -- clickable
                    {}      -- inventory_slots
                )
            end
            function update() end
            function stop() end
        "#;

        let blocks = Arc::new(Mutex::new(Blocks::new()));
        let mut mods = ScriptHost::new(vec![ScriptModule::new("test".to_owned(), lua.to_owned(), std::collections::HashMap::new())], MOD_FUNCTION_PREFIX);
        init_blocks_mod_interface(&blocks, &mut mods).unwrap();
        mods.init().unwrap();

        let blocks = blocks.lock().unwrap();
        let id = blocks.get_block_id_by_name("dirt").unwrap();
        let block = blocks.get_block_type(id).unwrap();

        assert_eq!(block.name, "dirt");
        assert_eq!(block.required_tool_power, 10);
        assert_eq!(block.break_time, Some(700));
        assert_eq!(block.light_emission, (1, 2, 3));
        assert!(block.can_update_states);
        assert!(!block.ghost);
    }

    /// `connect_blocks` is symmetric: each type ends up listing the other.
    #[test]
    fn test_lua_connect_blocks_is_symmetric() {
        use crate::libraries::scripting::{ScriptHost, ScriptModule};
        use crate::shared::blocks::init_blocks_mod_interface;
        use crate::shared::MOD_FUNCTION_PREFIX;
        use std::sync::{Arc, Mutex};

        let lua = r#"
            function make(name)
                return terralistic_register_block_type(nil, 0, false, false, name, {}, nil, 0, 0, 0, 1, 1, false, false, false, {})
            end
            function init()
                a = make("a")
                b = make("b")
                terralistic_connect_blocks(a, b)
            end
            function update() end
            function stop() end
        "#;

        let blocks = Arc::new(Mutex::new(Blocks::new()));
        let mut mods = ScriptHost::new(vec![ScriptModule::new("test".to_owned(), lua.to_owned(), std::collections::HashMap::new())], MOD_FUNCTION_PREFIX);
        init_blocks_mod_interface(&blocks, &mut mods).unwrap();
        mods.init().unwrap();

        let blocks = blocks.lock().unwrap();
        let a = blocks.get_block_id_by_name("a").unwrap();
        let b = blocks.get_block_id_by_name("b").unwrap();

        assert!(blocks.get_block_type(a).unwrap().connects_to.contains(&b));
        assert!(blocks.get_block_type(b).unwrap().connects_to.contains(&a));
    }

    /// Looking up a name that was never registered surfaces as a lua error, not a silent
    /// default.
    #[test]
    fn test_lua_unknown_block_name_is_an_error() {
        use crate::libraries::scripting::{ScriptHost, ScriptModule};
        use crate::shared::blocks::init_blocks_mod_interface;
        use crate::shared::MOD_FUNCTION_PREFIX;
        use std::sync::{Arc, Mutex};

        let lua = r#"
            function init() end
            function update() end
            function stop() end
            function look_up() return terralistic_get_block_id_by_name("nope") end
        "#;

        let blocks = Arc::new(Mutex::new(Blocks::new()));
        let mut mods = ScriptHost::new(vec![ScriptModule::new("test".to_owned(), lua.to_owned(), std::collections::HashMap::new())], MOD_FUNCTION_PREFIX);
        init_blocks_mod_interface(&blocks, &mut mods).unwrap();
        mods.init().unwrap();

        mods.get_module(0).unwrap().call_function::<(), i32>("look_up", ()).unwrap_err();
    }

    #[test]
    fn test_lua_can_register_a_tool() {
        use crate::libraries::scripting::{ScriptHost, ScriptModule};
        use crate::shared::blocks::init_blocks_mod_interface;
        use crate::shared::MOD_FUNCTION_PREFIX;
        use std::sync::{Arc, Mutex};

        let lua = r#"
            function init() pickaxe = terralistic_register_tool("pickaxe") end
            function update() end
            function stop() end
        "#;

        let blocks = Arc::new(Mutex::new(Blocks::new()));
        let mut mods = ScriptHost::new(vec![ScriptModule::new("test".to_owned(), lua.to_owned(), std::collections::HashMap::new())], MOD_FUNCTION_PREFIX);
        init_blocks_mod_interface(&blocks, &mut mods).unwrap();
        mods.init().unwrap();

        assert!(blocks.lock().unwrap().get_tool_id_by_name("pickaxe").is_some());
    }
}
