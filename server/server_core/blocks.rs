use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::{Mutex, MutexGuard, PoisonError};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::libraries::scripting::ScriptHost;
use crate::server::server_core::networking::SendTarget;
use crate::server::server_core::players::ServerPlayers;
use crate::shared::blocks::{
    init_blocks_mod_interface, BlockBreakEvent, BlockBreakStartPacket, BlockBreakStopPacket, BlockChangeEvent, BlockChangePacket, BlockId, BlockInventoryChangeEvent, BlockRightClickPacket,
    BlockStartedBreakingEvent, BlockStoppedBreakingEvent, BlockUpdateEvent, Blocks, BlocksWelcomePacket, ClientBlockBreakStartPacket,
};
use crate::shared::entities::Entities;
use crate::shared::inventory::Inventory;
use crate::shared::items::{ItemId, ItemStack, Items};
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

    pub fn init(&mut self, mods: &mut ScriptHost) -> Result<()> {
        init_blocks_mod_interface(&self.blocks, mods)?;
        let receiver = init_blocks_mod_interface_server(&self.blocks, mods)?;
        self.event_receiver = Some(receiver);
        Ok(())
    }

    pub fn get_blocks(&self) -> MutexGuard<'_, Blocks> {
        self.blocks.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether the cell holds a block type that covers more than one cell. Out of bounds is
    /// not a big block rather than an error, because this is asked about neighbours.
    fn is_big_block(&self, x: i32, y: i32) -> bool {
        let blocks = self.get_blocks();
        let (width, height) = blocks.get_size();

        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return false;
        }

        blocks.get_block_type_at(x, y).is_ok_and(|block_type| block_type.width > 1 || block_type.height > 1)
    }

    /// Grows every multiblock in the world into its full footprint.
    ///
    /// This is what a freshly generated world needs: the generator places only a multiblock's
    /// main cell - a tree's canopy is 5x5 in `blocks.lua` and one entry in the biome's `terrain`
    /// table - and `update_block` is what fills in the rest. It only reaches one cell right and
    /// one cell down, so the remaining cells come from updating the ones it just filled in;
    /// while the server is running that continues by itself through `BlockChangeEvent`, but
    /// generation has to finish before anyone can look at the world, so the walk happens here.
    ///
    /// The scan for where to start takes the lock once and clones nothing. On the default world
    /// it visits 5.4 million cells, which is enough for the difference between reading two
    /// `i32`s and cloning a `Block` with two `Vec`s in it to be seconds of startup.
    pub fn expand_big_blocks(&self, events: &mut EventManager) -> Result<()> {
        let mut queue = std::collections::VecDeque::new();

        {
            let blocks = self.get_blocks();
            let (width, height) = blocks.get_size();
            for x in 0..width as i32 {
                for y in 0..height as i32 {
                    let block_type = blocks.get_block_type_at(x, y)?;
                    if block_type.width > 1 || block_type.height > 1 {
                        queue.push_back((x, y));
                    }
                }
            }
        }

        let mut queued: std::collections::HashSet<(i32, i32)> = queue.iter().copied().collect();

        while let Some((x, y)) = queue.pop_front() {
            self.update_block(x, y, events)?;

            // whatever the update just filled in has a footprint of its own to grow
            for neighbor in [(x + 1, y), (x, y + 1)] {
                if self.is_big_block(neighbor.0, neighbor.1) && queued.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        Ok(())
    }

    /// Updates the block at the specified coordinates.
    pub fn update_block(&self, x: i32, y: i32, events: &mut EventManager) -> Result<()> {
        // check multiblock (big blocks)
        let block = self.get_blocks().get_block_type_at(x, y)?.clone();
        if block.width != 0 || block.height != 0 {
            let from_main = self.get_blocks().get_block_from_main(x, y)?;

            // if it is not the main block of the big block and if there is no main block anymore, break it
            if (from_main.0 > 0 && self.get_blocks().get_block_type_at(x - 1, y)?.get_id() != block.get_id())
                || (from_main.1 > 0 && self.get_blocks().get_block_type_at(x, y - 1)?.get_id() != block.get_id())
            {
                let air = self.get_blocks().air();
                self.get_blocks().set_block(events, x, y, air)?;
                return Ok(());
            }

            // recursively add blocks to the right and down
            if from_main.0 + 1 < block.width {
                self.get_blocks().set_big_block(events, x + 1, y, block.get_id(), (from_main.0 + 1, from_main.1))?;
            }

            if from_main.1 + 1 < block.height {
                self.get_blocks().set_big_block(events, x, y + 1, block.get_id(), (from_main.0, from_main.1 + 1))?;
            }
        }

        events.push_event(Event::new(BlockUpdateEvent { x, y }));

        Ok(())
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
        mods: &mut ScriptHost,
    ) -> Result<()> {
        self.flush_mods_events(events);
        handle_event_for_mods(mods, event)?;

        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(BlocksWelcomePacket { data: self.get_blocks().serialize()? })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        } else if let Some(event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<ClientBlockBreakStartPacket>() {
                if let Some(pos) = self.conns_breaking.get(&event.conn).copied() {
                    self.get_blocks().stop_breaking_block(events, pos.0, pos.1)?;
                }

                self.conns_breaking.insert(event.conn.clone(), (packet.x, packet.y));

                let player_id = players.get_player_from_connection(&event.conn)?;
                if let Some(player_id) = player_id {
                    let held_item = entities.ecs.get::<&Inventory>(player_id)?.get_selected_item();
                    let tool = if let Some(item) = &held_item { items.get_item_type(item.item)?.tool } else { None };

                    let tool_power = if let Some(item) = &held_item { items.get_item_type(item.item)?.tool_power } else { 0 };

                    self.get_blocks().start_breaking_block(events, packet.x, packet.y, tool, tool_power)?;
                }
            }

            if let Some(packet) = event.packet.try_deserialize::<BlockBreakStopPacket>() {
                self.conns_breaking.remove(&event.conn);
                self.get_blocks().stop_breaking_block(events, packet.x, packet.y)?;
            } else if let Some(packet) = event.packet.try_deserialize::<BlockRightClickPacket>() {
                let player = players.get_player_from_connection(&event.conn)?;
                if let Some(player) = player {
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
                                        let current_block = self.get_blocks().get_block(packet.x + x, packet.y - y)?;
                                        if current_block != self.get_blocks().air() {
                                            can_place = false;
                                        }
                                    }
                                }
                                can_place
                            };

                            if can_place {
                                self.get_blocks().set_block(events, packet.x, packet.y - block_height + 1, block)?;
                                selected_item.count -= 1;
                                let selected_slot = player_inventory.selected_slot.unwrap_or(0);
                                player_inventory.set_item(selected_slot, Some(selected_item))?;
                            }
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
            let block = self.get_blocks().get_block(event.x, event.y)?;
            let inventory = self.get_blocks().get_block_inventory_data(event.x, event.y)?;
            let packet = Packet::new(BlockChangePacket {
                x: event.x,
                y: event.y,
                from_main_x: from_main.0,
                from_main_y: from_main.1,
                block,
                inventory,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;

            let neighbors = [(event.x, event.y), (event.x - 1, event.y), (event.x + 1, event.y), (event.x, event.y - 1), (event.x, event.y + 1)];

            for (x, y) in neighbors {
                if x >= 0 && y >= 0 && x < self.get_blocks().get_size().0 as i32 && y < self.get_blocks().get_size().1 as i32 {
                    self.update_block(x, y, events)?;
                }
            }
        } else if let Some(event) = event.downcast::<BlockInventoryChangeEvent>() {
            let block = self.get_blocks().get_block(event.x, event.y)?;
            let from_main = self.get_blocks().get_block_from_main(event.x, event.y)?;
            let inventory = self.get_blocks().get_block_inventory_data(event.x, event.y)?;
            let packet = Packet::new(BlockChangePacket {
                x: event.x,
                y: event.y,
                block,
                from_main_x: from_main.0,
                from_main_y: from_main.1,
                inventory,
            })?;
            networking.send_packet(&packet, SendTarget::All)?;
        }
        Ok(())
    }

    fn flush_mods_events(&self, events: &mut EventManager) {
        if let Some(receiver) = &self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                events.push_event(event);
            }
        }
    }

    pub fn update(&self, events: &mut EventManager, frame_length: f32) -> Result<()> {
        self.flush_mods_events(events);

        self.get_blocks().update_breaking_blocks(events, frame_length)
    }
}

