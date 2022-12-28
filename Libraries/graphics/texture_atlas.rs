use crate::{Color, Rect, Surface, Texture};

/**
Texture atlas is a struct that holds a texture and a list of rectangles
that represent the position of each sprite in the texture.
 */
pub struct TextureAtlas {
    texture: Texture,
    rects: Vec<Rect>,
}

impl TextureAtlas {
    pub fn new(surfaces: Vec<Surface>) -> Self {
        if surfaces.len() == 0 {
            return Self {
                texture: Texture::new(),
                rects: Vec::new(),
            };
        }

        let mut total_width = 0;
        for surface in &surfaces {
            total_width += surface.get_width();
        }

        let mut max_height = 0;
        for surface in &surfaces {
            total_width += surface.get_width();
            if surface.get_height() > max_height {
                max_height = surface.get_height();
            }
        };

        let mut surface = Surface::new(total_width, max_height);
        let mut rects = Vec::new();

        let mut x = 0;
        for i in 0..surfaces.len() {
            rects.push(Rect::new(x, 0, surfaces[i].get_width(), surfaces[i].get_height()));
            surface.draw(x, 0, &surfaces[i], Color::new(255, 255, 255, 255));
            x += surfaces[i].get_width();
        }

        Self {
            texture: Texture::load_from_surface(&surface),
            rects,
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_rect(&self, index: usize) -> &Rect {
        &self.rects[index]
    }
}