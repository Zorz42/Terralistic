use super::networking::{Connection, NewConnectionEvent, PacketFromClientEvent, ServerNetworking};
use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::SendTarget;
use crate::server::server_core::players::ServerPlayers;
use crate::shared::blocks::{
    BlockBreakStartPacket, BlockBreakStopPacket, BlockChangeEvent, BlockChangePacket,
    BlockRightClickPacket, BlockStartedBreakingEvent, BlockStoppedBreakingEvent, Blocks,
    BlocksWelcomePacket,
};
use crate::shared::entities::Entities;
use crate::shared::inventory::Inventory;
use crate::shared::items::Items;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use anyhow::Result;
use std::collections::HashMap;

/// A struct that handles all block related stuff on the server side.
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

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.blocks.init(mods)
    }

    pub fn on_event(
        &mut self,
        event: &Event,
        events: &mut EventManager,
        networking: &mut ServerNetworking,
        entities: &mut Entities,
        players: &mut ServerPlayers,
        items: &mut Items,
    ) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.blocks.serialize()?,
            })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        } else if let Some(event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<BlockBreakStartPacket>() {
                if let Some(pos) = self.conns_breaking.get(&event.conn) {
                    self.blocks.stop_breaking_block(events, pos.0, pos.1)?;
                }

                self.conns_breaking
                    .insert(event.conn.clone(), (packet.x, packet.y));
                self.blocks
                    .start_breaking_block(events, packet.x, packet.y)?;
            }

            if let Some(packet) = event.packet.try_deserialize::<BlockBreakStopPacket>() {
                self.conns_breaking.remove(&event.conn);
                self.blocks
                    .stop_breaking_block(events, packet.x, packet.y)?;
            } else if let Some(packet) = event.packet.try_deserialize::<BlockRightClickPacket>() {
                let block = self.blocks.get_block(packet.x, packet.y)?;
                if block == self.blocks.air {
                    let player = players.get_player_from_connection(&event.conn)?;
                    let mut player_inventory = entities.ecs.get::<&mut Inventory>(player)?;
                    let selected_item = player_inventory.get_selected_item();
                    if let Some(mut selected_item) = selected_item {
                        let item_info = items.get_item_type(selected_item.item)?;
                        if let Some(block) = item_info.places_block {
                            self.blocks.set_block(events, packet.x, packet.y, block)?;
                            selected_item.count -= 1;
                            let selected_slot = player_inventory.selected_slot.unwrap_or(0);
                            player_inventory.set_item(selected_slot, Some(selected_item))?;
                        }
                    }
                }
            }
        } else if let Some(event) = event.downcast::<BlockStartedBreakingEvent>() {
            let packet = Packet::new(BlockBreakStartPacket {
                x: event.x,
                y: event.y,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        } else if let Some(event) = event.downcast::<BlockStoppedBreakingEvent>() {
            let packet = Packet::new(BlockBreakStopPacket {
                x: event.x,
                y: event.y,
                break_time: self.blocks.get_break_progress(event.x, event.y)?,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        } else if let Some(event) = event.downcast::<BlockChangeEvent>() {
            let from_main = self.blocks.get_block_from_main(event.x, event.y)?;
            let packet = Packet::new(BlockChangePacket {
                x: event.x,
                y: event.y,
                from_main_x: from_main.0,
                from_main_y: from_main.1,
                block: self.blocks.get_block(event.x, event.y)?,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;

            let neighbors = [
                (event.x, event.y),
                (event.x - 1, event.y),
                (event.x + 1, event.y),
                (event.x, event.y - 1),
                (event.x, event.y + 1),
            ];

            for (x, y) in neighbors {
                if x >= 0
                    && y >= 0
                    && x < self.blocks.get_width() as i32
                    && y < self.blocks.get_height() as i32
                {
                    self.blocks.update_block(x, y, events)?;
                }
            }
        }
        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager, frame_length: f32) -> Result<()> {
        self.blocks.update_breaking_blocks(events, frame_length)
    }
}
