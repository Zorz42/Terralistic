use crate::shared::blocks::{BlockId, Blocks};
use crate::shared::mod_manager::{get_mod_id, ModManager};
use crate::shared::walls::{WallId, Walls};
use noise::{NoiseFn, Perlin};
use rand::{RngCore, SeedableRng};
use rlua::prelude::LuaUserData;
use rlua::UserDataMethods;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn turbulence(noise: &Perlin, x: f32, y: f32) -> f32 {
    let mut value = 0.0;
    let mut size = 1.0;

    for _ in 0..3 {
        value += noise.get([(x / size) as f64, (y / size) as f64]) as f32 * size;
        size /= 2.0;
    }

    value / 2.0
}

fn convolve(array: &Vec<f32>, size: i32) -> Vec<f32> {
    let mut result = Vec::new();

    for i in 0..array.len() {
        let mut sum = 0.0;
        let left_index = i32::max(i as i32 - size / 2, 0);
        let right_index = i32::min(i as i32 + size / 2, array.len() as i32 - 1);
        for j in left_index..right_index {
            sum += array[j as usize];
        }
        result.push(sum / (right_index - left_index) as f32);
    }

    result
}

pub struct WorldGenerator {
    biomes: Arc<Mutex<Vec<Biome>>>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            biomes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        mods.add_global_function("new_biome", move |lua_ctx, _: ()| {
            let mod_id = get_mod_id(lua_ctx).unwrap();
            Ok(Biome::new(mod_id))
        });

        let biomes = self.biomes.clone();
        mods.add_global_function("register_biome", move |_, biome: Biome| {
            biomes.lock().unwrap().push(biome);
            Ok(biomes.lock().unwrap().len() - 1)
        });

