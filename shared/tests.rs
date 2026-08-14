#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::libraries::serialization;
    use crate::shared::blocks::{Block, BlockId, Blocks};
    use crate::shared::chat::ChatPacket;
    use crate::shared::entities::{Entities, PositionComponent};
    use crate::shared::lights::{LightColor, Lights};
    use crate::shared::players::{spawn_player, MovingType, PlayerComponent, PLAYER_HEIGHT, PLAYER_INVENTORY_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH};
    use crate::shared::versions::{VersionPacket, VERSION, WORLD_SAVE_HEADER_LEN, WORLD_SAVE_MAGIC, WORLD_SAVE_VERSION};

    // ---------------- lights ----------------

    /// A world of the given size where every block is transparent, or opaque.
    fn blocks_for_lights(size: (u32, u32), transparent: bool) -> Blocks {
        let mut blocks = Blocks::new();

        let mut block = Block::new();
        block.name = "test".to_owned();
        block.transparent = transparent;
        let id = blocks.register_new_block_type(block);

        blocks.create(size);

        let mut events = EventManager::new();
        for x in 0..size.0 as i32 {
            for y in 0..size.1 as i32 {
                blocks.set_block(&mut events, x, y, id).unwrap();
            }
        }
        blocks
    }

    #[test]
    fn test_new_lights_is_empty() {
        let lights = Lights::new();
        assert_eq!(lights.get_size(), (0, 0));
    }

    #[test]
    fn test_lights_create_sets_size() {
        let mut lights = Lights::new();
        lights.create((32, 48));
        assert_eq!(lights.get_size(), (32, 48));
    }

    #[test]
    fn test_light_out_of_bounds_is_an_error() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        assert!(lights.get_light(-1, 0).is_err());
        assert!(lights.get_light(0, -1).is_err());
        assert!(lights.get_light(32, 0).is_err());
        assert!(lights.get_light(0, 32).is_err());
    }

    #[test]
    fn test_new_lights_start_dark_and_scheduled() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        let light = lights.get_light(5, 5).unwrap();
        assert!(light.color == LightColor::new(0, 0, 0));
        assert!(!light.is_source);
        assert!(light.scheduled_light_update, "every light starts needing an update");
    }

    /// The chunk grid is the world size divided by the chunk size in each axis, so the
    /// last valid chunk is addressable and one past it is not.
    #[test]
    fn test_light_chunk_bounds() {
        let mut lights = Lights::new();
        lights.create((32, 48)); // 2 x 3 chunks at CHUNK_SIZE 16

        assert!(lights.get_light_chunk(0, 0).is_ok());
        assert!(lights.get_light_chunk(1, 2).is_ok());
        assert!(lights.get_light_chunk(2, 0).is_err());
        assert!(lights.get_light_chunk(0, 3).is_err());
    }

    #[test]
    fn test_set_light_source() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        lights.set_light_source(5, 5, LightColor::new(255, 128, 0)).unwrap();

        let light = lights.get_light(5, 5).unwrap();
        assert!(light.is_source);
        assert!(light.source_color == LightColor::new(255, 128, 0));
    }

    /// A source colour of pure black means "not a source".
    #[test]
    fn test_black_light_source_is_not_a_source() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        lights.set_light_source(5, 5, LightColor::new(255, 255, 255)).unwrap();
        assert!(lights.get_light(5, 5).unwrap().is_source);

        lights.set_light_source(5, 5, LightColor::new(0, 0, 0)).unwrap();
        assert!(!lights.get_light(5, 5).unwrap().is_source);
    }

    #[test]
    fn test_schedule_light_update_counts_per_chunk() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        // clear the initial scheduled flag for one light
        let blocks = blocks_for_lights((32, 32), true);
        lights.update_light(5, 5, &blocks).unwrap();
        assert!(!lights.get_light(5, 5).unwrap().scheduled_light_update);

        lights.schedule_light_update(5, 5).unwrap();
        assert!(lights.get_light(5, 5).unwrap().scheduled_light_update);
    }

    #[test]
    fn test_schedule_for_neighbours_ignores_out_of_bounds() {
        let mut lights = Lights::new();
        lights.create((32, 32));

        // a corner has neighbours outside the world; this must not panic or error
        lights.schedule_light_update_for_neighbours(0, 0);
        assert!(lights.get_light(0, 0).unwrap().scheduled_light_update);
    }

    /// With a fully transparent world, every column can see the sky, so the sky height is
    /// the bottom row.
    #[test]
    fn test_init_sky_heights_transparent_world() {
        let blocks = blocks_for_lights((16, 16), true);
        let mut lights = Lights::new();
        lights.create((16, 16));

        lights.init_sky_heights(&blocks).unwrap();
        lights.update_light_emitter(3, 0, &blocks).unwrap();

        // a tile at the top of a see-through column is lit by the sky
        assert!(lights.get_light(3, 0).unwrap().is_source);
    }

    /// With an opaque world nothing sees the sky, so no tile becomes a sky source.
    #[test]
    fn test_init_sky_heights_opaque_world() {
        let blocks = blocks_for_lights((16, 16), false);
        let mut lights = Lights::new();
        lights.create((16, 16));

        lights.init_sky_heights(&blocks).unwrap();
        lights.update_light_emitter(3, 0, &blocks).unwrap();

        assert!(!lights.get_light(3, 0).unwrap().is_source);
    }

    /// Light spreads from an emitting block to its neighbours, getting dimmer with
    /// distance.
    ///
    /// The source has to come from the block's `light_emission`, not from a direct
    /// `set_light_source` call: `update_light` calls `update_light_emitter` first, which
    /// re-derives the source from the block underneath and would overwrite anything set
    /// by hand.
    #[test]
    fn test_light_spreads_and_falls_off() {
        let mut blocks = Blocks::new();

        let mut air = Block::new();
        air.name = "clear".to_owned();
        air.transparent = true;
        let air_id = blocks.register_new_block_type(air);

        let mut torch = Block::new();
        torch.name = "torch".to_owned();
        torch.transparent = true;
        torch.light_emission = (255, 255, 255);
        let torch_id = blocks.register_new_block_type(torch);

        blocks.create((16, 16));
        let mut events = EventManager::new();
        for x in 0..16 {
            for y in 0..16 {
                blocks.set_block(&mut events, x, y, air_id).unwrap();
            }
        }
        blocks.set_block(&mut events, 8, 8, torch_id).unwrap();

        let mut lights = Lights::new();
        lights.create((16, 16));
        for _ in 0..12 {
            for x in 0..16 {
                for y in 0..16 {
                    lights.update_light(x, y, &blocks).unwrap();
                }
            }
        }

        let at_source = lights.get_light(8, 8).unwrap().color.r;
        let one_away = lights.get_light(9, 8).unwrap().color.r;
        let three_away = lights.get_light(11, 8).unwrap().color.r;

        assert!(at_source > 0, "the emitting block should be lit");
        assert!(one_away > 0, "light should reach a neighbour");
        assert!(one_away < at_source, "light should dim with distance, got {one_away} next to {at_source}");
        assert!(three_away < one_away, "light should keep dimming, got {three_away} at 3 away vs {one_away} at 1");
    }

    /// A tile far from any source stays dark.
    #[test]
    fn test_light_does_not_reach_the_far_corner() {
        let mut blocks = Blocks::new();

        let mut air = Block::new();
        air.name = "clear".to_owned();
        air.transparent = true;
        let air_id = blocks.register_new_block_type(air);

        let mut torch = Block::new();
        torch.name = "torch".to_owned();
        torch.transparent = true;
        torch.light_emission = (255, 255, 255);
        let torch_id = blocks.register_new_block_type(torch);

        blocks.create((32, 32));
        let mut events = EventManager::new();
        for x in 0..32 {
            for y in 0..32 {
                blocks.set_block(&mut events, x, y, air_id).unwrap();
            }
        }
        blocks.set_block(&mut events, 0, 0, torch_id).unwrap();

        let mut lights = Lights::new();
        lights.create((32, 32));
        for _ in 0..8 {
            for x in 0..32 {
                for y in 0..32 {
                    lights.update_light(x, y, &blocks).unwrap();
                }
            }
        }

        assert_eq!(lights.get_light(31, 31).unwrap().color.r, 0, "light should have fallen off to nothing across the world");
    }

    // ---------------- players ----------------

    #[test]
    fn test_player_component_defaults() {
        let player = PlayerComponent::new("jakob");
        assert_eq!(player.get_name(), "jakob");
        assert!(player.get_moving_type() == MovingType::Standing);
        assert!(!player.jumping);
    }

    /// Changing moving type applies the new state's acceleration and removes the old one,
    /// so going left then right does not leave the player accelerating both ways.
    #[test]
    fn test_moving_type_transitions_are_reversible() {
        use crate::shared::entities::PhysicsComponent;

        let mut player = PlayerComponent::new("p");
        let mut physics = PhysicsComponent::new(1.0, 1.0);
        let neutral = physics.acceleration_x;

        player.set_moving_type(MovingType::MovingRight, &mut physics);
        assert!(physics.acceleration_x > neutral, "moving right should accelerate right");

        player.set_moving_type(MovingType::MovingLeft, &mut physics);
        assert!(physics.acceleration_x < neutral, "moving left should accelerate left");

        player.set_moving_type(MovingType::Standing, &mut physics);
        assert!((physics.acceleration_x - neutral).abs() < f32::EPSILON, "standing should restore the neutral acceleration");
    }

    #[test]
    fn test_setting_the_same_moving_type_twice_does_not_stack() {
        use crate::shared::entities::PhysicsComponent;

        let mut player = PlayerComponent::new("p");
        let mut physics = PhysicsComponent::new(1.0, 1.0);

        player.set_moving_type(MovingType::MovingRight, &mut physics);
        let after_one = physics.acceleration_x;
        player.set_moving_type(MovingType::MovingRight, &mut physics);

        assert!((physics.acceleration_x - after_one).abs() < f32::EPSILON, "repeating a move should be a no-op");
    }

    #[test]
    fn test_spawn_player_creates_a_full_player() {
        use crate::shared::entities::HealthComponent;
        use crate::shared::inventory::Inventory;

        let mut entities = Entities::new();
        let id = entities.new_id();
        let entity = spawn_player(&mut entities, 3.0, 4.0, "jakob", id, HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH)).unwrap();

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert!((position.x() - 3.0).abs() < f32::EPSILON);
        assert!((position.y() - 4.0).abs() < f32::EPSILON);

        assert_eq!(entities.ecs.get::<&PlayerComponent>(entity).unwrap().get_name(), "jakob");
        assert_eq!(entities.ecs.get::<&Inventory>(entity).unwrap().get_size(), PLAYER_INVENTORY_SIZE);
        assert_eq!(entities.get_id_from_entity(entity).unwrap(), id);
    }

    /// These are compile time constants, so this is a static check rather than a test
    /// that could ever fail at runtime. It still fails the build if someone makes the
    /// player wider than it is tall, which would break collision against 1x1 tiles.
    const _: () = assert!(PLAYER_WIDTH > 0.0);
    const _: () = assert!(PLAYER_HEIGHT > PLAYER_WIDTH);

    // ---------------- small shared types ----------------

    #[test]
    fn test_chat_packet_round_trip() {
        let packet = ChatPacket { message: "hello world".to_owned() };
        let bytes = serialization::serialize(&packet).unwrap();
        assert_eq!(serialization::deserialize::<ChatPacket>(&bytes).unwrap().message, "hello world");
    }

    #[test]
    fn test_version_packet_reports_the_build_version() {
        assert_eq!(VersionPacket::current().version, VERSION);
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_version_packet_round_trip() {
        let bytes = serialization::serialize(&VersionPacket::current()).unwrap();
        assert_eq!(serialization::deserialize::<VersionPacket>(&bytes).unwrap().version, VERSION);
    }

    /// The postcard format is save version 3 or later; anything lower means someone
    /// lowered it without changing the format back.
    const _: () = assert!(WORLD_SAVE_VERSION >= 3);

    /// The header is what makes a version mismatch reportable, so its shape is pinned:
    /// eight bytes of magic then a little endian `u32`.
    #[test]
    fn test_world_save_header_shape() {
        assert_eq!(WORLD_SAVE_MAGIC, b"TERRAWLD");
        assert_eq!(WORLD_SAVE_HEADER_LEN, 12);
    }

    #[test]
    fn test_block_id_undefined_round_trips() {
        let bytes = serialization::serialize(&BlockId::undefined()).unwrap();
        assert!(serialization::deserialize::<BlockId>(&bytes).unwrap() == BlockId::undefined());
    }
}
