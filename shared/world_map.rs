use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};

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
    /// # Errors
    /// Returns an error if the coordinates are out of bounds.
    pub fn translate_coords(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            bail!("Coordinates are out of bounds! x: {}, y: {}", x, y);
        }

        Ok((x * self.height as i32 + y) as usize)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_translate_coords() {
        let map = WorldMap::new(10, 10);

        assert_eq!(map.translate_coords(0, 0).unwrap(), 0);
        assert_eq!(map.translate_coords(9, 9).unwrap(), 99);
        assert_eq!(map.translate_coords(5, 5).unwrap(), 55);
        map.translate_coords(10, 10).unwrap_err();
        map.translate_coords(-1, -1).unwrap_err();
        map.translate_coords(0, -1).unwrap_err();
        map.translate_coords(-1, 0).unwrap_err();
        map.translate_coords(10, 0).unwrap_err();
        map.translate_coords(0, 10).unwrap_err();
        map.translate_coords(1234, 1234).unwrap_err();
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_new_empty() {
        let map = WorldMap::new_empty();
        assert_eq!(map.get_width(), 0);
        assert_eq!(map.get_height(), 0);
        map.translate_coords(0, 0).unwrap_err();
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_deserialize_serialize() {
        let map = WorldMap::new(10, 10);
        let serialized = serde_json::to_string(&map).unwrap();
        let deserialized: WorldMap = serde_json::from_str(&serialized).unwrap();
        assert_eq!(map.get_width(), deserialized.get_width());
        assert_eq!(map.get_height(), deserialized.get_height());
    }
}
