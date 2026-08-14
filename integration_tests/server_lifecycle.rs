//! Starting, stepping, generating and stopping a whole server.
//!
//! `Server::run` is a loop around `start`, `update` and `stop`, and the game only ever
//! calls the loop - so nothing exercised those three in isolation, and nothing exercised
//! world generation at all outside of someone playing the game.
#![allow(clippy::unwrap_used, clippy::panic)] // tests assert on results directly
mod tests {
    use crate::integration_tests::harness::{free_port, write_world_save, TempDir, TestServer, BASE_GAME_MOD};
    use crate::libraries::events::EventManager;
    use crate::server::server_core::{BindAddress, Server};
    use crate::server::server_ui::ServerState;
    use crate::shared::liquids::MAX_LIQUID_LEVEL;

    /// A world small enough to generate and assert about in a test. The real game asks
    /// for 4400x1200, which would take long enough to make this suite useless.
    ///
    /// The height still has to clear the tallest biome - `biomes.lua` reaches a terrain
    /// height of 120 before turbulence - or the surface runs off the top of the world and
    /// there is no sky left to assert about.
    const SMALL: (i32, i32) = (200, 300);
    const SEED: u64 = 1234;

    /// A `run` that fails still takes its networking thread with it.
    ///
    /// This is the bug behind a report of a newly generated world hanging on its loading screen.
    /// `start` binds the port early and everything after it can fail, and a `?` out of `start` or
    /// `update` used to return while the networking thread was still running - and the `Server`
    /// that owns the receiving end of its channel was dropped a moment later. The thread went on
    /// holding the port and accepting connections it could no longer report to anyone: the server
    /// printed `Failed to send NewConnectionEvent: sending on a closed channel` and the client
    /// that connected was accepted and then never welcomed, which is a loading screen that never
    /// moves. The port also stayed taken for the rest of the process, so every world opened after
    /// it failed to bind and hung the same way.
    ///
    /// The failure here is a truncated world file, which is the cheapest way to fail `start`
    /// after the bind. What is being tested is the cleanup, not the parsing.
    #[test]
    fn test_a_server_that_fails_to_start_releases_its_port() {
        let dir = TempDir::new("lifecycle-failed-start");
        write_world_save(&dir.world_path(), (20, 20));
        let mut bytes = std::fs::read(dir.world_path()).unwrap();
        bytes.truncate(bytes.len() / 2);
        std::fs::write(dir.world_path(), &bytes).unwrap();

        let port = free_port();
        let mut server = Server::new(port, BindAddress::Loopback, None, None);
        let status = std::sync::Mutex::new(String::new());
        server
            .run(&std::sync::atomic::AtomicBool::new(true), &status, vec![BASE_GAME_MOD.to_vec()], &dir.world_path())
            .unwrap_err();
        assert!(server.get_state() == ServerState::Stopped, "a server that failed to start still says it is running");
        drop(server);

        // A live listener cannot be bound over, `SO_REUSEADDR` or not, so this succeeding is
        // exactly the statement that nothing is listening any more.
        assert!(
            std::net::TcpListener::bind(("127.0.0.1", port)).is_ok(),
            "the failed server left its networking thread holding port {port}"
        );
    }

    /// The states the ui shows a player, in the order they happen.
    #[test]
    fn test_the_server_runs_and_then_stops() {
        let mut server = TestServer::start_on_small_world("lifecycle-state", (40, 30)).unwrap();
        assert!(server.server.get_state() == ServerState::Running);

        server.update_times(5).unwrap();
        assert!(server.server.get_state() == ServerState::Running, "stepping the server changed its state");

        let status = std::sync::Mutex::new(String::new());
        let world_path = server.world_path();
        server.server.stop(&status, &world_path).unwrap();
        assert!(server.server.get_state() == ServerState::Stopped);
    }

    /// Stepping an idle server has to be free of side effects: no world change, and no
    /// error from the tick accumulator, which is the part with the most arithmetic in it.
    #[test]
    fn test_stepping_an_idle_server_changes_nothing() {
        let mut server = TestServer::start_on_small_world("lifecycle-idle", (40, 30)).unwrap();

        let before = block_grid(&server);
        server.update_times(50).unwrap();
        let after = block_grid(&server);

        assert!(before == after, "an idle server changed the world underneath itself");

        server.stop().unwrap();
    }

    /// The lua `/stop` command reaches the rust side and asks the server to stop, which
    /// is how a dedicated server is shut down from its console.
    #[test]
    fn test_the_stop_command_stops_the_server() {
        let mut server = TestServer::start_on_small_world("lifecycle-stop-command", (40, 30)).unwrap();

        server.server.execute_command("stop").unwrap();

        assert!(server.server.get_state() == ServerState::Stopping, "the /stop command did not ask the server to stop");

        server.stop().unwrap();
    }

