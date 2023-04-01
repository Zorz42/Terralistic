use crate::client::game::camera::Camera;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::lights::Lights;
use anyhow::Result;

pub struct ClientLights {
    pub lights: Lights,
}

impl ClientLights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lights: Lights::new(),
        }
    }

    pub fn render(
        &mut self,
        graphics: &mut GraphicsContext,
        camera: &Camera,
        blocks: &Blocks,
    ) -> Result<()> {
        let start_x = camera.get_top_left(graphics).0 as i32;
        let start_y = camera.get_top_left(graphics).1 as i32;
        let end_x = camera.get_bottom_right(graphics).0 as i32;
        let end_y = camera.get_bottom_right(graphics).1 as i32;

        for x in start_x..end_x {
            if x < 0 || x >= self.lights.get_width() as i32 {
                continue;
            }

            self.lights.update_column(x, blocks)?;
            for y in start_y..end_y {
                if y < 0 || y >= self.lights.get_height() as i32 {
                    continue;
                }

                if self.lights.has_scheduled_light_update(x, y)? {
                    self.lights.update_light(x, y, blocks)?;
                }

                let light = self.lights.get_light_color(x, y)?;

                let rect = gfx::Rect::new(
                    FloatPos(
                        (x as f32 - camera.get_top_left(graphics).0) * RENDER_BLOCK_WIDTH,
                        (y as f32 - camera.get_top_left(graphics).1) * RENDER_BLOCK_WIDTH,
                    ),
                    FloatSize(RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH),
                );

                let color = gfx::Color::new(light.r, light.g, light.b, 255);
                gfx::set_blend_mode(gfx::BlendMode::Multiply);
                rect.render(graphics, color);
                gfx::set_blend_mode(gfx::BlendMode::Alpha);
            }
        }

        Ok(())
    }
}
