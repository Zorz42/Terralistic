use std::collections::BTreeMap;

use serde_derive::{Deserialize, Serialize};

/// The serialized shape of a compiled script module: what a package file actually contains,
/// once decompressed.
///
/// This lives in its own leaf module, depending on nothing but serde, so that a build script
/// can *write* packages without pulling in the interpreter. `ScriptModule` owns an
/// `rlua::Lua`, and reaching it from a build script dragged Lua - and with it the rest of the
/// dependency tree - into `[build-dependencies]`.
///
/// `ScriptModule` serializes and deserializes through this type, so the format is defined in
/// exactly one place and the two sides cannot drift apart.
#[derive(Serialize, Deserialize)]
pub struct ScriptModuleData {
    pub name: String,
    pub source: String,
    /// Ordered on purpose. `ScriptModule` keeps resources in a `HashMap` for lookups, but a
    /// `HashMap` serializes in iteration order, which Rust randomises per process - so
    /// building the same sources twice produced different package bytes every time. When the
    /// package is a committed build artifact, that means it churns on every build.
    pub resources: BTreeMap<String, Vec<u8>>,
}
