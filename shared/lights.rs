use anyhow::{anyhow, Result};

use crate::libraries::events::Event;
use crate::libraries::grid::{Chunks, Grid};
use crate::shared::blocks::{BlockChangeEvent, Blocks};
use crate::shared::CHUNK_SIZE;

/// struct that contains the light rgb values
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LightColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LightColor {
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// struct that contains the light data for a given coordinate
#[derive(Clone, Copy)]
pub struct Light {
    pub color: LightColor,
    pub source_color: LightColor,
    pub is_source: bool,
    pub scheduled_light_update: bool,
}

impl Light {
    #[must_use]
    const fn new() -> Self {
        Self {
            color: LightColor::new(0, 0, 0),
            source_color: LightColor::new(0, 0, 0),
            is_source: false,
            scheduled_light_update: true,
        }
    }
}

/// struct that contains light data for a given chunk
#[derive(Clone, Copy)]
pub struct LightChunk {
    pub scheduled_light_update_count: i32,
}

impl LightChunk {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scheduled_light_update_count: CHUNK_SIZE * CHUNK_SIZE,
        }
    }
}

/// struct that manages all the lights in the world
pub struct Lights {
    lights: Grid<Light>,
    light_chunks: Vec<LightChunk>,
    chunks: Chunks,
    sky_heights: Vec<i32>,
}

