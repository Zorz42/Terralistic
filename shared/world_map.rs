use anyhow::{anyhow, Result};
use serde_derive::{Deserialize, Serialize};

/// `WorldMap` contains the width and height of the map and is used
/// for everything that needs to know the size of the map.
#[derive(Serialize, Deserialize)]
pub struct WorldMap {
    width: i32,
    height: i32,
}

impl WorldMap {
    pub fn new(width: i32, height: i32) -> Result<Self> {
        if width < 0 || height < 0 {
            return Err(anyhow!("Width and height must be positive"));
        }
        Ok(Self { width, height })
    }

    pub const fn new_empty() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }

    pub const fn get_width(&self) -> i32 {
        self.width
    }

    pub const fn get_height(&self) -> i32 {
        self.height
    }

    /// Translates a x y coordinate to a single number.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(anyhow!("Coordinates are out of bounds! x: {}, y: {}", x, y));
        }

        Ok((x * self.height + y) as usize)
    }
}
