use anyhow::Result;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::SendTarget;
use crate::shared::blocks::Blocks;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::walls::{init_walls_mod_interface, Walls, WallsWelcomePacket};

use super::networking::{NewConnectionEvent, ServerNetworking};

pub struct ServerWalls {
    walls: Arc<Mutex<Walls>>,
}

impl ServerWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Arc::new(Mutex::new(Walls::new(blocks))),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        init_walls_mod_interface(mods, &self.walls)
    }

    pub fn get_walls(&self) -> MutexGuard<Walls> {
        self.walls.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(WallsWelcomePacket { data: self.get_walls().serialize()? })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        }
        Ok(())
    }

    pub fn update(&mut self, frame_length: f32, events: &mut EventManager) -> Result<()> {
        self.get_walls().update_breaking_walls(frame_length, events)
    }
}
