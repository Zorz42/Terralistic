use crate::build_project::compile_mod::compile_mod;
use crate::build_project::compile_resource_pack::compile_resource_pack;

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
    // compile resource pack resources into Build/resources
    compile_resource_pack(
        std::path::PathBuf::from("resources"),
        std::path::PathBuf::from("Build/Resources"),
    );

    // compile mod base_game
    compile_mod(std::path::PathBuf::from("base_game"));
}
