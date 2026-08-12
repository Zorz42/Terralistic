use std::collections::HashMap;
use std::hash::Hash;

use crate::libraries::graphics as gfx;

use super::{Color, Rect, Surface, Texture};

/// Texture atlas is a struct that holds a texture and a list of rectangles
/// that represent the position of each sprite in the texture.
pub struct TextureAtlas<KeyType> {
    texture: Texture,
    rects: HashMap<KeyType, Rect>,
}

impl<KeyType: Eq + Hash + Clone + Ord> TextureAtlas<KeyType> {
    /// Packs every surface into one texture, left to right in key order.
    ///
    /// **The key order matters.** This used to pack in `HashMap` iteration order, which Rust
    /// randomises per process, so the same set of surfaces produced a different atlas layout
    /// on every launch. That is invisible as long as a region is sampled perfectly, but
    /// `Texture::render` samples half a texel wide on each side, so a region whose neighbour
    /// changed between runs could pick up a different edge colour - the same class of bug
    /// that made `GameModData.resources` a `BTreeMap`. Sorting by key makes the layout, and
    /// therefore anything rendered from it, reproducible.
    #[must_use]
    pub fn new(surfaces: &HashMap<KeyType, Surface>) -> Self {
        if surfaces.is_empty() {
            return Self {
                texture: Texture::new(),
                rects: HashMap::new(),
            };
        }

        // One row, so the atlas is as wide as the surfaces laid end to end and as tall as
        // the tallest. This used to add each surface's *height* into the width as well,
        // which packed correctly but left the texture wider than anything drawn into it -
        // for the block atlas, several megabytes of transparent pixels on the GPU.
        let mut total_width = 0;
        let mut max_height = 0;
        for surface in surfaces.values() {
            total_width += surface.get_size().0;
            max_height = max_height.max(surface.get_size().1);
        }

        let mut main_surface = Surface::new(gfx::IntSize(total_width, max_height));
        let mut rects = HashMap::new();

        let mut keys: Vec<&KeyType> = surfaces.keys().collect();
        keys.sort_unstable();

        let mut x = 0;
        for key in keys {
            let Some(surface) = surfaces.get(key) else {
                continue;
            };
            rects.insert(key.clone(), Rect::new(gfx::FloatPos(x as f32, 0.0), gfx::FloatSize::from(surface.get_size())));
            main_surface.draw(gfx::IntPos(x, 0), surface, Color::new(255, 255, 255, 255)).unwrap_or_else(|e| {
                println!("Failed to draw surface to main surface (unreachable) {e}");
            });
            x += surface.get_size().0 as i32;
        }

        Self {
            texture: Texture::load_from_surface(&main_surface),
            rects,
        }
    }

    #[must_use]
    pub const fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_rect(&self, index: &KeyType) -> Option<&Rect> {
        self.rects.get(index)
    }
}
