use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};

pub const CHUNK_SIZE: i32 = 16;

/// `WorldMap` contains the width and height of the map and is used
/// for everything that needs to know the size of the map.
#[derive(Serialize, Deserialize)]
pub struct WorldMap {
    width: u32,
    height: u32,
}

impl WorldMap {
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    #[must_use]
    pub const fn new_empty() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    #[must_use]
    pub const fn get_width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn get_height(&self) -> u32 {
        self.height
    }

    /// Translates a x y coordinate to a single number.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            bail!("Coordinates are out of bounds! x: {}, y: {}", x, y);
        }

        Ok((x * self.height as i32 + y) as usize)
    }

    /// Same as `translate_coords` but for chunks
    pub fn translate_chunk_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0
            || y < 0
            || x >= self.width as i32 / CHUNK_SIZE
            || y >= self.height as i32 / CHUNK_SIZE
        {
            bail!("Coordinates are out of bounds! x: {}, y: {}", x, y);
        }

        Ok((x + y * (self.width as i32 / CHUNK_SIZE)) as usize)
    }
}
