use std::rc::Rc;
use graphics::GraphicsContext;
use graphics as gfx;
use shared::blocks::{BLOCK_WIDTH, Blocks, RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::game::camera::Camera;

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

    pub fn render(&self, graphics: &mut GraphicsContext, camera: &Camera) {
        let (top_left_x, top_left_y) = camera.get_top_left(graphics);
        let (bottom_right_x, bottom_right_y) = camera.get_bottom_right(graphics);
        let (top_left_x, top_left_y) = (top_left_x as i32 / RENDER_BLOCK_WIDTH, top_left_y as i32 / RENDER_BLOCK_WIDTH);
        let (bottom_right_x, bottom_right_y) = (bottom_right_x as i32 / RENDER_BLOCK_WIDTH, bottom_right_y as i32 / RENDER_BLOCK_WIDTH);
        for x in top_left_x..bottom_right_x {
            for y in top_left_y..bottom_right_y {
                if x >= 0 && y >= 0 && x < self.blocks.get_width() as i32 && y < self.blocks.get_height() as i32 {
                    let screen_x = x * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).0 as i32;
                    let screen_y = y * RENDER_BLOCK_WIDTH - camera.get_top_left(graphics).1 as i32;

                    let block_type = self.blocks.get_block_type(x, y);
                    if block_type.image.get_height() != 0 && block_type.image.get_width() != 0 {
                        //let texture = gfx::Texture::load_from_surface(&block_type.image);
                        //texture.render(&graphics.renderer, RENDER_SCALE, screen_x, screen_y, None, false, None);
                    }
                }
            }
        }
    }
}