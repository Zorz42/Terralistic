#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::libraries::fixed::Fixed;
    use crate::shared::blocks::{Block, Blocks};
    use crate::shared::liquids::{LiquidChangeEvent, LiquidId, LiquidType, Liquids, MAX_LIQUID_LEVEL};

    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;
    const SHAFT_X: i32 = 3;

    /// A world that is solid along the bottom row and up both sides, so that liquid poured
    /// into it has somewhere to settle and cannot leave. Everything else is air.
    fn world_with_a_basin() -> Blocks {
        let mut blocks = Blocks::new();

        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);

        blocks.create((WIDTH as u32, HEIGHT as u32));

        let mut events = EventManager::new();
        for x in 0..WIDTH {
            blocks.set_block(&mut events, x, HEIGHT - 1, solid_id).unwrap();
        }
        for y in 0..HEIGHT {
            blocks.set_block(&mut events, 0, y, solid_id).unwrap();
            blocks.set_block(&mut events, WIDTH - 1, y, solid_id).unwrap();
        }

        blocks
    }

    /// A world that is solid apart from one column, so liquid poured into it can only fall.
    /// The spreading tests want a basin; the falling ones want somewhere liquid stays put.
    fn world_with_a_shaft() -> Blocks {
        let mut blocks = Blocks::new();

        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);

        blocks.create((WIDTH as u32, HEIGHT as u32));

        let mut events = EventManager::new();
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                if x != SHAFT_X || y == HEIGHT - 1 {
                    blocks.set_block(&mut events, x, y, solid_id).unwrap();
                }
            }
        }

        blocks
    }

    /// Liquids for that world, with one fast liquid and one ten times slower.
    fn liquids_with_water() -> (Liquids, LiquidId, LiquidId) {
        let mut liquids = Liquids::new();

        let mut water = LiquidType::new();
        "water".clone_into(&mut water.name);
        water.flow_time = 100;
        water.speed_multiplier = Fixed::from_num(2, 5);
        let water_id = liquids.register_new_liquid_type(water);

        let mut tar = LiquidType::new();
        "tar".clone_into(&mut tar.name);
        tar.flow_time = 1000;
        tar.speed_multiplier = Fixed::from_num(1, 10);
        let tar_id = liquids.register_new_liquid_type(tar);

        liquids.create((WIDTH as u32, HEIGHT as u32));

        (liquids, water_id, tar_id)
    }

    /// The total amount of liquid in the world. Flowing moves liquid around, it never
    /// creates or destroys any, so this is the invariant most of these tests lean on.
    fn total_liquid(liquids: &Liquids) -> u32 {
        let mut total = 0;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                total += u32::from(liquids.get_liquid_level(x, y).unwrap());
            }
        }
        total
    }

    /// Runs the simulation for `steps` flow steps of the fast liquid.
    fn run(liquids: &mut Liquids, blocks: &Blocks, events: &mut EventManager, steps: i32) {
        for _ in 0..steps {
            liquids.update_liquids(blocks, events, 100.0).unwrap();
        }
    }

    #[test]
    fn test_a_new_world_is_empty() {
        let (liquids, _water, _tar) = liquids_with_water();

        assert_eq!(total_liquid(&liquids), 0);
        assert_eq!(
            liquids.get_liquid_id_at(3, 3).unwrap(),
            liquids.empty,
            "a freshly created cell should hold the empty liquid, not an undefined one"
        );
    }

    #[test]
    fn test_water_falls_to_the_floor() {
        let blocks = world_with_a_shaft();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(SHAFT_X, 0, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 20);

        assert_eq!(liquids.get_liquid_level(SHAFT_X, 0).unwrap(), 0, "water should not have stayed where it was poured");
        assert_eq!(
            liquids.get_liquid_level(SHAFT_X, HEIGHT - 2).unwrap(),
            MAX_LIQUID_LEVEL,
            "water should have fallen to the lowest cell it can reach"
        );
    }

    #[test]
    fn test_water_spreads_sideways_along_the_floor() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, HEIGHT - 2, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 40);

        assert!(liquids.get_liquid_level(2, HEIGHT - 2).unwrap() > 0, "water should have spread left");
        assert!(liquids.get_liquid_level(4, HEIGHT - 2).unwrap() > 0, "water should have spread right");
    }

    #[test]
    fn test_flowing_conserves_the_total() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(2, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        liquids.set_liquid(4, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        let poured = total_liquid(&liquids);

        for _ in 0..60 {
            liquids.update_liquids(&blocks, &mut events, 100.0).unwrap();
            assert_eq!(total_liquid(&liquids), poured, "flowing changed how much liquid there is");
        }
    }

    /// The whole point of integer levels. The float version compared `level as i32`, so two
    /// cells that were never quite equal kept averaging each other forever - which on a
    /// server is a cell that sends a change packet twenty times a second and never settles.
    #[test]
    fn test_settled_water_stops_changing() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 200);

        while events.pop_event().is_some() {}

        run(&mut liquids, &blocks, &mut events, 20);

        let mut changes = 0;
        while let Some(event) = events.pop_event() {
            if event.downcast::<LiquidChangeEvent>().is_some() {
                changes += 1;
            }
        }

        assert_eq!(changes, 0, "settled water is still sloshing about");
    }

    #[test]
    fn test_water_does_not_enter_solid_blocks() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(1, HEIGHT - 2, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 40);

        assert_eq!(liquids.get_liquid_level(0, HEIGHT - 2).unwrap(), 0, "water flowed into the wall beside it");
        assert_eq!(liquids.get_liquid_level(1, HEIGHT - 1).unwrap(), 0, "water flowed into the floor below it");
    }

    /// A block placed on top of liquid displaces it. Nothing else clears a cell, so without
    /// this a filled-in pool stays full of invisible water.
    #[test]
    fn test_liquid_inside_a_block_is_removed() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        // the bottom row is solid, so this cell is inside a block
        liquids.set_liquid(3, HEIGHT - 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        liquids.schedule_update(3, HEIGHT - 1);
        run(&mut liquids, &blocks, &mut events, 2);

        assert_eq!(liquids.get_liquid_level(3, HEIGHT - 1).unwrap(), 0, "liquid stayed inside a solid block");
    }

    /// Each liquid type flows on its own clock, so a slow one is not dragged along by a
    /// fast one sharing the world.
    #[test]
    fn test_flow_time_paces_each_liquid() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(2, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        liquids.set_liquid(4, 1, tar, MAX_LIQUID_LEVEL, &mut events).unwrap();

        // one water step, a tenth of a tar step
        liquids.update_liquids(&blocks, &mut events, 100.0).unwrap();

        assert_eq!(liquids.get_liquid_level(2, 1).unwrap(), 0, "water should have moved on its first step");
        assert_eq!(liquids.get_liquid_level(4, 1).unwrap(), MAX_LIQUID_LEVEL, "tar flows every 1000ms and should not have moved yet");

        run(&mut liquids, &blocks, &mut events, 10);

        assert_eq!(liquids.get_liquid_level(4, 1).unwrap(), 0, "tar should have moved once a whole second had passed");
    }

    #[test]
    fn test_setting_a_level_of_zero_empties_the_cell() {
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, 3, water, 0, &mut events).unwrap();

        assert_eq!(
            liquids.get_liquid_id_at(3, 3).unwrap(),
            liquids.empty,
            "an empty cell should hold the empty liquid, whatever type was asked for"
        );
    }

    #[test]
    fn test_a_level_above_the_maximum_is_clamped() {
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, 3, water, 255, &mut events).unwrap();

        assert_eq!(liquids.get_liquid_level(3, 3).unwrap(), MAX_LIQUID_LEVEL);
    }

    #[test]
    fn test_setting_an_unknown_liquid_type_fails() {
        let (mut liquids, _water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        assert!(liquids.set_liquid(3, 3, LiquidId::undefined(), 50, &mut events).is_err());
        assert!(liquids.set_liquid(3, 3, LiquidId { id: 100 }, 50, &mut events).is_err());
    }

    #[test]
    fn test_setting_a_liquid_out_of_bounds_fails() {
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        assert!(liquids.set_liquid(-1, 3, water, 50, &mut events).is_err());
        assert!(liquids.set_liquid(3, HEIGHT, water, 50, &mut events).is_err());
    }

    #[test]
    fn test_setting_a_liquid_sends_one_event() {
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, 3, water, 50, &mut events).unwrap();
        // the same value again is not a change, so it says nothing
        liquids.set_liquid(3, 3, water, 50, &mut events).unwrap();

        let mut changes = Vec::new();
        while let Some(event) = events.pop_event() {
            if let Some(event) = event.downcast::<LiquidChangeEvent>() {
                changes.push((event.x, event.y));
            }
        }

        assert_eq!(changes, vec![(3, 3)]);
    }

    #[test]
    fn test_liquid_type_lookup_by_name() {
        let (liquids, water, _tar) = liquids_with_water();

        assert_eq!(liquids.get_liquid_id_by_name("water").unwrap(), water);
        liquids.get_liquid_id_by_name("custard").unwrap_err();
        assert_eq!(liquids.get_liquid_type(water).unwrap().name, "water");
    }

    #[test]
    fn test_serialize_round_trip() {
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(2, 2, water, 60, &mut events).unwrap();
        liquids.set_liquid(3, 2, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        let serialized = liquids.serialize().unwrap();

        let (mut loaded, _water, _tar) = liquids_with_water();
        loaded.deserialize(&serialized).unwrap();

        assert_eq!(loaded.get_size(), (WIDTH as u32, HEIGHT as u32));
        assert_eq!(loaded.get_liquid_level(2, 2).unwrap(), 60);
        assert_eq!(loaded.get_liquid_id_at(3, 2).unwrap(), water);
        assert_eq!(total_liquid(&loaded), total_liquid(&liquids));
    }

    /// A world that was saved mid-splash carries on flowing when it is loaded, because the
    /// scheduled set is rebuilt from the grid rather than saved with it.
    #[test]
    fn test_a_loaded_world_carries_on_flowing() {
        let blocks = world_with_a_shaft();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(SHAFT_X, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        let serialized = liquids.serialize().unwrap();

        let (mut loaded, _water, _tar) = liquids_with_water();
        loaded.deserialize(&serialized).unwrap();

        // nothing is scheduled yet, so the world is frozen where it was saved
        run(&mut loaded, &blocks, &mut events, 5);
        assert_eq!(loaded.get_liquid_level(SHAFT_X, 1).unwrap(), MAX_LIQUID_LEVEL, "a loaded world should not flow before it is scheduled");

        loaded.schedule_all_unsettled(&blocks).unwrap();
        run(&mut loaded, &blocks, &mut events, 20);

        assert_eq!(
            loaded.get_liquid_level(SHAFT_X, HEIGHT - 2).unwrap(),
            MAX_LIQUID_LEVEL,
            "water should have fallen once the loaded world was scheduled"
        );
    }

    /// The scan on load is what keeps the scheduled set from being millions of entries on a
    /// world full of settled ocean.
    #[test]
    fn test_schedule_all_unsettled_leaves_settled_liquid_alone() {
        let blocks = world_with_a_basin();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(3, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 200);

        liquids.schedule_all_unsettled(&blocks).unwrap();
        while events.pop_event().is_some() {}
        run(&mut liquids, &blocks, &mut events, 5);

        let mut changes = 0;
        while let Some(event) = events.pop_event() {
            if event.downcast::<LiquidChangeEvent>().is_some() {
                changes += 1;
            }
        }

        assert_eq!(changes, 0, "rescheduling a settled world made it move");
    }

    /// Digging out the block under a pool has to wake it up: a cell that has settled is no
    /// longer scheduled, so nothing else would ever look at it again.
    #[test]
    fn test_scheduling_wakes_settled_liquid() {
        let mut blocks = world_with_a_shaft();
        let (mut liquids, water, _tar) = liquids_with_water();
        let mut events = EventManager::new();

        liquids.set_liquid(SHAFT_X, 1, water, MAX_LIQUID_LEVEL, &mut events).unwrap();
        run(&mut liquids, &blocks, &mut events, 200);
        assert_eq!(liquids.get_liquid_level(SHAFT_X, HEIGHT - 2).unwrap(), MAX_LIQUID_LEVEL);

        // dig the floor out from under it
        let air = blocks.air();
        blocks.set_block(&mut events, SHAFT_X, HEIGHT - 1, air).unwrap();
        liquids.schedule_update(SHAFT_X, HEIGHT - 1);
        run(&mut liquids, &blocks, &mut events, 10);

        assert_eq!(
            liquids.get_liquid_level(SHAFT_X, HEIGHT - 1).unwrap(),
            MAX_LIQUID_LEVEL,
            "water should have fallen into the hole that was dug under it"
        );
    }
}