/// initialize the mod interface for the blocks module for server side
#[allow(clippy::too_many_lines)]
pub fn init_blocks_mod_interface_server(blocks: &Arc<Mutex<Blocks>>, mods: &mut ScriptHost) -> Result<Receiver<Event>> {
    let (sender, receiver) = std::sync::mpsc::channel();

    // a method to break a block
    let blocks_clone = blocks.clone();
    let sender_clone = sender.clone();
    mods.add_global_function("break_block", move |_lua, (x, y): (i32, i32)| {
        let mut blocks = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let mut events = EventManager::new();
        blocks.break_block(&mut events, x, y).ok().ok_or(rlua::Error::RuntimeError("block type id is invalid".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender_clone.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }
        Ok(())
    })?;

    // a method to set block id by position
    let blocks_clone = blocks.clone();
    let sender_clone = sender.clone();
    mods.add_global_function("set_block", move |_lua, (x, y, block_id): (i32, i32, BlockId)| {
        let mut events = EventManager::new();

        let mut blocks = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        blocks
            .set_block(&mut events, x, y, block_id)
            .ok()
            .ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender_clone.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }

        Ok(())
    })?;

    // a method to set block inventory items and their counts by position
    let blocks_clone = blocks.clone();
    let sender_clone = sender;
    mods.add_global_function("set_block_inventory_item", move |_lua, (x, y, index, item_id, count): (i32, i32, i32, ItemId, i32)| {
        let mut events = EventManager::new();
        let mut blocks = blocks_clone.lock().unwrap_or_else(PoisonError::into_inner);
        let mut inventory = blocks.get_block_inventory_data(x, y).ok().ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;

        if let Some(item) = inventory.get_mut(index as usize) {
            *item = Some(ItemStack::new(item_id, count));
        } else {
            return Err(rlua::Error::RuntimeError("index out of bounds".to_owned()));
        }

        blocks
            .set_block_inventory_data(x, y, inventory, &mut events)
            .ok()
            .ok_or(rlua::Error::RuntimeError("coordinates out of bounds".to_owned()))?;

        while let Some(event) = events.pop_event() {
            sender_clone.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }

        Ok(())
    })?;

    Ok(receiver)
}

fn handle_event_for_mods(mods: &mut ScriptHost, event: &Event) -> Result<()> {
    if let Some(event) = event.downcast::<BlockBreakEvent>() {
        for game_mod in mods.modules_iter_mut() {
            if game_mod.is_symbol_defined("on_block_break")? {
                game_mod.call_function::<(i32, i32, BlockId), ()>("on_block_break", (event.x, event.y, event.prev_block_id))?;
            }
        }
    }

    if let Some(event) = event.downcast::<BlockUpdateEvent>() {
        for game_mod in mods.modules_iter_mut() {
            if game_mod.is_symbol_defined("on_block_update")? {
                game_mod.call_function::<(i32, i32), ()>("on_block_update", (event.x, event.y))?;
            }
        }
    }

    Ok(())
}
