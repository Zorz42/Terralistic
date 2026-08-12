use anyhow::{anyhow, bail, Result};
use std::cell::RefCell;
use std::rc::Rc;

use crate::client::game::camera::Camera;
use crate::client::game::chunk_tracker::ChunkTracker;
use crate::client::settings::{Setting, Settings};
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::DrawTarget;
use crate::shared::blocks::{Blocks, RENDER_BLOCK_WIDTH};
use crate::shared::lights::Lights;
use crate::shared::world_map::CHUNK_SIZE;

const MAX_LOADED_CHUNKS: usize = 1000;

pub struct LightChunk {
    pub rect_array: gfx::RectArray,
    pub needs_update: bool,
}

impl LightChunk {
    pub const fn new() -> Self {
        Self {
            rect_array: gfx::RectArray::new(),
            needs_update: true,
        }
    }

    pub fn clear(&mut self) {
        self.rect_array = gfx::RectArray::new();
        self.needs_update = true;
    }

    pub fn update(&mut self, world_x: i32, world_y: i32, lights: &Lights, frame_timer: &std::time::Instant) -> Result<bool> {
        if self.needs_update && frame_timer.elapsed().as_millis() < 10 {
            self.needs_update = false;

            self.rect_array = gfx::RectArray::new();
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let light_1 = lights.get_light(i32::max(world_x + x - 1, 0), i32::max(world_y + y - 1, 0))?.color;
                    let light_2 = lights.get_light(world_x + x, i32::max(world_y + y - 1, 0))?.color;
                    let light_3 = lights.get_light(i32::max(world_x + x - 1, 0), world_y + y)?.color;
                    let light_4 = lights.get_light(world_x + x, world_y + y)?.color;

                    self.rect_array.add_rect(
                        &gfx::Rect::new(
                            gfx::FloatPos(x as f32 * RENDER_BLOCK_WIDTH, y as f32 * RENDER_BLOCK_WIDTH),
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

            return Ok(true);
        }

        Ok(false)
    }

    pub fn render(&self, graphics: &gfx::GraphicsContext, world_x: i32, world_y: i32, camera: &Camera) {
        let screen_x = world_x as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 * RENDER_BLOCK_WIDTH;
        let screen_y = world_y as f32 * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 * RENDER_BLOCK_WIDTH;
        graphics.set_blend_mode(gfx::BlendMode::Multiply);
        self.rect_array.render(graphics, None, gfx::FloatPos(screen_x.round(), screen_y.round()));
        graphics.set_blend_mode(gfx::BlendMode::Alpha);
    }
}

pub struct ClientLights {
    pub lights: Lights,
    chunks: Vec<LightChunk>,
    lights_setting: i32,
    chunk_tracker: ChunkTracker,
}

impl ClientLights {
    #[must_use]
    pub fn new() -> Self {
        Self {
            lights: Lights::new(),
            chunks: Vec::new(),
            lights_setting: 0,
            chunk_tracker: ChunkTracker::new(0),
        }
    }

    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0 || y < 0 || x >= self.lights.get_size().0 as i32 / CHUNK_SIZE || y >= self.lights.get_size().1 as i32 / CHUNK_SIZE {
            bail!("Tried to get light chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.lights.get_size().0 as i32 / CHUNK_SIZE)) as usize)
    }

    pub fn init(&mut self, blocks: &Blocks, settings: &Rc<RefCell<Settings>>) -> Result<()> {
        self.lights.create(blocks.get_size());
        self.lights.init_sky_heights(blocks)?;

        let chunk_width = (self.lights.get_size().0 as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_height = (self.lights.get_size().1 as f32 / RENDER_BLOCK_WIDTH) as usize;
        let chunk_count = chunk_width * chunk_height;
        for _ in 0..chunk_count {
            self.chunks.push(LightChunk::new());
        }

        self.chunk_tracker = ChunkTracker::new(chunk_count);

        let lights_settings = Setting::Toggle {
            text: "Enable lights".to_owned(),
            config_label: "enable_Lights".to_owned(),
            toggled: true,
        };

        self.lights_setting = settings.borrow_mut().register_setting(lights_settings);

        Ok(())
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, camera: &Camera, blocks: &Blocks, settings: &Rc<RefCell<Settings>>, frame_timer: &std::time::Instant) -> Result<()> {
        if let Setting::Toggle { toggled, .. } = settings.borrow_mut().get_setting(self.lights_setting)? {
            if !toggled {
                return Ok(());
            }
        }

        let width = self.lights.get_size().0 as i32;
        let height = self.lights.get_size().1 as i32;

        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);

        let (start_x, start_y) = (i32::max(0, top_left_x as i32 / CHUNK_SIZE), i32::max(0, top_left_y as i32 / CHUNK_SIZE));
        let (end_x, end_y) = (
            i32::min(width / CHUNK_SIZE, bottom_right_x as i32 / CHUNK_SIZE + 1),
            i32::min(height / CHUNK_SIZE, bottom_right_y as i32 / CHUNK_SIZE + 1),
        );

        let extended_view_distance = 5;
        let (extended_start_x, extended_start_y) = (i32::max(0, start_x - extended_view_distance), i32::max(0, start_y - extended_view_distance));
        let (extended_end_x, extended_end_y) = (
            i32::min(width / CHUNK_SIZE, end_x + extended_view_distance),
            i32::min(height / CHUNK_SIZE, end_y + extended_view_distance),
        );

        let mut updated = true;
        while updated {
            updated = false;
            for chunk_x in extended_start_x..extended_end_x {
                for chunk_y in extended_start_y..extended_end_y {
                    if self.lights.get_light_chunk(chunk_x, chunk_y)?.scheduled_light_update_count != 0 {
                        for x in chunk_x * CHUNK_SIZE..(chunk_x + 1) * CHUNK_SIZE {
                            for y in chunk_y * CHUNK_SIZE..(chunk_y + 1) * CHUNK_SIZE {
                                self.lights.update_light_emitter(x, y, blocks)?;
                                if self.lights.get_light(x, y)?.scheduled_light_update {
                                    self.lights.update_light(x, y, blocks)?;
                                    updated = true;
                                }
                            }
                        }

                        let pos = [(chunk_x, chunk_y), (chunk_x + 1, chunk_y), (chunk_x, chunk_y + 1), (chunk_x + 1, chunk_y + 1)];

                        for (x, y) in pos {
                            if y < 0 || y >= self.lights.get_size().1 as i32 / CHUNK_SIZE || x < 0 || x >= self.lights.get_size().0 as i32 / CHUNK_SIZE {
                                continue;
                            }

                            let chunk_index = self.get_chunk_index(x, y)?;
                            let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                            chunk.needs_update = true;
                        }
                    }
                }
            }
        }

        for chunk_x in extended_start_x..extended_end_x {
            for chunk_y in extended_start_y..extended_end_y {
                let chunk_index = self.get_chunk_index(chunk_x, chunk_y)?;
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                let has_updated = chunk.update(chunk_x * CHUNK_SIZE, chunk_y * CHUNK_SIZE, &self.lights, frame_timer)?;
                if has_updated {
                    self.chunk_tracker.update(chunk_index)?;
                }
            }
        }

        for x in start_x..end_x {
            for y in start_y..end_y {
                let chunk_index = self.get_chunk_index(x, y)?;
                let chunk = self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?;

                chunk.render(graphics, x * CHUNK_SIZE, y * CHUNK_SIZE, camera);
            }
        }

        while self.chunk_tracker.get_num_chunks() > MAX_LOADED_CHUNKS {
            let chunk_index = self.chunk_tracker.get_oldest_chunk()?;

            self.chunks.get_mut(chunk_index).ok_or_else(|| anyhow!("Chunk array malformed"))?.clear();
            self.chunk_tracker.remove_chunk(chunk_index)?;
        }

        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, blocks: &Blocks) -> Result<()> {
        self.lights.on_event(event, blocks)
    }

    pub fn stop(&self, settings: &Rc<RefCell<Settings>>) -> Result<()> {
        settings.borrow_mut().remove_setting(self.lights_setting)
    }
}
