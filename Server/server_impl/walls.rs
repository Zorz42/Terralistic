use shared::blocks::Blocks;
use shared::mod_manager::ModManager;
use shared::packet::Packet;
use shared::walls::{Walls, WallsWelcomePacket};
use events::{Event, EventManager};
use crate::networking::{NewConnectionEvent, ServerNetworking};

pub struct ServerWalls {
    pub walls: Walls,
}

impl ServerWalls {
    pub fn new(blocks: &mut Blocks) -> Self {
        Self {
            walls: Walls::new(blocks),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) {
        self.walls.init(mods);
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let welcome_packet = Packet::new(WallsWelcomePacket {
                data: self.walls.serialize().unwrap()
            });
            networking.send_packet(&welcome_packet, &event.conn);
        }
    }

    pub fn update(&mut self, frame_length: f32, events: &mut EventManager) {
        self.walls.update_breaking_walls(frame_length, events).unwrap();
    }
}
