use crate::shared::blocks::BlockId;
use crate::shared::walls::WallId;

#[derive(Clone)]
pub struct Ore {
    pub block: BlockId,
    pub start_noise: f32,
    pub end_noise: f32,
}

#[derive(Clone)]
pub struct Biome {
    pub min_width: u32,
    pub max_width: u32,
    pub min_terrain_height: u32,
    pub max_terrain_height: u32,
    pub base_block: BlockId,
    pub base_wall: WallId,
    // the first element is connection weight, the second is the biome id
    pub adjacent_biomes: Vec<(i32, i32)>,
    pub mod_id: i32,
    pub generator_function: Option<String>,
    pub ores: Vec<Ore>,
}

impl Biome {
    pub const fn new(mod_id: i32) -> Self {
        Self {
            min_width: 0,
            max_width: 0,
            min_terrain_height: 0,
            max_terrain_height: 0,
            base_block: BlockId::undefined(),
            base_wall: WallId::new(),
            adjacent_biomes: Vec::new(),
            mod_id,
            generator_function: None,
            ores: Vec::new(),
        }
    }
}
