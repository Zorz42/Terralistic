use super::Color;
use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

/// Surface is an image stored in ram.
#[derive(Serialize, Deserialize, Clone)]
pub struct Surface {
    pub(super) pixels: Vec<Color>,
    width: u32,
    height: u32,
}

impl Surface {
    /// Creates a new surface with all transparent pixels.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixels: std::vec![Color::new(0, 0, 0, 0); (width * height) as usize],
            width,
            height,
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
    fn get_index(&self, x: i32, y: i32) -> Result<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            bail!("Pixel out of bounds");
        }

        Ok((y * self.width as i32 + x) as usize)
    }

    /// Retrieves the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Result<&Color> {
        let index = self.get_index(x, y)?;
        self.pixels
            .get(index)
            .ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    /// Retrieves the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn get_pixel_mut(&mut self, x: i32, y: i32) -> Result<&mut Color> {
        let index = self.get_index(x, y)?;
        self.pixels
            .get_mut(index)
            .ok_or_else(|| anyhow!("Pixel array malformed"))
    }

    #[must_use]
    pub const fn get_width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn get_height(&self) -> u32 {
        self.height
    }

    /// Copies another surface to the specified location.
    /// # Errors
    /// Returns an error if the surface is out of bounds.
    pub fn draw(&mut self, x: i32, y: i32, surface: &Self, color: Color) -> Result<()> {
        for (xpos, ypos, surface_color) in surface.iter() {
            *self.get_pixel_mut(xpos + x, ypos + y)? = Color {
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
    x: i32,
    y: i32,
}

impl<'surface_lifetime> SurfaceIterator<'surface_lifetime> {
    pub const fn new(surface: &'surface_lifetime Surface) -> Self {
        Self {
            surface,
            x: 0,
            y: 0,
        }
    }
}

impl<'surface_lifetime> Iterator for SurfaceIterator<'surface_lifetime> {
    type Item = (i32, i32, &'surface_lifetime Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.surface.get_width() as i32 {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.surface.get_height() as i32 {
            return None;
        }

        let result = self.surface.get_pixel(self.x, self.y).ok();
        self.x += 1;
        if let Some(result) = result {
            Some((self.x - 1, self.y, result))
        } else {
            None
        }
    }
}

/// Surface iterator iterates through all pixels in a surface row by row.
pub struct MutSurfaceIterator<'surface_lifetime> {
    surface: &'surface_lifetime mut Surface,
    x: i32,
    y: i32,
}

impl<'surface_lifetime> MutSurfaceIterator<'surface_lifetime> {
    pub fn new(surface: &'surface_lifetime mut Surface) -> Self {
        Self {
            surface,
            x: 0,
            y: 0,
        }
    }
}

impl<'surface_lifetime> Iterator for MutSurfaceIterator<'surface_lifetime> {
    type Item = (i32, i32, &'surface_lifetime mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.surface.get_width() as i32 {
            self.x = 0;
            self.y += 1;
        }

        if self.y >= self.surface.get_height() as i32 {
            return None;
        }

        let result = self.surface.get_pixel_mut(self.x, self.y).ok();
        self.x += 1;
        if let Some(result) = result {
            // Safety: We know that the result is a valid mutable reference and 'surface_lifetime
            // outlives the iterator. Apparently mutable iterators are not possible without unsafe code.
            unsafe {
                Some((
                    self.x - 1,
                    self.y,
                    &mut *(result as *mut Color as *mut Color),
                ))
            }
        } else {
            None
        }
    }
}
