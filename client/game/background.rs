use super::camera::Camera;
use crate::libraries::graphics as gfx;

/**
Background is a struct that holds the background image and renders it.
 */
pub struct Background {
    image: gfx::Texture,
}

impl Background {
    pub fn new() -> Self {
        Self {
            image: gfx::Texture::new(),
        }
    }

    pub fn init(&mut self) {
        self.image = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(include_bytes!(
            "../../Build/Resources/background.opa"
        )));
    }

    pub fn render(&self, graphics: &mut gfx::GraphicsContext, camera: &Camera) {
        //float scale = (float)gfx::getWindowHeight() / background.getTextureHeight();
        let scale =
            graphics.renderer.get_window_height() as f32 / self.image.get_texture_height() as f32;
        //int position_x = -int(camera->getX() * scale / 20) % int(background.getTextureWidth() * scale);
        let position_x = -((camera.get_position().0 * scale / 20.0) as i32)
            % ((self.image.get_texture_width() as f32 * scale) as i32);
        //for(int i = 0; i < gfx::getWindowWidth() / (background.getTextureWidth() * scale) + 2; i++)
        for i in 0..(graphics.renderer.get_window_width() as f32
            / (self.image.get_texture_width() as f32 * scale)
            + 2.0) as i32
        {
            //background.render(position_x + i * int(background.getTextureWidth() * scale), 0, scale);
            self.image.render(
                &graphics.renderer,
                scale,
                (
                    position_x + i * (self.image.get_texture_width() as f32 * scale) as i32,
                    0,
                ),
                None,
                false,
                None,
            );
        }
    }
}
