use std::rc::Rc;
use super::{blocks::blocks::*};
use shared_mut::SharedMut;

const MAX_LIQUID_LEVEL: i32 = 100;

//TODO: new events idk

/**struct with information about a liquid type*/
pub struct LiquidType {
    pub name: String,
    pub flow_time: i32,
    pub speed_multiplier: f64,
    id: i32,
}

impl LiquidType {
    pub fn new(name: String) -> LiquidType {
        LiquidType {
            name: name,
            flow_time: 1,
            speed_multiplier: 1.0,
            id: 0,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

/**struct with information about a liquid*/
struct liquid {
    pub id: i32,
    pub level: f64,
}

impl liquid {
    pub fn new() -> liquid {
        liquid {
            id: 0,
            level: 0.0,
        }
    }
}

/**struct that manages all the liquids*/
pub struct Liquids {
    liquid_types: Vec<Rc<LiquidType>>,
    liquids: Vec<liquid>,
    blocks: SharedMut<Blocks>,
    pub empty: Rc<LiquidType>,
    //TODO: new event sender
}

impl Liquids {
    pub fn new(blocks: SharedMut<Blocks>) -> Liquids {
        let mut temp = LiquidType::new("temp".to_string());
        let mut liquids_object = Liquids {
            liquid_types: Vec::new(),
            liquids: Vec::new(),
            blocks: blocks,
            empty: Rc::new(temp),//temporarily assign
        };
        let mut empty = LiquidType::new("empty".to_string());
        empty.flow_time = 0;
        empty.speed_multiplier = 1.0;
        liquids_object.register_liquid_type(empty);
        liquids_object.empty = liquids_object.liquid_types[0].clone();//assign the real empty liquid Rc
        liquids_object
    }

    /**this function returns a liquid at the given position*/
    fn get_liquid(&self, x: i32, y: i32) -> &liquid {
        if x < 0 || y < 0 || x >= self.blocks.borrow().get_width() || y >= self.blocks.borrow().get_height() {
            panic!("Liquid is accessed out of the bounds! ({}, {})", x, y);
        }
        &self.liquids[(y * self.blocks.borrow().get_width() + x) as usize]
    }

    /**this function returns a mutable liquid at the given position*/
    fn get_liquid_mut(&mut self, x: i32, y: i32) -> &mut liquid {
        if x < 0 || y < 0 || x >= self.blocks.borrow().get_width() || y >= self.blocks.borrow().get_height() {
            panic!("Liquid is accessed out of the bounds! ({}, {})", x, y);
        }
        &mut self.liquids[(y * self.blocks.borrow().get_width() + x) as usize]
    }

    /**returns whether the given liquid is flowable*/
    fn is_flowable(&self, x: i32, y: i32) -> bool {
        self.blocks.borrow().get_block_type(x, y).ghost && self.get_liquid_type(x, y).id == self.empty.id
    }

    /**creates the liquid array*/
    pub fn create(&mut self) {
        self.liquids = Vec::new();
        self.liquids.resize_with((self.blocks.borrow().get_width() * self.blocks.borrow().get_height()) as usize, || liquid::new());
    }

    /**returns the width of the liquid array*/
    pub fn get_width(&self) -> i32 {
        self.blocks.borrow().get_width()
    }

    /**returns the height of the liquid array*/
    pub fn get_height(&self) -> i32 {
        self.blocks.borrow().get_height()
    }

    /**returns the liquid type at the given position*/
    pub fn get_liquid_type(&self, x: i32, y: i32) -> Rc<LiquidType> {
        self.get_liquid_type_by_id(self.get_liquid(x, y).id)
    }

    /**returns the liquid type by id*/
    pub fn get_liquid_type_by_id(&self, id: i32) -> Rc<LiquidType> {
        if id < 0 || id >= self.liquid_types.len() as i32 {
            panic!("Liquid type id is out of bounds! ({})", id);
        }
        return self.liquid_types[id as usize].clone();
    }

    /**returns the liquid type by name*/
    pub fn get_liquid_type_by_name(&self, name: &str) -> Option<Rc<LiquidType>> {
        for i in 0..self.liquid_types.len() {
            if self.liquid_types[i].name == name {
                return Some(self.liquid_types[i].clone());
            }
        }
        None
    }

    /**sets the liquid type at the given position without updates*/
    pub fn set_liquid_type_siletnly(&mut self, x: i32, y: i32, liquid_type: Rc<LiquidType>) {
        self.get_liquid_mut(x, y).id = liquid_type.id;
    }

    /**sets the liquid type at the given position with updates*/
    pub fn set_liquid_type(&mut self, x: i32, y: i32, liquid_type: Rc<LiquidType>) {
        if liquid_type.id != self.get_liquid(x, y).id {
            if liquid_type.id == self.empty.id {
                self.set_liquid_level(x, y, 0.0);
            }
            self.set_liquid_type_siletnly(x, y, liquid_type);

            //TODO: implement new events
        }
    }

    /**updates the liquid at the given position*/
    pub fn update_liquid(&mut self, x: i32, y: i32) {
        if self.get_liquid_level(x, y) == 0.0 {
            self.set_liquid_type(x, y, self.empty.clone());
            return;
        }

        if !self.blocks.borrow().get_block_type(x, y).ghost {
            self.set_liquid_type(x, y, self.empty.clone());
        }

        let mut under_exists = false;
        let mut left_exists = false;
        let mut right_exists = false;

        if y < self.get_height() - 1 && (self.is_flowable(x, y + 1) || (self.get_liquid_type(x, y + 1).id == self.get_liquid_type(x, y).id && self.get_liquid_level(x, y + 1) as i32 != MAX_LIQUID_LEVEL)) {
            under_exists = true
        }

        if x > 0 && (self.is_flowable(x - 1, y) || (self.get_liquid_type(x - 1, y).id == self.get_liquid_type(x, y).id && self.get_liquid_level(x - 1, y) as i32 != MAX_LIQUID_LEVEL)) {
            left_exists = true
        }

        if x < self.get_width() - 1 && (self.is_flowable(x + 1, y) || (self.get_liquid_type(x + 1, y).id == self.get_liquid_type(x, y).id && self.get_liquid_level(x + 1, y) as i32 != MAX_LIQUID_LEVEL)) {
            right_exists = true
        }

        if under_exists {
            self.set_liquid_type(x, y + 1, self.get_liquid_type(x, y).clone());

            let liquid_sum = self.get_liquid_level(x, y) + self.get_liquid_level(x, y + 1);
            if liquid_sum as i32 > MAX_LIQUID_LEVEL {
                self.set_liquid_level(x, y, liquid_sum - MAX_LIQUID_LEVEL as f64);
                self.set_liquid_level(x, y + 1, MAX_LIQUID_LEVEL as f64);
            } else {
                self.set_liquid_level(x, y, 0.0);
                self.set_liquid_level(x, y + 1, liquid_sum);
            }
        }

        if self.get_liquid_level(x, y) == 0.0 {
            return;
        }

        if right_exists {
            self.set_liquid_type(x + 1, y, self.get_liquid_type(x, y).clone());
        }
        if left_exists {
            self.set_liquid_type(x - 1, y, self.get_liquid_type(x, y).clone());
        }

        if left_exists && right_exists && self.get_liquid_level(x + 1, y) as i32 != self.get_liquid_level(x, y) as i32 && self.get_liquid_level(x - 1, y) as i32 != self.get_liquid_level(x, y) as i32 {
            let avg = (self.get_liquid_level(x + 1, y) + self.get_liquid_level(x - 1, y) + self.get_liquid_level(x, y)) / 3.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x + 1, y, avg);
            self.set_liquid_level(x - 1, y, avg);
        } else if left_exists && self.get_liquid_level(x - 1, y) as i32 != self.get_liquid_level(x, y) as i32 {
            let avg = (self.get_liquid_level(x - 1, y) + self.get_liquid_level(x, y)) / 2.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x - 1, y, avg);
        } else if right_exists && self.get_liquid_level(x + 1, y) as i32 != self.get_liquid_level(x, y) as i32 {
            let avg = (self.get_liquid_level(x + 1, y) + self.get_liquid_level(x, y)) / 2.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x + 1, y, avg);
        }
    }

    /**returns the liquid level at the given position*/
    pub fn get_liquid_level(&self, x: i32, y: i32) -> f64 {
        self.get_liquid(x, y).level
    }

    /**sets the liquid level at the given position without updating*/
    pub fn set_liquid_level_siletnly(&mut self, x: i32, y: i32, level: f64) {
        self.get_liquid_mut(x, y).level = level;
    }

    /**sets the liquid level at the given position with updating*/
    pub fn set_liquid_level(&mut self, x: i32, y: i32, level: f64) {
        if level != self.get_liquid_level(x, y) {
            self.set_liquid_level_siletnly(x, y, level);
            if level <= 0.0 {
                self.set_liquid_type(x, y, self.empty.clone());
            }
            //TODO: implement new events
        }
    }

    //TODO: to_serial, from_serial

    /**registers a new liquid type*/
    pub fn register_liquid_type(&mut self, mut liquid_type: LiquidType) {
        liquid_type.id = self.liquid_types.len() as i32;
        self.liquid_types.insert(liquid_type.id as usize, Rc::new(liquid_type));
    }

    /**returns the number of liquid types*/
    pub fn get_liquid_type_count(&self) -> u32 {
        self.liquid_types.len() as u32
    }
}