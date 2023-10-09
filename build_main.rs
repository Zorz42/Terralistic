#![allow(clippy::all)]

use crate::build_project::compile_mod::compile_mod;
use crate::build_project::compile_resource_pack::compile_resource_pack;
use winres::WindowsResource;

pub mod build_project {
    pub mod compile_mod;
    pub mod compile_resource_pack;
    pub mod png_to_opa;
}

pub mod libraries {
    pub mod events;
    pub mod graphics;
}

pub mod shared;

fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    #[cfg(target_os = "windows")]
    {
        WindowsResource::new()
            .set_icon("resources/icon.ico")
            .compile()
            .unwrap();
    }

    // compile resource pack resources into Build/resources
    compile_resource_pack(
        std::path::PathBuf::from("resources"),
        std::path::PathBuf::from("Build/Resources"),
    );

    // compile mod base_game
    compile_mod(std::path::PathBuf::from("base_game"));
}
