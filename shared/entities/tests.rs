#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::libraries::fixed::Fixed;
    use crate::shared::blocks::{Block, BlockId, Blocks};
    use crate::shared::entities::{
        collides_with_blocks, is_touching_ground, reduce_by, state_hash, step_entity, Entities, EntityDespawnEvent, HealthChangeEvent, HealthComponent, PhysicsComponent, PositionComponent,
    };
    use crate::shared::liquids::Liquids;

    /// A liquid grid the size of `world_with_ground`'s world, with nothing in it. The
    /// physics step needs one, and an entity in air is what these tests are about.
    fn dry_world() -> Liquids {
        let mut liquids = Liquids::new();
        liquids.create((10, 10));
        liquids
    }

    /// A 10x10 world that is solid from `ground_y` down, using a non-ghost block.
    fn world_with_ground(ground_y: i32) -> Blocks {
        let mut blocks = Blocks::new();

        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);

        blocks.create((10, 10));

        let mut events = EventManager::new();
        for x in 0..10 {
            for y in ground_y..10 {
                blocks.set_block(&mut events, x, y, solid_id).unwrap();
            }
        }
        blocks
    }

    // --- reduce_by ---

    #[test]
    fn test_reduce_by_moves_towards_zero() {
        let mut value = Fixed::from_int(10);
        reduce_by(&mut value, Fixed::from_int(3));
        assert_eq!(value, Fixed::from_int(7));

        let mut value = Fixed::from_int(-10);
        reduce_by(&mut value, Fixed::from_int(3));
        assert_eq!(value, Fixed::from_int(-7));
    }

    /// It never overshoots past zero, in either direction.
    #[test]
    fn test_reduce_by_clamps_at_zero() {
        let mut value = Fixed::from_int(2);
        reduce_by(&mut value, Fixed::from_int(5));
        assert_eq!(value, Fixed::ZERO);

        let mut value = Fixed::from_int(-2);
        reduce_by(&mut value, Fixed::from_int(5));
        assert_eq!(value, Fixed::ZERO);
    }

    /// The amount is taken as a magnitude, so a negative argument still reduces.
    #[test]
    fn test_reduce_by_uses_the_absolute_amount() {
        let mut value = Fixed::from_int(10);
        reduce_by(&mut value, Fixed::from_int(-3));
        assert_eq!(value, Fixed::from_int(7));
    }

    #[test]
    fn test_reduce_by_zero_is_a_noop() {
        let mut value = Fixed::from_num(9, 2);
        reduce_by(&mut value, Fixed::ZERO);
        assert_eq!(value, Fixed::from_num(9, 2));
    }

    // --- collision ---

    #[test]
    fn test_collides_with_blocks() {
        let blocks = world_with_ground(5);
        let physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));

        // well above the ground
        assert!(!collides_with_blocks(&PositionComponent::new(Fixed::from_int(2), Fixed::from_int(1)), &physics, &blocks));
        // inside the ground
        assert!(collides_with_blocks(&PositionComponent::new(Fixed::from_int(2), Fixed::from_int(6)), &physics, &blocks));
    }

    /// Out of bounds coordinates are not solid, so an entity outside the world does not
    /// collide with anything.
    #[test]
    fn test_collides_outside_the_world_is_false() {
        let blocks = world_with_ground(5);
        let physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));

        assert!(!collides_with_blocks(&PositionComponent::new(-Fixed::from_int(50), -Fixed::from_int(50)), &physics, &blocks));
    }

    /// `is_touching_ground` probes 0.02 blocks below the entity, so it reports true only
    /// once the entity actually overlaps the tile beneath it - which is where the physics
    /// step leaves it after landing. Rather than guess a coordinate, drop one and ask.
    #[test]
    fn test_is_touching_ground_after_landing() {
        let blocks = world_with_ground(5);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity = entities.ecs.spawn((
            PositionComponent::new(Fixed::from_int(2), Fixed::from_int(0)),
            PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1)),
        ));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        for _ in 0..400 {
            entities.update_entities_ms(&blocks, &dry_world(), &mut events).unwrap();
        }

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap().clone();
        let physics = entities.ecs.get::<&PhysicsComponent>(entity).unwrap().clone();

        assert!(is_touching_ground(&position, &physics, &blocks), "a landed entity should be touching ground at y {}", position.y());
    }

    #[test]
    fn test_is_not_touching_ground_in_mid_air() {
        let blocks = world_with_ground(5);
        let physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));

        assert!(!is_touching_ground(&PositionComponent::new(Fixed::from_int(2), Fixed::from_int(1)), &physics, &blocks));
    }

    /// Falling fast is not "touching ground" even when overlapping, so landing logic does
    /// not trigger mid fall.
    #[test]
    fn test_is_touching_ground_requires_low_vertical_speed() {
        let blocks = world_with_ground(5);
        let mut physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));
        physics.velocity_y = Fixed::from_int(20);

        assert!(!is_touching_ground(&PositionComponent::new(Fixed::from_int(2), Fixed::from_int(4)), &physics, &blocks));
    }

    // --- entity id mapping ---

    #[test]
    fn test_new_id_is_unique() {
        let mut entities = Entities::new();
        let a = entities.new_id();
        let b = entities.new_id();
        assert_ne!(a, b);
    }

    #[test]
    fn test_assign_and_look_up_id() {
        let mut entities = Entities::new();
        let entity = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));
        let id = entities.new_id();

        entities.assign_id(entity, id).unwrap();

        assert_eq!(entities.get_entity_from_id(id).unwrap(), entity);
        assert_eq!(entities.get_id_from_entity(entity).unwrap(), id);
    }

    #[test]
    fn test_assigning_the_same_id_twice_fails() {
        let mut entities = Entities::new();
        let a = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));
        let b = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));
        let id = entities.new_id();

        entities.assign_id(a, id).unwrap();
        assert!(entities.assign_id(b, id).is_err(), "an id should not map to two entities");
    }

    #[test]
    fn test_assigning_two_ids_to_one_entity_fails() {
        let mut entities = Entities::new();
        let entity = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));

        let first = entities.new_id();
        let second = entities.new_id();
        entities.assign_id(entity, first).unwrap();

        assert!(entities.assign_id(entity, second).is_err(), "an entity should not have two ids");
    }

    #[test]
    fn test_unknown_id_and_entity_are_errors() {
        let mut entities = Entities::new();
        let id = entities.new_id();
        let entity = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));

        entities.get_entity_from_id(id).unwrap_err();
        entities.get_id_from_entity(entity).unwrap_err();
    }

    #[test]
    fn test_despawn_removes_the_entity_and_fires_an_event() {
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let entity = entities.ecs.spawn((PositionComponent::new(Fixed::from_int(0), Fixed::from_int(0)),));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        entities.despawn_entity(id, &mut events).unwrap();

        let event = events.pop_event().unwrap();
        let event = event.downcast::<EntityDespawnEvent>().unwrap();
        assert_eq!(event.id, id);
    }

    #[test]
    fn test_despawning_an_unknown_id_is_an_error() {
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let id = entities.new_id();

        entities.despawn_entity(id, &mut events).unwrap_err();
    }

    // --- physics stepping ---

    /// With gravity and nothing under it, an entity accelerates downwards.
    #[test]
    fn test_entity_falls_under_gravity() {
        let blocks = world_with_ground(9);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity = entities.ecs.spawn((
            PositionComponent::new(Fixed::from_int(2), Fixed::from_int(0)),
            PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1)),
        ));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        for _ in 0..20 {
            entities.update_entities_ms(&blocks, &dry_world(), &mut events).unwrap();
        }

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!(position.y() > Fixed::from_int(0), "entity should have fallen, y is {}", position.y());
    }

    /// It lands rather than falling through the floor.
    #[test]
    fn test_falling_entity_lands_on_the_ground() {
        let blocks = world_with_ground(5);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity = entities.ecs.spawn((
            PositionComponent::new(Fixed::from_int(2), Fixed::from_int(0)),
            PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1)),
        ));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        for _ in 0..400 {
            entities.update_entities_ms(&blocks, &dry_world(), &mut events).unwrap();
        }

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!(position.y() <= Fixed::from_int(4) + Fixed::from_num(1, 10), "entity fell through the floor, y is {}", position.y());
    }

    // --- components ---

    #[test]
    fn test_position_component_accessors() {
        let mut position = PositionComponent::new(Fixed::from_int(1), Fixed::from_int(2));
        assert_eq!(position.x(), Fixed::ONE);
        assert_eq!(position.y(), Fixed::from_int(2));

        position.set_x(Fixed::from_int(5));
        position.set_y(Fixed::from_int(6));
        assert_eq!(position.x(), Fixed::from_int(5));
        assert_eq!(position.y(), Fixed::from_int(6));
    }

    #[test]
    fn test_physics_component_defaults_to_falling() {
        let physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(2));
        assert_eq!(physics.velocity_x, Fixed::ZERO);
        assert_eq!(physics.velocity_y, Fixed::ZERO);
        assert!(physics.acceleration_y > Fixed::from_int(0), "gravity should pull downwards by default");
    }

    #[test]
    fn test_health_component_clamps() {
        let mut events = EventManager::new();
        let mut entities = Entities::new();
        let id = entities.new_id();
        let mut health = HealthComponent::new(50, 100);

        assert_eq!(health.health(), 50);
        assert_eq!(health.max_health(), 100);

        health.set_health(500, &mut events, id);
        assert_eq!(health.health(), 100, "health should be capped at max");

        health.set_health(-20, &mut events, id);
        assert_eq!(health.health(), 0, "health should not go below zero");
    }

    #[test]
    fn test_health_change_fires_an_event_only_on_change() {
        let mut events = EventManager::new();
        let mut entities = Entities::new();
        let id = entities.new_id();
        let mut health = HealthComponent::new(50, 100);

        health.set_health(50, &mut events, id);
        assert!(events.pop_event().is_none(), "setting the same health should not fire an event");

        health.set_health(40, &mut events, id);
        let event = events.pop_event().unwrap();
        assert_eq!(event.downcast::<HealthChangeEvent>().unwrap().entity, id);
    }

    #[test]
    fn test_increase_health_is_relative() {
        let mut events = EventManager::new();
        let mut entities = Entities::new();
        let id = entities.new_id();
        let mut health = HealthComponent::new(50, 100);

        health.increase_health(20, &mut events, id);
        assert_eq!(health.health(), 70);

        health.increase_health(-30, &mut events, id);
        assert_eq!(health.health(), 40);
    }

    #[test]
    fn test_block_id_undefined_is_distinct() {
        let blocks = Blocks::new();
        assert!(BlockId::undefined() != blocks.air());
    }

    // --- determinism ---

    /// The property everything else is built on: the same state stepped the same number of
    /// times lands on the *same* answer, not a nearby one. This is what lets the client store
    /// a tick, replay it later and compare the result to the server's with `==` - no epsilon,
    /// and so no threshold that is either too tight or too loose.
    #[test]
    fn test_stepping_is_bit_reproducible() {
        let blocks = world_with_ground(9);
        let liquids = dry_world();

        let run = || {
            let mut position = PositionComponent::new(Fixed::from_num(5, 2), Fixed::ZERO);
            let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);
            physics.velocity_x = Fixed::from_num(37, 10);
            for _ in 0..500 {
                step_entity(&mut position, &mut physics, &blocks, &liquids);
            }
            (position, physics)
        };

        assert_eq!(run(), run(), "two runs from one state must agree exactly");
    }

    /// Replaying from a stored state reaches the same place as never having stopped. The
    /// rollback path does exactly this: restore a tick, then step forward again.
    #[test]
    fn test_replaying_from_a_stored_state_catches_up_exactly() {
        let blocks = world_with_ground(9);
        let liquids = dry_world();

        let mut position = PositionComponent::new(Fixed::from_int(2), Fixed::ZERO);
        let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);
        physics.velocity_x = Fixed::from_int(4);

        // run 30 ticks, remembering the state at tick 10 on the way past
        let mut checkpoint = None;
        for tick in 0..30 {
            if tick == 10 {
                checkpoint = Some((position, physics));
            }
            step_entity(&mut position, &mut physics, &blocks, &liquids);
        }

        let (mut replay_position, mut replay_physics) = checkpoint.unwrap();
        for _ in 10..30 {
            step_entity(&mut replay_position, &mut replay_physics, &blocks, &liquids);
        }

        assert_eq!((replay_position, replay_physics), (position, physics), "a replay must land where the original did");
    }

    /// Moving left is the mirror of moving right. With floats this held only approximately,
    /// and a floored fixed-point multiply would break it outright - an entity would shed
    /// speed faster in one direction than the other.
    #[test]
    fn test_movement_is_symmetric_between_left_and_right() {
        let blocks = world_with_ground(9);
        let liquids = dry_world();

        let run = |velocity: Fixed| {
            let mut position = PositionComponent::new(Fixed::from_int(5), Fixed::ZERO);
            let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);
            physics.velocity_x = velocity;
            for _ in 0..40 {
                step_entity(&mut position, &mut physics, &blocks, &liquids);
            }
            (position.x() - Fixed::from_int(5), physics.velocity_x)
        };

        let (right_travel, right_velocity) = run(Fixed::from_num(23, 10));
        let (left_travel, left_velocity) = run(-Fixed::from_num(23, 10));

        assert_eq!(right_travel, -left_travel, "distance travelled must mirror");
        assert_eq!(right_velocity, -left_velocity, "remaining velocity must mirror");
    }

    /// An entity coming to rest actually reaches zero. Held as floats, drag multiplies a
    /// velocity that never quite arrives - the same bug liquid levels avoid by being whole
    /// numbers, and on a server it is a change packet sent twenty times a second forever.
    #[test]
    fn test_a_sliding_entity_comes_completely_to_rest() {
        let blocks = world_with_ground(9);
        let liquids = dry_world();

        let mut position = PositionComponent::new(Fixed::from_int(2), Fixed::from_int(7));
        let mut physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);
        physics.velocity_x = Fixed::from_int(3);

        for _ in 0..20000 {
            step_entity(&mut position, &mut physics, &blocks, &liquids);
        }

        assert_eq!(physics.velocity_x, Fixed::ZERO, "drag must bring it to a stop, not near one");
    }

    /// State is hashable, which is what a desync check needs. Two equal states must hash the
    /// same - `gfx::FloatPos` cannot promise this, which is why it refuses to implement `Hash`.
    #[test]
    fn test_equal_states_hash_equally() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let hash = |position: &PositionComponent, physics: &PhysicsComponent| {
            let mut hasher = DefaultHasher::new();
            position.hash(&mut hasher);
            physics.hash(&mut hasher);
            hasher.finish()
        };

        let blocks = world_with_ground(9);
        let liquids = dry_world();
        let mut a = (PositionComponent::new(Fixed::ONE, Fixed::ZERO), PhysicsComponent::new(Fixed::ONE, Fixed::ONE));
        let mut b = a;
        for _ in 0..25 {
            step_entity(&mut a.0, &mut a.1, &blocks, &liquids);
            step_entity(&mut b.0, &mut b.1, &blocks, &liquids);
        }

        assert_eq!(hash(&a.0, &a.1), hash(&b.0, &b.1));
    }

    /// The checksum that crosses the network. Equal states must hash equally, or the desync
    /// check would cry wolf; different ones must not, or it would never notice.
    #[test]
    fn test_state_hash_distinguishes_states() {
        let position = PositionComponent::new(Fixed::from_int(3), Fixed::from_int(4));
        let physics = PhysicsComponent::new(Fixed::ONE, Fixed::ONE);

        assert_eq!(state_hash(&position, &physics), state_hash(&position, &physics), "the same state must hash the same");

        let moved = PositionComponent::new(Fixed::from_int(3) + Fixed::EPSILON, Fixed::from_int(4));
        assert_ne!(state_hash(&position, &physics), state_hash(&moved, &physics), "one step of position should change the hash");

        let mut faster = physics;
        faster.velocity_x += Fixed::EPSILON;
        assert_ne!(state_hash(&position, &physics), state_hash(&position, &faster), "one step of velocity should change the hash");
    }

    /// Two runs of the same simulation hash the same at every tick. This is the property the
    /// desync check relies on - and the one floats could not give, since two float
    /// simulations agreeing to within a rounding error still hash differently.
    #[test]
    fn test_two_identical_runs_hash_the_same_every_tick() {
        let blocks = world_with_ground(9);
        let liquids = dry_world();

        let mut a = (PositionComponent::new(Fixed::from_num(5, 2), Fixed::ZERO), PhysicsComponent::new(Fixed::ONE, Fixed::ONE));
        let mut b = a;
        a.1.velocity_x = Fixed::from_num(31, 10);
        b.1.velocity_x = Fixed::from_num(31, 10);

        for tick in 0..300 {
            step_entity(&mut a.0, &mut a.1, &blocks, &liquids);
            step_entity(&mut b.0, &mut b.1, &blocks, &liquids);
            assert_eq!(state_hash(&a.0, &a.1), state_hash(&b.0, &b.1), "the two runs parted company at tick {tick}");
        }
    }
}
