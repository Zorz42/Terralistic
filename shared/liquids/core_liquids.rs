/*use super::liquid_type::LiquidType;
use crate::shared::blocks::Blocks;
use std::rc::Rc;

const MAX_LIQUID_LEVEL: i32 = 100;

//TODO: new events idk

/**struct with information about a liquid*/
struct Liquid {
    pub id: i32,
    pub level: f32,
}

impl Liquid {
    pub fn new() -> Self {
        Self {
            id: 0, //TODO: change to Rc<LiquidType>?
            level: 0.0,
        }
    }
}

/**struct that manages all the liquids*/
pub struct Liquids {
    liquid_types: Vec<Rc<LiquidType>>,
    liquids: Vec<Liquid>,
    pub empty: Rc<LiquidType>,
    width: u32,
    height: u32,
    //TODO: new event sender
}

impl Liquids {
    #[must_use]
    pub fn new(blocks: &Blocks) -> Self {
        let temp = LiquidType::new("temp".to_string());
        let mut liquids_object = Self {
            liquid_types: Vec::new(),
            liquids: Vec::new(),
            empty: Rc::new(temp), //temporarily assign
            width: blocks.get_width(),
            height: blocks.get_height(),
        };
        let mut empty = LiquidType::new("empty".to_string());
        empty.flow_time = 0;
        empty.speed_multiplier = 1.0;
        liquids_object.register_liquid_type(empty);
        liquids_object.empty = liquids_object.liquid_types[0].clone(); //assign the real empty liquid Rc
        liquids_object
    }

    /**this function returns a liquid at the given position*/
    fn get_liquid(&self, x: i32, y: i32) -> &Liquid {
        assert!(
            !(x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32),
            "Liquid is accessed out of the bounds! ({x}, {y})"
        );
        &self.liquids[(y * self.width as i32 + x) as usize]
    }

    /**this function returns a mutable liquid at the given position*/
    fn get_liquid_mut(&mut self, x: i32, y: i32) -> &mut Liquid {
        assert!(
            !(x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32),
            "Liquid is accessed out of the bounds! ({x}, {y})"
        );
        &mut self.liquids[(y * self.width as i32 + x) as usize]
    }

    /**returns whether the given liquid is flowable*/
    fn is_flowable(&self, x: i32, y: i32, blocks: &Blocks) -> bool {
        blocks.get_block_type_at(x, y).unwrap().ghost
            && self.get_liquid_type(x, y).id == self.empty.id
    }

    /**creates the liquid array*/
    pub fn create(&mut self, blocks: &Blocks) {
        self.liquids = Vec::new();
        self.height = blocks.get_height();
        self.width = blocks.get_width();
        self.liquids
            .resize_with((self.width * self.height) as usize, Liquid::new);
    }

    /**returns the width of the liquid array*/
    #[must_use]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /**returns the height of the liquid array*/
    #[must_use]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /**returns the liquid type at the given position*/
    #[must_use]
    pub fn get_liquid_type(&self, x: i32, y: i32) -> Rc<LiquidType> {
        self.get_liquid_type_by_id(self.get_liquid(x, y).id)
    }

    /**returns the liquid type by id*/
    #[must_use]
    pub fn get_liquid_type_by_id(&self, id: i32) -> Rc<LiquidType> {
        assert!(
            !(id < 0 || id >= self.liquid_types.len() as i32),
            "Liquid type id is out of bounds! ({id})"
        );
        self.liquid_types[id as usize].clone()
    }

