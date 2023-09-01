use super::Color;
use super::IntPos;
use crate::libraries::graphics::IntSize;
use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

/// Surface is an image stored in ram.
#[derive(Serialize, Deserialize, Clone)]
pub struct Surface {
    pub(super) pixels: Vec<Color>,
    size: IntSize,
}

impl Surface {
    /// Creates a new surface with all transparent pixels.
    #[must_use]
    pub fn new(size: IntSize) -> Self {
        Self {
            pixels: std::vec![Color::new(0, 0, 0, 0); (size.0 * size.1) as usize],
            size,
        }
    }

    /// Serializes the surface into a vector of bytes.
    /// It is serialized with bincode and compressed with snap.
    /// # Errors
    /// Returns an error if the surface cannot be serialized.
    pub fn serialize_to_bytes(&self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        bincode::serialize_into(&mut buffer, &self)?;
        Ok(snap::raw::Encoder::new().compress_vec(&buffer)?)
    }

    /// Deserializes a surface from a vector of bytes the same way it was serialized.
    /// # Errors
    /// Returns an error if the surface cannot be deserialized.
    pub fn deserialize_from_bytes(buffer: &[u8]) -> Result<Self> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(buffer)?;
        Ok(bincode::deserialize(&decompressed)?)
    }

    /// Converts 2D location to a linear location in color array.
    /// The index points to the red bit of the color and the next
    /// three to green, blue, alpha.
    fn get_index(&self, pos: IntPos) -> Result<usize> {
        if pos.0 < 0 || pos.0 >= self.size.0 as i32 || pos.1 < 0 || pos.1 >= self.size.1 as i32 {
            bail!("Pixel out of bounds");
        }

        Ok((pos.1 * self.size.0 as i32 + pos.0) as usize)
    }

    /// Retrieves the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn get_pixel(&self, pos: IntPos) -> Result<&Color> {
        let index = self.get_index(pos)?;
        self.pixels
            .get(index)
            .ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    /// Retrieves the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn get_pixel_mut(&mut self, pos: IntPos) -> Result<&mut Color> {
        let index = self.get_index(pos)?;
        self.pixels
            .get_mut(index)
            .ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    #[must_use]
    pub const fn get_size(&self) -> IntSize {
        self.size
    }

    /// Copies another surface to the specified location.
    /// # Errors
    /// Returns an error if the surface is out of bounds.
    pub fn draw(&mut self, pos: IntPos, surface: &Self, color: Color) -> Result<()> {
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

    /// Returns a surface iterator.
    #[must_use]
    pub const fn iter(&self) -> SurfaceIterator {
        SurfaceIterator::new(self)
    }

    /// Returns a surface iterator.
    #[must_use]
    pub fn iter_mut(&mut self) -> MutSurfaceIterator {
        MutSurfaceIterator::new(self)
    }
}

/// Surface iterator iterates through all pixels in a surface row by row.
pub struct SurfaceIterator<'surface_lifetime> {
    surface: &'surface_lifetime Surface,
    pos: IntPos,
}

impl<'surface_lifetime> SurfaceIterator<'surface_lifetime> {
    pub const fn new(surface: &'surface_lifetime Surface) -> Self {
        Self {
            surface,
            pos: IntPos(0, 0),
        }
    }
}

impl<'surface_lifetime> Iterator for SurfaceIterator<'surface_lifetime> {
    type Item = (IntPos, &'surface_lifetime Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.0 >= self.surface.get_size().0 as i32 {
            self.pos.0 = 0;
            self.pos.1 += 1;
        }

        if self.pos.1 >= self.surface.get_size().1 as i32 {
            return None;
        }

        let result = self.surface.get_pixel(self.pos).ok();
        self.pos.0 += 1;
        if let Some(result) = result {
            Some((self.pos - IntPos(1, 0), result))
        } else {
            None
        }
    }
}

/// Surface iterator iterates through all pixels in a surface row by row.
pub struct MutSurfaceIterator<'surface_lifetime> {
    surface: &'surface_lifetime mut Surface,
    pos: IntPos,
}

impl<'surface_lifetime> MutSurfaceIterator<'surface_lifetime> {
    pub fn new(surface: &'surface_lifetime mut Surface) -> Self {
        Self {
            surface,
            pos: IntPos(0, 0),
        }
    }
}

impl<'surface_lifetime> Iterator for MutSurfaceIterator<'surface_lifetime> {
    type Item = (IntPos, &'surface_lifetime mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.0 >= self.surface.get_size().0 as i32 {
            self.pos.0 = 0;
            self.pos.1 += 1;
        }

        if self.pos.1 >= self.surface.get_size().1 as i32 {
            return None;
        }

        let result = self.surface.get_pixel_mut(self.pos).ok();
        self.pos.0 += 1;
        if let Some(result) = result {
            // Safety: We know that the result is a valid mutable reference and 'surface_lifetime
            // outlives the iterator. Apparently mutable iterators are not possible without unsafe code.
            unsafe { Some((self.pos - IntPos(1, 0), &mut *(result as *mut Color))) }
        } else {
            None
        }
    }
}
