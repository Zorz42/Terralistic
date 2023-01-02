use std::collections::HashMap;
use std::path::PathBuf;
use darklua_core::generator::{DenseLuaGenerator, LuaGenerator};
use darklua_core::Parser;
use shared::mod_manager::{GameMod, GameModData};
use crate::png_to_opa::png_file_to_opa_bytes;

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

    let mod_obj = GameMod::new(GameModData::new(minified_lua_code, generate_resources(mod_path.join("Resources"), "".to_string())));

    // write the mod to a file that has the same name as the mod's directory and a .mod extension
    std::fs::write(mod_path.join(format!("{}.mod", mod_path.file_name().unwrap().to_str().unwrap())), mod_obj.to_bytes()).unwrap();
}

/**
This function takes the Resources folder, goes through all of the recursively,
changes the file paths to use : instead of / and adds the files to a map.
 */
fn generate_resources(resources_path: PathBuf, prefix: String) -> HashMap<String, Vec<u8>> {
    let mut resources = HashMap::new();

    for entry in std::fs::read_dir(resources_path).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            resources.extend(generate_resources(path.clone(), format!("{}{}:", prefix, path.file_name().unwrap().to_str().unwrap())));
        } else {
            let (file_name, data) = process_file(path);
            resources.insert(format!("{}{}", prefix, file_name), data);
        }
    }

    resources
}

/**
This function processes a file in the Resources folder.
It takes the path to the file as input. It returns the
new file name and the file contents.
 */

fn process_file(file_path: PathBuf) -> (String, Vec<u8>) {
    let mut file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    // if file name has .png extension, change it to .opa extension
    if file_name.ends_with(".png") {
        file_name = file_name.replace(".png", ".opa");

        // convert the png to opa
        let data = png_file_to_opa_bytes(file_path);
        (file_name, data)
    } else {
        let data = std::fs::read(file_path).unwrap();
        (file_name, data)
    }
}