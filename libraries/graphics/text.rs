use super::color::Color;
use super::surface::Surface;
use std::cmp::max;

pub struct Font {
    font_surfaces: Vec<Surface>,
}

const CHAR_SPACING: i32 = 1;
const SPACE_WIDTH: i32 = 2;

/**
Check if the column of a surface is empty.
 */
fn is_column_empty(surface: &Surface, column: i32) -> bool {
    for y in 0..surface.get_height() {
        if surface.get_pixel(column, y)
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
    true
}

impl Font {
    /**
    Loads a font from a data vector, which is deserialized into a surface.
    Loads all the characters in the file and stores them in a
    surface array. The index of the array is the ascii value.
     */
    #[must_use]
    pub fn new(font_data: &[u8]) -> Self {
        let mut font_surfaces = vec![];
        let font_surface = Surface::deserialize(font_data);

        for y in 0..16 {
            for x in 0..16 {
                let mut surface = Surface::new(16, 16);
                for x2 in 0..16 {
                    for y2 in 0..16 {
                        surface.set_pixel(x2, y2, font_surface.get_pixel(x * 16 + x2, y * 16 + y2));
                    }
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
                let mut new_surface = Surface::new(16 - left - right, 16);
                for x2 in 0..16 - left - right {
                    for y2 in 0..16 {
                        new_surface.set_pixel(x2, y2, surface.get_pixel(x2 + left, y2));
                    }
                }

                font_surfaces.push(new_surface);
            }
        }

        Self { font_surfaces }
    }

    /**
    This function creates a surface with the text on it.
     */
    #[must_use]
    pub fn create_text_surface(&self, text: &str) -> Surface {
        let mut width = 0;
        let height = self.font_surfaces[0].get_height();
        for c in text.chars() {
            // is its space it makes it a little bigger
            width += self.font_surfaces[c as usize].get_width() + CHAR_SPACING;
            if c == ' ' {
                width += SPACE_WIDTH;
            }
        }
        // if width is 0, set it to 1
        width = max(width, 1);

        let mut surface = Surface::new(width, height);
        let mut x = 0;
        for c in text.chars() {
            for x2 in 0..self.font_surfaces[c as usize].get_width() {
                for y2 in 0..self.font_surfaces[c as usize].get_height() {
                    surface.set_pixel(x + x2, y2, self.font_surfaces[c as usize].get_pixel(x2, y2));
                }
            }
            x += self.font_surfaces[c as usize].get_width() + CHAR_SPACING;
            if c == ' ' {
                x += SPACE_WIDTH;
            }
        }

        surface
    }
}
