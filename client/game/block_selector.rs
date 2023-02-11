use super::camera::Camera;
use super::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize};
use crate::shared::blocks::{
    BlockBreakStartPacket, BlockBreakStopPacket, RENDER_BLOCK_WIDTH, RENDER_SCALE,
};
use crate::shared::packet::Packet;
use anyhow::Result;

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
    pub const fn new() -> Self {
        Self {
            prev_selected: (0, 0),
            breaking: false,
        }
    }

    /**
    This function gets the current block that is selected.
     */
    pub fn get_selected_block(graphics: &mut gfx::GraphicsContext, camera: &Camera) -> (i32, i32) {
        let mouse_x = graphics.renderer.get_mouse_pos().0;
        let mouse_y = graphics.renderer.get_mouse_pos().1;

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
    ) -> Result<()> {
        let selected_block = Self::get_selected_block(graphics, camera);

        let x = selected_block.0 * RENDER_BLOCK_WIDTH
            - (camera.get_top_left(graphics).0 * RENDER_SCALE) as i32;
        let y = selected_block.1 * RENDER_BLOCK_WIDTH
            - (camera.get_top_left(graphics).1 * RENDER_SCALE) as i32;

        gfx::Rect::new(
            FloatPos(x as f32, y as f32),
            FloatSize(RENDER_BLOCK_WIDTH as f32, RENDER_BLOCK_WIDTH as f32),
        )
        .render_outline(graphics, gfx::Color::new(255, 0, 0, 255));

        if self.prev_selected != selected_block {
            if self.breaking {
                self.start_breaking(networking, selected_block)?;
            }
            self.prev_selected = selected_block;
        }
        Ok(())
    }

    fn start_breaking(&mut self, networking: &mut ClientNetworking, pos: (i32, i32)) -> Result<()> {
        self.breaking = true;

        networking.send_packet(&Packet::new(BlockBreakStartPacket { x: pos.0, y: pos.1 })?)?;
        Ok(())
    }

    fn stop_breaking(&mut self, networking: &mut ClientNetworking, pos: (i32, i32)) -> Result<()> {
        self.breaking = false;

        networking.send_packet(&Packet::new(BlockBreakStopPacket {
            x: pos.0,
            y: pos.1,
            break_time: 0, // server ignores this
        })?)?;
        Ok(())
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
    ) -> Result<()> {
        if let Some(event) = event.downcast::<gfx::Event>() {
            match event {
                gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) => {
                    self.start_breaking(networking, Self::get_selected_block(graphics, camera))?;
                }
                gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) => {
                    self.stop_breaking(networking, Self::get_selected_block(graphics, camera))?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