impl Lights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lights: Grid::new_empty(),
            light_chunks: Vec::new(),
            chunks: Chunks::new((0, 0), CHUNK_SIZE),
            sky_heights: Vec::new(),
        }
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.lights.get_size()
    }

    /// returns the light at the given coordinate
    pub fn get_light(&self, x: i32, y: i32) -> Result<&Light> {
        self.lights.get(x, y)
    }

    /// returns mutable light at the given coordinate
    fn get_light_mut(&mut self, x: i32, y: i32) -> Result<&mut Light> {
        self.lights.get_mut(x, y)
    }

    /// returns the light chunk at the given coordinate
    pub fn get_light_chunk(&self, x: i32, y: i32) -> Result<&LightChunk> {
        self.light_chunks
            .get(self.chunks.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Light chunk not found! x: {x}, y: {y}"))
    }

    /// returns mutable light chunk at the given chunk coordinate
    fn get_light_chunk_mut(&mut self, x: i32, y: i32) -> Result<&mut LightChunk> {
        self.light_chunks
            .get_mut(self.chunks.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Light chunk not found! x: {x}, y: {y}"))
    }

    /// Returns the mutable light chunk containing the given *cell*, rather than the chunk
    /// with those chunk coordinates. The scheduled update counts are per chunk while
    /// everything that changes them is per cell, so this is the conversion those callers
    /// were doing by hand.
    fn get_light_chunk_mut_at(&mut self, cell_x: i32, cell_y: i32) -> Result<&mut LightChunk> {
        let (chunk_x, chunk_y) = self.chunks.chunk_at(cell_x, cell_y);
        self.get_light_chunk_mut(chunk_x, chunk_y)
    }

    /// sets the light color at the given coordinate
    fn set_light_color(&mut self, x: i32, y: i32, color: LightColor) -> Result<()> {
        if self.get_light(x, y)?.color != color {
            self.get_light_mut(x, y)?.color = color;

            self.schedule_light_update(x + 1, y).ok();
            self.schedule_light_update(x - 1, y).ok();
            self.schedule_light_update(x, y + 1).ok();
            self.schedule_light_update(x, y - 1).ok();
        }
        Ok(())
    }

    /// creates an empty light vector
    pub fn create(&mut self, size: (u32, u32)) {
        self.lights = Grid::filled(size, Light::new());
        self.chunks = Chunks::new(size, CHUNK_SIZE);
        self.light_chunks = vec![LightChunk::new(); self.chunks.count()];
        self.sky_heights = vec![-1; size.0 as usize];
    }

    /// initializes sky heights
    pub fn init_sky_heights(&mut self, blocks: &Blocks) -> Result<()> {
        for x in 0..self.lights.get_size().0 {
            for y in 0..self.lights.get_size().1 {
                if blocks.get_block_type_at(x as i32, y as i32)?.transparent {
                    *self.sky_heights.get_mut(x as usize).ok_or_else(|| anyhow!("sky_heights out of bounds"))? = y as i32;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    /// updates the light at the given coordinate
    pub fn update_light(&mut self, x: i32, y: i32, blocks: &Blocks) -> Result<()> {
        if self.get_light_mut(x, y)?.scheduled_light_update {
            self.get_light_mut(x, y)?.scheduled_light_update = false;
            self.get_light_chunk_mut_at(x, y)?.scheduled_light_update_count -= 1;
        }
        self.update_light_emitter(x, y, blocks)?;

        let mut neighbours = [[-1, 0], [-1, 0], [-1, 0], [-1, 0]];
        {
            if x != 0 {
                neighbours[0][0] = x - 1;
                neighbours[0][1] = y;
            }
            if x != self.lights.get_size().0 as i32 - 1 {
                neighbours[1][0] = x + 1;
                neighbours[1][1] = y;
            }
            if y != 0 {
                neighbours[2][0] = x;
                neighbours[2][1] = y - 1;
            }
            if y != self.lights.get_size().1 as i32 - 1 {
                neighbours[3][0] = x;
                neighbours[3][1] = y + 1;
            }
        }

        let mut color_to_be = LightColor::new(0, 0, 0);
        for neighbour in neighbours {
            if neighbour[0] != -1 {
                let light_step = if blocks.get_block_type_at(neighbour[0], neighbour[1])?.transparent { 0.95 } else { 0.7 };

                let r = (self.get_light(neighbour[0], neighbour[1])?.color.r as f32 * light_step).floor() as u8;
                let g = (self.get_light(neighbour[0], neighbour[1])?.color.g as f32 * light_step).floor() as u8;
                let b = (self.get_light(neighbour[0], neighbour[1])?.color.b as f32 * light_step).floor() as u8;

                if r > color_to_be.r {
                    color_to_be.r = r;
                }
                if g > color_to_be.g {
                    color_to_be.g = g;
                }
                if b > color_to_be.b {
                    color_to_be.b = b;
                }
            }
        }

        if self.get_light(x, y)?.is_source {
            color_to_be.r = u8::max(color_to_be.r, self.get_light(x, y)?.source_color.r);
            color_to_be.g = u8::max(color_to_be.g, self.get_light(x, y)?.source_color.g);
            color_to_be.b = u8::max(color_to_be.b, self.get_light(x, y)?.source_color.b);
        }

        if color_to_be != self.get_light(x, y)?.color {
            self.set_light_color(x, y, color_to_be)?;
            self.schedule_light_update(x, y)?;
        }
        Ok(())
    }

    /// sets the coordinates x and y to be a light source
    pub fn set_light_source(&mut self, x: i32, y: i32, color: LightColor) -> Result<()> {
        self.get_light_mut(x, y)?.is_source = color != LightColor::new(0, 0, 0);
        if self.get_light(x, y)?.source_color != color {
            self.get_light_mut(x, y)?.source_color = color;
            self.schedule_light_update_for_neighbours(x, y);
        }
        Ok(())
    }

    /// schedules a light update for the given coordinate
    pub fn schedule_light_update(&mut self, x: i32, y: i32) -> Result<()> {
        if !self.get_light_mut(x, y)?.scheduled_light_update {
            self.get_light_mut(x, y)?.scheduled_light_update = true;
            self.get_light_chunk_mut_at(x, y)?.scheduled_light_update_count += 1;
        }
        Ok(())
    }

    /// schedules a light update for the neighbours of the given coordinate
    pub fn schedule_light_update_for_neighbours(&mut self, x: i32, y: i32) {
        // ignore all out of bounds errors
        self.schedule_light_update(x, y).ok();
        self.schedule_light_update(x - 1, y).ok();
        self.schedule_light_update(x + 1, y).ok();
        self.schedule_light_update(x, y - 1).ok();
        self.schedule_light_update(x, y + 1).ok();
    }

    /// updates the light emitter at the given coordinate
    pub fn update_light_emitter(&mut self, x: i32, y: i32, blocks: &Blocks) -> Result<()> {
        let block_type = blocks.get_block_type_at(x, y)?;
        let mut light_emission_r = block_type.light_emission.0;
        let mut light_emission_g = block_type.light_emission.1;
        let mut light_emission_b = block_type.light_emission.2;

        if y <= *self.sky_heights.get(x as usize).unwrap_or(&-1) {
            light_emission_r = u8::max(light_emission_r, 255);
            light_emission_g = u8::max(light_emission_g, 255);
            light_emission_b = u8::max(light_emission_b, 255);
        }

        self.set_light_source(x, y, LightColor::new(light_emission_r, light_emission_g, light_emission_b))?;
        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, blocks: &Blocks) -> Result<()> {
        if let Some(event) = event.downcast::<BlockChangeEvent>() {
            let curr_block_transparent = blocks.get_block_type_at(event.x, event.y)?.transparent;
            let prev_block_transparent = blocks.get_block_type(event.prev_block)?.transparent;

            if curr_block_transparent && !prev_block_transparent && *self.sky_heights.get(event.x as usize).unwrap_or(&-1) == event.y - 1 {
                let mut sky_height = event.y;
                while sky_height < self.get_size().1 as i32 && blocks.get_block_type_at(event.x, sky_height)?.transparent {
                    sky_height += 1;
                }

                *self.sky_heights.get_mut(event.x as usize).ok_or_else(|| anyhow!("sky_heights out of bounds"))? = sky_height - 1;
            } else if !curr_block_transparent && prev_block_transparent && *self.sky_heights.get(event.x as usize).unwrap_or(&-1) >= event.y {
                *self.sky_heights.get_mut(event.x as usize).ok_or_else(|| anyhow!("sky_heights out of bounds"))? = event.y - 1;
            }

            self.schedule_light_update_for_neighbours(event.x, event.y);
        }
        Ok(())
    }
}
