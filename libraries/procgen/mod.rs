//! The arithmetic behind procedural generation.
//!
//! Fractal noise, a 1D box filter for smoothing a per-column array, and a weighted pick over
//! a list of edges. Three small functions that are easy to get subtly wrong and hard to test
//! where they are used, since the thing around them is a whole world.
//!
//! # Not in scope
//!
//! What is being generated, and in what order. Nothing here knows about terrain, biomes,
//! ores or caves; a caller feeds it numbers and decides what they mean.
//!
//! Reproducibility is the caller's too: everything here that needs randomness takes an `Rng`,
//! so seeding - and therefore whether the same seed gives the same world - is decided
//! outside.

pub use procgen::*;

mod procgen;
mod tests;
