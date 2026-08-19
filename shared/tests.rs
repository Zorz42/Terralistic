#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::libraries::fixed::Fixed;
    use crate::libraries::serialization;
    use crate::shared::blocks::{Block, BlockId, Blocks};
    use crate::shared::chat::ChatPacket;
    use crate::shared::entities::{Entities, EntityId, PhysicsComponent, PositionComponent};
    use crate::shared::items::Items;
    use crate::shared::lights::{LightColor, Lights};
    use crate::shared::liquids::Liquids;
    use crate::shared::players::{attract_items_to_players, remove_all_picked_items, spawn_player, MovingType, PlayerComponent, PLAYER_HEIGHT, PLAYER_INVENTORY_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH};
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
        assert_eq!(player.get_moving_type(), MovingType::Standing);
        assert!(!player.jumping);
    }

    /// Changing moving type applies the new state's acceleration and removes the old one,
    /// so going left then right does not leave the player accelerating both ways.
    #[test]
    fn test_moving_type_transitions_are_reversible() {
        use crate::shared::entities::PhysicsComponent;

        let mut player = PlayerComponent::new("p");
        let mut physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));
        let neutral = physics.acceleration_x;

        player.set_moving_type(MovingType::MovingRight, &mut physics);
        assert!(physics.acceleration_x > neutral, "moving right should accelerate right");

        player.set_moving_type(MovingType::MovingLeft, &mut physics);
        assert!(physics.acceleration_x < neutral, "moving left should accelerate left");

        player.set_moving_type(MovingType::Standing, &mut physics);
        assert_eq!(physics.acceleration_x, neutral, "standing should restore the neutral acceleration");
    }

    #[test]
    fn test_setting_the_same_moving_type_twice_does_not_stack() {
        use crate::shared::entities::PhysicsComponent;

        let mut player = PlayerComponent::new("p");
        let mut physics = PhysicsComponent::new(Fixed::from_int(1), Fixed::from_int(1));

        player.set_moving_type(MovingType::MovingRight, &mut physics);
        let after_one = physics.acceleration_x;
        player.set_moving_type(MovingType::MovingRight, &mut physics);

        assert_eq!(physics.acceleration_x, after_one, "repeating a move should be a no-op");
    }

    #[test]
    fn test_spawn_player_creates_a_full_player() {
        use crate::shared::entities::HealthComponent;
        use crate::shared::inventory::Inventory;

        let mut entities = Entities::new();
        let id = entities.new_id();
        let entity = spawn_player(
            &mut entities,
            Fixed::from_int(3),
            Fixed::from_int(4),
            "jakob",
            id,
            HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
        )
        .unwrap();

        let position = entities.ecs.get::<&PositionComponent>(entity).unwrap();
        assert_eq!(position.x(), Fixed::from_int(3));
        assert_eq!(position.y(), Fixed::from_int(4));

        assert_eq!(entities.ecs.get::<&PlayerComponent>(entity).unwrap().get_name(), "jakob");
        assert_eq!(entities.ecs.get::<&Inventory>(entity).unwrap().get_size(), PLAYER_INVENTORY_SIZE);
        assert_eq!(entities.get_id_from_entity(entity).unwrap(), id);
    }

    /// These are compile time constants, so this is a static check rather than a test
    /// that could ever fail at runtime. It still fails the build if someone makes the
    /// player wider than it is tall, which would break collision against 1x1 tiles.
    const _: () = assert!(PLAYER_WIDTH.raw() > 0);
    const _: () = assert!(PLAYER_HEIGHT.raw() > PLAYER_WIDTH.raw());

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

    // ---------------- item pickup ----------------

    /// A player standing in an empty world, an item beside it, and enough of a world for the
    /// physics step to read.
    fn player_and_item(offset: (i32, i32), velocity: (Fixed, Fixed), ground: bool) -> (Entities, Blocks, Liquids, Items, hecs::Entity) {
        // with ground under it, so the player stands where a real one would rather than
        // falling out of the world while the test measures it
        let mut blocks = Blocks::new();
        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);
        blocks.create((60, 60));
        let mut events = EventManager::new();
        if ground {
            for x in 0..60 {
                for y in 33..60 {
                    blocks.set_block(&mut events, x, y, solid_id).unwrap();
                }
            }
        }
        let mut liquids = Liquids::new();
        liquids.create((60, 60));

        let mut items = Items::new();
        let item_type = items.register_item_type(crate::shared::items::Item::new());

        let mut entities = Entities::new();
        spawn_player(
            &mut entities,
            Fixed::from_int(30),
            Fixed::from_int(30),
            "Collector",
            EntityId::from_raw(1000),
            crate::shared::entities::HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
        )
        .unwrap();

        let item = items
            .spawn_item(
                &mut events,
                &mut entities,
                item_type,
                Fixed::from_int(30 + offset.0),
                Fixed::from_int(30 + offset.1),
                EntityId::from_raw(1001),
            )
            .unwrap();
        {
            let mut physics = entities.ecs.get::<&mut PhysicsComponent>(item).unwrap();
            physics.velocity_x = velocity.0;
            physics.velocity_y = velocity.1;
        }

        (entities, blocks, liquids, items, item)
    }

    /// How far the item is from the player it is being pulled towards.
    fn gap(entities: &mut Entities, item: hecs::Entity) -> Fixed {
        let player = {
            let mut found = (Fixed::ZERO, Fixed::ZERO);
            for (position, _player) in entities.ecs.query_mut::<(&PositionComponent, &PlayerComponent)>() {
                found = (position.x(), position.y());
            }
            found
        };
        let position = *entities.ecs.query_one_mut::<&PositionComponent>(item).unwrap();
        let (dx, dy) = (player.0 - position.x(), player.1 - position.y());
        (dx * dx + dy * dy).sqrt()
    }

    /// **An item thrown past a player must not go into orbit around it.**
    ///
    /// The pull used to be a force added to whatever the item was already doing, so an item
    /// arriving off-centre kept its sideways velocity, missed, swung round and came back -
    /// circling until it happened to clip the pickup radius. The pull closes the gap to a
    /// velocity aimed at the player instead, which damps that component out.
    #[test]
    fn test_an_item_thrown_sideways_does_not_orbit_the_player() {
        // beside the player and moving fast across it, which is the worst case for an orbit
        let (mut entities, blocks, liquids, items, item) = player_and_item((-3, 0), (Fixed::ZERO, Fixed::from_int(-14)), true);
        let mut events = EventManager::new();

        let mut ticks_taken = None;
        for tick in 0..400_i32 {
            attract_items_to_players(&mut entities);
            crate::shared::players::update_players_ms(&mut entities, &blocks, &liquids);
            entities.update_entities_ms(&blocks, &liquids, &mut events).unwrap();
            remove_all_picked_items(&mut entities, &mut events, &items).unwrap();

            if entities.ecs.query_one_mut::<&PositionComponent>(item).is_err() {
                ticks_taken = Some(tick);
                break;
            }
        }

        // three blocks away at a pull that reaches 30 blocks a second; never, or anything
        // near the 400 tick limit, means it went round rather than in
        let ticks_taken = ticks_taken.unwrap_or(i32::MAX);
        assert!(ticks_taken < 120, "the item took {ticks_taken} ticks to be picked up, which is a lap not a line");
    }

    /// **Once the pull has hold of an item, the gap only ever closes.** An orbit shows up here
    /// as the distance growing again after it has started shrinking, whether or not the item
    /// is eventually caught - which is what made a pickup look like the item was circling.
    #[test]
    fn test_the_gap_to_the_player_only_ever_closes() {
        // thrown up and away from the player, so the pull has to turn it around first
        let (mut entities, blocks, liquids, items, item) = player_and_item((-4, -2), (Fixed::from_int(-6), Fixed::from_int(-10)), true);
        let mut events = EventManager::new();

        let mut previous = gap(&mut entities, item);
        let mut closing = false;
        let mut picked_up = false;
        for _ in 0..300 {
            attract_items_to_players(&mut entities);
            crate::shared::players::update_players_ms(&mut entities, &blocks, &liquids);
            entities.update_entities_ms(&blocks, &liquids, &mut events).unwrap();
            remove_all_picked_items(&mut entities, &mut events, &items).unwrap();

            if entities.ecs.query_one_mut::<&PositionComponent>(item).is_err() {
                picked_up = true;
                break;
            }

            let now = gap(&mut entities, item);
            if closing {
                assert!(now <= previous + Fixed::from_num(1, 10), "the item swung back out, from {previous} to {now}");
            }
            closing |= now < previous;
            previous = now;
        }

        assert!(closing, "the item never closed on the player at all");
        assert!(picked_up, "the item never reached the player");
    }

    /// **The pull is a speed relative to the player, not through the world.**
    ///
    /// A player falling in open air reaches `DEFAULT_GRAVITY` blocks a second, well over the
    /// fastest the pull closes at. Aiming the item at a fixed world speed makes that speed a
    /// limit, and the player simply drops away from an item it is supposedly collecting.
    #[test]
    fn test_an_item_keeps_up_with_a_player_falling_faster_than_the_pull() {
        // no ground, so the player is at terminal velocity within a second
        let (mut entities, blocks, liquids, items, item) = player_and_item((-2, 0), (Fixed::ZERO, Fixed::ZERO), false);
        let mut events = EventManager::new();

        let mut picked_up = false;
        for _ in 0..400 {
            attract_items_to_players(&mut entities);
            crate::shared::players::update_players_ms(&mut entities, &blocks, &liquids);
            entities.update_entities_ms(&blocks, &liquids, &mut events).unwrap();
            remove_all_picked_items(&mut entities, &mut events, &items).unwrap();

            if entities.ecs.query_one_mut::<&PositionComponent>(item).is_err() {
                picked_up = true;
                break;
            }
        }

        assert!(picked_up, "a falling player left its own item behind");
    }

    // ---------------- movement feel ----------------

    /// A player standing on solid ground, with enough of it either side to walk at full speed
    /// for several seconds - a player that runs off the end is a player in free fall, and
    /// ground friction rightly does not apply to one of those.
    fn standing_player() -> (Entities, Blocks, Liquids, hecs::Entity) {
        let mut blocks = Blocks::new();
        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);
        blocks.create((400, 60));
        let mut events = EventManager::new();
        for x in 0..400 {
            for y in 33..60 {
                blocks.set_block(&mut events, x, y, solid_id).unwrap();
            }
        }

        let mut liquids = Liquids::new();
        liquids.create((400, 60));

        let mut entities = Entities::new();
        let player = spawn_player(
            &mut entities,
            Fixed::from_int(30),
            Fixed::from_int(30),
            "Walker",
            EntityId::from_raw(1000),
            crate::shared::entities::HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH),
        )
        .unwrap();

        (entities, blocks, liquids, player)
    }

    /// Runs the player for `ticks`, returning where it ended up and how fast it was going.
    fn walk(entities: &mut Entities, blocks: &Blocks, liquids: &Liquids, player: hecs::Entity, ticks: u32) -> (Fixed, Fixed) {
        let mut events = EventManager::new();
        for _ in 0..ticks {
            crate::shared::players::update_players_ms(entities, blocks, liquids);
            entities.update_entities_ms(blocks, liquids, &mut events).unwrap();
        }
        let position = *entities.ecs.query_one_mut::<&PositionComponent>(player).unwrap();
        let physics = *entities.ecs.query_one_mut::<&PhysicsComponent>(player).unwrap();
        (position.x(), physics.velocity_x)
    }

    fn hold(entities: &mut Entities, player: hecs::Entity, moving_type: MovingType) {
        let (physics, component) = entities.ecs.query_one_mut::<(&mut PhysicsComponent, &mut PlayerComponent)>(player).unwrap();
        component.set_moving_type(moving_type, physics);
    }

    /// **Letting go of a key has to stop the player.** The only drag used to be the air's,
    /// applied to a player standing on stone exactly as to one falling through the sky, so a
    /// release slid twelve blocks - six times the player's own width - and the game felt like
    /// it was lagging behind the keyboard.
    #[test]
    fn test_releasing_a_key_stops_the_player_within_a_block() {
        let (mut entities, blocks, liquids, player) = standing_player();
        hold(&mut entities, player, MovingType::MovingRight);
        let (_, top_speed) = walk(&mut entities, &blocks, &liquids, player, 600);
        assert!(top_speed > Fixed::from_int(10), "the player never got up to speed: {top_speed}");

        let (released_at, _) = walk(&mut entities, &blocks, &liquids, player, 0);
        hold(&mut entities, player, MovingType::Standing);
        let (stopped_at, speed) = walk(&mut entities, &blocks, &liquids, player, 100);

        // 0.15 blocks in 13 ticks, against 1.71 blocks over 85 ticks with no ground friction
        assert!(speed.abs() < Fixed::from_num(1, 10), "still moving at {speed} half a second after letting go");
        assert!(stopped_at - released_at < Fixed::from_num(1, 2), "slid {} blocks after letting go", stopped_at - released_at);
    }

    /// Friction must not become a speed limit. It applies only to speed the input is not
    /// asking for, so a held key still reaches the same top speed it always did - drag on a
    /// commanded direction would cap it at acceleration over friction, under a block a second.
    #[test]
    fn test_ground_friction_does_not_cap_a_held_key() {
        let (mut entities, blocks, liquids, player) = standing_player();
        hold(&mut entities, player, MovingType::MovingRight);
        let (_, speed) = walk(&mut entities, &blocks, &liquids, player, 600);

        // 13.51 blocks a second, which is what it was before there was any ground friction at
        // all - the same run with the friction taken out reaches exactly the same speed
        assert!(speed > Fixed::from_int(13), "a held key should still reach the walking speed it always did, got {speed}");
    }

    /// Turning round is the case A/D spam is made of, and the one that felt worst: the speed
    /// of the direction you have just stopped holding is exactly what friction is for.
    #[test]
    fn test_turning_round_does_not_take_a_quarter_of_a_second() {
        let (mut entities, blocks, liquids, player) = standing_player();
        hold(&mut entities, player, MovingType::MovingRight);
        walk(&mut entities, &blocks, &liquids, player, 600);

        hold(&mut entities, player, MovingType::MovingLeft);
        // twenty ticks is a tenth of a second
        let (_, speed) = walk(&mut entities, &blocks, &liquids, player, 20);

        assert!(speed < Fixed::ZERO, "still travelling the old way a tenth of a second after turning: {speed}");
    }
}
