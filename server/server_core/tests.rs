#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::libraries::events::EventManager;
    use crate::server::server_core::commands::CommandManager;
    use crate::server::server_core::players::ServerPlayers;
    use crate::shared::blocks::{Block, Blocks};
    use crate::shared::mod_manager::ModManager;
    use crate::shared::players::PLAYER_HEIGHT;

    /// Runs a command against a `CommandManager` that has no mods loaded.
    fn execute(command: &str) -> anyhow::Result<String> {
        let commands = CommandManager::new();
        let mut mods = ModManager::new(Vec::new());
        commands.execute_command(command, &mut mods, None)
    }

    /// A chat message of just "/" is stripped to an empty string before it reaches
    /// `execute_command`. That used to panic on `Vec::remove(0)`, which took the whole
    /// server down and was reachable by any connected player from the stock client.
    #[test]
    fn test_empty_command_does_not_panic() {
        assert!(execute("").is_err());
    }

    /// Same path, but the message was "/ " or similar: `split_whitespace` still yields
    /// no tokens.
    #[test]
    fn test_whitespace_only_command_does_not_panic() {
        assert!(execute("   ").is_err());
        assert!(execute("\t").is_err());
    }

    /// An unknown command is reported back to the caller rather than being an error.
    #[test]
    fn test_unknown_command_is_reported() {
        let result = execute("definitely_not_a_command").unwrap();
        assert!(result.contains("definitely_not_a_command"), "unexpected output: {result}");
    }

    /// The builtin help command works without any mods loaded.
    #[test]
    fn test_help_command() {
        let result = execute("help").unwrap();
        assert!(result.contains("/help"), "unexpected output: {result}");
    }

    /// Help rejects more than one argument instead of panicking or silently ignoring.
    #[test]
    fn test_help_with_too_many_arguments() {
        assert!(execute("help one two").is_err());
    }

    /// Builds a world of the given size that is solid from `ground_y` downwards.
    fn world_with_ground_at(size: (u32, u32), ground_y: Option<u32>) -> Blocks {
        let mut blocks = Blocks::new();

        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);

        blocks.create(size);

        if let Some(ground_y) = ground_y {
            let mut events = EventManager::new();
            for x in 0..size.0 {
                for y in ground_y..size.1 {
                    blocks.set_block(&mut events, x as i32, y as i32, solid_id).unwrap();
                }
            }
        }

        blocks
    }

    /// The player is placed standing on the first solid tile from the top.
    #[test]
    fn test_spawn_lands_on_the_surface() {
        let blocks = world_with_ground_at((32, 64), Some(40));
        let (x, y) = ServerPlayers::get_spawn_coords(&blocks);

        assert!((x - 16.0).abs() < f32::EPSILON, "expected the middle column, got {x}");
        assert!((y - (40.0 - PLAYER_HEIGHT)).abs() < f32::EPSILON, "expected to stand on the ground at y=40, got {y}");
    }

    /// Ghost blocks are walked through, so a layer of them above the ground does not
    /// become the spawn point.
    #[test]
    fn test_spawn_ignores_ghost_blocks() {
        let mut blocks = Blocks::new();

        let mut ghost = Block::new();
        ghost.name = "ghost".to_owned();
        ghost.ghost = true;
        let ghost_id = blocks.register_new_block_type(ghost);

        let mut solid = Block::new();
        solid.name = "solid".to_owned();
        solid.ghost = false;
        let solid_id = blocks.register_new_block_type(solid);

        blocks.create((32, 64));

        let mut events = EventManager::new();
        for x in 0..32 {
            // a band of ghost blocks well above the ground, like foliage
            for y in 10..20 {
                blocks.set_block(&mut events, x, y, ghost_id).unwrap();
            }
            for y in 40..64 {
                blocks.set_block(&mut events, x, y, solid_id).unwrap();
            }
        }

        let (_x, y) = ServerPlayers::get_spawn_coords(&blocks);
        assert!((y - (40.0 - PLAYER_HEIGHT)).abs() < f32::EPSILON, "spawned on the ghost band instead of the ground, got {y}");
    }

    /// A column with nothing solid in it puts the player at the bottom of the map, not
    /// at the very top where they would fall the whole height of the world.
    #[test]
    fn test_spawn_in_an_empty_world() {
        let blocks = world_with_ground_at((32, 64), None);
        let (_x, y) = ServerPlayers::get_spawn_coords(&blocks);

        assert!(y > 0.0, "an empty column should not spawn the player at the top of the map, got {y}");
        assert!((y - (64.0 - PLAYER_HEIGHT)).abs() < f32::EPSILON, "expected the bottom of the map, got {y}");
    }
}
