use crate::compile_mod::compile_mod;
use crate::compile_resource_pack::compile_resource_pack;

mod compile_resource_pack;
mod png_to_opa;
mod compile_mod;

fn main() {
    // compile resource pack Resources into Build/Resources
    compile_resource_pack(
        std::path::PathBuf::from("../../Resources"),
        std::path::PathBuf::from("../../Build/Resources"),
    );

    // compile mod BaseGame
    compile_mod(std::path::PathBuf::from("../../BaseGame"));
}