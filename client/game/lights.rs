use crate::client::game::camera::Camera;
use crate::libraries::events::EventManager;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::lights::Lights;
use anyhow::Result;

pub struct LightChunk {
    pub rect_array: gfx::RectArray,
    pub needs_update: bool,
}

impl LightChunk {
    pub fn new() -> Self {
        Self {
            rect_array: gfx::RectArray::new(),
            needs_update: true,
        }
    }
}

pub struct ClientLights {
    pub lights: Lights,
    chunks: Vec<LightChunk>,
}

impl ClientLights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lights: Lights::new(),
            chunks: Vec::new(),
        }
    }

    pub fn init(&mut self, blocks: &Blocks) -> Result<()> {
        self.lights.create(blocks.get_width(), blocks.get_height());
        self.lights.init_sky_heights(blocks)?;

        let chunk_width = (self.lights.get_width() as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_height = (self.lights.get_height() as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_count = chunk_width * chunk_height;
        for _ in 0..chunk_count {
            self.chunks.push(LightChunk::new());
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        graphics: &mut GraphicsContext,
        camera: &Camera,
        blocks: &Blocks,
        events: &mut EventManager,
    ) -> Result<()> {
        let start_x = camera.get_top_left(graphics).0 as i32;
        let start_y = camera.get_top_left(graphics).1 as i32;
        let end_x = camera.get_bottom_right(graphics).0 as i32 + 1;
        let end_y = camera.get_bottom_right(graphics).1 as i32 + 1;

        let extended_view_distance = 10;
        let extended_start_x = start_x - extended_view_distance;
        let extended_start_y = start_y - extended_view_distance;
        let extended_end_x = end_x + extended_view_distance;
        let extended_end_y = end_y + extended_view_distance;

        for x in extended_start_x..extended_end_x {
            for y in extended_start_y..extended_end_y {
                if y < 0
                    || y >= self.lights.get_height() as i32
                    || x < 0
                    || x >= self.lights.get_width() as i32
                {
                    continue;
                }

                self.lights.update_light_emitter(x, y, blocks)?;
                if self.lights.has_scheduled_light_update(x, y)? {
                    self.lights.update_light(x, y, blocks, events)?;
                }
            }
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                if y < 0
                    || y >= self.lights.get_height() as i32
                    || x < 0
                    || x >= self.lights.get_width() as i32
                {
                    continue;
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
