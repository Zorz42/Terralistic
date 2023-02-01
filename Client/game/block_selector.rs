use shared::blocks::{RENDER_BLOCK_WIDTH, RENDER_SCALE};
use crate::game::camera::Camera;
use graphics as gfx;

/**
Block selector is used to select a block.
It draws a red rectangle around the block
that is currently selected with the mouse.
 */
pub struct BlockSelector {}

impl BlockSelector {
    pub fn new() -> Self {
        Self {}
    }

    /**
    This function gets the current block that is selected.
     */
    pub fn get_selected_block(&self, graphics: &mut gfx::GraphicsContext, camera: &Camera) -> (i32, i32) {
        let mouse_x = graphics.renderer.get_mouse_x();
        let mouse_y = graphics.renderer.get_mouse_y();

        let mouse_x = mouse_x + camera.get_top_left(graphics).0 * RENDER_SCALE;
        let mouse_y = mouse_y + camera.get_top_left(graphics).1 * RENDER_SCALE;

        let block_x = mouse_x as i32 / RENDER_BLOCK_WIDTH;
        let block_y = mouse_y as i32 / RENDER_BLOCK_WIDTH;

        (block_x, block_y)
    }

    /**
    This function is called on every frame
     */
    pub fn render(&self, graphics: &mut gfx::GraphicsContext, camera: &Camera) {
        let selected_block = self.get_selected_block(graphics, camera);

        let x = selected_block.0 * RENDER_BLOCK_WIDTH - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32;
        let y = selected_block.1 * RENDER_BLOCK_WIDTH - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32;

        gfx::Rect::new(x, y, RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH).render_outline(graphics, gfx::Color::new(255, 0, 0, 255));
    }
}