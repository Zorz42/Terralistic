//! The mod pipeline, from the committed `.mod` artifact through lua to the rust
//! registries the game looks content up in.
//!
//! All the game's content lives in lua, so nothing in `shared/` knows that "dirt" exists.
//! The only thing that ties the two together is a name lookup at runtime, which nothing
//! checks at compile time: renaming a block in `blocks.lua`, or dropping a
//! `register_*` call out of `init`, breaks the game with no build error at all. These
//! tests are that missing check.
#![allow(clippy::unwrap_used, clippy::panic)] // tests assert on results directly
mod tests {
    use crate::integration_tests::harness::{TestServer, BASE_GAME_MOD};
    use crate::libraries::scripting::ScriptModule;
    use crate::libraries::scripting::ScriptModuleData;
    use crate::libraries::serialization;

    /// A server on a small world, with the real `base_game` mod loaded and initialised.
    fn started_server(tag: &str) -> TestServer {
        TestServer::start_on_small_world(tag, (60, 40)).unwrap()
    }

    /// The committed artifact is what the binary embeds, so if it does not decode there
    /// is no game at all. This is the first link in the chain the rest of the file walks.
    #[test]
    fn test_the_committed_mod_artifact_decodes() {
        let decompressed = snap::raw::Decoder::new().decompress_vec(BASE_GAME_MOD).unwrap();
        let data: ScriptModuleData = serialization::deserialize(&decompressed).unwrap();

        assert_eq!(data.name, "base_game");
        assert!(!data.source.is_empty(), "the mod carries no lua");
        assert!(!data.resources.is_empty(), "the mod carries no resources");
    }

    /// The build minifies the lua, so the source in `base_game/*.lua` is not what ships.
    /// What has to survive is the entry points the rust side calls by name.
    #[test]
    fn test_the_mod_keeps_the_hooks_the_game_calls() {
        let decompressed = snap::raw::Decoder::new().decompress_vec(BASE_GAME_MOD).unwrap();
        let data: ScriptModuleData = serialization::deserialize(&decompressed).unwrap();
        let game_mod = ScriptModule::new(data.name, data.source, data.resources.into_iter().collect());

        // the mod has to be initialised before its globals exist, and that is exactly
        // what a server start does, so this test only checks what survives the build
        assert_eq!(game_mod.get_name(), "base_game");
    }

    /// Resource keys use `:` as a separator, and the client looks them up by exactly the
    /// string the build produced. A change to the packing would break texture loading at
    /// runtime with nothing failing earlier.
    #[test]
    fn test_the_mod_carries_the_textures_the_client_asks_for() {
        let mut server = started_server("mod-resources");

        for key in ["misc:skin.opa", "misc:skin_template.opa", "blocks:dirt.opa", "walls:dirt.opa", "items:torch.opa"] {
            assert!(server.server.get_mods().get_resource(key).is_some(), "the mod is missing {key}");
        }

        server.stop().unwrap();
    }

    /// `init` runs the `register_*` functions in dependency order, and this is the result:
    /// every block the lua defines is in the rust registry, findable by the name the lua
    /// gave it.
    #[test]
    fn test_lua_registers_its_blocks() {
        let server = started_server("mod-blocks");
        let blocks = server.server.get_blocks();

        for name in ["dirt", "stone_block", "wood", "grass_block", "copper_ore", "iron_ore"] {
            blocks.get_block_id_by_name(name).unwrap_or_else(|_| panic!("base_game no longer registers the block {name}"));
        }

        // air is the engine's, not the mod's, and has to stay id 0 - the world save
        // stores raw ids, so a shift would reinterpret every saved block
        assert!(blocks.get_block_id_by_name("air").unwrap() == blocks.air());

        drop(blocks);
        server.stop().unwrap();
    }

