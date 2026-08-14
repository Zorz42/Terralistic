//! Sandboxed script modules.
//!
//! A `ScriptModule` is source, an interpreter state of its own, and a bag of named binary
//! resources; a `ScriptHost` drives a set of them together and registers host functions
//! across all of them under one prefix. `ScriptModuleData` is what a module looks like on
//! disk, and is a dependency-free leaf so a build script can write packages without linking
//! the interpreter.
//!
//! # Not in scope
//!
//! Everything about what the scripts are *for*. Which host functions exist, what the
//! lifecycle hooks mean, what a symbol named by convention implies, and how a package is
//! compressed or where it is found are all the owner's. This layer loads modules, calls
//! named functions, and reports what a module defines.
//!
//! Isolation is per module, not a sandbox: each module gets its own globals, but lua's own
//! standard library is whatever `rlua` provides. A module is trusted code.

pub use host::*;
pub use module_data::*;

mod host;
mod module_data;
mod tests;
