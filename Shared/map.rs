use anyhow::{anyhow, Result};

/**
Map contains the width and height of the map and is used
for everything that needs to know the size of the map.
 */
pub struct Map {
    width: i32,
    height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    /**
    Translates a x y coordinate to a single number.
     */
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<i32> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(anyhow!("Coordinates are out of bounds! x: {}, y: {}", x, y));
        }

        Ok(x + y * self.width)
    }
}