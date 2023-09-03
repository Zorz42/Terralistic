use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::build_project::png_to_opa::png_file_to_opa_bytes;
use darklua_core::generator::{DenseLuaGenerator, LuaGenerator};
use darklua_core::Parser;

use crate::libraries::graphics as gfx;
use crate::shared::mod_manager::GameMod;

/// This function compiles a game mod from a directory.
/// It takes the path to the directory as input.
pub fn compile_mod(mod_path: PathBuf) {
    // make sure that cargo reruns this script if the input mod changes (or any of its files)
    println!("cargo:rerun-if-changed={}", mod_path.to_str().unwrap());
    for entry in std::fs::read_dir(mod_path.clone()).unwrap() {
        let path = entry.unwrap().path();
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    let mut lua_code = String::new();
    // iterate through all files in the mod directory
    // if the file ends with .lua then add it to the lua code
    for entry in std::fs::read_dir(mod_path.clone()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().unwrap_or(OsStr::new("")) == "lua" {
            lua_code.push_str(&std::fs::read_to_string(path).unwrap());
            lua_code.push('\n');
        }
    }

    //Use darklua to minify the mod's lua code. Use DenseLuaGenerator to generate the minified code.
    let parser = Parser::default();
    let block = std::thread::Builder::new()
        .stack_size(100_000_000)
        .spawn(move || parser.parse(&lua_code))
        .unwrap()
        .join()
        .unwrap()
        .unwrap();

    let mut generator = DenseLuaGenerator::default();
    generator.write_block(&block);

    let minified_lua_code = generator.into_string();
    let resources = generate_resources(mod_path.join("resources"), String::new());
    let mod_obj = GameMod::new(minified_lua_code, resources);

    // serialize the mod to a byte array
    let mod_bytes = bincode::serialize(&mod_obj).unwrap();
    // compress the mod with snap
    let mod_bytes = snap::raw::Encoder::new().compress_vec(&mod_bytes).unwrap();

    // write the mod to a file that has the same name as the mod's directory and a .mod extension
    std::fs::write(
        mod_path.join(format!(
            "{}.mod",
            mod_path.file_name().unwrap().to_str().unwrap()
        )),
        mod_bytes,
    )
    .unwrap();
}

/// This function takes the resources folder, goes through all of the recursively,
/// changes the file paths to use : instead of / and adds the files to a map.
fn generate_resources(resources_path: PathBuf, prefix: String) -> HashMap<String, Vec<u8>> {
    println!(
        "Generating resource pack... {}",
        resources_path.to_str().unwrap()
    );

    let mut resources = HashMap::new();

    for entry in std::fs::read_dir(resources_path).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            resources.extend(generate_resources(
                path.clone(),
                format!("{}{}:", prefix, path.file_name().unwrap().to_str().unwrap()),
            ));
        } else {
            let (file_name, data) = process_file(path);
            resources.insert(format!("{prefix}{file_name}"), data);
        }
    }

    resources
}

/// This function processes a file in the resources folder.
/// It takes the path to the file as input. It returns the
/// new file name and the file contents.
fn process_file(file_path: PathBuf) -> (String, Vec<u8>) {
    let mut file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    // if file name has .png extension, change it to .opa extension
    if file_name.ends_with(".png") {
        file_name = file_name.replace(".png", ".opa");

        // convert the png to opa
        let mut data = png_file_to_opa_bytes(file_path);

        // if the file name starts with Template_, then process it as a template and remove the Template_ prefix
        if file_name.starts_with("Template_") {
            file_name = file_name.replace("Template_", "");
            data = process_template(data);
        }

        (file_name, data)
    } else {
        let data = std::fs::read(file_path).unwrap();
        (file_name, data)
    }
}

fn process_template(data: Vec<u8>) -> Vec<u8> {
    let surface = gfx::Surface::deserialize_from_bytes(&data).unwrap();
    let mut new_surface = gfx::Surface::new(gfx::IntSize(8, 8 * 16));

    // first take first 8x8 area from surface and copy it to 16 times in the new surface
    for step in 0..16 {
        for y in 0..8 {
            for x in 0..8 {
                *new_surface
                    .get_pixel_mut(gfx::IntPos(x, y + step * 8))
                    .unwrap() = *surface.get_pixel(gfx::IntPos(x, y)).unwrap();
            }
        }
    }

    /*
    take the second 8x8 area, which is below the
    first 8x8 area and copy it to the every second
    8x8 area in the new surface, starting from the
    first one and then third, fifth, etc. Then do the
    same for the third 8x8 area, but copy it to first,
    second and skip the next two and so on.
    */
    let num_textures = surface.get_size().1 / 8 - 1;
    for i in 0..num_textures {
        for step in 0..16 {
            if step & (1 << (i % 4)) == 0 {
                copy_edge(&surface, 0, 8 + 8 * i as i32, &mut new_surface, 0, step * 8);
            }
        }
    }

    for step in 0..16 {
        if step & 8 == 0 && step & 1 == 0 {
            *new_surface.get_pixel_mut(gfx::IntPos(0, step * 8)).unwrap() =
                gfx::Color::new(0, 0, 0, 0);
        }

        if step & 1 == 0 && step & 2 == 0 {
            *new_surface.get_pixel_mut(gfx::IntPos(7, step * 8)).unwrap() =
                gfx::Color::new(0, 0, 0, 0);
        }

        if step & 2 == 0 && step & 4 == 0 {
            *new_surface
                .get_pixel_mut(gfx::IntPos(7, step * 8 + 7))
                .unwrap() = gfx::Color::new(0, 0, 0, 0);
        }

        if step & 4 == 0 && step & 8 == 0 {
            *new_surface
                .get_pixel_mut(gfx::IntPos(0, step * 8 + 7))
                .unwrap() = gfx::Color::new(0, 0, 0, 0);
        }
    }

    new_surface.serialize_to_bytes().unwrap()
}

fn copy_edge(
    source: &gfx::Surface,
    source_x: i32,
    source_y: i32,
    target: &mut gfx::Surface,
    target_x: i32,
    target_y: i32,
) {
    for y in 0..8 {
        for x in 0..8 {
            let pixel = *source
                .get_pixel(gfx::IntPos(source_x + x, source_y + y))
                .unwrap();
            if pixel.a != 0 {
                let applied_pixel = if pixel == gfx::Color::new(0, 255, 0, 255) {
                    gfx::Color::new(0, 0, 0, 0)
                } else {
                    pixel
                };

                *target
                    .get_pixel_mut(gfx::IntPos(target_x + x, target_y + y))
                    .unwrap() = applied_pixel;
            }
        }
    }
}
