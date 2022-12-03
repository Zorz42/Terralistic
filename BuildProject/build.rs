use crate::compile_resource_pack::compile_resource_pack;

mod compile_resource_pack;
mod png_to_opa;
mod create_texture;

fn main() {
    // compile resource pack Resources into Build/Resources
    compile_resource_pack(String::from("Resources"), String::from("Build/Resources"));
}