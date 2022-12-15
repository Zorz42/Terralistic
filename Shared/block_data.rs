pub trait CustomDataType{
    fn save(&self, data: &mut Vec<u8>, index: &mut u32);
    fn load(&mut self, data: &Vec<u8>, index: &mut u32);
    fn get_saved_size(&self) -> i32;
}

#[derive(Clone)]//TODO: write a macro that procedurally generates save/load/get_saved_size for structs
pub enum DataType {
    Mod(ModData),
    Furnace(FurnaceData)
}

#[derive(Clone)]
pub struct ModData {
    id: u32,
    data: Vec<u8>
}

impl ModData {
    pub fn new(id: u32, data: Vec<u8>) -> Self {
        ModData {
            id,
            data
        }
    }
}
impl CustomDataType for ModData{
    fn save(&self, data: &mut Vec<u8>, index: &mut u32) {
        data[(*index    ) as usize] = (self.id >> 24) as u8;
        data[(*index + 1) as usize] = (self.id >> 16) as u8;
        data[(*index + 2) as usize] = (self.id >>  8) as u8;
        data[(*index + 3) as usize] = (self.id      ) as u8;
        *index += 4;

        data[(*index    ) as usize] = (self.data.len() >> 24) as u8;
        data[(*index + 1) as usize] = (self.data.len() >> 16) as u8;
        data[(*index + 2) as usize] = (self.data.len() >>  8) as u8;
        data[(*index + 3) as usize] = (self.data.len()      ) as u8;
        *index += 4;

        for i in 0..self.data.len() {
            data[(*index) as usize] = self.data[i];
            *index += 1;
        }
    }
    fn load(&mut self, data: &Vec<u8>, index: &mut u32) {
        self.id = ((data[(*index    ) as usize] as u32) << 24) |
                  ((data[(*index + 1) as usize] as u32) << 16) |
                  ((data[(*index + 2) as usize] as u32) <<  8) |
                  ((data[(*index + 3) as usize] as u32)      );
        *index += 4;

        let data_len: u32 = ((data[(*index    ) as usize] as u32) << 24) |
                            ((data[(*index + 1) as usize] as u32) << 16) |
                            ((data[(*index + 2) as usize] as u32) <<  8) |
                            ((data[(*index + 3) as usize] as u32)      );
        *index += 4;

        self.data = vec![0; data_len as usize];
        for i in 0..data_len {
            self.data[i as usize] = data[(*index) as usize];
            *index += 1;
        }
    }
    fn get_saved_size(&self) -> i32 {
        8 + self.data.len() as i32
    }
}


#[derive(Clone)]
pub struct FurnaceData {
    burn_time: i32,
    heat: i32,
    //TODO: implement fuel and heated items
}
impl FurnaceData {
    fn new() -> Self {
        FurnaceData {
            burn_time: 0,
            heat: 0
        }
    }
}
impl CustomDataType for FurnaceData{
    fn save(&self, data: &mut Vec<u8>, index: &mut u32) {
        data[(*index    ) as usize] = (self.burn_time >> 24) as u8;
        data[(*index + 1) as usize] = (self.burn_time >> 16) as u8;
        data[(*index + 2) as usize] = (self.burn_time >>  8) as u8;
        data[(*index + 3) as usize] = (self.burn_time      ) as u8;
        *index += 4;

        data[(*index    ) as usize] = (self.heat >> 24) as u8;
        data[(*index + 1) as usize] = (self.heat >> 16) as u8;
        data[(*index + 2) as usize] = (self.heat >>  8) as u8;
        data[(*index + 3) as usize] = (self.heat      ) as u8;
        *index += 4;
    }
    fn load(&mut self, data: &Vec<u8>, index: &mut u32) {
        self.burn_time = ((data[(*index    ) as usize] as i32) << 24) |
                         ((data[(*index + 1) as usize] as i32) << 16) |
                         ((data[(*index + 2) as usize] as i32) <<  8) |
                         ((data[(*index + 3) as usize] as i32)      );
        *index += 4;

        self.heat = ((data[(*index    ) as usize] as i32) << 24) |
                    ((data[(*index + 1) as usize] as i32) << 16) |
                    ((data[(*index + 2) as usize] as i32) <<  8) |
                    ((data[(*index + 3) as usize] as i32)      );
        *index += 4;
    }
    fn get_saved_size(&self) -> i32 {
        8
    }
}