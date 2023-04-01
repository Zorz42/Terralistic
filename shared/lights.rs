use crate::libraries::events::Event;
use crate::shared::blocks::{BlockChangeEvent, Blocks};
use crate::shared::world_map::WorldMap;
use anyhow::{anyhow, Result};

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
    pub source: bool,
    pub update_light: bool,
}

impl Default for Light {
    fn default() -> Self {
        Self::new()
    }
}

impl Light {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            color: LightColor::new(0, 0, 0),
            source_color: LightColor::new(0, 0, 0),
            source: false,
            update_light: true,
        }
    }
}

/// struct that manages all the lights in the world
pub struct Lights {
    lights: Vec<Light>,
    map: WorldMap,
    sky_heights: Vec<i32>,
}

impl Lights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            lights: Vec::new(),
            map: WorldMap::new_empty(),
            sky_heights: Vec::new(),
        }
    }

    /// returns the width of the light vector
    #[must_use]
    pub const fn get_width(&self) -> u32 {
        self.map.get_width()
    }

    /// returns the height of the light vector
    #[must_use]
    pub const fn get_height(&self) -> u32 {
        self.map.get_height()
    }

    /// returns the light at the given coordinate
    fn get_light(&self, x: i32, y: i32) -> Result<&Light> {
        self.lights
            .get(self.map.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Light not found! x: {}, y: {}", x, y))
    }

    /// returns mutable light at the given coordinate
    fn get_light_mut(&mut self, x: i32, y: i32) -> Result<&mut Light> {
        self.lights
            .get_mut(self.map.translate_coords(x, y)?)
            .ok_or_else(|| anyhow!("Light not found! x: {}, y: {}", x, y))
    }

    /// sets the light color at the given coordinate
    fn set_light_color(&mut self, x: i32, y: i32, color: LightColor) -> Result<()> {
        if self.get_light(x, y)?.color != color {
            self.get_light_mut(x, y)?.color = color;
            //self.light_color_change_event.send(LightColorChangeEvent::new(x, y));
            if x < self.map.get_width() as i32 - 1 {
                self.get_light_mut(x + 1, y)?.update_light = true;
            }
            if x > 0 {
                self.get_light_mut(x - 1, y)?.update_light = true;
            }
            if y < self.map.get_height() as i32 - 1 {
                self.get_light_mut(x, y + 1)?.update_light = true;
            }
            if y > 0 {
                self.get_light_mut(x, y - 1)?.update_light = true;
            }
        }
        Ok(())
    }

    /// creates an empty light vector
    pub fn create(&mut self, width: u32, height: u32) {
        self.lights = vec![Light::new(); (width * height) as usize];
        self.map = WorldMap::new(width, height);
        self.sky_heights = vec![-1; width as usize];
    }

    /// initializes sky heights
    /// # Errors
    /// returns an error if the block at the given coordinate is not found
    pub fn init_sky_heights(&mut self, blocks: &Blocks) -> Result<()> {
        for x in 0..self.map.get_width() {
            for y in 0..self.map.get_height() {
                if blocks.get_block_type_at(x as i32, y as i32)?.transparent {
                    *self
                        .sky_heights
                        .get_mut(x as usize)
                        .ok_or_else(|| anyhow!("sky_heights out of bounds"))? = y as i32;
                } else {
                    break;
                }
            }
        }
        Ok(())
    }

    /// updates the light at the given coordinate
    /// # Errors
    /// returns an error if the light at the given coordinate is not found
    pub fn update_light(&mut self, x: i32, y: i32, blocks: &Blocks) -> Result<()> {
        self.get_light_mut(x, y)?.update_light = false;
        self.update_light_emitter(x, y, blocks)?;

        let mut neighbours = [[-1, 0], [-1, 0], [-1, 0], [-1, 0]];
        {
            if x != 0 {
                neighbours[0][0] = x - 1;
                neighbours[0][1] = y;
            }
            if x != self.map.get_width() as i32 - 1 {
                neighbours[1][0] = x + 1;
                neighbours[1][1] = y;
            }
            if y != 0 {
                neighbours[2][0] = x;
                neighbours[2][1] = y - 1;
            }
            if y != self.map.get_height() as i32 - 1 {
                neighbours[3][0] = x;
                neighbours[3][1] = y + 1;
            }
        }

        let mut color_to_be = LightColor::new(0, 0, 0);
        for neighbour in neighbours {
            if neighbour[0] != -1 {
                let light_step = if blocks
                    .get_block_type_at(neighbour[0], neighbour[1])?
                    .transparent
                {
                    8
                } else {
                    40
                };

                let r = if light_step > self.get_light_color(neighbour[0], neighbour[1])?.r {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1])?.r - light_step
                };
                let g = if light_step > self.get_light_color(neighbour[0], neighbour[1])?.g {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1])?.g - light_step
                };
                let b = if light_step > self.get_light_color(neighbour[0], neighbour[1])?.b {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1])?.b - light_step
                };

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

        if self.is_light_source(x, y)? {
            color_to_be.r = u8::max(color_to_be.r, self.get_light_source_color(x, y)?.r);
            color_to_be.g = u8::max(color_to_be.g, self.get_light_source_color(x, y)?.g);
            color_to_be.b = u8::max(color_to_be.b, self.get_light_source_color(x, y)?.b);
        }

        if color_to_be != self.get_light_color(x, y)? {
            self.set_light_color(x, y, color_to_be)?;
            self.schedule_light_update(x, y)?;
        }
        Ok(())
    }

    /// sets the coordinates x and y to be a light source
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn set_light_source(&mut self, x: i32, y: i32, color: LightColor) -> Result<()> {
        self.get_light_mut(x, y)?.source = color != LightColor::new(0, 0, 0);
        if self.get_light(x, y)?.source_color != color {
            self.get_light_mut(x, y)?.source_color = color;
            self.schedule_light_update_for_neighbours(x, y);
        }
        Ok(())
    }

    /// returns whether the coordinates x and y are a light source
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn is_light_source(&self, x: i32, y: i32) -> Result<bool> {
        Ok(self.get_light(x, y)?.source)
    }

    /// returns the light color at the given coordinate
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn get_light_color(&self, x: i32, y: i32) -> Result<LightColor> {
        Ok(self.get_light(x, y)?.color)
    }

    /// returns the light source color at the given coordinate
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn get_light_source_color(&self, x: i32, y: i32) -> Result<LightColor> {
        Ok(self.get_light(x, y)?.source_color)
    }

    /// schedules a light update for the given coordinate
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn schedule_light_update(&mut self, x: i32, y: i32) -> Result<()> {
        self.get_light_mut(x, y)?.update_light = true;
        //self.light_update_schedule_event.send(LightUpdateScheduleEvent::new(x, y));
        Ok(())
    }

    /// returns whether the light at the given coordinate needs to be updated
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn has_scheduled_light_update(&self, x: i32, y: i32) -> Result<bool> {
        Ok(self.get_light(x, y)?.update_light)
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
    /// # Errors
    /// returns an error if the coordinates are out of bounds
    pub fn update_light_emitter(&mut self, x: i32, y: i32, blocks: &Blocks) -> Result<()> {
        let block_type = blocks.get_block_type_at(x, y)?;
        let mut light_emission_r = block_type.light_emission_r;
        let mut light_emission_g = block_type.light_emission_g;
        let mut light_emission_b = block_type.light_emission_b;

        if y <= *self.sky_heights.get(x as usize).unwrap_or(&-1) {
            light_emission_r = u8::max(light_emission_r, 255);
            light_emission_g = u8::max(light_emission_g, 255);
            light_emission_b = u8::max(light_emission_b, 255);
        }

        self.set_light_source(
            x,
            y,
            LightColor::new(light_emission_r, light_emission_g, light_emission_b),
        )?;
        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, blocks: &Blocks) -> Result<()> {
        if let Some(event) = event.downcast::<BlockChangeEvent>() {
            let curr_block_transparent = blocks.get_block_type_at(event.x, event.y)?.transparent;
            let prev_block_transparent = blocks.get_block_type(event.prev_block)?.transparent;

            if curr_block_transparent
                && !prev_block_transparent
                && *self.sky_heights.get(event.x as usize).unwrap_or(&0) == event.y - 1
            {
                let mut sky_height = event.y;
                while sky_height < self.get_height() as i32
                    && blocks.get_block_type_at(event.x, sky_height)?.transparent
                {
                    sky_height += 1;
                }

                *self
                    .sky_heights
                    .get_mut(event.x as usize)
                    .ok_or_else(|| anyhow!("sky_heights out of bounds"))? = sky_height - 1;
            } else if !curr_block_transparent
                && prev_block_transparent
                && *self.sky_heights.get(event.x as usize).unwrap_or(&0) >= event.y
            {
                *self
                    .sky_heights
                    .get_mut(event.x as usize)
                    .ok_or_else(|| anyhow!("sky_heights out of bounds"))? = event.y - 1;
            }
        }
        Ok(())
    }
}

/// event that fires when the light color is changed
pub struct LightColorChangeEvent {
    pub x: i32,
    pub y: i32,
}

/// event that fires when the light is scheduled to be updated
pub struct LightUpdateScheduleEvent {
    pub x: i32,
    pub y: i32,
}
