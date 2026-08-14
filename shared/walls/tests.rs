#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::shared::blocks::{Blocks, BREAK_STAGES};
    use crate::shared::walls::{Wall, WallId, Walls};

    /// Builds a 5x5 world of walls with one breakable and one unbreakable type.
    /// Returns the walls plus the ids, with the whole map set to the breakable type.
    fn walls_with_types() -> (Walls, crate::shared::walls::WallId, crate::shared::walls::WallId) {
        let mut blocks = Blocks::new();
        let mut walls = Walls::new(&mut blocks);

        let mut breakable = Wall::new();
        breakable.name = "breakable".to_owned();
        breakable.break_time = Some(1000);
        let breakable_id = Walls::register_new_wall_type(&mut walls.wall_types, breakable);

        let mut unbreakable = Wall::new();
        unbreakable.name = "unbreakable".to_owned();
        unbreakable.break_time = None;
        let unbreakable_id = Walls::register_new_wall_type(&mut walls.wall_types, unbreakable);

        walls.create((5, 5));

        (walls, breakable_id, unbreakable_id)
    }

    /// An unbreakable wall must never break, however long it is hit for.
    ///
    /// This passes on the old code too: `start_breaking_wall` is the only way into
    /// `breaking_walls` and it already refuses unbreakable walls, so the `unwrap_or(1)`
    /// in `update_breaking_walls` was unreachable. Blocks are the ones with a second,
    /// unguarded entry point. Kept as a guard against that changing.
    #[test]
    fn test_unbreakable_wall_never_breaks() {
        let (mut walls, _breakable, unbreakable) = walls_with_types();
        let mut events = EventManager::new();

        walls.set_wall_type(1, 1, unbreakable).unwrap();
        walls.start_breaking_wall(1, 1).unwrap();

        for _ in 0..100 {
            walls.update_breaking_walls(1000.0, &mut events).unwrap();
        }

        assert!(walls.get_wall_type_at(1, 1).unwrap().get_id() == unbreakable, "an unbreakable wall was broken");
    }

    #[test]
    fn test_breakable_wall_breaks() {
        let (mut walls, breakable, _unbreakable) = walls_with_types();
        let mut events = EventManager::new();

        walls.set_wall_type(1, 1, breakable).unwrap();
        walls.start_breaking_wall(1, 1).unwrap();

        for _ in 0..10 {
            walls.update_breaking_walls(1000.0, &mut events).unwrap();
        }

        assert!(walls.get_wall_type_at(1, 1).unwrap().get_id() != breakable, "a breakable wall was not broken");
    }

    /// The break stage indexes the breaking texture, so it has to stay in range even
    /// when progress has run past `break_time`.
    #[test]
    fn test_break_stage_stays_in_texture_range() {
        let (mut walls, breakable, unbreakable) = walls_with_types();

        walls.set_wall_type(0, 0, unbreakable).unwrap();
        assert_eq!(walls.get_break_stage(0, 0).unwrap(), 0, "an unbreakable wall should show no breaking");

        walls.set_wall_type(1, 1, breakable).unwrap();
        walls.start_breaking_wall(1, 1).unwrap();

        let mut events = EventManager::new();

        // Advance exactly to break_time. The wall is not removed until progress goes
        // strictly past it, so this is a real state the renderer can observe - and it is
        // where the old `progress * 9 / break_time` produced 9, two rows past the end of
        // the 8 frame breaking texture.
        walls.update_breaking_walls(1000.0, &mut events).unwrap();
        assert_eq!(walls.get_break_progress(1, 1).unwrap(), 1000, "test setup: expected progress to sit exactly on break_time");

        let stage = walls.get_break_stage(1, 1).unwrap();
        assert!((0..BREAK_STAGES).contains(&stage), "break stage {stage} is outside 0..{BREAK_STAGES}");
    }

    #[test]
    fn test_start_breaking_ignores_unbreakable() {
        let (mut walls, _breakable, unbreakable) = walls_with_types();

        walls.set_wall_type(2, 2, unbreakable).unwrap();
        walls.start_breaking_wall(2, 2).unwrap();

        assert_eq!(walls.get_break_progress(2, 2).unwrap(), 0);
    }

    // --- wall storage ---

    #[test]
    fn test_new_walls_have_only_the_clear_type() {
        let mut blocks = Blocks::new();
        let walls = Walls::new(&mut blocks);

        assert_eq!(walls.get_size(), (0, 0));
        assert_eq!(walls.get_all_wall_ids().len(), 1, "Walls::new registers exactly one type, clear");
        assert!(walls.get_wall_id_by_name("clear").unwrap() == walls.clear);
    }

    /// `Walls::create` fills the map with `WallId::undefined()`, unlike `Blocks::create`
    /// which fills with air. So reading a wall before setting one is an error, not a
    /// "clear" wall.
    ///
    /// This is not reachable in the running game: the only caller is
    /// `create_from_wall_ids`, which overwrites every cell straight afterwards, and the
    /// world generator always goes through that. It is pinned here because the asymmetry
    /// with `Blocks::create` is a trap for anything that calls `create` directly.
    #[test]
    fn test_create_leaves_walls_undefined() {
        let (mut walls, _breakable, _unbreakable) = walls_with_types();

        walls.create((8, 6));
        assert_eq!(walls.get_size(), (8, 6));
        assert!(walls.get_wall_type_at(0, 0).is_err(), "a freshly created wall map has no usable wall type yet");
    }

    #[test]
    fn test_set_and_get_wall_type() {
        let (mut walls, breakable, _unbreakable) = walls_with_types();

        walls.set_wall_type(2, 2, walls.clear).unwrap();
        walls.set_wall_type(2, 3, breakable).unwrap();

        assert!(walls.get_wall_type_at(2, 3).unwrap().get_id() == breakable);
        // neighbours are untouched
        assert!(walls.get_wall_type_at(2, 2).unwrap().get_id() == walls.clear);
    }

    #[test]
    fn test_wall_out_of_bounds() {
        let (mut walls, breakable, _unbreakable) = walls_with_types();

        assert!(walls.get_wall_type_at(5, 0).is_err());
        assert!(walls.get_wall_type_at(0, 5).is_err());
        assert!(walls.get_wall_type_at(-1, 0).is_err());
        walls.set_wall_type(9, 9, breakable).unwrap_err();
    }

    #[test]
    fn test_unknown_wall_id_is_an_error() {
        let mut blocks = Blocks::new();
        let walls = Walls::new(&mut blocks);
        assert!(walls.get_wall_type(WallId::undefined()).is_err());
        assert!(walls.get_wall_id_by_name("not_a_wall").is_err());
    }

    /// This round trip is what the world save relies on.
    ///
    /// Note what is and is not saved: the grid of ids travels, the registry of wall types
    /// does not. On load the server registers types from mods first and only then
    /// deserializes, so the ids line up. Registering the same types in the same order is
    /// what makes this test mirror that.
    #[test]
    fn test_walls_serialize_round_trip() {
        let (mut walls, breakable, unbreakable) = walls_with_types();
        // fill first, since create leaves the map undefined
        for x in 0..5 {
            for y in 0..5 {
                walls.set_wall_type(x, y, walls.clear).unwrap();
            }
        }
        walls.set_wall_type(1, 1, breakable).unwrap();
        walls.set_wall_type(4, 4, unbreakable).unwrap();

        let bytes = walls.serialize().unwrap();

        let (mut restored, _b, _u) = walls_with_types();
        restored.deserialize(&bytes).unwrap();

        assert_eq!(restored.get_size(), (5, 5));
        assert!(restored.get_wall_type_at(1, 1).unwrap().get_id() == breakable);
        assert!(restored.get_wall_type_at(4, 4).unwrap().get_id() == unbreakable);
        assert!(restored.get_wall_type_at(0, 0).unwrap().get_id() == walls.clear);
    }

    #[test]
    fn test_deserialize_rejects_garbage() {
        let mut blocks = Blocks::new();
        let mut walls = Walls::new(&mut blocks);
        walls.deserialize(&[1, 2, 3, 4]).unwrap_err();
    }

    #[test]
    fn test_create_from_wall_ids() {
        let (mut walls, breakable, _unbreakable) = walls_with_types();

        let grid = vec![vec![breakable, walls.clear], vec![walls.clear, breakable]];
        walls.create_from_wall_ids(&grid).unwrap();

        assert_eq!(walls.get_size(), (2, 2));
        assert!(walls.get_wall_type_at(0, 0).unwrap().get_id() == breakable);
        assert!(walls.get_wall_type_at(1, 1).unwrap().get_id() == breakable);
    }

    #[test]
    fn test_create_from_ragged_wall_ids_fails() {
        let (mut walls, breakable, _unbreakable) = walls_with_types();

        let ragged = vec![vec![breakable, walls.clear], vec![walls.clear]];
        assert!(walls.create_from_wall_ids(&ragged).is_err(), "rows of different lengths should be rejected");
    }

    #[test]
    fn test_create_from_empty_wall_ids_fails() {
        let (mut walls, _breakable, _unbreakable) = walls_with_types();
        walls.create_from_wall_ids(&[]).unwrap_err();
    }
}
