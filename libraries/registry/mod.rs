//! Id-indexed registries of named things.
//!
//! Register a value, get a typed handle back; look it up by handle or by name; iterate
//! every handle. One registry per kind of thing, with the handle type keeping them apart at
//! compile time.
//!
//! # Not in scope
//!
//! What the entries mean, when they are registered, and whether the same set is registered
//! twice. Removal, too: handles are never reused, so nothing here can invalidate one that
//! has already been handed out.

pub use registry::*;

mod registry;
mod tests;
