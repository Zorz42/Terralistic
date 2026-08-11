#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::shared::blocks::{Blocks, BREAK_STAGES};
    use crate::shared::walls::{Wall, Walls};

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
}
