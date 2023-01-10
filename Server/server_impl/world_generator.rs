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

fn turbulence(noise: &Perlin, x: f64, y: f64) -> f64 {
    let mut value = 0.0;
    let mut size = 1.0;

    for _ in 0..3 {
        value += noise.get([x / size, y / size]) * size;
        size /= 2.0;
    }

    value / 2.0
}

fn convolve(array: &Vec<f64>, size: i32) -> Vec<f64> {
    let mut result = Vec::new();

    for i in 0..array.len() {
        let mut sum = 0.0;
        let left_index = i32::max(i as i32 - size / 2, 0);
        let right_index = i32::min(i as i32 + size / 2, array.len() as i32 - 1);
        for j in left_index..right_index {
            sum += array[j as usize];
        }
        result.push(sum / (right_index - left_index) as f64);
    }

    result
}

pub struct WorldGenerator {
    biomes: SharedMut<Vec<Biome>>,
    total_tasks: i32,
    current_task: i32,
    status_text: SharedMut<String>,
}

impl WorldGenerator {
    pub fn new() -> Self {
        Self {
            biomes: SharedMut::new(Vec::new()),
            total_tasks: 0,
            current_task: 0,
            status_text: SharedMut::new(String::new()),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        let biomes = self.biomes.clone();
        mods.add_global_function("add_biome", move |_, frequency| {
            biomes.borrow().push(Biome::new(frequency));
            Ok(biomes.borrow().len() - 1)
        });
    }

    fn next_task(&mut self) {
        self.current_task += 1;
        *self.status_text.borrow() = format!("Generating world {}%", (self.current_task as f32 / self.total_tasks as f32 * 100.0) as i32);
    }

    pub fn generate(&mut self, blocks: &mut Blocks, mods: &mut ModManager, width: i32, height: i32, seed: u32, status_text: SharedMut<String>) {
        self.total_tasks = width * height * 2;
        self.status_text = status_text.clone();

        if self.biomes.borrow().len() == 0 {
            panic!("No biomes were added! Cannot generate world!");
        }

        let start_time = std::time::Instant::now();

        *status_text.borrow() = "Generating world".to_string();
        blocks.create(width, height);

        let mut terrain = vec![vec![0; height as usize]; width as usize];

        let noise = Perlin::new(seed);

        let mut min_heights = vec![(height * 2 / 3) as f64; width as usize];
        let mut max_heights = vec![(height * 2 / 3 + 80) as f64; width as usize];
        let mut min_cave_thresholds = vec![0.0; width as usize];
        let mut max_cave_thresholds = vec![0.15; width as usize];

        for x in 300..1000 {
            min_heights[x as usize] = (height * 2 / 3 - 180) as f64;
            max_heights[x as usize] = (height * 2 / 3 - 100) as f64;
            min_cave_thresholds[x as usize] = 0.05;
            max_cave_thresholds[x as usize] = 0.2;
        }

        let convolution_size = 50;
        for _ in 0..5 {
            min_heights = convolve(&min_heights, convolution_size);
            max_heights = convolve(&max_heights, convolution_size);
            min_cave_thresholds = convolve(&min_cave_thresholds, convolution_size);
            max_cave_thresholds = convolve(&max_cave_thresholds, convolution_size);
        }

        for x in 0..width {
            let terrain_noise = ((turbulence(&noise, x as f64 / 150.0, 0.0) + 1.0) * (max_heights[x as usize] - min_heights[x as usize])) as i32 + min_heights[x as usize] as i32;
            for y in 0..height {
                self.next_task();
                let terrain_height = height - y;

                let cave_noise = f64::abs(turbulence(&noise, x as f64 / 80.0, y as f64 / 80.0));
                let cave_threshold = y as f64 / height as f64 * (max_cave_thresholds[x as usize] - min_cave_thresholds[x as usize]) + min_cave_thresholds[x as usize];

                let value= if terrain_height < terrain_noise && cave_threshold < cave_noise { 1 } else { 0 };

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
                self.next_task();
                let block_type = blocks.get_block_type_by_id(generated_world[x as usize][y as usize]);
                blocks.set_block(x, y, block_type);
            }
        }

        println!("World generated in {}ms", start_time.elapsed().as_millis());

        if self.current_task != self.total_tasks {
            panic!("Not all tasks were completed! {} != {}", self.current_task, self.total_tasks);
        }
    }
}