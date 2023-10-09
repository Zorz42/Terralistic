use anyhow::{anyhow, bail, Result};

use crate::client::game::camera::Camera;
use crate::client::settings::{Setting, Settings};
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::lights::{LightColorChangeEvent, Lights};
use crate::shared::world_map::CHUNK_SIZE;

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

    pub fn render(
        &mut self,
        graphics: &gfx::GraphicsContext,
        world_x: i32,
        world_y: i32,
        lights: &Lights,
        camera: &Camera,
    ) -> Result<()> {
        if self.needs_update {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let light_1 = lights
                        .get_light(i32::max(world_x + x - 1, 0), i32::max(world_y + y - 1, 0))?
                        .color;
                    let light_2 = lights
                        .get_light(world_x + x, i32::max(world_y + y - 1, 0))?
                        .color;
                    let light_3 = lights
                        .get_light(i32::max(world_x + x - 1, 0), world_y + y)?
                        .color;
                    let light_4 = lights.get_light(world_x + x, world_y + y)?.color;

                    self.rect_array.add_rect(
                        &gfx::Rect::new(
                            gfx::FloatPos(
                                x as f32 * RENDER_BLOCK_WIDTH,
                                y as f32 * RENDER_BLOCK_WIDTH,
                            ),
                            gfx::FloatSize(RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH),
                        ),
                        &[
                            gfx::Color::new(light_1.r, light_1.g, light_1.b, 255),
                            gfx::Color::new(light_2.r, light_2.g, light_2.b, 255),
                            gfx::Color::new(light_3.r, light_3.g, light_3.b, 255),
                            gfx::Color::new(light_4.r, light_4.g, light_4.b, 255),
                        ],
                        &gfx::Rect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
                    );
                }
            }

            self.rect_array.update();
        }

        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH
            - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH
            - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;
        gfx::set_blend_mode(gfx::BlendMode::Multiply);
        self.rect_array.render(
            graphics,
            None,
            gfx::FloatPos(screen_x.round(), screen_y.round()),
        );
        gfx::set_blend_mode(gfx::BlendMode::Alpha);
        Ok(())
    }
}

pub struct ClientLights {
    pub lights: Lights,
    chunks: Vec<LightChunk>,
    lights_setting: i32,
}

impl ClientLights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lights: Lights::new(),
            chunks: Vec::new(),
            lights_setting: 0,
        }
    }

    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0
            || y < 0
            || x >= self.lights.get_width() as i32 / CHUNK_SIZE
            || y >= self.lights.get_height() as i32 / CHUNK_SIZE
        {
            bail!("Tried to get light chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.lights.get_width() as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn init(&mut self, blocks: &Blocks, settings: &mut Settings) -> Result<()> {
        self.lights.create(blocks.get_width(), blocks.get_height());
        self.lights.init_sky_heights(blocks)?;

        let chunk_width = (self.lights.get_width() as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_height = (self.lights.get_height() as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_count = chunk_width * chunk_height;
        for _ in 0..chunk_count {
            self.chunks.push(LightChunk::new());
        }

        let lights_settings = Setting::Toggle {
            text: "Enable lights".to_owned(),
            config_label: "enable_Lights".to_owned(),
            toggled: true,
        };

        self.lights_setting = settings.register_setting(lights_settings);

        Ok(())
    }

    pub fn render(
        &mut self,
        graphics: &gfx::GraphicsContext,
        camera: &Camera,
        blocks: &Blocks,
        events: &mut EventManager,
        settings: &Settings,
    ) -> Result<()> {
        if let Setting::Toggle { toggled, .. } = settings.get_setting(self.lights_setting)? {
            if !toggled {
                return Ok(());
            }
        }

        let start_x = i32::max(0, camera.get_top_left(graphics).0 as i32);
        let start_y = i32::max(0, camera.get_top_left(graphics).1 as i32);
        let end_x = i32::min(
            self.lights.get_width() as i32 - 1,
            camera.get_bottom_right(graphics).0 as i32,
        );
        let end_y = i32::min(
            self.lights.get_height() as i32 - 1,
            camera.get_bottom_right(graphics).1 as i32,
        );

        let extended_view_distance = 10;
        let extended_start_x = i32::max(0, start_x - extended_view_distance);
        let extended_start_y = i32::max(0, start_y - extended_view_distance);
        let extended_end_x = i32::min(
            self.lights.get_width() as i32 - 1,
            end_x + extended_view_distance,
        );
        let extended_end_y = i32::min(
            self.lights.get_height() as i32 - 1,
            end_y + extended_view_distance,
        );

        let mut updated = true;
        while updated {
            updated = false;
            for chunk_x in extended_start_x / CHUNK_SIZE..=extended_end_x / CHUNK_SIZE {
                for chunk_y in extended_start_y / CHUNK_SIZE..=extended_end_y / CHUNK_SIZE {
                    if self
                        .lights
                        .get_light_chunk(chunk_x, chunk_y)?
                        .scheduled_light_update_count
                        != 0
                    {
                        for x in chunk_x * CHUNK_SIZE..(chunk_x + 1) * CHUNK_SIZE {
                            for y in chunk_y * CHUNK_SIZE..(chunk_y + 1) * CHUNK_SIZE {
                                self.lights.update_light_emitter(x, y, blocks)?;
                                if self.lights.get_light(x, y)?.scheduled_light_update {
                                    self.lights.update_light(x, y, blocks, events)?;
                                    updated = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        for x in start_x / CHUNK_SIZE..=end_x / CHUNK_SIZE {
            for y in start_y / CHUNK_SIZE..=end_y / CHUNK_SIZE {
                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self
                    .chunks
                    .get_mut(chunk_index)
                    .ok_or_else(|| anyhow!("Chunk array malformed"))?;

                chunk.render(
                    graphics,
                    x * CHUNK_SIZE,
                    y * CHUNK_SIZE,
                    &self.lights,
                    camera,
                )?;
            }
        }

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, blocks: &Blocks) -> Result<()> {
        self.lights.on_event(event, blocks)?;

        if let Some(event) = event.downcast::<LightColorChangeEvent>() {
            let pos = [
                (event.x / CHUNK_SIZE, event.y / CHUNK_SIZE),
                ((event.x + 1) / CHUNK_SIZE, event.y / CHUNK_SIZE),
                (event.x / CHUNK_SIZE, (event.y + 1) / CHUNK_SIZE),
                ((event.x + 1) / CHUNK_SIZE, (event.y + 1) / CHUNK_SIZE),
            ];

            for (x, y) in pos {
                if y < 0
                    || y >= self.lights.get_height() as i32 / CHUNK_SIZE
                    || x < 0
                    || x >= self.lights.get_width() as i32 / CHUNK_SIZE
                {
                    continue;
                }

                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self
                    .chunks
                    .get_mut(chunk_index)
                    .ok_or_else(|| anyhow!("Chunk array malformed"))?;

                chunk.needs_update = true;
            }
        }
        Ok(())
    }

    pub fn stop(&self, settings: &mut Settings) -> Result<()> {
        settings.remove_setting(self.lights_setting)
    }
}
