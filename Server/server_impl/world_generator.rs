use noise::{NoiseFn, Perlin};
use shared::blocks::blocks::Blocks;
use shared::mod_manager::ModManager;
use shared_mut::SharedMut;

struct Biome {
    frequency: f32,
}

impl Biome {
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
        }
    }
}

pub struct WorldGenerator {
    biomes: SharedMut<Vec<Biome>>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            biomes: SharedMut::new(Vec::new()),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        let biomes = self.biomes.clone();
        mods.add_global_function("add_biome", move |_, frequency| {
            biomes.borrow().push(Biome::new(frequency));
            Ok(biomes.borrow().len() - 1)
        });
    }

    pub fn generate(&mut self, blocks: &mut Blocks, mods: &mut ModManager, width: i32, height: i32, seed: u32) {
        if self.biomes.borrow().len() == 0 {
            panic!("No biomes were added! Cannot generate world!");
        }

        let start_time = std::time::Instant::now();

        blocks.create(width, height);

        let mut terrain = vec![vec![0; height as usize]; width as usize];

        let noise = Perlin::new(seed);

        let mut max_heights = vec![700; width as usize];
        let mut min_heights = vec![600; width as usize];
        let mut cave_thresholds = vec![0.6; width as usize];

        for x in 0..width {
            for y in 0..height {
                let value=
                    if (height - y - min_heights[x as usize]) < ((noise.get([x as f64 / 100.0, 0.8]) + 1.0) * (max_heights[x as usize] - min_heights[x as usize]) as f64) as i32
                    && cave_thresholds[x as usize] > noise.get([x as f64 / 100.0, y as f64 / 100.0])
                    { 1 } else { 0 };

                terrain[x as usize][y as usize] = value;
            }
        }

        // create a border in terrain
        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    terrain[x as usize][y as usize] = 1;
                }
            }
        }

        let mut generated_world: Vec<Vec<i32>> = Vec::new();

        for game_mod in mods.mods_mut() {
            generated_world = game_mod.call_function("generate_world", (width, height, terrain)).unwrap();
            break;
        }

        for x in 0..blocks.get_width() {
            for y in 0..blocks.get_height() {
                let block_type = blocks.get_block_type_by_id(generated_world[x as usize][y as usize]);
                blocks.set_block(x, y, block_type);
            }
        }

        println!("World generated in {}ms", start_time.elapsed().as_millis());
    }
}