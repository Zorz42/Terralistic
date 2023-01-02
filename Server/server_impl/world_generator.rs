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

        blocks.create(width, height);
        println!("Generating world...");
        let start_time = std::time::Instant::now();
        for x in 0..blocks.get_width() {
            for y in 0..blocks.get_height() {
                let mut curr_block_id = 0;
                for game_mod in mods.mods_mut() {
                    let result: i32 = game_mod.call_function("generate_block", (x, y)).unwrap();
                    if result != -1 {
                        curr_block_id = result;
                    }
                }

                blocks.set_block(x, y, blocks.get_block_type_by_id(curr_block_id).clone());
            }
        }
        println!("World generated in {}ms", start_time.elapsed().as_millis());
        // count how many blocks of air type there are
        let mut air_count = 0;
        for x in 0..blocks.get_width() {
            for y in 0..blocks.get_height() {
                if blocks.get_block_type(x, y).id == 0 {
                    air_count += 1;
                }
            }
        }
        println!("{}% of the world is air", air_count as f32 / (blocks.get_width() * blocks.get_height()) as f32 * 100.0);
    }
}