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

        let mut terrain = vec![vec![0; width as usize]; height as usize];

        for x in 0..blocks.get_width() {
            for y in 0..blocks.get_height() {
                if y < height - 20 {
                    terrain[y as usize][x as usize] = 1;
                } else {
                    terrain[y as usize][x as usize] = 0;
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
                let block_type = blocks.get_block_type_by_id(generated_world[y as usize][x as usize]);
                blocks.set_block(x, y, block_type);
            }
        }

        println!("World generated in {}ms", start_time.elapsed().as_millis());
    }
}