//! The arithmetic behind procedural generation: fractal noise, a 1D box filter, and a weighted pick
//! over a list of edges.
//!
//! Three small functions that are easy to get subtly wrong and hard to test where they are used,
//! the thing around them being a whole world.
//!
//! **Not in scope**: what is being generated. Nothing here knows about terrain, biomes, ores
//! or caves - a caller feeds it numbers and decides what they mean. Reproducibility is the
//! caller's too: everything that needs randomness takes an `Rng`, so seeding is decided
//! outside.

pub use procgen::*;

mod procgen;
mod tests;
