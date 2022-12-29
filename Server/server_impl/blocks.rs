use std::any::Any;
use std::rc::Rc;
use bincode::Options;
use shared::blocks::{Blocks, BlocksWelcomePacket};
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

    pub fn init(&mut self) {
        self.blocks.create(1024, 1024);

        // set each block in the world to be either air or test_block randomly
        for x in 0..self.blocks.get_width() {
            for y in 0..self.blocks.get_height() {
                if rand::random::<i32>() % 10 != 0 {
                    self.blocks.set_block(x, y, Rc::clone(&self.blocks.air));
                } else {
                    self.blocks.set_block(x, y, Rc::clone(&self.blocks.test_block));
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
            networking.get_connection_by_id(event.connection_id).unwrap().send_packet(welcome_packet);
        }
    }
}