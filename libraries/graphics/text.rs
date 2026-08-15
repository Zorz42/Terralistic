use anyhow::Result;

use crate::libraries::graphics as gfx;

use super::color::Color;
use super::surface::Surface;

/// A bitmap font: one surface and one texture per character, indexed by ascii value.
pub struct Font {
    font_surfaces: Vec<Surface>,
    font_textures: Vec<gfx::Texture>,
    /// Every glyph was padded out to the same width, so the space needs no width of its own.
    /// See `advance`.
    mono: bool,
}

const CHAR_SPACING: i32 = 1;
const SPACE_WIDTH: i32 = 2;
const GLYPH_SIZE: i32 = 16;

/// Whether a column of a surface is entirely transparent. Out of bounds counts as non-empty,
/// so trimming stops rather than running away.
fn is_column_empty(surface: &Surface, column: i32) -> bool {
    (0..surface.get_size().1).all(|y| surface.get_pixel(gfx::IntPos(column, y as i32)).is_ok_and(|pixel| *pixel == Color::new(0, 0, 0, 0)))
}

impl Font {
    /// Cuts a 16x16 atlas into one surface per character, trimming the empty columns either
    /// side for proportional spacing - or padding back out to 8 wide when `mono`. All the CPU
    /// work, kept apart from the upload so measuring and rasterising are testable with no GPU.
    fn load_surfaces(font_data: &[u8], mono: bool) -> Result<Vec<Surface>> {
        let atlas = Surface::deserialize_from_bytes(font_data)?;
        let mut font_surfaces = Vec::new();

        for y in 0..GLYPH_SIZE {
            for x in 0..GLYPH_SIZE {
                let mut glyph = Surface::new(gfx::IntSize(GLYPH_SIZE as u32, GLYPH_SIZE as u32));
                for (pos, pixel) in glyph.iter_mut() {
                    *pixel = *atlas.get_pixel(gfx::IntPos(x * GLYPH_SIZE, y * GLYPH_SIZE) + pos).unwrap_or(&Color::new(0, 0, 0, 0));
                }

                let mut left = 0;
                while left < GLYPH_SIZE && is_column_empty(&glyph, left) {
                    left += 1;
                }
                let mut right = 0;
                while right < GLYPH_SIZE - left && is_column_empty(&glyph, GLYPH_SIZE - 1 - right) {
                    right += 1;
                }

                // Pad a narrow glyph back out to 8, half each side, so a mono font advances by
                // the same amount everywhere. The trim may go negative, widening the glyph
                // past its cell: those columns read out of bounds and come out transparent.
                if mono {
                    let width = GLYPH_SIZE - left - right;
                    if width < 8 {
                        left -= (8 - width) / 2;
                        right -= (8 - width) / 2 + width % 2;
                    }
                }

                let mut trimmed = Surface::new(gfx::IntSize((GLYPH_SIZE - left - right) as u32, GLYPH_SIZE as u32));
                for (pos, pixel) in trimmed.iter_mut() {
                    *pixel = *glyph.get_pixel(gfx::IntPos(left, 0) + pos).unwrap_or(&Color::new(0, 0, 0, 0));
                }

                font_surfaces.push(trimmed);
            }
        }

        Ok(font_surfaces)
    }

    /// Loads a font from a serialized `Surface` and uploads its glyphs.
    pub fn new(font_data: &[u8], mono: bool) -> Result<Self> {
        let font_surfaces = Self::load_surfaces(font_data, mono)?;
        let font_textures = font_surfaces.iter().map(gfx::Texture::load_from_surface).collect();
        Ok(Self { font_surfaces, font_textures, mono })
    }

    /// A font whose glyphs know their size and own no pixels, for tests. Only the upload is
    /// skipped, so measuring, rasterising and the commands `render_text` records are real.
    #[cfg(test)]
    pub fn new_headless(font_data: &[u8], mono: bool) -> Result<Self> {
        let font_surfaces = Self::load_surfaces(font_data, mono)?;
        let font_textures = font_surfaces.iter().map(|glyph| gfx::Texture::new_sized(gfx::FloatSize::from(glyph.get_size()))).collect();
        Ok(Self { font_surfaces, font_textures, mono })
    }

    /// How far the pen moves after `c`. Shared by `layout` and `render_text`, which have to
    /// agree to the pixel: `TextInput` places its cursor by measuring, and the glyph is then
    /// drawn wherever this says.
    ///
    /// A space is the one character whose advance is not its glyph's width, trimming having
    /// left it empty. **Not in a mono font** - `load_surfaces` already padded it to the common
    /// width, and more would make the space the one character that breaks the grid.
    const fn advance(&self, c: char, glyph: &Surface) -> i32 {
        let advance = glyph.get_size().0 as i32 + CHAR_SPACING;
        if c == ' ' && !self.mono {
            advance + SPACE_WIDTH
        } else {
            advance
        }
    }

