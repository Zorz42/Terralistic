use shared::blocks::Blocks;
use shared::mod_manager::ModManager;
use shared::walls::Walls;

pub struct ServerWalls {
    pub walls: Walls,
}

impl ServerWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.walls.init(mods);
    }
}