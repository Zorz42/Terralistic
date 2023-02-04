use shared::blocks::Blocks;
use shared::mod_manager::ModManager;
use shared::walls::Walls;

/**
Client walls handle wall changes and rendering.
*/
pub struct ClientWalls {
    pub walls: Walls,
}

impl ClientWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.walls.init(mods);
    }
}
