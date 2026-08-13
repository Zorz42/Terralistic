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
    use crate::shared::mod_data::GameModData;
    use crate::shared::mod_manager::{GameMod, ModManager};
    use crate::shared::players::{spawn_player, MovingType, PlayerComponent, PLAYER_HEIGHT, PLAYER_INVENTORY_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH};
    use crate::shared::versions::{VersionPacket, VERSION, WORLD_SAVE_VERSION, WORLD_SAVE_VERSION_KEY};
    use std::collections::BTreeMap;

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

    // ---------------- mod system ----------------

    fn test_mod(name: &str, lua: &str) -> GameMod {
        GameMod::new(name.to_owned(), lua.to_owned(), std::collections::HashMap::new())
    }

    const MINIMAL_LUA: &str = "
        init_called = false
        update_count = 0
        stop_called = false
        function init() init_called = true end
        function update() update_count = update_count + 1 end
        function stop() stop_called = true end
    ";

    #[test]
    fn test_mod_lifecycle_hooks_run() {
        let mut mods = ModManager::new(vec![test_mod("test", MINIMAL_LUA)]);

        mods.init().unwrap();
        mods.update().unwrap();
        mods.update().unwrap();
        mods.stop().unwrap();

        let game_mod = mods.get_mod(0).unwrap();
        assert!(game_mod.is_symbol_defined("init").unwrap());
        assert!(game_mod.is_symbol_defined("update").unwrap());
        assert!(game_mod.is_symbol_defined("stop").unwrap());
    }

    #[test]
    fn test_mod_name_is_kept() {
        let mut mods = ModManager::new(vec![test_mod("base_game", MINIMAL_LUA)]);
        mods.init().unwrap();
        assert_eq!(mods.get_mod(0).unwrap().get_name(), "base_game");
    }

    #[test]
    fn test_undefined_symbol_is_reported() {
        let mut mods = ModManager::new(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        let game_mod = mods.get_mod(0).unwrap();
        assert!(!game_mod.is_symbol_defined("not_a_real_function").unwrap());
    }

    #[test]
    fn test_get_all_symbols_includes_defined_functions() {
        let mut mods = ModManager::new(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        let symbols = mods.get_mod(0).unwrap().get_all_symbols();
        assert!(symbols.iter().any(|s| s == "init"));
        assert!(symbols.iter().any(|s| s == "update"));
    }

    /// Rust functions are exposed to lua with a `terralistic_` prefix added automatically.
    #[test]
    fn test_global_functions_get_the_terralistic_prefix() {
        let lua = "
            function init() end
            function update() end
            function stop() end
            function call_it() return terralistic_double(21) end
        ";
        let mut mods = ModManager::new(vec![test_mod("test", lua)]);
        mods.add_global_function("double", |_, value: i32| Ok(value * 2)).unwrap();
        mods.init().unwrap();

        let result: i32 = mods.get_mod(0).unwrap().call_function("call_it", ()).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_calling_a_missing_function_is_an_error() {
        let mut mods = ModManager::new(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        mods.get_mod(0).unwrap().call_function::<(), ()>("nope", ()).unwrap_err();
    }

    #[test]
    fn test_broken_lua_fails_to_init() {
        let mut mods = ModManager::new(vec![test_mod("test", "this is not lua ((")]);
        mods.init().unwrap_err();
    }

    #[test]
    fn test_get_mod_out_of_range() {
        let mut mods = ModManager::new(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        assert!(mods.get_mod(5).is_none());
        assert!(mods.get_mod(-1).is_none());
    }

    #[test]
    fn test_resources_are_looked_up_across_mods() {
        let mut a = std::collections::HashMap::new();
        a.insert("blocks:dirt.opa".to_owned(), vec![1, 2, 3]);

        let mods = ModManager::new(vec![GameMod::new("a".to_owned(), MINIMAL_LUA.to_owned(), a)]);

        assert_eq!(mods.get_resource("blocks:dirt.opa"), Some(&vec![1, 2, 3]));
        assert_eq!(mods.get_resource("blocks:nothing.opa"), None);
    }

    /// Later mods win, which is how a mod overrides a base game resource.
    #[test]
    fn test_later_mods_override_resources() {
        let mut first = std::collections::HashMap::new();
        first.insert("misc:icon.opa".to_owned(), vec![1]);
        let mut second = std::collections::HashMap::new();
        second.insert("misc:icon.opa".to_owned(), vec![2]);

        let mods = ModManager::new(vec![
            GameMod::new("first".to_owned(), MINIMAL_LUA.to_owned(), first),
            GameMod::new("second".to_owned(), MINIMAL_LUA.to_owned(), second),
        ]);

        assert_eq!(mods.get_resource("misc:icon.opa"), Some(&vec![2]));
    }

    #[test]
    fn test_mods_iter_sees_every_mod() {
        let mods = ModManager::new(vec![test_mod("a", MINIMAL_LUA), test_mod("b", MINIMAL_LUA)]);
        let names: Vec<&str> = mods.mods_iter().map(GameMod::get_name).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    /// A `GameMod` serializes through `GameModData`, which is the on disk `.mod` format.
    #[test]
    fn test_game_mod_serialize_round_trip() {
        let mut resources = std::collections::HashMap::new();
        resources.insert("blocks:dirt.opa".to_owned(), vec![9, 8, 7]);
        let game_mod = GameMod::new("round_trip".to_owned(), MINIMAL_LUA.to_owned(), resources);

        let bytes = serialization::serialize(&game_mod).unwrap();
        let restored: GameMod = serialization::deserialize(&bytes).unwrap();

        assert_eq!(restored.get_name(), "round_trip");

        let mods = ModManager::new(vec![restored]);
        assert_eq!(mods.get_resource("blocks:dirt.opa"), Some(&vec![9, 8, 7]));
    }

    /// The build script writes `GameModData` directly, so it has to produce the same bytes
    /// a `GameMod` would.
    #[test]
    fn test_game_mod_data_matches_game_mod_bytes() {
        let mut resources = std::collections::HashMap::new();
        resources.insert("a:b.opa".to_owned(), vec![1, 2]);
        let game_mod = GameMod::new("same".to_owned(), "x = 1".to_owned(), resources.clone());

        let mut ordered = BTreeMap::new();
        for (key, value) in resources {
            ordered.insert(key, value);
        }
        let data = GameModData {
            name: "same".to_owned(),
            lua_code: "x = 1".to_owned(),
            resources: ordered,
        };

        assert_eq!(serialization::serialize(&game_mod).unwrap(), serialization::serialize(&data).unwrap());
    }

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

    #[test]
    fn test_world_save_version_key() {
        assert_eq!(WORLD_SAVE_VERSION_KEY, "version");
    }

    #[test]
    fn test_block_id_undefined_round_trips() {
        let bytes = serialization::serialize(&BlockId::undefined()).unwrap();
        assert!(serialization::deserialize::<BlockId>(&bytes).unwrap() == BlockId::undefined());
    }
}
