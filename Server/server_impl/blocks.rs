use std::any::Any;
use shared::blocks::blocks::{Blocks, BlocksWelcomePacket};
use shared::mod_manager::ModManager;
use shared::packet::Packet;
use events::Event;
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
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.blocks.serialize(),
                width: self.blocks.get_width(),
                height: self.blocks.get_height(),
            });
            networking.send_packet(&welcome_packet, &event.conn);
        }
    }
}