    /// Generation end to end, with the real biomes from `biomes.lua`: the world comes out
    /// the size that was asked for, and it is terrain rather than a flat slab.
    #[test]
    fn test_generating_a_world_produces_terrain() {
        let server = TestServer::start_on_generated_world("gen-terrain", SMALL, SEED).unwrap();
        let blocks = server.server.get_blocks();

        let (width, height) = blocks.get_size();
        assert!(width as i32 >= SMALL.0, "the world came out narrower than the minimum width");
        assert_eq!(height as i32, SMALL.1, "the world came out the wrong height");

        let air = blocks.air();
        let mut air_count = 0_u32;
        let mut solid_count = 0_u32;
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                if blocks.get_block(x, y).unwrap() == air {
                    air_count += 1;
                } else {
                    solid_count += 1;
                }
            }
        }

        assert!(air_count > 0, "the generated world has no sky");
        assert!(solid_count > 0, "the generated world has no ground");

        drop(blocks);
        server.stop().unwrap();
    }

    /// The trees a generated world is decorated with have a 5x5 canopy on top, and the biome
    /// generator places a single cell of it - growing that into the full footprint is what the
    /// post-generation sweep over the blocks is for.
    ///
    /// It is checked before the server is ever stepped, because `start` has to return a world
    /// that is finished: the client is welcomed with a copy of it, and a save written before
    /// the first update has to be the same world when it is loaded again.
    #[test]
    fn test_generation_grows_the_tree_canopies() {
        let server = TestServer::start_on_generated_world("gen-canopies", SMALL, SEED).unwrap();
        let blocks = server.server.get_blocks();
        let canopy = blocks.get_block_id_by_name("canopy").unwrap();
        let (width, height) = blocks.get_size();
        let footprint = blocks.get_block_type(canopy).unwrap().width;

        let mut mains = 0;
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                if blocks.get_block(x, y).unwrap() != canopy || blocks.get_block_from_main(x, y).unwrap() != (0, 0) {
                    continue;
                }
                mains += 1;

                // every cell of the footprint is the same canopy, and each one knows where its
                // main cell is - which is what makes the whole thing break as one block
                for offset_x in 0..footprint {
                    for offset_y in 0..footprint {
                        let (cell_x, cell_y) = (x + offset_x, y + offset_y);
                        assert!(blocks.get_block(cell_x, cell_y).unwrap() == canopy, "the canopy at ({x}, {y}) did not grow into ({cell_x}, {cell_y})");
                        assert_eq!(
                            blocks.get_block_from_main(cell_x, cell_y).unwrap(),
                            (offset_x, offset_y),
                            "the cell at ({cell_x}, {cell_y}) does not point back at the canopy at ({x}, {y})"
                        );
                    }
                }
            }
        }

        assert!(mains > 0, "the generated world has no trees to check");

        drop(blocks);
        server.stop().unwrap();
    }

    /// Generation queues the events its own work produced, not one per block in the world.
    ///
    /// The sweep above used to run over every cell, and each one pushed a `BlockUpdateEvent`:
    /// 5.4 million of them for the default world, waiting in the queue for the first `update()`
    /// to offer each to every subsystem and hand it to lua's `on_block_update`. That update took
    /// 18 seconds in a debug build and held about a gigabyte, and it happened after the loading
    /// screen had closed, with a client already connected and waiting to be welcomed - so a
    /// newly generated world looked like a game that had hung.
    #[test]
    fn test_generation_does_not_queue_an_event_per_block() {
        let server = TestServer::start_on_generated_world("gen-event-queue", SMALL, SEED).unwrap();

        let (width, height) = server.server.get_blocks().get_size();
        let cells = (width * height) as usize;
        let queued = server.server.queued_event_count();

        // the bound is loose on purpose: what matters is that the number belongs to the trees
        // rather than to the size of the world, and the old sweep queued more than `cells`
        assert!(queued < cells / 10, "generating a {cells} cell world queued {queued} events");

        server.stop().unwrap();
    }

    /// Every column has ground under sky: the world is open at the top and solid at the
    /// bottom. A generator that got the vertical order wrong would still produce a mix of
    /// blocks and pass the test above.
    ///
    /// y grows downwards here - `generate_column` fills from `height - y`, so row 0 is
    /// the sky and row `height - 1` is the deepest rock. Worth stating, because the same
    /// codebase measures player positions the other way up.
    #[test]
    fn test_the_generated_world_has_sky_above_and_ground_below() {
        let server = TestServer::start_on_generated_world("gen-columns", SMALL, SEED).unwrap();
        let blocks = server.server.get_blocks();

        let (width, height) = blocks.get_size();
        let air = blocks.air();

        let mut solid_floor = 0;
        for x in 0..width as i32 {
            assert!(blocks.get_block(x, 0).unwrap() == air, "column {x} is not open at the top");
            assert!((0..height as i32).any(|y| blocks.get_block(x, y).unwrap() != air), "column {x} is empty from top to bottom");

            if blocks.get_block(x, height as i32 - 1).unwrap() != air {
                solid_floor += 1;
            }
        }

        // the deepest row is rock with caves in it, not the other way round. It is not
        // solid everywhere - caves do reach the bottom - so this is a proportion rather
        // than a rule about every column.
        assert!(solid_floor * 2 > width as i32, "most of the deepest row is open air, so the world has no ground under it");

        drop(blocks);
        server.stop().unwrap();
    }

    /// The generator floods the lowest ground, and what it pours is settled: every cell of
    /// it is full, sits in something a player could walk through, and has ground or more
    /// water directly beneath it. Water hanging in the air would mean the fill walked past
    /// a surface it should have stopped at.
    #[test]
    fn test_the_generated_world_has_water_and_it_is_settled() {
        let server = TestServer::start_on_generated_world("gen-water", SMALL, SEED).unwrap();

        let (width, height) = server.server.get_blocks().get_size();
        let mut water_cells = 0;
        let mut deepest_water = 0;

        for x in 0..width as i32 {
            for y in 0..height as i32 {
                // one lock at a time - both of these take a mutex on the server
                let level = server.server.get_liquids().get_liquid_level(x, y).unwrap();
                if level == 0 {
                    continue;
                }

                water_cells += 1;
                deepest_water = deepest_water.max(y);

                assert_eq!(level, MAX_LIQUID_LEVEL, "the generator left a half filled cell at ({x}, {y})");
                assert!(server.server.get_blocks().get_block_type_at(x, y).unwrap().ghost, "there is water inside a solid block at ({x}, {y})");

                let supported = server.server.get_liquids().get_liquid_level(x, y + 1).unwrap_or(0) > 0 || !server.server.get_blocks().get_block_type_at(x, y + 1).unwrap().ghost;
                assert!(supported, "the water at ({x}, {y}) is hanging in mid air");
            }
        }

        assert!(water_cells > 0, "the generated world has no water in it at all");
        assert!(deepest_water < height as i32 - 1, "water reached the bottom row, so it is not sitting on the surface");

        server.stop().unwrap();
    }

    /// Once the world is live, the simulation moves the generator's water around but never
    /// makes or loses any. The lakes sit on the surface, so nothing is buried in a block for
    /// the flow step to legitimately delete - every drop that leaves a cell has to arrive in
    /// another one.
    #[test]
    fn test_the_simulation_conserves_the_generated_water() {
        let server = TestServer::start_on_generated_world("gen-water-conserved", SMALL, SEED).unwrap();

        let poured: u32 = liquid_grid(&server).iter().map(|level| u32::from(*level)).sum();
        assert!(poured > 0, "the generator poured no water, so this proves nothing");

        // stepped directly rather than through `Server::update`, which paces flow off the
        // measured frame length - a few hundred flow steps would be a few seconds of test
        let mut events = EventManager::new();
        for _ in 0..300 {
            let blocks = server.server.get_blocks();
            server.server.get_liquids().update_liquids(&blocks, &mut events, 100.0).unwrap();
        }

        let after: u32 = liquid_grid(&server).iter().map(|level| u32::from(*level)).sum();
        assert_eq!(after, poured, "the simulation changed how much water the world holds");

        server.stop().unwrap();
    }

    /// Water is part of the world the same way blocks are, so the same seed has to put it
    /// in the same places.
    #[test]
    fn test_water_is_reproducible_from_the_seed() {
        let first = TestServer::start_on_generated_world("gen-water-seed-a", SMALL, SEED).unwrap();
        let first_levels = liquid_grid(&first);
        first.stop().unwrap();

        let second = TestServer::start_on_generated_world("gen-water-seed-b", SMALL, SEED).unwrap();
        let second_levels = liquid_grid(&second);
        second.stop().unwrap();

        assert!(!first_levels.is_empty());
        assert!(first_levels == second_levels, "the same seed generated water in different places");
    }

    /// Only blocks the mods registered end up in the world. An id the registry does not
    /// know would render as nothing and break on the next save.
    #[test]
    fn test_the_generated_world_only_uses_registered_blocks() {
        let server = TestServer::start_on_generated_world("gen-ids", SMALL, SEED).unwrap();
        let blocks = server.server.get_blocks();

        let (width, height) = blocks.get_size();
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let id = blocks.get_block(x, y).unwrap();
                blocks.get_block_type(id).unwrap_or_else(|_| panic!("the generator placed an unregistered block at ({x}, {y})"));
            }
        }

        drop(blocks);
        server.stop().unwrap();
    }

    /// The same seed has to give the same terrain.
    ///
    /// This checks the walls rather than the blocks, and the reason is worth knowing:
    /// walls are generated purely from the seeded terrain, while the blocks are handed to
    /// each biome's lua `generator_function` afterwards to have trees put on them - and
    /// that lua uses `math.random`, which is seeded per lua state, not from the world
    /// seed. So decoration is still different every run and the block grid with it. The
    /// rust half of generation is what this pins.
    #[test]
    fn test_generation_is_deterministic_for_a_seed() {
        let first = TestServer::start_on_generated_world("gen-seed-a", SMALL, SEED).unwrap();
        let second = TestServer::start_on_generated_world("gen-seed-b", SMALL, SEED).unwrap();

        assert_eq!(
            first.server.get_blocks().get_size(),
            second.server.get_blocks().get_size(),
            "the same seed generated worlds of different sizes"
        );

        let first_walls = first.server.get_walls().serialize().unwrap();
        let second_walls = second.server.get_walls().serialize().unwrap();
        assert!(first_walls == second_walls, "the same seed generated two different terrains");

        first.stop().unwrap();
        second.stop().unwrap();
    }

    /// And a different seed gives a different world, which is what stops the test above
    /// from passing on a generator that ignores its seed entirely.
    #[test]
    fn test_a_different_seed_generates_a_different_world() {
        let first = TestServer::start_on_generated_world("gen-seed-c", SMALL, SEED).unwrap();
        let second = TestServer::start_on_generated_world("gen-seed-d", SMALL, SEED + 1).unwrap();

        let first_walls = first.server.get_walls().serialize().unwrap();
        let second_walls = second.server.get_walls().serialize().unwrap();

        assert!(first_walls != second_walls, "two seeds generated the same terrain");

        first.stop().unwrap();
        second.stop().unwrap();
    }

    /// Every block of a world, in a form two runs can be compared by.
    ///
    /// Not the serialized bytes: `BlocksData` carries hash maps for block data and
    /// inventories, and a map that has been through a save and load iterates in a
    /// different order, so the bytes differ even when every block is the same.
    fn block_grid(server: &TestServer) -> Vec<crate::shared::blocks::BlockId> {
        let blocks = server.server.get_blocks();
        let (width, height) = blocks.get_size();
        (0..width as i32)
            .flat_map(|x| (0..height as i32).map(move |y| (x, y)))
            .map(|(x, y)| blocks.get_block(x, y).unwrap())
            .collect()
    }

    fn liquid_grid(server: &TestServer) -> Vec<u8> {
        let liquids = server.server.get_liquids();
        let (width, height) = liquids.get_size();
        (0..width as i32)
            .flat_map(|x| (0..height as i32).map(move |y| (x, y)))
            .map(|(x, y)| liquids.get_liquid_level(x, y).unwrap())
            .collect()
    }

    /// A generated world survives being saved and loaded, which is the path every world
    /// takes the second time a player opens it.
    #[test]
    fn test_a_generated_world_survives_a_restart() {
        let server = TestServer::start_on_generated_world("gen-restart", SMALL, SEED).unwrap();
        let before = block_grid(&server);
        let dir = server.stop().unwrap();

        let reloaded = TestServer::start_in(dir, "gen-restart").unwrap();
        let after = block_grid(&reloaded);

        assert!(before == after, "the generated world came back different after a restart");

        reloaded.stop().unwrap();
    }

    /// Walls are generated alongside blocks and share the world's dimensions. A mismatch
    /// would be an out of bounds read the moment anything looked behind a block.
    #[test]
    fn test_the_generated_walls_match_the_world_size() {
        let server = TestServer::start_on_generated_world("gen-walls", SMALL, SEED).unwrap();

        let block_size = server.server.get_blocks().get_size();
        let wall_size = server.server.get_walls().get_size();

        assert_eq!(block_size, wall_size, "the wall grid is a different size from the block grid");

        server.stop().unwrap();
    }

    /// Starting a server twice on the same directory is what happens when a player
    /// reopens a world, and both runs have to end with the world intact.
    #[test]
    fn test_a_world_can_be_opened_repeatedly() {
        let mut dir = TempDir::new("lifecycle-reopen");
        crate::integration_tests::harness::write_world_save(&dir.world_path(), (40, 30));

        for run in 0..3 {
            let server = TestServer::start_in(dir, "lifecycle-reopen").unwrap();
            assert_eq!(server.server.get_blocks().get_size(), (40, 30), "the world changed size on run {run}");
            dir = server.stop().unwrap();
        }
    }
}
