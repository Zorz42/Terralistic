use anyhow::{bail, Result};

/// A partition of a grid into fixed size square chunks.
///
/// This is the addressing only - which chunk a cell is in, how many chunks there are, and
/// what index a chunk has. What a chunk *holds* is the caller's, which is why this is a
/// separate type from `Grid` rather than a method on it: the three things that partition a
/// grid here all store something different per chunk.
///
/// # Layout
///
/// Chunk indices are **row major**: `index = x + y * chunks_wide`. `Grid`'s cells are
/// column major. The two disagree on purpose - both are internally consistent, and
/// swapping either to match the other silently reinterprets every index that was computed
/// with the old one. Don't "fix" one in isolation.
///
/// A grid whose size is not a whole number of chunks **truncates**: the partial chunk along
/// the edge is not addressable, and no chunk index ever refers to a cell outside the grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chunks {
    /// Size of the partitioned grid, in cells.
    grid_size: (u32, u32),
    /// Width and height of one chunk, in cells. Always at least 1.
    chunk_size: i32,
}

impl Chunks {
    /// Partitions a grid of `grid_size` cells into chunks `chunk_size` cells across.
    ///
    /// A `chunk_size` below 1 is raised to 1 rather than rejected: every operation here
    /// divides by it, and a partition of nothing has no useful meaning to return.
    #[must_use]
    pub const fn new(grid_size: (u32, u32), chunk_size: i32) -> Self {
        Self {
            grid_size,
            chunk_size: if chunk_size < 1 { 1 } else { chunk_size },
        }
    }

    #[must_use]
    pub const fn chunk_size(&self) -> i32 {
        self.chunk_size
    }

    /// How many chunks there are across and down. Partial edge chunks are not counted.
    #[must_use]
    pub const fn get_size(&self) -> (i32, i32) {
        (self.grid_size.0 as i32 / self.chunk_size, self.grid_size.1 as i32 / self.chunk_size)
    }

    /// Total number of chunks, which is how big a per-chunk `Vec` has to be.
    #[must_use]
    pub const fn count(&self) -> usize {
        // the parentheses matter: `/` and `*` are left associative, so without them this
        // reads as `((w / chunk) * h) / chunk`, which is a different number whenever the
        // height is not a multiple of the chunk size
        let (width, height) = self.get_size();
        (width * height) as usize
    }

    /// Turns chunk coordinates into an index, or an error if they are outside the grid.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        let (width, height) = self.get_size();
        if x < 0 || y < 0 || x >= width || y >= height {
            bail!("Coordinates are out of bounds! x: {x}, y: {y}");
        }

        Ok((x + y * width) as usize)
    }

    /// Which chunk a cell falls in. Does not check that the cell is inside the grid -
    /// `translate_coords` is what answers that.
    #[must_use]
    pub const fn chunk_at(&self, cell_x: i32, cell_y: i32) -> (i32, i32) {
        (cell_x / self.chunk_size, cell_y / self.chunk_size)
    }
}
