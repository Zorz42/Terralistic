use shared::blocks::Blocks;
use shared::walls::Walls;

/**
Client walls handle wall changes and rendering.
*/
pub struct ClientWalls {
    walls: Walls,
}

impl ClientWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
        }
    }
}
