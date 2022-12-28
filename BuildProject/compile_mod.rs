use std::collections::HashMap;
use std::path::PathBuf;
use darklua_core::generator::{DenseLuaGenerator, LuaGenerator};
use darklua_core::Parser;
use shared::mod_manager::{GameMod, GameModData};

/**
This function compiles a game mod from a directory.
It takes the path to the directory as input.
 */
pub fn compile_mod(mod_path: PathBuf) {
    // make sure that cargo reruns this script if the input mod changes (or any of its files)
    println!("cargo:rerun-if-changed={}", mod_path.to_str().unwrap());
    for entry in std::fs::read_dir(mod_path.clone()).unwrap() {
        let path = entry.unwrap().path();
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    // read the mod's lua code
    let lua_code = std::fs::read_to_string(mod_path.join("mod.lua")).unwrap();


    //Use darklua to minify the mod's lua code. Use DenseLuaGenerator to generate the minified code.
    let parser = Parser::default();
    let block = parser.parse(&lua_code).unwrap();

    let mut generator = DenseLuaGenerator::default();
    generator.write_block(&block);

    let minified_lua_code = generator.into_string();

    let mod_obj = GameMod::new(GameModData::new(minified_lua_code, HashMap::new()));

    // write the mod to a file that has the same name as the mod's directory and a .mod extension
    std::fs::write(mod_path.join(format!("{}.mod", mod_path.file_name().unwrap().to_str().unwrap())), mod_obj.to_bytes()).unwrap();
}