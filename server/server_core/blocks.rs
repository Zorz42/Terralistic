use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::SendTarget;
use crate::server::server_core::players::ServerPlayers;
use crate::shared::blocks::{
    handle_event_for_blocks_interface, init_blocks_mod_interface, BlockBreakStartPacket,
    BlockBreakStopPacket, BlockChangeEvent, BlockChangePacket, BlockInventoryUpdateEvent,
    BlockInventoryUpdatePacket, BlockRightClickPacket, BlockStartedBreakingEvent,
    BlockStoppedBreakingEvent, Blocks, BlocksWelcomePacket, ClientBlockBreakStartPacket,
};
use crate::shared::entities::Entities;
use crate::shared::inventory::Inventory;
use crate::shared::items::Items;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;

use super::networking::{Connection, NewConnectionEvent, PacketFromClientEvent, ServerNetworking};

/// A struct that handles all block related stuff on the server side.
pub struct ServerBlocks {
    blocks: Arc<Mutex<Blocks>>,
    conns_breaking: HashMap<Connection, (i32, i32)>,
    event_receiver: Option<Receiver<Event>>,
}

impl ServerBlocks {
    pub fn new() -> Self {
        Self {
            blocks: Arc::new(Mutex::new(Blocks::new())),
            conns_breaking: HashMap::new(),
            event_receiver: None,
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        let receiver = init_blocks_mod_interface(&self.blocks, mods)?;
        self.event_receiver = Some(receiver);
        Ok(())
    }

    pub fn get_blocks(&self) -> MutexGuard<Blocks> {
        self.blocks.lock().unwrap_or_else(PoisonError::into_inner)
    }

    #[allow(clippy::too_many_lines)]
    pub fn on_event(
        &mut self,
        event: &Event,
        events: &mut EventManager,
        networking: &mut ServerNetworking,
        entities: &Entities,
        players: &ServerPlayers,
        items: &Items,
        mods: &mut ModManager,
    ) -> Result<()> {
        self.flush_mods_events(events);
        handle_event_for_blocks_interface(mods, event)?;

        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket {
                data: self.get_blocks().serialize()?,
            })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        } else if let Some(event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = event
                .packet
                .try_deserialize::<ClientBlockBreakStartPacket>()
            {
                if let Some(pos) = self.conns_breaking.get(&event.conn).copied() {
                    self.get_blocks()
                        .stop_breaking_block(events, pos.0, pos.1)?;
                }

                self.conns_breaking
                    .insert(event.conn.clone(), (packet.x, packet.y));

                let player_id = players.get_player_from_connection(&event.conn)?;
                let held_item = entities
                    .ecs
                    .get::<&Inventory>(player_id)?
                    .get_selected_item();
                let tool = if let Some(item) = &held_item {
                    items.get_item_type(item.item)?.tool
                } else {
                    None
                };

                let tool_power = if let Some(item) = &held_item {
                    items.get_item_type(item.item)?.tool_power
                } else {
                    0
                };

                self.get_blocks()
                    .start_breaking_block(events, packet.x, packet.y, tool, tool_power)?;
            }

            if let Some(packet) = event.packet.try_deserialize::<BlockBreakStopPacket>() {
                self.conns_breaking.remove(&event.conn);
                self.get_blocks()
                    .stop_breaking_block(events, packet.x, packet.y)?;
            } else if let Some(packet) = event.packet.try_deserialize::<BlockRightClickPacket>() {
                let player = players.get_player_from_connection(&event.conn)?;
                let mut player_inventory = entities.ecs.get::<&mut Inventory>(player)?;
                let selected_item = player_inventory.get_selected_item();

                if let Some(mut selected_item) = selected_item {
                    let item_info = items.get_item_type(selected_item.item)?;

                    if let Some(block) = item_info.places_block {
                        let block_width = self.get_blocks().get_block_type(block)?.width;
                        let block_height = self.get_blocks().get_block_type(block)?.height;

                        let can_place = {
                            let mut can_place = true;
                            for x in 0..block_width {
                                for y in 0..block_height {
                                    let current_block =
                                        self.get_blocks().get_block(packet.x + x, packet.y - y)?;
                                    if current_block != self.get_blocks().air() {
                                        can_place = false;
                                    }
                                }
                            }
                            can_place
                        };

                        if can_place {
                            self.get_blocks().set_block(
                                events,
                                packet.x,
                                packet.y - block_height + 1,
                                block,
                            )?;
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
                tool: event.tool,
                tool_power: event.tool_power,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        } else if let Some(event) = event.downcast::<BlockStoppedBreakingEvent>() {
            let packet = Packet::new(BlockBreakStopPacket {
                x: event.x,
                y: event.y,
                break_time: self.get_blocks().get_break_progress(event.x, event.y)?,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        } else if let Some(event) = event.downcast::<BlockChangeEvent>() {
            let from_main = self.get_blocks().get_block_from_main(event.x, event.y)?;
            let packet = Packet::new(BlockChangePacket {
                x: event.x,
                y: event.y,
                from_main_x: from_main.0,
                from_main_y: from_main.1,
                block: self.get_blocks().get_block(event.x, event.y)?,
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
                    && x < self.get_blocks().get_width() as i32
                    && y < self.get_blocks().get_height() as i32
                {
                    self.get_blocks().update_block(x, y, events)?;
                }
            }
        } else if let Some(event) = event.downcast::<BlockInventoryUpdateEvent>() {
            let packet = Packet::new(BlockInventoryUpdatePacket {
                x: event.x,
                y: event.y,
                inventory: self
                    .get_blocks()
                    .get_block_inventory_data(event.x, event.y)?
                    .unwrap_or(&vec![])
                    .clone(),
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        }
        Ok(())
    }

    fn flush_mods_events(&mut self, events: &mut EventManager) {
        if let Some(receiver) = &self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                events.push_event(event);
            }
        }
    }

    pub fn update(&mut self, events: &mut EventManager, frame_length: f32) -> Result<()> {
        self.flush_mods_events(events);

        self.get_blocks()
            .update_breaking_blocks(events, frame_length)
    }
}
