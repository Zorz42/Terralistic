//! Saving a world and reading it back, through a real `Server`.
//!
//! The save format has no self description beyond a version number: it stores raw block
//! and wall grids and relies on the mods registering their types in the same order on the
//! way back in. There is nothing to catch a mistake here at compile time, and getting it
//! wrong means someone's world reads as the wrong blocks or does not open at all.
#![allow(clippy::unwrap_used)] // tests assert on results directly
mod tests {
    use std::collections::HashMap;

    use crate::integration_tests::harness::{expect_start_error, write_world_save, TempDir, TestServer};
    use crate::libraries::events::EventManager;
    use crate::libraries::serialization;
    use crate::shared::versions::{WORLD_SAVE_VERSION, WORLD_SAVE_VERSION_KEY};

    /// The round trip that matters: a block placed in one run is still there in the next.
    ///
    /// Blocks are saved as raw ids and the type registry is not saved at all, so this
    /// only works because the mods register their types in the same order both times.
    #[test]
    fn test_a_block_survives_a_save_and_reload() {
        let server = TestServer::start_on_small_world("persist-block", (60, 40)).unwrap();

        let stone = {
            let mut blocks = server.server.get_blocks();
            let stone = blocks.get_block_id_by_name("stone_block").unwrap();
            let mut events = EventManager::new();
            blocks.set_block(&mut events, 7, 9, stone).unwrap();
            stone
        };

        let dir = server.stop().unwrap();

        let reloaded = TestServer::start_in(dir, "persist-block").unwrap();
        let blocks = reloaded.server.get_blocks();

        assert!(blocks.get_block(7, 9).unwrap() == stone, "the block did not survive the reload");
        assert!(blocks.get_block(8, 9).unwrap() == blocks.air(), "an untouched block came back as something else");
        assert_eq!(blocks.get_size(), (60, 40), "the world changed size across a save");

        drop(blocks);
        reloaded.stop().unwrap();
    }

    /// Walls are a separate grid with a separate registry, saved under its own key.
    #[test]
    fn test_a_wall_survives_a_save_and_reload() {
        let server = TestServer::start_on_small_world("persist-wall", (40, 30)).unwrap();

        let dirt_wall = {
            let mut walls = server.server.get_walls();
            let dirt_wall = walls.get_wall_id_by_name("dirt").unwrap();
            walls.set_wall_type(3, 4, dirt_wall).unwrap();
            dirt_wall
        };

        let dir = server.stop().unwrap();

        let reloaded = TestServer::start_in(dir, "persist-wall").unwrap();
        let walls = reloaded.server.get_walls();

        assert!(walls.get_wall_type_at(3, 4).unwrap().get_id() == dirt_wall, "the wall did not survive the reload");

        drop(walls);
        reloaded.stop().unwrap();
    }

    /// Stopping twice must not write the world twice or fail - `stop` is called from both
    /// the run loop and the caller that asked it to stop.
    #[test]
    fn test_stopping_twice_is_harmless() {
        let mut server = TestServer::start_on_small_world("persist-double-stop", (30, 20)).unwrap();
        let status = std::sync::Mutex::new(String::new());
        let world_path = server.world_path();

        server.server.stop(&status, &world_path).unwrap();
        server.server.stop(&status, &world_path).unwrap();
    }

    /// The save is written where the game expects it, with the keys the loader reads.
    #[test]
    fn test_the_save_has_the_keys_the_loader_reads() {
        let server = TestServer::start_on_small_world("persist-keys", (30, 20)).unwrap();
        let dir = server.stop().unwrap();

        let world: HashMap<String, Vec<u8>> = serialization::deserialize(&std::fs::read(dir.world_path()).unwrap()).unwrap();

        for key in [WORLD_SAVE_VERSION_KEY, "blocks", "walls", "players"] {
            assert!(world.contains_key(key), "the save has no {key} in it");
        }

        let version: u32 = serialization::deserialize(world.get(WORLD_SAVE_VERSION_KEY).unwrap()).unwrap();
        assert_eq!(version, WORLD_SAVE_VERSION);
    }

    /// A world from a future version is refused with an explanation rather than being
    /// read as whatever the current layout happens to make of those bytes.
    #[test]
    fn test_a_newer_save_version_is_refused() {
        let dir = TempDir::new("persist-newer");
        write_world_save(&dir.world_path(), (20, 20));

        // rewrite just the version, leaving a save that is otherwise perfectly valid
        let mut world: HashMap<String, Vec<u8>> = serialization::deserialize(&std::fs::read(dir.world_path()).unwrap()).unwrap();
        world.insert(WORLD_SAVE_VERSION_KEY.to_owned(), serialization::serialize(&(WORLD_SAVE_VERSION + 1)).unwrap());
        std::fs::write(dir.world_path(), serialization::serialize(&world).unwrap()).unwrap();

        let error = expect_start_error(dir, "persist-newer");
        assert!(error.contains("save version"), "unexpected error: {error}");
    }

    /// A save with no version key at all predates versioning, which also means it was
    /// written by bincode 1 and cannot be read now. It has to say so rather than
    /// producing a world of nonsense.
    #[test]
    fn test_a_save_without_a_version_is_refused() {
        let dir = TempDir::new("persist-unversioned");
        write_world_save(&dir.world_path(), (20, 20));

        let mut world: HashMap<String, Vec<u8>> = serialization::deserialize(&std::fs::read(dir.world_path()).unwrap()).unwrap();
        world.remove(WORLD_SAVE_VERSION_KEY);
        std::fs::write(dir.world_path(), serialization::serialize(&world).unwrap()).unwrap();

        expect_start_error(dir, "persist-unversioned");
    }

    /// A truncated or corrupted file is an error, not a panic. Worlds get cut short by
    /// full disks and killed processes, and the server must survive meeting one.
    #[test]
    fn test_a_corrupt_save_is_refused() {
        let dir = TempDir::new("persist-corrupt");
        write_world_save(&dir.world_path(), (20, 20));

        let mut bytes = std::fs::read(dir.world_path()).unwrap();
        bytes.truncate(bytes.len() / 2);
        std::fs::write(dir.world_path(), &bytes).unwrap();

        expect_start_error(dir, "persist-corrupt");
    }

    /// A file that is not a world at all is refused the same way.
    #[test]
    fn test_a_file_that_is_not_a_world_is_refused() {
        let dir = TempDir::new("persist-garbage");
        std::fs::write(dir.world_path(), b"this is not a world, it is a text file").unwrap();

        expect_start_error(dir, "persist-garbage");
    }

    /// Saving into a directory that does not exist yet creates it, which is what happens
    /// the first time a player makes a world.
    #[test]
    fn test_saving_creates_the_directory() {
        let dir = TempDir::new("persist-mkdir");
        let nested = dir.path().join("worlds").join("new").join("server.world");

        let mut server = TestServer::start_on_small_world("persist-mkdir-server", (20, 20)).unwrap();
        let status = std::sync::Mutex::new(String::new());
        server.server.stop(&status, &nested).unwrap();

        assert!(nested.exists(), "the world was not written to a new directory");
    }
}
