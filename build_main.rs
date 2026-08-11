#![allow(clippy::all)]

use crate::build_project::compile_mod::compile_mod;
use crate::build_project::compile_resource_pack::compile_resource_pack;
#[cfg(target_os = "windows")]
use winres::WindowsResource;

pub mod build_project {
    pub mod compile_mod;
    pub mod compile_resource_pack;
    pub mod png_to_opa;
}

// The build script only needs a handful of plain data types: Surface/Color/IntPos/IntSize
// to convert PNGs to .opa, and GameModData to write .mod files.
//
// Declaring the real `libraries::graphics` and `shared` modules here would compile their
// whole module trees into the build script, which is what dragged SDL2, OpenGL, rustls,
// message-io, rlua and friends into [build-dependencies] - all of them then built twice,
// once for the host and once for the target. So we name the individual leaf files instead.
//
// The module paths have to match main.rs, because these files refer to themselves through
// `crate::libraries::graphics` and `crate::shared`.
pub mod libraries {
    pub mod graphics {
        mod color;
        mod position;
        mod surface;

        pub use color::*;
        pub use position::*;
        pub use surface::*;
    }
}

pub mod shared {
    pub mod mod_data;
}

fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    #[cfg(target_os = "windows")]
    {
        WindowsResource::new().set_icon("resources/icon.ico").compile().unwrap();
    }

    // compile resource pack resources into Build/resources
    compile_resource_pack(std::path::PathBuf::from("resources"), std::path::PathBuf::from("Build/Resources"));

    // compile mod base_game
    compile_mod(std::path::PathBuf::from("base_game"));
}
