use super::color::Color;
use super::surface::Surface;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext, IntPos, IntSize, Texture};
use anyhow::Result;
use core::cmp::max;

pub struct Font {
    font_surfaces: Vec<Surface>,
    font_textures: Vec<Texture>,
}

const CHAR_SPACING: i32 = 1;
const SPACE_WIDTH: i32 = 2;

/// Check if the column of a surface is empty.
fn is_column_empty(surface: &Surface, column: i32) -> bool {
    for y in 0..surface.get_size().1 {
        let pixel = surface.get_pixel(IntPos(column, y as i32));
        match pixel {
            Ok(pixel) => {
                if *pixel
                    != (Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    })
                {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    true
}

impl Font {
    /// Loads a font from a data vector, which is deserialized into a surface.
    /// Loads all the characters in the file and stores them in a
    /// surface array. The index of the array is the ascii value.
    /// # Errors
    /// Returns an error if the font data is invalid.
    pub fn new(font_data: &[u8], mono: bool) -> Result<Self> {
        let mut font_surfaces = vec![];
        let font_surface = Surface::deserialize_from_bytes(font_data)?;

        for y in 0..16 {
            for x in 0..16 {
                let mut surface = Surface::new(IntSize(16, 16));
                for (pos, pixel) in surface.iter_mut() {
                    *pixel = *font_surface
                        .get_pixel(IntPos(x * 16, y * 16) + pos)
                        .unwrap_or(&Color::new(0, 0, 0, 0));
                }

                // get number of empty columns on the left
                let mut left = 0;
                while left < 16 && is_column_empty(&surface, left) {
                    left += 1;
                }

                // get number of empty columns on the right
                let mut right = 0;
                while right < 16 - left && is_column_empty(&surface, 15 - right) {
                    right += 1;
                }
                if mono {
                    let width = 16 - left - right;
                    if width < 8 {
                        left -= (8 - width) / 2;
                        right -= (8 - width) / 2 + width % 2;
                    }
                }

                // create new surface with the correct width
                let mut new_surface = Surface::new(IntSize((16 - left - right) as u32, 16));
                for (pos, pixel) in new_surface.iter_mut() {
                    *pixel = *surface
                        .get_pixel(IntPos(left, 0) + pos)
                        .unwrap_or(&Color::new(0, 0, 0, 0));
                }

                font_surfaces.push(new_surface);
            }
        }

        let mut font_textures = Vec::new();
        for surface in &font_surfaces {
            font_textures.push(Texture::load_from_surface(surface));
        }

        Ok(Self {
            font_surfaces,
            font_textures,
        })
    }

    /// This function returns the size of the text.
    #[must_use]
    pub fn get_text_size(&self, text: &str) -> IntSize {
        let mut width = 0;
        let height = 16;
        for c in text.chars() {
            // is its space it makes it a little bigger
            if let Some(surface) = self.font_surfaces.get(c as usize) {
                width += surface.get_size().0 as i32;
            }
            width += CHAR_SPACING;
            if c == ' ' {
                width += SPACE_WIDTH;
            }
        }
        // if width is 0, set it to 1
        width = max(width, 1);

        IntSize(width as u32, height)
    }

    /// This function returns the size of the text scaled.
    #[must_use]
    pub fn get_text_size_scaled(&self, text: &str, scale: f32) -> FloatSize {
        let size = self.get_text_size(text);
        FloatSize(size.0 as f32 * scale, size.1 as f32 * scale)
    }

    /// This function creates a surface with the text on it.
    #[must_use]
    pub fn create_text_surface(&self, text: &str) -> Surface {
        let text_size = self.get_text_size(text);

        let mut surface = Surface::new(text_size);
        let mut x = 0;
        for c in text.chars() {
            if let Some(char_surface) = self.font_surfaces.get(c as usize) {
                for (pos, pixel) in char_surface.iter() {
                    if let Ok(surface_pixel) = surface.get_pixel_mut(IntPos(x, 0) + pos) {
                        *surface_pixel = *pixel;
                    }
                }

                x += char_surface.get_size().0 as i32 + CHAR_SPACING;
                if c == ' ' {
                    x += SPACE_WIDTH;
                }
            }
        }

        surface
    }

    /// This function renders text on the window.
    pub fn render_text(
        &self,
        graphics: &GraphicsContext,
        text: &str,
        mut pos: FloatPos,
        scale: f32,
    ) {
        for c in text.chars() {
            if let Some(char_surface) = self.font_surfaces.get(c as usize) {
                if let Some(char_texture) = self.font_textures.get(c as usize) {
                    char_texture.render(&graphics.renderer, scale, pos, None, false, None);
                    pos.0 += (char_surface.get_size().0 as i32 + CHAR_SPACING) as f32 * scale;
                    if c == ' ' {
                        pos.0 += SPACE_WIDTH as f32 * scale;
                    }
                }
            }
        }
    }
}
