use crate::game::camera::Camera;
use crate::game::networking::ClientNetworking;
use events::Event;
use graphics as gfx;
use shared::blocks::{
    BlockBreakStartPacket, BlockBreakStopPacket, RENDER_BLOCK_WIDTH, RENDER_SCALE,
};
use shared::packet::Packet;

/**
Block selector is used to select a block.
It draws a red rectangle around the block
that is currently selected with the mouse.
 */
pub struct BlockSelector {
    prev_selected: (i32, i32),
    breaking: bool,
}

impl BlockSelector {
    pub fn new() -> Self {
        Self {
            prev_selected: (0, 0),
            breaking: false,
        }
    }

    /**
    This function gets the current block that is selected.
     */
    pub fn get_selected_block(
        &self,
        graphics: &mut gfx::GraphicsContext,
        camera: &Camera,
    ) -> (i32, i32) {
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
    pub fn render(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        networking: &mut ClientNetworking,
        camera: &Camera,
    ) {
        let selected_block = self.get_selected_block(graphics, camera);

        let x = selected_block.0 * RENDER_BLOCK_WIDTH
            - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32;
        let y = selected_block.1 * RENDER_BLOCK_WIDTH
            - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32;

        gfx::Rect::new(x, y, RENDER_BLOCK_WIDTH, RENDER_BLOCK_WIDTH)
            .render_outline(graphics, gfx::Color::new(255, 0, 0, 255));

        if self.prev_selected != selected_block {
            if self.breaking {
                self.start_breaking(networking, selected_block);
            }
            self.prev_selected = selected_block;
        }
    }

    fn start_breaking(&mut self, networking: &mut ClientNetworking, pos: (i32, i32)) {
        self.breaking = true;

        networking.send_packet(&Packet::new(BlockBreakStartPacket { x: pos.0, y: pos.1 }))
    }

    fn stop_breaking(&mut self, networking: &mut ClientNetworking, pos: (i32, i32)) {
        self.breaking = false;

        networking.send_packet(&Packet::new(BlockBreakStopPacket {
            x: pos.0,
            y: pos.1,
            break_time: 0, // server ignores this
        }))
    }

    /**
    This function is called on some event
     */
    pub fn on_event(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        networking: &mut ClientNetworking,
        camera: &Camera,
        event: &Event,
    ) {
        if let Some(event) = event.downcast::<gfx::Event>() {
            match event {
                gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) => {
                    self.start_breaking(networking, self.get_selected_block(graphics, camera));
                }
                gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) => {
                    self.stop_breaking(networking, self.get_selected_block(graphics, camera));
                }
                gfx::Event::KeyPress(gfx::Key::MouseRight, ..) => {}
                _ => {}
            }
        }
    }
}
