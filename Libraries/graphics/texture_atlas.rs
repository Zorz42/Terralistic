use crate::{Color, Rect, Surface, Texture};
use std::collections::HashMap;
use std::hash::Hash;

/**
Texture atlas is a struct that holds a texture and a list of rectangles
that represent the position of each sprite in the texture.
 */
pub struct TextureAtlas<KeyType> {
    texture: Texture,
    rects: HashMap<KeyType, Rect>,
}

impl<KeyType: Eq + Hash + Clone> TextureAtlas<KeyType> {
    pub fn new(surfaces: &HashMap<KeyType, Surface>) -> Self {
        if surfaces.is_empty() {
            return Self {
                texture: Texture::new(),
                rects: HashMap::new(),
            };
        }

        let mut total_width = 0;
        for surface in surfaces.values() {
            total_width += surface.get_width();
        }

        let mut max_height = 0;
        for surface in surfaces.values() {
            total_width += surface.get_width();
            if surface.get_height() > max_height {
                max_height = surface.get_height();
            }
        }

        let mut main_surface = Surface::new(total_width, max_height);
        let mut rects = HashMap::new();

        let mut x = 0;
        for (key, surface) in surfaces {
            rects.insert(
                key.clone(),
                Rect::new(x, 0, surface.get_width(), surface.get_height()),
            );
            main_surface.draw(x, 0, surface, Color::new(255, 255, 255, 255));
            x += surface.get_width();
        }

        Self {
            texture: Texture::load_from_surface(&main_surface),
            rects,
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_rect(&self, index: KeyType) -> Option<&Rect> {
        self.rects.get(&index)
    }
}
