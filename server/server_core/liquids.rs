use std::collections::BTreeSet;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::libraries::scripting::ScriptHost;
use crate::server::server_core::networking::SendTarget;
use crate::shared::blocks::{BlockChangeEvent, Blocks};
use crate::shared::liquids::{init_liquids_mod_interface, LiquidChange, LiquidChangeEvent, LiquidChangesPacket, LiquidId, Liquids, LiquidsWelcomePacket};
use crate::shared::packet::Packet;

use super::networking::{NewConnectionEvent, ServerNetworking};

/// The authoritative liquid grid. Clients never simulate liquids - they are told what
/// changed, once per flow step.
pub struct ServerLiquids {
    liquids: Arc<Mutex<Liquids>>,
    /// Cells changed since the last packet went out, deduplicated: one flow step commonly
    /// touches the same cell from both sides.
    pending_changes: BTreeSet<(i32, i32)>,
    event_receiver: Option<Receiver<Event>>,
}

impl ServerLiquids {
    pub fn new() -> Self {
        Self {
            liquids: Arc::new(Mutex::new(Liquids::new())),
            pending_changes: BTreeSet::new(),
            event_receiver: None,
        }
    }

    pub fn init(&mut self, mods: &mut ScriptHost) -> Result<()> {
        init_liquids_mod_interface(mods, &self.liquids)?;
        self.event_receiver = Some(init_liquids_mod_interface_server(&self.liquids, mods)?);
        Ok(())
    }

    pub fn get_liquids(&self) -> MutexGuard<'_, Liquids> {
        self.liquids.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(LiquidsWelcomePacket {
                data: self.get_liquids().serialize()?,
            })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        } else if let Some(event) = event.downcast::<LiquidChangeEvent>() {
            self.pending_changes.insert((event.x, event.y));
        } else if let Some(event) = event.downcast::<BlockChangeEvent>() {
            // a liquid does not only move when another liquid moves: digging out the block
            // under a pool has to wake it up again
            self.get_liquids().schedule_update(event.x, event.y);
        }
        Ok(())
    }

    pub fn update(&mut self, blocks: &Blocks, events: &mut EventManager, networking: &mut ServerNetworking, frame_length: f32) -> Result<()> {
        self.flush_mods_events(events);
        self.send_pending_changes(networking)?;
        self.get_liquids().update_liquids(blocks, events, frame_length)
    }

    /// Sends everything that changed since the last update as one packet.
    fn send_pending_changes(&mut self, networking: &mut ServerNetworking) -> Result<()> {
        if self.pending_changes.is_empty() {
            return Ok(());
        }

        let mut changes = Vec::new();
        let pending = std::mem::take(&mut self.pending_changes);
        {
            let liquids = self.get_liquids();
            for (x, y) in pending {
                let liquid = liquids.get_liquid(x, y)?;
                changes.push(LiquidChange {
                    x,
                    y,
                    liquid: liquid.id,
                    level: liquid.level,
                });
            }
        }

        networking.send_packet(&Packet::new(LiquidChangesPacket { changes })?, SendTarget::All)
    }

    fn flush_mods_events(&self, events: &mut EventManager) {
        if let Some(receiver) = &self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                events.push_event(event);
            }
        }
    }
}

/// Initializes the parts of the liquids mod interface that only exist on the server.
///
/// Placing liquid is server side for the same reason breaking a block is: the client's copy
/// is a replica, and anything that changes the world has to happen where the authority is
/// and come back as a packet.
pub fn init_liquids_mod_interface_server(liquids: &Arc<Mutex<Liquids>>, mods: &mut ScriptHost) -> Result<Receiver<Event>> {
    let (sender, receiver) = std::sync::mpsc::channel();

    let liquids_clone = liquids.clone();
    mods.add_global_function("set_liquid", move |_lua, (x, y, liquid_id, level): (i32, i32, LiquidId, i32)| {
        let mut events = EventManager::new();

        let level = u8::try_from(level.max(0)).unwrap_or(u8::MAX);
        liquids_clone
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .set_liquid(x, y, liquid_id, level, &mut events)
            .map_err(|err| rlua::Error::RuntimeError(err.to_string()))?;

        while let Some(event) = events.pop_event() {
            sender.send(event).ok().ok_or(rlua::Error::RuntimeError("could not send event".to_owned()))?;
        }

        Ok(())
    })?;

    Ok(receiver)
}
