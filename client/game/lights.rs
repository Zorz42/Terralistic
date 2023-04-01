use crate::client::game::camera::Camera;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::shared::blocks::{Blocks, CHUNK_SIZE, RENDER_BLOCK_WIDTH};
use crate::shared::lights::{LightColorChangeEvent, Lights};
use anyhow::{anyhow, bail, Result};

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
        graphics: &mut GraphicsContext,
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
                    let light_1 = lights.get_light_color(
                        i32::max(world_x + x - 1, 0),
                        i32::max(world_y + y - 1, 0),
                    )?;
                    let light_2 =
                        lights.get_light_color(world_x + x, i32::max(world_y + y - 1, 0))?;
                    let light_3 =
                        lights.get_light_color(i32::max(world_x + x - 1, 0), world_y + y)?;
                    let light_4 = lights.get_light_color(world_x + x, world_y + y)?;

                    self.rect_array.add_rect(
                        &gfx::Rect::new(
                            FloatPos(x as f32 * RENDER_BLOCK_WIDTH, y as f32 * RENDER_BLOCK_WIDTH),
                            FloatSize(RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH),
                        ),
                        &[
                            gfx::Color::new(light_1.r, light_1.g, light_1.b, 255),
                            gfx::Color::new(light_2.r, light_2.g, light_2.b, 255),
                            gfx::Color::new(light_3.r, light_3.g, light_3.b, 255),
                            gfx::Color::new(light_4.r, light_4.g, light_4.b, 255),
                        ],
                        &gfx::Rect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
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
        self.rect_array
            .render(graphics, None, FloatPos(screen_x.round(), screen_y.round()));
        gfx::set_blend_mode(gfx::BlendMode::Alpha);
        Ok(())
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

    fn get_chunk_index(&self, x: i32, y: i32) -> Result<usize> {
        // check if x and y are in bounds
        if x < 0
            || y < 0
            || x >= self.lights.get_width() as i32 / CHUNK_SIZE
            || y >= self.lights.get_height() as i32 / CHUNK_SIZE
        {
            bail!("Tried to get chunk at {x}, {y} but it is out of bounds");
        }

        Ok((x + y * (self.lights.get_width() as i32 / CHUNK_SIZE)) as usize)
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
        let end_x = camera.get_bottom_right(graphics).0 as i32;
        let end_y = camera.get_bottom_right(graphics).1 as i32;

        let extended_view_distance = 10;
        let extended_start_x = start_x - extended_view_distance;
        let extended_start_y = start_y - extended_view_distance;
        let extended_end_x = end_x + extended_view_distance;
        let extended_end_y = end_y + extended_view_distance;

        loop {
            let mut updated = false;
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
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
        }

        for x in start_x / CHUNK_SIZE..=end_x / CHUNK_SIZE {
            for y in start_y / CHUNK_SIZE..=end_y / CHUNK_SIZE {
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
}
