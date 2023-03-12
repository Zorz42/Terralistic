use super::networking::{NewConnectionEvent, ServerNetworking};
use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::SendTarget;
use crate::shared::blocks::Blocks;
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::walls::{Walls, WallsWelcomePacket};
use anyhow::Result;

pub struct ServerWalls {
    pub walls: Walls,
}

impl ServerWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.walls.init(mods)
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(WallsWelcomePacket {
                data: self.walls.serialize()?,
            })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        }
        Ok(())
    }

    pub fn update(&mut self, frame_length: f32, events: &mut EventManager) -> Result<()> {
        self.walls.update_breaking_walls(frame_length, events)
    }
}
