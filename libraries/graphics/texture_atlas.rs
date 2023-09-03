use core::hash::Hash;
use std::collections::HashMap;

use crate::libraries::graphics as gfx;

use super::{Color, Rect, Surface, Texture};

/// Texture atlas is a struct that holds a texture and a list of rectangles
/// that represent the position of each sprite in the texture.
pub struct TextureAtlas<KeyType> {
    texture: Texture,
    rects: HashMap<KeyType, Rect>,
}

impl<KeyType: Eq + Hash + Clone> TextureAtlas<KeyType> {
    #[must_use]
    pub fn new(surfaces: &HashMap<KeyType, Surface>) -> Self {
        if surfaces.is_empty() {
            return Self {
                texture: Texture::new(),
                rects: HashMap::new(),
            };
        }

        let mut total_width = 0;
        for surface in surfaces.values() {
            total_width += surface.get_size().1;
        }

        let mut max_height = 0;
        for surface in surfaces.values() {
            total_width += surface.get_size().0;
            if surface.get_size().1 > max_height {
                max_height = surface.get_size().1;
            }
        }

        let mut main_surface = Surface::new(gfx::IntSize(total_width, max_height));
        let mut rects = HashMap::new();

        let mut x = 0;
        for (key, surface) in surfaces {
            rects.insert(
                key.clone(),
                Rect::new(gfx::FloatPos(x as f32, 0.0), gfx::FloatSize::from(surface.get_size())),
            );
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