    /// How far the pen moves over the run of non-whitespace characters `text` starts with -
    /// one word, as far as wrapping is concerned.
    fn word_advance(&self, text: &str) -> i32 {
        text.chars()
            .take_while(|c| !c.is_whitespace())
            .filter_map(|c| self.font_surfaces.get(c as usize).map(|glyph| self.advance(c, glyph)))
            .sum()
    }

    /// Walks `text`, handing each glyph and its position to `place`, and returns the size of
    /// the block. Measuring and rasterising share this walk rather than each keeping a copy of
    /// the wrapping rules, which is how the two drift apart.
    ///
    /// **A width limit breaks between words, not inside them** - the only text in the game
    /// that wraps is an error message, which is where a reader needs the words whole.
    fn layout<F: FnMut(gfx::IntPos, &Surface)>(&self, text: &str, width_limit: Option<i32>, mut place: F) -> gfx::IntSize {
        // Glyphs are cut at the full cell height, so every line is a cell tall.
        const LINE_HEIGHT: i32 = GLYPH_SIZE + CHAR_SPACING;

        let mut x = 0;
        let mut y = 0;
        let mut max_width = 1;
        // A trailing newline starts a line lower, so the empty last line is not counted twice.
        let mut height = if text.ends_with('\n') { 0 } else { GLYPH_SIZE };

        let mut rest = text;
        let mut word_start = true;
        while let Some(c) = rest.chars().next() {
            let from_here = rest;
            rest = rest.get(c.len_utf8()..).unwrap_or("");

            if c == '\n' {
                x = 0;
                y += LINE_HEIGHT;
                height += LINE_HEIGHT;
                word_start = true;
                continue;
            }

            if c.is_whitespace() {
                word_start = true;
                // A space a wrap stepped over is not drawn: it would indent the line it lands
                // on. Only when wrapping, so measuring a prefix - `TextInput`'s cursor, always
                // without a limit - stays additive.
                if width_limit.is_some() && x == 0 {
                    continue;
                }
            }

            let Some(glyph) = self.font_surfaces.get(c as usize) else {
                continue;
            };

            let advance = self.advance(c, glyph);
            if let Some(limit) = width_limit {
                // At a word's first character the whole word has to fit, so it moves down
                // together. `min` stops a word too wide for any line wrapping forever: it
                // falls back to breaking at whichever character overruns, one line down.
                let needed = if word_start { self.word_advance(from_here).min(limit) } else { advance };
                // `x > 0` keeps an over-wide glyph on the line it is on; wrapping leaves a
                // blank line above it, counts its height, and repeats for every character.
                if x > 0 && x + needed > limit {
                    x = 0;
                    y += LINE_HEIGHT;
                    height += LINE_HEIGHT;
                }
            }
            word_start = c.is_whitespace();

            place(gfx::IntPos(x, y), glyph);
            // The advance is part of the width, not something only the *next* character sees:
            // sampling before a space's extra gap put `TextInput`'s cursor two pixels short of
            // the glyph `create_text_surface` then drew.
            x += advance;
            // A space counts only if something follows it on its line - left hanging by a wrap
            // it would push the block past the limit it just wrapped to.
            if !(c.is_whitespace() && width_limit.is_some()) {
                max_width = max_width.max(x);
            }
        }

        gfx::IntSize(max_width as u32, height as u32)
    }

    /// The size one line - or several, if the text wraps or contains newlines - would occupy.
    #[must_use]
    pub fn get_text_size(&self, text: &str, width_limit: Option<i32>) -> gfx::IntSize {
        self.layout(text, width_limit, |_, _| {})
    }

    #[must_use]
    pub fn get_text_size_scaled(&self, text: &str, scale: f32, width_limit: Option<i32>) -> gfx::FloatSize {
        let size = self.get_text_size(text, width_limit);
        gfx::FloatSize(size.0 as f32 * scale, size.1 as f32 * scale)
    }

    /// Rasterises text into a new surface of exactly `get_text_size`.
    #[must_use]
    pub fn create_text_surface(&self, text: &str, width_limit: Option<i32>) -> Surface {
        let mut surface = Surface::new(self.get_text_size(text, width_limit));
        self.layout(text, width_limit, |pos, glyph| {
            for (glyph_pos, pixel) in glyph.iter() {
                if let Ok(target) = surface.get_pixel_mut(pos + glyph_pos) {
                    *target = *pixel;
                }
            }
        });
        surface
    }

    /// Draws **one line** of text straight from the glyph textures, honouring neither `\n` nor
    /// a width limit - so a multi-line string measures as several and draws as one. Anything
    /// that might wrap goes through `create_text_surface` and a `Texture`, as `Sprite` does.
    pub fn render_text(&self, target: &dyn gfx::DrawTarget, text: &str, mut pos: gfx::FloatPos, scale: f32) {
        for c in text.chars() {
            let (Some(glyph), Some(texture)) = (self.font_surfaces.get(c as usize), self.font_textures.get(c as usize)) else {
                continue;
            };
            texture.render(target, scale, pos, None, false, None);
            pos.0 += self.advance(c, glyph) as f32 * scale;
        }
    }
}
