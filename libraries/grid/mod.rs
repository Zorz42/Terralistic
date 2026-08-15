//! Bounded 2D grids: a dense grid checked against its own size, a partition of one into fixed size
//! chunks, and a tracker for evicting the least recently touched.
//!
//! Between them they answer "where is cell (x, y) in this `Vec`" - a question easy to answer four
//! ways in four places.
//!
//! **Not in scope**: what a cell *means*. Nothing here emits events, knows about neighbours,
//! or has an opinion on what empty is - that value is passed to `Grid::filled`. Chunk loading
//! and whatever a chunk holds are the caller's too; `Chunks` is addressing only.

pub use chunk_tracker::*;
pub use chunks::*;
pub use grid::*;

mod chunk_tracker;
mod chunks;
mod grid;
mod tests;
