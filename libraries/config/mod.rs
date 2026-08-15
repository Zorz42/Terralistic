//! Settings registered at runtime, and a file to keep them in: a stable handle per setting, a
//! value remembered under a config key, and one flat key-to-number file for the lot.
//!
//! **Not in scope**: *which* settings exist, what they do when they change, and how they are
//! drawn - the owner holds the list, which is what lets a setting be registered when a world
//! loads and removed when it closes.
//!
//! **A handle is not a row.** Ids come from a counter that never reuses one, so a setting
//! registered and removed repeatedly gets a higher id each time. Lay settings out by position
//! in a sorted list, not by id.

pub use settings::*;

mod settings;
mod tests;
