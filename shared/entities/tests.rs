#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::shared::blocks::{Block, BlockId, Blocks};
    use crate::shared::entities::{collides_with_blocks, is_touching_ground, reduce_by, Entities, EntityDespawnEvent, HealthChangeEvent, HealthComponent, PhysicsComponent, PositionComponent};
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
        let mut value = 10.0;
        reduce_by(&mut value, 3.0);
        assert!((value - 7.0).abs() < f32::EPSILON);

        let mut value = -10.0;
        reduce_by(&mut value, 3.0);
        assert!((value + 7.0).abs() < f32::EPSILON);
    }

    /// It never overshoots past zero, in either direction.
    #[test]
    fn test_reduce_by_clamps_at_zero() {
        let mut value = 2.0;
        reduce_by(&mut value, 5.0);
        assert!(value.abs() < f32::EPSILON);

        let mut value = -2.0;
        reduce_by(&mut value, 5.0);
        assert!(value.abs() < f32::EPSILON);
    }

    /// The amount is taken as a magnitude, so a negative argument still reduces.
    #[test]
    fn test_reduce_by_uses_the_absolute_amount() {
        let mut value = 10.0;
        reduce_by(&mut value, -3.0);
        assert!((value - 7.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reduce_by_zero_is_a_noop() {
        let mut value = 4.5;
        reduce_by(&mut value, 0.0);
        assert!((value - 4.5).abs() < f32::EPSILON);
    }

    // --- collision ---

    #[test]
    fn test_collides_with_blocks() {
        let blocks = world_with_ground(5);
        let physics = PhysicsComponent::new(1.0, 1.0);

        // well above the ground
        assert!(!collides_with_blocks(&PositionComponent::new(2.0, 1.0), &physics, &blocks));
        // inside the ground
        assert!(collides_with_blocks(&PositionComponent::new(2.0, 6.0), &physics, &blocks));
    }

    /// Out of bounds coordinates are not solid, so an entity outside the world does not
    /// collide with anything.
    #[test]
    fn test_collides_outside_the_world_is_false() {
        let blocks = world_with_ground(5);
        let physics = PhysicsComponent::new(1.0, 1.0);

        assert!(!collides_with_blocks(&PositionComponent::new(-50.0, -50.0), &physics, &blocks));
    }

    /// `is_touching_ground` probes 0.02 blocks below the entity, so it reports true only
    /// once the entity actually overlaps the tile beneath it - which is where the physics
    /// step leaves it after landing. Rather than guess a coordinate, drop one and ask.
    #[test]
    fn test_is_touching_ground_after_landing() {
        let blocks = world_with_ground(5);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity = entities.ecs.spawn((PositionComponent::new(2.0, 0.0), PhysicsComponent::new(1.0, 1.0)));
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
        let physics = PhysicsComponent::new(1.0, 1.0);

        assert!(!is_touching_ground(&PositionComponent::new(2.0, 1.0), &physics, &blocks));
    }

    /// Falling fast is not "touching ground" even when overlapping, so landing logic does
    /// not trigger mid fall.
    #[test]
    fn test_is_touching_ground_requires_low_vertical_speed() {
        let blocks = world_with_ground(5);
        let mut physics = PhysicsComponent::new(1.0, 1.0);
        physics.velocity_y = 20.0;

        assert!(!is_touching_ground(&PositionComponent::new(2.0, 4.0), &physics, &blocks));
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
        let entity = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));
        let id = entities.new_id();

        entities.assign_id(entity, id).unwrap();

        assert_eq!(entities.get_entity_from_id(id).unwrap(), entity);
        assert_eq!(entities.get_id_from_entity(entity).unwrap(), id);
    }

    #[test]
    fn test_assigning_the_same_id_twice_fails() {
        let mut entities = Entities::new();
        let a = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));
        let b = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));
        let id = entities.new_id();

        entities.assign_id(a, id).unwrap();
        assert!(entities.assign_id(b, id).is_err(), "an id should not map to two entities");
    }

    #[test]
    fn test_assigning_two_ids_to_one_entity_fails() {
        let mut entities = Entities::new();
        let entity = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));

        let first = entities.new_id();
        let second = entities.new_id();
        entities.assign_id(entity, first).unwrap();

        assert!(entities.assign_id(entity, second).is_err(), "an entity should not have two ids");
    }

    #[test]
    fn test_unknown_id_and_entity_are_errors() {
        let mut entities = Entities::new();
        let id = entities.new_id();
        let entity = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));

        entities.get_entity_from_id(id).unwrap_err();
        entities.get_id_from_entity(entity).unwrap_err();
    }

    #[test]
    fn test_despawn_removes_the_entity_and_fires_an_event() {
        let mut entities = Entities::new();
        let mut events = EventManager::new();
        let entity = entities.ecs.spawn((PositionComponent::new(0.0, 0.0),));
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

        let entity = entities.ecs.spawn((PositionComponent::new(2.0, 0.0), PhysicsComponent::new(1.0, 1.0)));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        for _ in 0..20 {
            entities.update_entities_ms(&blocks, &dry_world(), &mut events).unwrap();
        }

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!(position.y() > 0.0, "entity should have fallen, y is {}", position.y());
    }

    /// It lands rather than falling through the floor.
    #[test]
    fn test_falling_entity_lands_on_the_ground() {
        let blocks = world_with_ground(5);
        let mut entities = Entities::new();
        let mut events = EventManager::new();

        let entity = entities.ecs.spawn((PositionComponent::new(2.0, 0.0), PhysicsComponent::new(1.0, 1.0)));
        let id = entities.new_id();
        entities.assign_id(entity, id).unwrap();

        for _ in 0..400 {
            entities.update_entities_ms(&blocks, &dry_world(), &mut events).unwrap();
        }

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!(position.y() <= 4.0 + 0.1, "entity fell through the floor, y is {}", position.y());
    }

    // --- components ---

    #[test]
    fn test_position_component_accessors() {
        let mut position = PositionComponent::new(1.0, 2.0);
        assert!((position.x() - 1.0).abs() < f32::EPSILON);
        assert!((position.y() - 2.0).abs() < f32::EPSILON);

        position.set_x(5.0);
        position.set_y(6.0);
        assert!((position.x() - 5.0).abs() < f32::EPSILON);
        assert!((position.y() - 6.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_physics_component_defaults_to_falling() {
        let physics = PhysicsComponent::new(1.0, 2.0);
        assert!(physics.velocity_x.abs() < f32::EPSILON);
        assert!(physics.velocity_y.abs() < f32::EPSILON);
        assert!(physics.acceleration_y > 0.0, "gravity should pull downwards by default");
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
}