        // lua function connect_biomes(biome1, biome2, weight) takes two biome ids and a weight and connects them
        // the weight is how likely it is to go from biome1 to biome2 (and vice versa)
        let biomes = self.biomes.clone();
        mods.add_global_function(
            "connect_biomes",
            move |_, (biome1, biome2, weight): (i32, i32, i32)| {
                biomes.lock().unwrap()[biome1 as usize]
                    .adjacent_biomes
                    .push((weight, biome2));
                biomes.lock().unwrap()[biome2 as usize]
                    .adjacent_biomes
                    .push((weight, biome1));
                Ok(())
            },
        );
    }

    pub fn generate(
        &mut self,
        world: (&mut Blocks, &mut Walls),
        mods: &mut ModManager,
        min_width: i32,
        height: i32,
        seed: u64,
        status_text: &Mutex<String>,
    ) {
        let blocks = world.0;
        let walls = world.1;
        // create a random number generator with seed
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        if self.biomes.lock().unwrap().len() == 0 {
            panic!("No biomes were added! Cannot generate world!");
        }

        let mut min_heights = Vec::new();
        let mut max_heights = Vec::new();
        let mut biome_ids = Vec::new();

        let mut width = 0;

        // walk on the graph of biomes
        // initial biome is random
        let mut curr_biome = rand::random::<i32>().abs() % self.biomes.lock().unwrap().len() as i32;
        while width < min_width {
            // determine the width of the current biome
            // the width is a random number between the min and max width
            let biome = &self.biomes.lock().unwrap()[curr_biome as usize];
            let biome_width =
                rand::random::<i32>().abs() % (biome.max_width - biome.min_width) + biome.min_width;
            for _ in 0..biome_width {
                min_heights.push(biome.min_terrain_height as f32);
                max_heights.push(biome.max_terrain_height as f32);
                biome_ids.push(curr_biome);
            }
            width += biome_width;

            // determine the next biome
            // the next biome is chosen randomly based on the weights of the edges
            let mut total_weight = 0;
            for (weight, _) in &biome.adjacent_biomes {
                total_weight += weight;
            }
            let mut rand = rand::random::<i32>().abs() % total_weight;
            for (weight, next_biome) in &biome.adjacent_biomes {
                rand -= weight;
                if rand < 0 {
                    curr_biome = *next_biome;
                    break;
                }
            }
        }

        println!("Creating a world with size {width}x{height}");

        let mut current_task = 0;
        let total_tasks = width * height;

        let mut next_task = || {
            current_task += 1;
            *status_text.lock().unwrap() = format!(
                "Generating world {}%",
                (current_task as f32 / total_tasks as f32 * 100.0) as i32
            );
        };

        let start_time = std::time::Instant::now();

        *status_text.lock().unwrap() = "Generating world".to_string();
        blocks.create(width, height).unwrap();

        let mut block_terrain = vec![vec![BlockId::new(); height as usize]; width as usize];
        let mut wall_terrain = vec![vec![WallId::new(); height as usize]; width as usize];

        let mut min_cave_thresholds = vec![0.0; width as usize];
        let mut max_cave_thresholds = vec![0.15; width as usize];

        let mut ores_start_noises = HashMap::new();
        let mut ores_end_noises = HashMap::new();

        for block_id in blocks.get_all_block_ids() {
            ores_start_noises.insert(block_id, vec![-1.0; width as usize]);
            ores_end_noises.insert(block_id, vec![-1.0; width as usize]);
        }

        for x in 0..width {
            let biome = &self.biomes.lock().unwrap()[biome_ids[x as usize] as usize];
            for ore in &biome.ores {
                ores_start_noises.get_mut(&ore.block).unwrap()[x as usize] = ore.start_noise;
                ores_end_noises.get_mut(&ore.block).unwrap()[x as usize] = ore.end_noise;
            }
        }

        let convolution_size = 50;
        for _ in 0..5 {
            min_heights = convolve(&min_heights, convolution_size);
            max_heights = convolve(&max_heights, convolution_size);
            min_cave_thresholds = convolve(&min_cave_thresholds, convolution_size);
            max_cave_thresholds = convolve(&max_cave_thresholds, convolution_size);
        }

        for block_id in blocks.get_all_block_ids() {
            for _ in 0..5 {
                ores_start_noises.insert(
                    block_id,
                    convolve(&ores_start_noises[&block_id], convolution_size),
                );
                ores_end_noises.insert(
                    block_id,
                    convolve(&ores_end_noises[&block_id], convolution_size),
                );
            }
        }

        let cave_noise = Perlin::new(rng.next_u32());
        let terrain_noise = Perlin::new(rng.next_u32());
        let mut ore_noises = HashMap::new();
        for block_id in blocks.get_all_block_ids() {
            ore_noises.insert(block_id, Perlin::new(rng.next_u32()));
        }

        let mut curr_terrain = Vec::new();
        let mut prev_x = 0;

        let heights = {
            let mut heights = Vec::new();
            for x in 0..width {
                let terrain_noise_val = ((turbulence(&terrain_noise, x as f32 / 150.0, 0.0) + 1.0)
                    * (max_heights[x as usize] - min_heights[x as usize]))
                    as i32
                    + min_heights[x as usize] as i32
                    + height * 2 / 3;
                heights.push(terrain_noise_val);
            }
            heights
        };

        for x in 0..width {
            curr_terrain.push(vec![BlockId::new(); height as usize]);

            let terrain_noise_val = heights[x as usize];
            let mut walls_height = heights[x as usize];
            if let Some(height) = heights.get(x as usize + 1) {
                walls_height = i32::min(walls_height, *height);
            }
            if x > 0 {
                if let Some(height) = heights.get(x as usize - 1) {
                    walls_height = i32::min(walls_height, *height);
                }
            }

            for y in 0..height {
                next_task();
                let terrain_height = height - y;

                let cave_noise_val =
                    f32::abs(turbulence(&cave_noise, x as f32 / 80.0, y as f32 / 80.0));
                let cave_threshold = y as f32 / height as f32
                    * (max_cave_thresholds[x as usize] - min_cave_thresholds[x as usize])
                    + min_cave_thresholds[x as usize];

                let mut curr_block =
                    self.biomes.lock().unwrap()[biome_ids[x as usize] as usize].base_block;

                if terrain_height > terrain_noise_val || cave_threshold > cave_noise_val {
                    curr_block = blocks.air;
                } else {
                    for block in blocks.get_all_block_ids() {
                        let start_noise = ores_start_noises[&block][x as usize];
                        let end_noise = ores_end_noises[&block][x as usize];
                        if (start_noise, end_noise) != (-1.0, -1.0) {
                            let ore_noise = turbulence(
                                ore_noises.get(&block).unwrap(),
                                x as f32 / 15.0,
                                y as f32 / 15.0,
                            );
                            let ore_threshold =
                                y as f32 / height as f32 * (end_noise - start_noise) + start_noise;
                            if ore_threshold > ore_noise {
                                curr_block = block;
                            }
                        }
                    }
                }

                curr_terrain[(x - prev_x) as usize][y as usize] = curr_block;

                if terrain_height < walls_height {
                    wall_terrain[x as usize][y as usize] =
                        self.biomes.lock().unwrap()[biome_ids[x as usize] as usize].base_wall;
                } else {
                    wall_terrain[x as usize][y as usize] = walls.clear;
                }
            }

            if x == width - 1 || biome_ids[x as usize] != biome_ids[(x + 1) as usize] {
                let curr_biome = &self.biomes.lock().unwrap()[biome_ids[x as usize] as usize];
                if let Some(generator_function) = &curr_biome.generator_function {
                    curr_terrain = mods
                        .get_mod(curr_biome.mod_id)
                        .unwrap()
                        .call_function(generator_function, (curr_terrain, x - prev_x + 1, height))
                        .unwrap();
                }

                for y in 0..height {
                    for x in prev_x..x + 1 {
                        block_terrain[x as usize][y as usize] =
                            curr_terrain[(x - prev_x) as usize][y as usize];
                    }
                }
                curr_terrain.clear();
                prev_x = x + 1;
            }
        }

        blocks.create_from_block_ids(&block_terrain).unwrap();
        walls.create_from_wall_ids(&wall_terrain).unwrap();

        println!("World generated in {}ms", start_time.elapsed().as_millis());

        if current_task != total_tasks {
            panic!("Not all tasks were completed! {current_task} != {total_tasks}");
        }
    }
}