    /**returns the liquid type by name*/
    #[must_use]
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
    pub fn update_liquid(&mut self, x: i32, y: i32, blocks: &Blocks) {
        if self.get_liquid_level(x, y) == 0.0 {
            self.set_liquid_type(x, y, self.empty.clone());
            return;
        }

        if !blocks.get_block_type_at(x, y).unwrap().ghost {
            self.set_liquid_type(x, y, self.empty.clone());
        }

        let mut under_exists = false;
        let mut left_exists = false;
        let mut right_exists = false;

        if y < self.get_height() as i32 - 1
            && (self.is_flowable(x, y + 1, blocks)
                || (self.get_liquid_type(x, y + 1).id == self.get_liquid_type(x, y).id
                    && self.get_liquid_level(x, y + 1) as i32 != MAX_LIQUID_LEVEL))
        {
            under_exists = true
        }

        if x > 0
            && (self.is_flowable(x - 1, y, blocks)
                || (self.get_liquid_type(x - 1, y).id == self.get_liquid_type(x, y).id
                    && self.get_liquid_level(x - 1, y) as i32 != MAX_LIQUID_LEVEL))
        {
            left_exists = true
        }

        if x < self.get_width() as i32 - 1
            && (self.is_flowable(x + 1, y, blocks)
                || (self.get_liquid_type(x + 1, y).id == self.get_liquid_type(x, y).id
                    && self.get_liquid_level(x + 1, y) as i32 != MAX_LIQUID_LEVEL))
        {
            right_exists = true
        }

        if under_exists {
            self.set_liquid_type(x, y + 1, self.get_liquid_type(x, y));

            let liquid_sum = self.get_liquid_level(x, y) + self.get_liquid_level(x, y + 1);
            if liquid_sum as i32 > MAX_LIQUID_LEVEL {
                self.set_liquid_level(x, y, liquid_sum - MAX_LIQUID_LEVEL as f32);
                self.set_liquid_level(x, y + 1, MAX_LIQUID_LEVEL as f32);
            } else {
                self.set_liquid_level(x, y, 0.0);
                self.set_liquid_level(x, y + 1, liquid_sum);
            }
        }

        if self.get_liquid_level(x, y) == 0.0 {
            return;
        }

        if right_exists {
            self.set_liquid_type(x + 1, y, self.get_liquid_type(x, y));
        }
        if left_exists {
            self.set_liquid_type(x - 1, y, self.get_liquid_type(x, y));
        }

        if left_exists
            && right_exists
            && self.get_liquid_level(x + 1, y) as i32 != self.get_liquid_level(x, y) as i32
            && self.get_liquid_level(x - 1, y) as i32 != self.get_liquid_level(x, y) as i32
        {
            let avg = (self.get_liquid_level(x + 1, y)
                + self.get_liquid_level(x - 1, y)
                + self.get_liquid_level(x, y))
                / 3.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x + 1, y, avg);
            self.set_liquid_level(x - 1, y, avg);
        } else if left_exists
            && self.get_liquid_level(x - 1, y) as i32 != self.get_liquid_level(x, y) as i32
        {
            let avg = (self.get_liquid_level(x - 1, y) + self.get_liquid_level(x, y)) / 2.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x - 1, y, avg);
        } else if right_exists
            && self.get_liquid_level(x + 1, y) as i32 != self.get_liquid_level(x, y) as i32
        {
            let avg = (self.get_liquid_level(x + 1, y) + self.get_liquid_level(x, y)) / 2.0;

            self.set_liquid_level(x, y, avg);
            self.set_liquid_level(x + 1, y, avg);
        }
    }

    /**returns the liquid level at the given position*/
    #[must_use]
    pub fn get_liquid_level(&self, x: i32, y: i32) -> f32 {
        self.get_liquid(x, y).level
    }

    /**sets the liquid level at the given position without updating*/
    pub fn set_liquid_level_siletnly(&mut self, x: i32, y: i32, level: f32) {
        self.get_liquid_mut(x, y).level = level;
    }

    /**sets the liquid level at the given position with updating*/
    pub fn set_liquid_level(&mut self, x: i32, y: i32, level: f32) {
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
        self.liquid_types
            .insert(liquid_type.id as usize, Rc::new(liquid_type));
    }

    /**returns the number of liquid types*/
    #[must_use]
    pub fn get_liquid_type_count(&self) -> u32 {
        self.liquid_types.len() as u32
    }
}*/