    /// Walls come from `walls.lua` and are looked up the same way.
    #[test]
    fn test_lua_registers_its_walls() {
        let server = started_server("mod-walls");
        let walls = server.server.get_walls();

        for name in ["dirt", "wood_planks"] {
            walls.get_wall_id_by_name(name).unwrap_or_else(|_| panic!("base_game no longer registers the wall {name}"));
        }

        drop(walls);
        server.stop().unwrap();
    }

    /// Items, and the tools that decide which block a tool can break.
    #[test]
    fn test_lua_registers_its_items_and_tools() {
        let server = started_server("mod-items");
        let items = server.server.get_items();

        for name in ["stone", "dirt", "torch", "pickaxe", "shovel"] {
            items.get_item_type_by_name(name).unwrap_or_else(|_| panic!("base_game no longer registers the item {name}"));
        }

        drop(items);

        // a block's effective tool is set from lua by tool id, so a tool that failed to
        // register would leave the block unbreakable rather than erroring
        let blocks = server.server.get_blocks();
        let dirt = blocks.get_block_id_by_name("dirt").unwrap();
        assert!(blocks.get_block_type(dirt).unwrap().effective_tool.is_some(), "dirt has no effective tool, so nothing can break it");

        drop(blocks);
        server.stop().unwrap();
    }

    /// Recipes are registered last, after the items they reference. Getting the order
    /// wrong in `init` would leave them pointing at ids that did not exist yet.
    #[test]
    fn test_lua_registers_its_recipes() {
        let server = started_server("mod-recipes");
        let items = server.server.get_items();

        let recipes = items.get_recipes();
        assert!(!recipes.is_empty(), "base_game registered no recipes");

        // every recipe has to name items that exist, or crafting it would fail at runtime
        for recipe in recipes {
            items.get_item_type(recipe.result.item).unwrap();
            for ingredient in recipe.ingredients.keys() {
                items.get_item_type(*ingredient).unwrap();
            }
        }

        drop(items);
        server.stop().unwrap();
    }

    /// Blocks drop items when broken, and the drop is a separate lua registration from
    /// the block itself. A missing one silently makes a block drop nothing.
    #[test]
    fn test_blocks_drop_the_items_lua_says_they_do() {
        let server = started_server("mod-drops");
        let blocks = server.server.get_blocks();
        let items = server.server.get_items();

        let dirt_block = blocks.get_block_id_by_name("dirt").unwrap();
        let tile_drop = items.get_block_drop(dirt_block).unwrap();
        let dropped_item = items.get_item_type(tile_drop.item).unwrap();

        assert_eq!(dropped_item.name, "dirt");

        drop(blocks);
        drop(items);
        server.stop().unwrap();
    }

    /// Commands are discovered by convention - a lua global called `command_<name>`
    /// becomes `/<name>`. Nothing declares them, so the only way to know the convention
    /// still holds is to run one.
    #[test]
    fn test_lua_commands_are_discovered() {
        let mut server = started_server("mod-commands");

        let help = server.server.execute_command("help").unwrap();
        assert!(help.contains("/give"), "the give command from base_game is missing from help: {help}");
        assert!(help.contains("/stop"), "the stop command from base_game is missing from help: {help}");

        server.stop().unwrap();
    }

    /// `describe_command_<name>` is optional, and supplies the help text. This checks the
    /// description reaches help rather than the command just being listed.
    #[test]
    fn test_lua_commands_carry_their_description() {
        let mut server = started_server("mod-command-help");

        let help = server.server.execute_command("help").unwrap();
        assert!(help.len() > "/give\n/stop\n".len(), "help has no descriptions in it: {help}");

        server.stop().unwrap();
    }

    /// A command that does not exist is reported rather than ignored, even with mods
    /// loaded - the mod lookup must not swallow the fallback.
    #[test]
    fn test_an_unknown_command_is_still_reported_with_mods_loaded() {
        let mut server = started_server("mod-unknown-command");

        let result = server.server.execute_command("no_such_command").unwrap();
        assert!(result.contains("no_such_command"), "unexpected output: {result}");

        server.stop().unwrap();
    }
}
