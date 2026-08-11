use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

/// The serialized shape of a compiled game mod: what a `.mod` file actually contains,
/// once snap decompressed.
///
/// This lives in its own leaf module, depending on nothing but serde, so the build
/// script can write `.mod` files without pulling in `mod_manager`. `GameMod` owns a
/// `rlua::Lua`, and reaching it from the build script dragged Lua - and with it the rest
/// of the game's dependency tree - into `[build-dependencies]`.
///
/// `GameMod` serializes and deserializes through this type, so the format is defined in
/// exactly one place and the two sides cannot drift apart.
#[derive(Serialize, Deserialize)]
pub struct GameModData {
    pub name: String,
    pub lua_code: String,
    /// Ordered on purpose. `GameMod` keeps resources in a `HashMap` for lookups, but a
    /// `HashMap` serializes in iteration order, which Rust randomises per process - so
    /// building the same sources twice produced different `.mod` bytes every time. Since
    /// `base_game.mod` is a committed artifact, that meant it churned on every build.
    pub resources: BTreeMap<String, Vec<u8>>,
}
