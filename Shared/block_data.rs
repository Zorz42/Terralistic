use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomData{
    pub serial_length: u32,
    pub data_id: u32,
    pub data: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize)]
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