use std::collections::HashMap;
use std::hash::Hash;

use crate::libraries::graphics as gfx;

use super::{Color, Rect, Surface, Texture};

/// One texture holding many surfaces, plus the rectangle each of them occupies.
pub struct TextureAtlas<KeyType> {
    texture: Texture,
    rects: HashMap<KeyType, Rect>,
}

impl<KeyType: Eq + Hash + Clone + Ord> TextureAtlas<KeyType> {
    /// Packs every surface into one texture, left to right **in key order** - hence the `Ord`
    /// bound. Packing in `HashMap` order gives a different layout every launch, Rust
    /// randomising it per process. The result is one row: the surfaces end to end, as tall as
    /// the tallest.
    #[must_use]
    pub fn new(surfaces: &HashMap<KeyType, Surface>) -> Self {
        if surfaces.is_empty() {
            return Self {
                texture: Texture::new(),
                rects: HashMap::new(),
            };
        }

        let mut sorted: Vec<(&KeyType, &Surface)> = surfaces.iter().collect();
        sorted.sort_unstable_by_key(|(key, _)| *key);

        let total_width = sorted.iter().map(|(_, surface)| surface.get_size().0).sum();
        let max_height = sorted.iter().map(|(_, surface)| surface.get_size().1).max().unwrap_or(0);

        let mut main_surface = Surface::new(gfx::IntSize(total_width, max_height));
        let mut rects = HashMap::new();
        let mut x = 0;
        for (key, surface) in sorted {
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
