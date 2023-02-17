pub struct IdComponent {
    pub id: u32,
}

pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
}

pub struct VelocityComponent {
    pub x: f32,
    pub y: f32,
}

pub struct Entities {
    pub ecs: hecs::World,
    current_id: u32,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            ecs: hecs::World::new(),
            current_id: 0,
        }
    }

    pub fn update_entities(&mut self, delta_time: f32) {}

    pub fn unwrap_id(&mut self, id: Option<u32>) -> u32 {
        match id {
            Some(id) => id,
            None => {
                self.current_id += 1;
                self.current_id
            }
        }
    }
}
