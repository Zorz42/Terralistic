#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::libraries::scripting::{ScriptHost, ScriptModule, ScriptModuleData};
    use crate::libraries::serialization;

    const TEST_PREFIX: &str = "test_";

    fn host(modules: Vec<ScriptModule>) -> ScriptHost {
        ScriptHost::new(modules, TEST_PREFIX)
    }

    fn test_mod(name: &str, lua: &str) -> ScriptModule {
        ScriptModule::new(name.to_owned(), lua.to_owned(), std::collections::HashMap::new())
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
        let mut mods = host(vec![test_mod("test", MINIMAL_LUA)]);

        mods.init().unwrap();
        mods.update().unwrap();
        mods.update().unwrap();
        mods.stop().unwrap();

        let game_mod = mods.get_module(0).unwrap();
        assert!(game_mod.is_symbol_defined("init").unwrap());
        assert!(game_mod.is_symbol_defined("update").unwrap());
        assert!(game_mod.is_symbol_defined("stop").unwrap());
    }

    #[test]
    fn test_mod_name_is_kept() {
        let mut mods = host(vec![test_mod("base_game", MINIMAL_LUA)]);
        mods.init().unwrap();
        assert_eq!(mods.get_module(0).unwrap().get_name(), "base_game");
    }

    #[test]
    fn test_undefined_symbol_is_reported() {
        let mut mods = host(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        let game_mod = mods.get_module(0).unwrap();
        assert!(!game_mod.is_symbol_defined("not_a_real_function").unwrap());
    }

    #[test]
    fn test_get_all_symbols_includes_defined_functions() {
        let mut mods = host(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        let symbols = mods.get_module(0).unwrap().get_all_symbols();
        assert!(symbols.iter().any(|s| s == "init"));
        assert!(symbols.iter().any(|s| s == "update"));
    }

    /// Host functions are exposed to lua with the host's prefix added automatically, which
    /// is what keeps them out of the way of a module's own names.
    #[test]
    fn test_global_functions_get_the_hosts_prefix() {
        let lua = "
            function init() end
            function update() end
            function stop() end
            function call_it() return test_double(21) end
        ";
        let mut mods = host(vec![test_mod("test", lua)]);
        mods.add_global_function("double", |_, value: i32| Ok(value * 2)).unwrap();
        mods.init().unwrap();

        let result: i32 = mods.get_module(0).unwrap().call_function("call_it", ()).unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_calling_a_missing_function_is_an_error() {
        let mut mods = host(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        mods.get_module(0).unwrap().call_function::<(), ()>("nope", ()).unwrap_err();
    }

    #[test]
    fn test_broken_lua_fails_to_init() {
        let mut mods = host(vec![test_mod("test", "this is not lua ((")]);
        mods.init().unwrap_err();
    }

    #[test]
    fn test_get_mod_out_of_range() {
        let mut mods = host(vec![test_mod("test", MINIMAL_LUA)]);
        mods.init().unwrap();

        assert!(mods.get_module(5).is_none());
        assert!(mods.get_module(-1).is_none());
    }

    #[test]
    fn test_resources_are_looked_up_across_mods() {
        let mut a = std::collections::HashMap::new();
        a.insert("blocks:dirt.opa".to_owned(), vec![1, 2, 3]);

        let mods = host(vec![ScriptModule::new("a".to_owned(), MINIMAL_LUA.to_owned(), a)]);

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

        let mods = host(vec![
            ScriptModule::new("first".to_owned(), MINIMAL_LUA.to_owned(), first),
            ScriptModule::new("second".to_owned(), MINIMAL_LUA.to_owned(), second),
        ]);

        assert_eq!(mods.get_resource("misc:icon.opa"), Some(&vec![2]));
    }

    #[test]
    fn test_mods_iter_sees_every_mod() {
        let mods = host(vec![test_mod("a", MINIMAL_LUA), test_mod("b", MINIMAL_LUA)]);
        let names: Vec<&str> = mods.modules_iter().map(ScriptModule::get_name).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    /// A `ScriptModule` serializes through `ScriptModuleData`, which is the on disk `.mod` format.
    #[test]
    fn test_game_mod_serialize_round_trip() {
        let mut resources = std::collections::HashMap::new();
        resources.insert("blocks:dirt.opa".to_owned(), vec![9, 8, 7]);
        let game_mod = ScriptModule::new("round_trip".to_owned(), MINIMAL_LUA.to_owned(), resources);

        let bytes = serialization::serialize(&game_mod).unwrap();
        let restored: ScriptModule = serialization::deserialize(&bytes).unwrap();

        assert_eq!(restored.get_name(), "round_trip");

        let mods = host(vec![restored]);
        assert_eq!(mods.get_resource("blocks:dirt.opa"), Some(&vec![9, 8, 7]));
    }

    /// The build script writes `ScriptModuleData` directly, so it has to produce the same bytes
    /// a `ScriptModule` would.
    #[test]
    fn test_game_mod_data_matches_game_mod_bytes() {
        let mut resources = std::collections::HashMap::new();
        resources.insert("a:b.opa".to_owned(), vec![1, 2]);
        let game_mod = ScriptModule::new("same".to_owned(), "x = 1".to_owned(), resources.clone());

        let mut ordered = BTreeMap::new();
        for (key, value) in resources {
            ordered.insert(key, value);
        }
        let data = ScriptModuleData {
            name: "same".to_owned(),
            source: "x = 1".to_owned(),
            resources: ordered,
        };

        assert_eq!(serialization::serialize(&game_mod).unwrap(), serialization::serialize(&data).unwrap());
    }
}
