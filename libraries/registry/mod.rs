//! Id-indexed registries of named things: register a value, get a typed handle back, look it
//! up by handle or by name. One registry per kind of thing, kept apart at compile time by the
//! handle type.
//!
//! **Not in scope**: what the entries mean, when they are registered, and removal - handles
//! are never reused, so nothing here can invalidate one already handed out.

pub use registry::*;

mod registry;
mod tests;
