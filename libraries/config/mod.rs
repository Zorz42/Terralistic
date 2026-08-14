//! Settings registered at runtime, and a file to keep them in.
//!
//! A `Settings` store hands out a stable handle per registered setting, remembers the value
//! under a config key, and loads and saves the lot as one flat key-to-number file.
//!
//! # Not in scope
//!
//! *Which* settings exist, what they do when they change, and how they are drawn. The store
//! holds the mechanism; the owner holds the list - which is what lets a setting be registered
//! when a world loads and removed when it closes.
//!
//! **A handle is not a row.** Ids come from a counter that never reuses one, so a setting
//! registered and removed repeatedly gets a higher id every time. Anything laying settings
//! out by id drifts; lay them out by position in a sorted list.

pub use settings::*;

mod settings;
mod tests;
