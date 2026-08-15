//! Sandboxed script modules.
//!
//! A `ScriptModule` is source, an interpreter state of its own and a bag of named resources; a
//! `ScriptHost` drives a set of them and registers host functions across all of them under one
//! prefix. `ScriptModuleData` is the on-disk shape, kept a dependency-free leaf so a build script
//! can write packages without linking the interpreter.
//!
//! **Not in scope**: everything about what the scripts are *for* - which host functions exist,
//! what the lifecycle hooks mean, and where a package is found. This loads modules, calls
//! named functions, and reports what a module defines.
//!
//! Isolation is per module, not a sandbox: each gets its own globals, but lua's standard
//! library is whatever `rlua` provides. A module is trusted code.

pub use handle::*;
pub use host::*;
pub use module_data::*;

mod handle;
mod host;
mod module_data;
mod tests;
