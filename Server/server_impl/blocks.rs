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
        self.blocks.create(64, 64);

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

        // count number of test blocks
        let mut num_test_blocks = 0;
        for x in 0..self.blocks.get_width() {
            for y in 0..self.blocks.get_height() {
                if self.blocks.get_block_type(x, y) == self.blocks.test_block {
                    num_test_blocks += 1;
                }
            }
        }
        println!("Number of test blocks: {}", num_test_blocks);
    }

    pub fn on_event(&mut self, event: &Box<dyn Any>, networking: &mut ServerNetworking) {
        if let Some(event) = event.downcast_ref::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.blocks.serialize(),
                width: self.blocks.get_width(),
                height: self.blocks.get_height(),
            });
            // print size of packet
            println!("Size of blocks welcome packet: {}", welcome_packet.data.len());
            networking.send_packet(&welcome_packet, event.conn);
        }
    }
}