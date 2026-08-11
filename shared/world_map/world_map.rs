use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};

pub const CHUNK_SIZE: i32 = 16;

/// `WorldMap` contains the width and height of the map and is used
/// for everything that needs to know the size of the map.
#[derive(Serialize, Deserialize)]
pub struct WorldMap {
    size: (u32, u32),
}

impl WorldMap {
    #[must_use]
    pub const fn new(size: (u32, u32)) -> Self {
        Self { size }
    }

    #[must_use]
    pub const fn new_empty() -> Self {
        Self::new((0, 0))
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Translates a x y coordinate to a single number.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.size.0 as i32 || y >= self.size.1 as i32 {
            bail!("Coordinates are out of bounds! x: {x}, y: {y}");
        }

        Ok((x * self.size.1 as i32 + y) as usize)
    }

    /// Same as `translate_coords` but for chunks
    pub fn translate_chunk_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.size.0 as i32 / CHUNK_SIZE || y >= self.size.1 as i32 / CHUNK_SIZE {
            bail!("Coordinates are out of bounds! x: {x}, y: {y}");
        }

        Ok((x + y * (self.size.0 as i32 / CHUNK_SIZE)) as usize)
    }
}
