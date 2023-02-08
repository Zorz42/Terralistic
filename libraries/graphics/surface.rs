use super::Color;
use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};

/// Surface is an image stored in ram.
#[derive(Serialize, Deserialize, Clone)]
pub struct Surface {
    pub(crate) pixels: Vec<u8>,
    width: i32,
    height: i32,
}

impl Surface {
    /// Creates a new surface with all transparent pixels.
    /// # Errors
    /// Returns an error if the width or height is negative.
    pub fn new(width: i32, height: i32) -> Result<Self> {
        if width < 0 || height < 0 {
            bail!("Dimensions negative");
        }

        Ok(Self {
            pixels: std::vec![0; (width * height * 4) as usize],
            width,
            height,
        })
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
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            bail!("Pixel out of bounds");
        }

        Ok((y * self.width + x) as usize * 4)
    }

    /// Retrieves the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Result<Color> {
        let index = self.get_index(x, y)?;
        let mut result = Color::new(0, 0, 0, 0);
        for (i, channel) in [&mut result.r, &mut result.g, &mut result.b, &mut result.a]
            .iter_mut()
            .enumerate()
        {
            **channel = *self
                .pixels
                .get(index + i)
                .ok_or_else(|| anyhow!("Pixel out of bounds"))?;
        }

        Ok(result)
    }

    /// Sets the pixel color on a specified location.
    /// # Errors
    /// Returns an error if the pixel is out of bounds.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<()> {
        let index = self.get_index(x, y)?;
        *self
            .pixels
            .get_mut(index)
            .ok_or_else(|| anyhow!("Pixel out of bounds"))? = color.r;
        *self
            .pixels
            .get_mut(index + 1)
            .ok_or_else(|| anyhow!("Pixel out of bounds"))? = color.g;
        *self
            .pixels
            .get_mut(index + 2)
            .ok_or_else(|| anyhow!("Pixel out of bounds"))? = color.b;
        *self
            .pixels
            .get_mut(index + 3)
            .ok_or_else(|| anyhow!("Pixel out of bounds"))? = color.a;
        Ok(())
    }

    #[must_use]
    pub const fn get_width(&self) -> i32 {
        self.width
    }

    #[must_use]
    pub const fn get_height(&self) -> i32 {
        self.height
    }

    /// Copies another surface to the specified location.
    /// # Errors
    /// Returns an error if the surface is out of bounds.
    pub fn draw(&mut self, x: i32, y: i32, surface: &Self, color: Color) -> Result<()> {
        for xpos in 0..surface.get_width() {
            for ypos in 0..surface.get_height() {
                let surface_color = surface.get_pixel(xpos, ypos)?;
                self.set_pixel(
                    xpos + x,
                    ypos + y,
                    Color {
                        r: (surface_color.r as f32 * (color.r as f32 / 255.0)) as u8,
                        g: (surface_color.g as f32 * (color.g as f32 / 255.0)) as u8,
                        b: (surface_color.b as f32 * (color.b as f32 / 255.0)) as u8,
                        a: (surface_color.a as f32 * (color.a as f32 / 255.0)) as u8,
                    },
                )?;
            }
        }
        Ok(())
    }
}
