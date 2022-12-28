use std::rc::Rc;
use graphics::GraphicsContext;
use shared::blocks::Blocks;

/**
Client blocks handles client side block stuff, such as rendering
 */
pub struct ClientBlocks {
    pub blocks: Blocks,
}

impl ClientBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
        }
    }

    pub fn init(&mut self, graphics: &mut GraphicsContext) {
        self.blocks.create(1000, 1000);
        // set each block in the world to be either air or test_block randomly
        for x in 0..self.blocks.get_width() {
            for y in 0..self.blocks.get_height() {
                if rand::random::<bool>() {
                    self.blocks.set_block(x, y, Rc::clone(&self.blocks.air));
                } else {
                    self.blocks.set_block(x, y, Rc::clone(&self.blocks.test_block));
                }
            }
        }
    }

    pub fn render(&self, graphics: &mut GraphicsContext) {

    }
}