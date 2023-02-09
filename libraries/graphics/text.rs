use super::color::Color;
use super::surface::Surface;
use anyhow::Result;
use core::cmp::max;

pub struct Font {
    font_surfaces: Vec<Surface>,
}

const CHAR_SPACING: i32 = 1;
const SPACE_WIDTH: i32 = 2;

/// Check if the column of a surface is empty.
fn is_column_empty(surface: &Surface, column: i32) -> bool {
    for y in 0..surface.get_height() {
        let pixel = surface.get_pixel(column, y as i32);
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
    pub fn new(font_data: &[u8]) -> Result<Self> {
        let mut font_surfaces = vec![];
        let font_surface = Surface::deserialize_from_bytes(font_data)?;

        for y in 0..16 {
            for x in 0..16 {
                let mut surface = Surface::new(16, 16);
                for (x2, y2, pixel) in surface.iter_mut() {
                    *pixel = *font_surface
                        .get_pixel(x * 16 + x2, y * 16 + y2)
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

                // create new surface with the correct width
                let mut new_surface = Surface::new((16 - left - right) as u32, 16);
                for (x2, y2, pixel) in new_surface.iter_mut() {
                    *pixel = *surface
                        .get_pixel(x2 + left, y2)
                        .unwrap_or(&Color::new(0, 0, 0, 0));
                }

                font_surfaces.push(new_surface);
            }
        }

        Ok(Self { font_surfaces })
    }

    /// This function creates a surface with the text on it.
    #[must_use]
    pub fn create_text_surface(&self, text: &str) -> Surface {
        let mut width = 0;
        let height = 16;
        for c in text.chars() {
            // is its space it makes it a little bigger
            if let Some(surface) = self.font_surfaces.get(c as usize) {
                width += surface.get_width() as i32;
            }
            width += CHAR_SPACING;
            if c == ' ' {
                width += SPACE_WIDTH;
            }
        }
        // if width is 0, set it to 1
        width = max(width, 1);

        let mut surface = Surface::new(width as u32, height as u32);
        let mut x = 0;
        for c in text.chars() {
            if let Some(char_surface) = self.font_surfaces.get(c as usize) {
                for (x2, y2, pixel) in char_surface.iter() {
                    if let Ok(surface_pixel) = surface.get_pixel_mut(x + x2, y2) {
                        *surface_pixel = *pixel;
                    }
                }

                x += char_surface.get_width() as i32 + CHAR_SPACING;
                if c == ' ' {
                    x += SPACE_WIDTH;
                }
            }
        }

        surface
    }
}