#[derive(Clone)]
struct Ore {
    pub block: BlockId,
    pub start_noise: f32,
    pub end_noise: f32,
}

#[derive(Clone)]
struct Biome {
    pub min_width: i32,
    pub max_width: i32,
    pub min_terrain_height: i32,
    pub max_terrain_height: i32,
    pub base_block: BlockId,
    pub base_wall: WallId,
    // the first element is connection weight, the second is the biome id
    pub adjacent_biomes: Vec<(i32, i32)>,
    pub mod_id: i32,
    pub generator_function: Option<String>,
    pub ores: Vec<Ore>,
}

impl Biome {
    fn new(mod_id: i32) -> Self {
        Self {
            min_width: 0,
            max_width: 0,
            min_terrain_height: 0,
            max_terrain_height: 0,
            base_block: BlockId::new(),
            base_wall: WallId::new(),
            adjacent_biomes: Vec::new(),
            mod_id,
            generator_function: None,
            ores: Vec::new(),
        }
    }
}

// make Biome compatible with Lua
impl LuaUserData for Biome {
    // implement index and new_index metamethods to allow reading and writing to fields
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to set fields
        methods.add_meta_method_mut(
            rlua::MetaMethod::NewIndex,
            |_lua_ctx, this, (key, value): (String, rlua::Value)| {
                match key.as_str() {
                    "min_width" => {
                        match value {
                            rlua::Value::Integer(b) => this.min_width = b as i32,
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for min_width".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "max_width" => {
                        match value {
                            rlua::Value::Integer(b) => this.max_width = b as i32,
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for max_width".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "min_terrain_height" => {
                        match value {
                            rlua::Value::Integer(b) => this.min_terrain_height = b as i32,
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for min_terrain_height".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "max_terrain_height" => {
                        match value {
                            rlua::Value::Integer(b) => this.max_terrain_height = b as i32,
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for max_terrain_height".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "base_block" => {
                        // base_block is a BlockId, so we need to convert the value to a BlockId
                        match value {
                            rlua::Value::UserData(b) => match b.borrow::<BlockId>() {
                                Ok(b) => this.base_block = *b,
                                Err(_) => {
                                    return Err(rlua::Error::RuntimeError(
                                        "value is not a valid value for base_block".to_string(),
                                    ))
                                }
                            },
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for base_block".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "base_wall" => {
                        // base_wall is a WallId, so we need to convert the value to a WallId
                        match value {
                            rlua::Value::UserData(b) => match b.borrow::<WallId>() {
                                Ok(b) => this.base_wall = *b,
                                Err(_) => {
                                    return Err(rlua::Error::RuntimeError(
                                        "value is not a valid value for base_wall".to_string(),
                                    ))
                                }
                            },
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for base_wall".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    "generator_function" => {
                        match value {
                            rlua::Value::String(b) => {
                                this.generator_function = Some(b.to_str().unwrap().to_string())
                            }
                            _ => {
                                return Err(rlua::Error::RuntimeError(
                                    "value is not a valid value for generator_function".to_string(),
                                ))
                            }
                        }
                        Ok(())
                    }
                    _ => Err(rlua::Error::RuntimeError(format!(
                        "{key} is not a valid field of Biome"
                    ))),
                }
            },
        );

        // add method to add an ore
        methods.add_method_mut(
            "add_ore",
            |_, this, (block, start_noise, end_noise): (BlockId, f32, f32)| {
                this.ores.push(Ore {
                    block,
                    start_noise,
                    end_noise,
                });
                Ok(())
            },
        );
    }
}
