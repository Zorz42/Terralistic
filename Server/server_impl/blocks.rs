use std::any::Any;
use std::rc::Rc;
use shared::blocks::blocks::{Blocks, BlocksWelcomePacket};
use shared::mod_manager::ModManager;
use shared::packet::Packet;
use crate::networking::{NewConnectionEvent, ServerNetworking};

/**
A struct that handles all block related stuff on the server side.
 */
pub struct ServerBlocks {
    pub blocks: Blocks,
}

impl ServerBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.blocks.init(mods);
        self.blocks.create(1024, 1024);

        // set each block in the world to be either air or test_block randomly
        for x in 0..self.blocks.get_width() {
            for y in 0..self.blocks.get_height() {
                if rand::random::<i32>() % 10 != 0 {
                    self.blocks.set_block(x, y, self.blocks.air.clone());
                } else {
                    self.blocks.set_block(x, y, self.blocks.test_block.clone());
                }
            }
        }
    }

    pub fn on_event(&mut self, event: &Box<dyn Any>, networking: &mut ServerNetworking) {
        if let Some(event) = event.downcast_ref::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.blocks.serialize(),
                width: self.blocks.get_width(),
                height: self.blocks.get_height(),
            });
            networking.send_packet(&welcome_packet, &event.conn);
        }
    }
}