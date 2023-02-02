use std::collections::HashMap;
use shared::blocks::{BlockBreakStartPacket, BlockBreakStopPacket, Blocks, BlocksWelcomePacket};
use shared::mod_manager::ModManager;
use shared::packet::Packet;
use events::{Event, EventManager};
use crate::networking::{Connection, NewConnectionEvent, PacketFromClientEvent, ServerNetworking};

/**
A struct that handles all block related stuff on the server side.
 */
pub struct ServerBlocks {
    pub blocks: Blocks,
    conns_breaking: HashMap<Connection, (i32, i32)>,
}

impl ServerBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Blocks::new(),
            conns_breaking: HashMap::new(),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.blocks.init(mods);
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.blocks.serialize().unwrap(),
                width: self.blocks.get_width(),
                height: self.blocks.get_height(),
            });
            networking.send_packet(&welcome_packet, &event.conn);
        } else if let Some(event) = event.downcast::<PacketFromClientEvent>() {

            if let Some(packet) = event.packet.deserialize::<BlockBreakStartPacket>() {
                if let Some(pos) = self.conns_breaking.get(&event.conn) {
                    self.blocks.stop_breaking_block(pos.0, pos.1).unwrap();
                }

                self.conns_breaking.insert(event.conn.clone(), (packet.x, packet.y));
                self.blocks.start_breaking_block(packet.x, packet.y).unwrap();
            }

            if let Some(packet) = event.packet.deserialize::<BlockBreakStopPacket>() {
                self.conns_breaking.remove(&event.conn);
                self.blocks.stop_breaking_block(packet.x, packet.y).unwrap();
            }
        }
    }

    pub fn update(&mut self, events: &mut EventManager, frame_length: f32) {
        self.blocks.update_breaking_blocks(events, frame_length).unwrap();
    }
}