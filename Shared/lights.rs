use super::{blocks::*};
use {events::*, shared_mut::SharedMut};

const MAX_LIGHT: i32 = 100;

/**event that fires when the light color is changed*/
pub struct LightColorChangeEvent {
    pub x: i32,
    pub y: i32,
}
impl LightColorChangeEvent {
    pub fn new(x: i32, y: i32) -> Self {
        LightColorChangeEvent{ x, y }
    }
}

/**event that fires when the light is scheduled to be updated*/
pub struct LightUpdateScheduleEvent {
    pub x: i32,
    pub y: i32,
}
impl LightUpdateScheduleEvent {
    pub fn new(x: i32, y: i32) -> Self {
        LightUpdateScheduleEvent{ x, y }
    }
}
impl Event for LightColorChangeEvent {}

/**struct that contains the light rgb values*/
#[derive(PartialEq)]
pub struct LightColor {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}
impl LightColor {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        LightColor { r, g, b }
    }
}

/**struct that contains the light data for a given coordinate*/
struct Light {
    pub color: LightColor,
    pub source_color: LightColor,
    pub source: bool,
    pub update_light: bool,
}
impl Light {
    pub fn new() -> Self {
        Light {
            color: LightColor::new(0, 0, 0),
            source_color: LightColor::new(0, 0, 0),
            source: false,
            update_light: true,
        }
    }
}
impl Event for LightUpdateScheduleEvent {}

/**struct that manages all the lights in the world*/
struct Lights {
    blocks: SharedMut<Blocks>,
    lights: Vec<Light>,
    pub light_color_change_event: Sender<LightColorChangeEvent>,
    pub light_update_schedule_event: Sender<LightUpdateScheduleEvent>,
}

impl Lights {
    pub fn new(blocks: SharedMut<Blocks>) -> Self {

        Lights {
            blocks,
            lights: Vec::new(),
            light_color_change_event: Sender::new(),
            light_update_schedule_event: Sender::new(),
        }
    }
    /**returns the light at the given coordinate*/
    fn get_light(&self, x: i32, y: i32) -> &Light {
        &self.lights[(x + y * self.blocks.get_lock().get_width()) as usize]
    }
    /**returns mutable light at the given coordinate*/
    fn get_light_mut(&mut self, x: i32, y: i32) -> &mut Light {
        &mut self.lights[(x + y * self.blocks.get_lock().get_width()) as usize]
    }
    /**creates an empty light vector*/
    pub fn create(&mut self) {
        let blocks = self.blocks.get_lock();
        for _ in 0..blocks.get_width() * blocks.get_height() {
            self.lights.push(Light::new());
        }
    }
    /**adds a block change event listener to blocks*/
    pub fn init(&self, self_shared_mut: SharedMut<Lights>) {
        self.blocks.get_lock().block_change_event.add_listener(&self_shared_mut);
    }
    /*pub fn stop(&self, self_shared_mut: SharedMut<Lights>) {//TODO: does remove_listener exist?
        self.blocks.get_lock().block_change_event.remove_listener(self_shared_mut);
    }*/
    /**returns the wodth of the light vector*/
    pub fn get_width(&self) -> i32 {
        self.blocks.get_lock().get_width()
    }
    /**returns the height of the light vector*/
    pub fn get_height(&self) -> i32 {
        self.blocks.get_lock().get_height()
    }
    /**updates the light at the given coordinate*/
    pub fn update(&mut self, x: i32, y: i32) {
        self.get_light_mut(x, y).update_light = false;
        let blocks = self.blocks.get_lock();
        //TODO: update emitter

        let mut neighbours = [[-1, 0], [-1, 0], [-1, 0], [-1, 0]];

        if x != 0 {
            neighbours[0][0] = x - 1;
            neighbours[0][1] = y;
        }
        if x != blocks.get_width() - 1 {
            neighbours[1][0] = x + 1;
            neighbours[1][1] = y;
        }
        if y != 0 {
            neighbours[2][0] = x;
            neighbours[2][1] = y - 1;
        }
        if y != blocks.get_height() - 1 {
            neighbours[3][0] = x;
            neighbours[3][1] = y + 1;
        }

        let color_to_be = LightColor::new(0, 0, 0);
        //TODO finish
    }
}

impl Listener<BlockChangeEvent> for Lights {
    fn on_event(&mut self, event: &BlockChangeEvent) {
        //TODO: update light
    }
}