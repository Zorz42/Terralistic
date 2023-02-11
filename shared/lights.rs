/*use crate::shared::blocks::Blocks;
use std::cmp::max;

pub const MAX_LIGHT: i32 = 100;

/**event that fires when the light color is changed*/
pub struct LightColorChangeEvent {
    pub x: i32,
    pub y: i32,
}
/**event that fires when the light is scheduled to be updated*/
pub struct LightUpdateScheduleEvent {
    pub x: i32,
    pub y: i32,
}

/**struct that contains the light rgb values*/
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LightColor {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}
impl LightColor {
    #[must_use]
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self { r, g, b }
    }
}

/**struct that contains the light data for a given coordinate*/
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
    pub fn new() -> Self {
        Self {
            color: LightColor::new(0, 0, 0),
            source_color: LightColor::new(0, 0, 0),
            source: false,
            update_light: true,
        }
    }
}

/**struct that manages all the lights in the world*/
pub struct Lights {
    lights: Vec<Light>,
    width: u32,
    height: u32,
    //pub light_color_change_event: Sender<LightColorChangeEvent>,
    //pub light_update_schedule_event: Sender<LightUpdateScheduleEvent>,
}

impl Lights {
    #[must_use]
    pub fn new(blocks: &Blocks) -> Self {
        Self {
            lights: Vec::new(),
            width: blocks.get_width(),
            height: blocks.get_height(),
            //light_color_change_event: Sender::new(),
            //light_update_schedule_event: Sender::new(),
        }
    }
    /**returns the light at the given coordinate*/
    fn get_light(&self, x: i32, y: i32) -> &Light {
        &self.lights[(x + y * self.width as i32) as usize]
    }
    /**returns mutable light at the given coordinate*/
    fn get_light_mut(&mut self, x: i32, y: i32) -> &mut Light {
        &mut self.lights[(x + y * self.width as i32) as usize]
    }
    /**sets the light color at the given coordinate*/
    fn set_light_color(&mut self, x: i32, y: i32, color: LightColor) {
        if self.get_light(x, y).color != color {
            self.get_light_mut(x, y).color = color;
            //self.light_color_change_event.send(LightColorChangeEvent::new(x, y));
            if x < self.width as i32 - 1 {
                self.get_light_mut(x + 1, y).update_light = true;
            }
            if x > 0 {
                self.get_light_mut(x - 1, y).update_light = true;
            }
            if y < self.height as i32 - 1 {
                self.get_light_mut(x, y + 1).update_light = true;
            }
            if y > 0 {
                self.get_light_mut(x, y - 1).update_light = true;
            }
        }
    }
    /**creates an empty light vector*/
    pub fn create(&mut self, blocks: &Blocks) {
        for _ in 0..blocks.get_width() * blocks.get_height() {
            self.lights.push(Light::new());
        }
    }
    /**adds a block change event listener to blocks*/
    /*pub fn init(&self, self_shared_mut: SharedMut<Lights>) {//TODO: implement new events
        //self.blocks.borrow().block_change_event.add_listener(&self_shared_mut);
    }*/
    /**returns the wodth of the light vector*/
    #[must_use]
    pub fn get_width(&self) -> u32 {
        self.width
    }
    /**returns the height of the light vector*/
    #[must_use]
    pub fn get_height(&self) -> u32 {
        self.height
    }
    /**updates the light at the given coordinate*/
    pub fn update_light(&mut self, x: i32, y: i32, blocks: &Blocks) {
        self.get_light_mut(x, y).update_light = false;
        self.update_light_emitter(x, y, blocks);

        let mut neighbours = [[-1, 0], [-1, 0], [-1, 0], [-1, 0]];
        {
            if x != 0 {
                neighbours[0][0] = x - 1;
                neighbours[0][1] = y;
            }
            if x != self.width as i32 - 1 {
                neighbours[1][0] = x + 1;
                neighbours[1][1] = y;
            }
            if y != 0 {
                neighbours[2][0] = x;
                neighbours[2][1] = y - 1;
            }
            if y != self.height as i32 - 1 {
                neighbours[3][0] = x;
                neighbours[3][1] = y + 1;
            }
        }

        let mut color_to_be = LightColor::new(0, 0, 0);
        for neighbour in neighbours {
            if neighbour[0] != -1 {
                let light_step = if blocks
                    .get_block_type_at(neighbour[0], neighbour[1])
                    .unwrap()
                    .transparent
                {
                    3
                } else {
                    15
                };

                let r = if light_step > self.get_light_color(neighbour[0], neighbour[1]).r {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1]).r - light_step
                };
                let g = if light_step > self.get_light_color(neighbour[0], neighbour[1]).g {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1]).g - light_step
                };
                let b = if light_step > self.get_light_color(neighbour[0], neighbour[1]).b {
                    0
                } else {
                    self.get_light_color(neighbour[0], neighbour[1]).b - light_step
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

        if self.is_light_source(x, y) {
            color_to_be.r = max(color_to_be.r, self.get_light_source_color(x, y).r);
            color_to_be.g = max(color_to_be.g, self.get_light_source_color(x, y).g);
            color_to_be.b = max(color_to_be.b, self.get_light_source_color(x, y).b);
        }

        if color_to_be != self.get_light_color(x, y) {
            self.set_light_color(x, y, color_to_be);
            self.schedule_light_update(x, y);
        }
    }
    /**sets the coordinates x and y to be a light source*/
    pub fn set_light_source(&mut self, x: i32, y: i32, color: LightColor) {
        self.get_light_mut(x, y).source = color != LightColor::new(0, 0, 0);
        if self.get_light(x, y).source_color != color {
            self.get_light_mut(x, y).source_color = color;
            self.schedule_light_update_for_neighbours(x, y);
        }
    }
    /**returns whether the coordinates x and y are a light source*/
    #[must_use]
    pub fn is_light_source(&self, x: i32, y: i32) -> bool {
        self.get_light(x, y).source
    }
    /**returns the light color at the given coordinate*/
    #[must_use]
    pub fn get_light_color(&self, x: i32, y: i32) -> LightColor {
        self.get_light(x, y).color
    }
    /**returns the light source color at the given coordinate*/
    #[must_use]
    pub fn get_light_source_color(&self, x: i32, y: i32) -> LightColor {
        self.get_light(x, y).source_color
    }
    /**schedules a light update for the given coordinate*/
    pub fn schedule_light_update(&mut self, x: i32, y: i32) {
        self.get_light_mut(x, y).update_light = true;
        //self.light_update_schedule_event.send(LightUpdateScheduleEvent::new(x, y));
    }
    /**returns whether the light at the given coordinate needs to be updated*/
    #[must_use]
    pub fn has_scheduled_light_update(&self, x: i32, y: i32) -> bool {
        self.get_light(x, y).update_light
    }
    /**schedules a light update for the neighbours of the given coordinate*/
    pub fn schedule_light_update_for_neighbours(&mut self, x: i32, y: i32) {
        let width = self.width;
        let height = self.height;
        self.schedule_light_update(x, y);
        if x != 0 {
            self.schedule_light_update(x - 1, y);
        }
        if x != width as i32 - 1 {
            self.schedule_light_update(x + 1, y);
        }
        if y != 0 {
            self.schedule_light_update(x, y - 1);
        }
        if y != height as i32 - 1 {
            self.schedule_light_update(x, y + 1);
        }
    }
    /**updates the light emitter at the given coordinate*/
    pub fn update_light_emitter(&mut self, x: i32, y: i32, blocks: &Blocks) {
        let block_type = blocks.get_block_type_at(x, y).unwrap();
        if block_type.get_id() != blocks.air {
            self.set_light_source(
                x,
                y,
                LightColor::new(
                    block_type.light_emission_r as i32,
                    block_type.light_emission_g as i32,
                    block_type.light_emission_b as i32,
                ),
            );
        }
    }
}

/*impl Listener<BlockChangeEvent> for Lights {
    fn on_event(&mut self, event: &BlockChangeEvent) {
        self.schedule_light_update(event.x, event.y);
    }
}*/
*/
