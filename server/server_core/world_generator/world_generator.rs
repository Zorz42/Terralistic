use std::collections::HashMap;
use std::sync::{Arc, MutexGuard};
use std::sync::{Mutex, PoisonError};

use anyhow::{anyhow, bail, Result};
use noise::Perlin;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::libraries::events::EventManager;
use crate::server::server_core::world_generator::biome::Biome;
use crate::server::server_core::world_generator::noise::{convolve, turbulence};
use crate::shared::blocks::{BlockId, Blocks};
use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{WallId, Walls};

fn update_all_blocks(blocks: &mut Blocks) -> Result<()> {
    let mut dummy_events = EventManager::new();
    for x in 0..blocks.get_width() as i32 {
        for y in 0..blocks.get_height() as i32 {
            blocks.update_block(x, y, &mut dummy_events)?;
        }
    }
    Ok(())
}

pub struct WorldGenerator {
    pub(super) biomes: Arc<Mutex<Vec<Biome>>>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            biomes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_biomes(&self) -> MutexGuard<Vec<Biome>> {
        self.biomes.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// This function creates a array of biome ids and returns it along with the width of the world.
    /// Each biome id is for each column of the world.
    fn generate_biome_ids(&mut self, min_width: u32) -> Result<(Vec<i32>, u32)> {
        let mut biome_ids = Vec::new();
        let mut width = 0;

        // walk on the graph of biomes
        // initial biome is random
        let mut curr_biome = rand::random::<i32>().abs() % self.get_biomes().len() as i32;
        while width < min_width {
            // determine the width of the current biome
            // the width is a random number between the min and max width
            let biomes = self.get_biomes();
            let biome = biomes
                .get(curr_biome as usize)
                .ok_or_else(|| anyhow!("Biome with id {} does not exist!", curr_biome))?;
            let biome_width =
                rand::random::<u32>() % (biome.max_width - biome.min_width) + biome.min_width;
            for _ in 0..biome_width {
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

        Ok((biome_ids, width))
    }

    /// The noise parameters are used to determine the start and end noise of each ore.
    fn generate_ore_noise_parameters(
        &self,
        blocks: &Blocks,
        width: u32,
        biome_ids: &[i32],
    ) -> Result<(HashMap<BlockId, Vec<f32>>, HashMap<BlockId, Vec<f32>>)> {
        let mut ores_start_noises = HashMap::new();
        let mut ores_end_noises = HashMap::new();

        for block_id in blocks.get_all_block_ids() {
            ores_start_noises.insert(block_id, vec![-1.0; width as usize]);
            ores_end_noises.insert(block_id, vec![-1.0; width as usize]);
        }

        for x in 0..width {
            let biomes = self.get_biomes();
            let biome = biomes
                .get(
                    *biome_ids
                        .get(x as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?
                        as usize,
                )
                .ok_or_else(|| anyhow!("invalid biome id"))?;
            for ore in &biome.ores {
                *ores_start_noises
                    .get_mut(&ore.block)
                    .ok_or_else(|| anyhow!("invalid block"))?
                    .get_mut(x as usize)
                    .ok_or_else(|| anyhow!("invalid coordinate"))? = ore.start_noise;
                *ores_end_noises
                    .get_mut(&ore.block)
                    .ok_or_else(|| anyhow!("invalid block"))?
                    .get_mut(x as usize)
                    .ok_or_else(|| anyhow!("invalid coordinate"))? = ore.end_noise;
            }
        }

        let convolution_size = 50;
        for block_id in blocks.get_all_block_ids() {
            for _ in 0..5 {
                ores_start_noises.insert(
                    block_id,
                    convolve(
                        ores_start_noises
                            .get(&block_id)
                            .ok_or_else(|| anyhow!("invalid block id"))?,
                        convolution_size,
                    ),
                );
                ores_end_noises.insert(
                    block_id,
                    convolve(
                        ores_end_noises
                            .get(&block_id)
                            .ok_or_else(|| anyhow!("invalid block id"))?,
                        convolution_size,
                    ),
                );
            }
        }

        Ok((ores_start_noises, ores_end_noises))
    }

    /// This function generates the heights of the terrain.
    fn generate_heights(
        rng: &mut StdRng,
        width: u32,
        height: u32,
        min_heights: &[f32],
        max_heights: &[f32],
    ) -> Result<Vec<u32>> {
        let terrain_noise = Perlin::new(rng.next_u32());
        let mut heights = Vec::new();
        for x in 0..width {
            let terrain_noise_val = ((turbulence(&terrain_noise, x as f32 / 150.0, 0.0) + 1.0)
                * (max_heights
                    .get(x as usize)
                    .ok_or_else(|| anyhow!("invalid x coordinate"))?
                    - min_heights
                        .get(x as usize)
                        .ok_or_else(|| anyhow!("invalid block id"))?))
                as u32
                + *min_heights
                    .get(x as usize)
                    .ok_or_else(|| anyhow!("invalid block id"))? as u32
                + height * 2 / 3;
            heights.push(terrain_noise_val);
        }
        Ok(heights)
    }

    #[allow(clippy::too_many_lines)] // TODO: split this function up
    pub fn generate(
        &mut self,
        world: (&mut Blocks, &mut Walls),
        mods: &mut ModManager,
        min_width: u32,
        height: u32,
        seed: u64,
        status_text: &Mutex<String>,
    ) -> Result<()> {
        let start_time = std::time::Instant::now();

        let blocks = world.0;
        let walls = world.1;
        // create a random number generator with seed
        let mut rng = StdRng::seed_from_u64(seed);

        if self.get_biomes().is_empty() {
            bail!("No biomes were added! Cannot generate world!")
        }

        let (biome_ids, width) = self.generate_biome_ids(min_width)?;

        let mut min_heights = Vec::new();
        let mut max_heights = Vec::new();

        println!("Creating a world with size {width}x{height}");

        // extract the min and max heights from the biomes
        for biome_id in &biome_ids {
            let biomes = self.get_biomes();
            let biome = biomes
                .get(*biome_id as usize)
                .ok_or_else(|| anyhow!("Biome with id {} does not exist!", *biome_id))?;
            min_heights.push(biome.min_terrain_height as f32);
            max_heights.push(biome.max_terrain_height as f32);
        }

        // tasks are for loading bar
        let mut current_task = 0;
        let total_tasks = width * height;

        let mut next_task = || {
            current_task += 1;
            *status_text.lock().unwrap_or_else(PoisonError::into_inner) = format!(
                "Generating world {}%",
                (current_task as f32 / total_tasks as f32 * 100.0) as i32
            );
        };

        *status_text.lock().unwrap_or_else(PoisonError::into_inner) = "Generating world".to_owned();
        blocks.create(width, height);

        let mut block_terrain = vec![vec![BlockId::undefined(); height as usize]; width as usize];
        let mut wall_terrain = vec![vec![WallId::new(); height as usize]; width as usize];

        // TODO: add ores generation
        let (_ores_start_noises, _ores_end_noises) =
            self.generate_ore_noise_parameters(blocks, width, &biome_ids)?;

        let mut min_cave_thresholds = vec![0.0; width as usize];
        let mut max_cave_thresholds = vec![0.15; width as usize];

        // convolve the min and max heights and cave thresholds
        let convolution_size = 50;
        for _ in 0..5 {
            min_heights = convolve(&min_heights, convolution_size);
            max_heights = convolve(&max_heights, convolution_size);
            min_cave_thresholds = convolve(&min_cave_thresholds, convolution_size);
            max_cave_thresholds = convolve(&max_cave_thresholds, convolution_size);
        }

        let cave_noise = Perlin::new(rng.next_u32());

        let mut curr_terrain = Vec::new();
        let mut curr_heights = Vec::new();
        let mut prev_x = 0;

        let heights = Self::generate_heights(&mut rng, width, height, &min_heights, &max_heights)?;

        for x in 0..width {
            curr_terrain.push(vec![BlockId::undefined(); height as usize]);
            curr_heights.push(
                *heights
                    .get(x as usize)
                    .ok_or_else(|| anyhow!("invalid x coordinate"))?,
            );

            let terrain_noise_val = *heights
                .get(x as usize)
                .ok_or_else(|| anyhow!("invalid x coordinate"))?;
            let mut walls_height = *heights
                .get(x as usize)
                .ok_or_else(|| anyhow!("invalid x coordinate"))?;
            if let Some(height2) = heights.get(x as usize + 1) {
                walls_height = u32::min(walls_height, *height2);
            }
            if x > 0 {
                if let Some(height2) = heights.get(x as usize - 1) {
                    walls_height = u32::min(walls_height, *height2);
                }
            }

            for y in 0..height {
                next_task();
                let terrain_height = height - y;

                let cave_noise_val =
                    f32::abs(turbulence(&cave_noise, x as f32 / 80.0, y as f32 / 80.0));
                let cave_threshold = y as f32 / height as f32
                    * (max_cave_thresholds
                        .get(x as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?
                        - min_cave_thresholds
                            .get(x as usize)
                            .ok_or_else(|| anyhow!("invalid x coordinate"))?)
                    + min_cave_thresholds
                        .get(x as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?;

                let curr_block =
                    if terrain_height > terrain_noise_val || cave_threshold > cave_noise_val {
                        blocks.air()
                    } else {
                        self.get_biomes()
                            .get(
                                *biome_ids
                                    .get(x as usize)
                                    .ok_or_else(|| anyhow!("invalid x coordinate"))?
                                    as usize,
                            )
                            .ok_or_else(|| anyhow!("invalid biome id"))?
                            .base_block
                    };

                *curr_terrain
                    .get_mut((x - prev_x) as usize)
                    .ok_or_else(|| anyhow!("invalid x coordinate"))?
                    .get_mut(y as usize)
                    .ok_or_else(|| anyhow!("invalid y coordinate"))? = curr_block;

                if terrain_height < walls_height {
                    *wall_terrain
                        .get_mut(x as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?
                        .get_mut(y as usize)
                        .ok_or_else(|| anyhow!("invalid y coordinate"))? = self
                        .get_biomes()
                        .get(
                            *biome_ids
                                .get(x as usize)
                                .ok_or_else(|| anyhow!("invalid x coordinate"))?
                                as usize,
                        )
                        .ok_or_else(|| anyhow!("invalid biome id"))?
                        .base_wall;
                } else {
                    *wall_terrain
                        .get_mut(x as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?
                        .get_mut(y as usize)
                        .ok_or_else(|| anyhow!("invalid y coordinate"))? = walls.clear;
                }
            }

            if x == width - 1
                || biome_ids
                    .get(x as usize)
                    .ok_or_else(|| anyhow!("invalid x coordinate"))?
                    != biome_ids
                        .get((x + 1) as usize)
                        .ok_or_else(|| anyhow!("invalid x coordinate"))?
            {
                let biomes = self.get_biomes();
                let curr_biome2 = biomes
                    .get(
                        *biome_ids
                            .get(x as usize)
                            .ok_or_else(|| anyhow!("invalid x coordinate"))?
                            as usize,
                    )
                    .ok_or_else(|| anyhow!("invalid biome id"))?;
                if let Some(generator_function) = &curr_biome2.generator_function {
                    curr_terrain = mods
                        .get_mod(curr_biome2.mod_id)
                        .ok_or_else(|| anyhow!("invalid mod id"))?
                        .call_function(
                            generator_function,
                            (curr_terrain, curr_heights.clone(), x - prev_x + 1, height),
                        )?;
                }

                for y2 in 0..height {
                    for x2 in prev_x..=x {
                        *block_terrain
                            .get_mut(x2 as usize)
                            .ok_or_else(|| anyhow!("invalid x coordinate"))?
                            .get_mut(y2 as usize)
                            .ok_or_else(|| anyhow!("invalid y coordinate"))? = *curr_terrain
                            .get((x2 - prev_x) as usize)
                            .ok_or_else(|| anyhow!("invalid x coordinate"))?
                            .get(y2 as usize)
                            .ok_or_else(|| anyhow!("invalid y coordinate"))?;
                    }
                }
                curr_terrain.clear();
                curr_heights.clear();
                prev_x = x + 1;
            }
        }

        blocks.create_from_block_ids(&block_terrain)?;
        walls.create_from_wall_ids(&wall_terrain)?;

        update_all_blocks(blocks)?;

        println!("World generated in {}ms", start_time.elapsed().as_millis());

        if current_task != total_tasks {
            println!("Not all tasks were completed! {current_task} != {total_tasks}");
        }

        Ok(())
    }
}
