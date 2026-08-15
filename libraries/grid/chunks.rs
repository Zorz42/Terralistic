use anyhow::{bail, Result};

/// A partition of a grid into fixed size square chunks: the addressing only.
///
/// What a chunk *holds* is the caller's, which is why this is a type of its own - the three things
/// that partition a grid all store something different per chunk.
///
/// Chunk indices are **row major** (`index = x + y * chunks_wide`) where `Grid`'s cells are
/// column major. Both are internally consistent, and swapping either to match reinterprets
/// every index computed with the old one, so don't "fix" one in isolation.
///
/// A grid that is not a whole number of chunks **truncates**: the partial edge chunk is not
/// addressable, and no chunk index ever refers to a cell outside the grid.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Chunks {
    /// Size of the partitioned grid, in cells.
    grid_size: (u32, u32),
    /// Width and height of one chunk, in cells. Always at least 1.
    chunk_size: i32,
}

impl Chunks {
    /// Partitions a grid of `grid_size` cells into chunks `chunk_size` across. Below 1 is
    /// raised to 1 rather than rejected: every operation here divides by it.
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
        // Via `get_size`, so both divisions truncate before the multiply: written out inline
        // it reads as `((w / chunk) * h) / chunk`, which is a different number.
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
