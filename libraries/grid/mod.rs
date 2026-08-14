//! Bounded 2D grids.
//!
//! A dense grid of cells that is always checked against its own size, a partition of one
//! into fixed size chunks, and a tracker for evicting the least recently touched of
//! something indexed. Between them they answer "where is cell (x, y) in this `Vec`", which
//! is a question that is easy to answer four slightly different ways in four places.
//!
//! # Not in scope
//!
//! What a cell *means*, and what should happen when one changes. Nothing here emits events,
//! knows about neighbours, or has an opinion on what an empty cell is - a caller that wants
//! cells meaning "nothing here yet" passes that value to `Grid::filled`. Chunk *loading* and
//! whatever a chunk holds belong to the caller too; `Chunks` is addressing only.

pub use chunk_tracker::*;
pub use chunks::*;
pub use grid::*;

mod chunk_tracker;
mod chunks;
mod grid;
mod tests;
