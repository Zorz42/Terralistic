use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

/// A compiled script module as it is stored: what a package file contains, once decompressed.
///
/// A leaf module depending on nothing but serde, so a build script can *write* packages
/// without the interpreter - `ScriptModule` owns an `rlua::Lua`, and naming it from a build
/// script dragged Lua into `[build-dependencies]`. `ScriptModule` serializes through this
/// type, so the format is defined once and the two sides cannot drift apart.
#[derive(Serialize, Deserialize)]
pub struct ScriptModuleData {
    pub name: String,
    pub source: String,
    /// Ordered on purpose: `ScriptModule` looks resources up in a `HashMap`, which serializes
    /// in an order Rust randomises per process - so building the same sources twice produced
    /// different bytes, and the committed package churned on every build.
    pub resources: BTreeMap<String, Vec<u8>>,
}
