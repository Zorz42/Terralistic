use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

use crate::libraries::graphics as gfx;

use super::Color;
use crate::libraries::serialization;

/// Surface is an image stored in ram.
#[derive(Serialize, Deserialize, Clone)]
pub struct Surface {
    pub(super) pixels: Vec<Color>,
    size: gfx::IntSize,
}

impl Surface {
    /// Creates a new surface with all transparent pixels.
    #[must_use]
    pub fn new(size: gfx::IntSize) -> Self {
        Self {
            pixels: std::vec![Color::new(0, 0, 0, 0); (size.0 * size.1) as usize],
            size,
        }
    }

    /// Serializes the surface into a vector of bytes.
    /// It goes through `libraries::serialization` and is compressed with snap.
    pub fn serialize_to_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        serialization::serialize_into(&mut buffer, &self)?;
        Ok(snap::raw::Encoder::new().compress_vec(&buffer)?)
    }

    /// Deserializes a surface from a vector of bytes the same way it was serialized.
    pub fn deserialize_from_bytes(buffer: &[u8]) -> Result<Self> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(buffer)?;
        serialization::deserialize(&decompressed)
    }

    /// Converts 2D location to a linear location in color array.
    /// The index points to the red bit of the color and the next
    /// three to green, blue, alpha.
    fn get_index(&self, pos: gfx::IntPos) -> Result<usize> {
        if pos.0 < 0 || pos.0 >= self.size.0 as i32 || pos.1 < 0 || pos.1 >= self.size.1 as i32 {
            bail!("Pixel out of bounds");
        }

        Ok((pos.1 * self.size.0 as i32 + pos.0) as usize)
    }

    /// Retrieves the pixel color on a specified location.
    pub fn get_pixel(&self, pos: gfx::IntPos) -> Result<&Color> {
        let index = self.get_index(pos)?;
        self.pixels.get(index).ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    /// Retrieves the pixel color on a specified location.
    pub fn get_pixel_mut(&mut self, pos: gfx::IntPos) -> Result<&mut Color> {
        let index = self.get_index(pos)?;
        self.pixels.get_mut(index).ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    #[must_use]
    pub const fn get_size(&self) -> gfx::IntSize {
        self.size
    }

    /// Copies another surface to the specified location.
    pub fn draw(&mut self, pos: gfx::IntPos, surface: &Self, color: Color) -> Result<()> {
        for (pos2, surface_color) in surface.iter() {
            *self.get_pixel_mut(pos + pos2)? = Color {
                r: (surface_color.r as f32 * (color.r as f32 / 255.0)) as u8,
                g: (surface_color.g as f32 * (color.g as f32 / 255.0)) as u8,
                b: (surface_color.b as f32 * (color.b as f32 / 255.0)) as u8,
                a: (surface_color.a as f32 * (color.a as f32 / 255.0)) as u8,
            };
        }

        Ok(())
    }

    /// Every pixel with its position, row by row.
    ///
    /// Walking the backing slice rather than calling `get_pixel` per step: the position is
    /// derived from the index instead of the index from the position, which is the same
    /// order and skips a bounds check and a `Result` for every pixel. The mutable version
    /// used to need `unsafe` to hand out a borrow the compiler could not see was disjoint;
    /// `iter_mut` on the slice already knows that.
    pub fn iter(&self) -> impl Iterator<Item = (gfx::IntPos, &Color)> {
        let width = self.size.0 as i32;
        self.pixels.iter().enumerate().map(move |(index, pixel)| (index_to_pos(index, width), pixel))
    }

    /// Every pixel with its position, row by row, mutably.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (gfx::IntPos, &mut Color)> {
        let width = self.size.0 as i32;
        self.pixels.iter_mut().enumerate().map(move |(index, pixel)| (index_to_pos(index, width), pixel))
    }
}

/// The inverse of `get_index`. A zero width surface has no pixels, so the division is only
/// ever reached with a positive one.
const fn index_to_pos(index: usize, width: i32) -> gfx::IntPos {
    if width <= 0 {
        return gfx::IntPos(0, 0);
    }
    let index = index as i32;
    gfx::IntPos(index % width, index / width)
}
