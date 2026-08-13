use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

use crate::libraries::graphics as gfx;

use super::Color;
use crate::libraries::serialization;

/// An image stored in RAM.
#[derive(Serialize, Deserialize, Clone)]
pub struct Surface {
    pub(super) pixels: Vec<Color>,
    size: gfx::IntSize,
}

impl Surface {
    /// Creates a new surface with all transparent pixels.
    ///
    /// The pixel count is computed in `usize` rather than in `u32`, because everything
    /// downstream trusts that it matches `size` - see `deserialize_from_bytes` - and a `u32`
    /// product that wrapped would hand out a surface smaller than it claims to be.
    #[must_use]
    pub fn new(size: gfx::IntSize) -> Self {
        Self {
            pixels: std::vec![Color::new(0, 0, 0, 0); size.0 as usize * size.1 as usize],
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
    ///
    /// **The pixel count is checked against the declared size**, because everything downstream
    /// takes `get_size` at its word. `GpuDevice::create_texture` in particular tells wgpu the
    /// texture is `size` big and hands it `pixels`; a surface claiming 64x64 while holding four
    /// pixels is a validation error, and wgpu turns that into a panic. Surfaces are read out of
    /// `.mod` files - ordinary files a player can replace - so a well formed one is not
    /// something this is entitled to assume.
    pub fn deserialize_from_bytes(buffer: &[u8]) -> Result<Self> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(buffer)?;
        let surface: Self = serialization::deserialize(&decompressed)?;

        let expected = surface.size.0 as usize * surface.size.1 as usize;
        if surface.pixels.len() != expected {
            bail!("surface is {:?} but holds {} pixels rather than {expected}", surface.size, surface.pixels.len());
        }
        Ok(surface)
    }

    /// Converts a 2D location into an index into the colour array.
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

    /// Retrieves the pixel color on a specified location, mutably.
    pub fn get_pixel_mut(&mut self, pos: gfx::IntPos) -> Result<&mut Color> {
        let index = self.get_index(pos)?;
        self.pixels.get_mut(index).ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    #[must_use]
    pub const fn get_size(&self) -> gfx::IntSize {
        self.size
    }

    /// Copies another surface to the specified location, multiplied by `color`.
    pub fn draw(&mut self, pos: gfx::IntPos, surface: &Self, color: Color) -> Result<()> {
        let tint = |channel: u8, by: u8| (channel as f32 * (by as f32 / 255.0)) as u8;
        for (source_pos, source) in surface.iter() {
            *self.get_pixel_mut(pos + source_pos)? = Color::new(tint(source.r, color.r), tint(source.g, color.g), tint(source.b, color.b), tint(source.a, color.a));
        }

        Ok(())
    }

    /// Every pixel with its position, row by row.
    ///
    /// Walks the backing slice and derives the position from the index rather than the other
    /// way round, which is the same order and skips a bounds check and a `Result` per pixel.
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
