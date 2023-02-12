use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::ServerNetworking;
use crate::shared::blocks::{BlockBreakEvent, BLOCK_WIDTH};
use crate::shared::items::{ItemSpawnEvent, ItemSpawnPacket, Items};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use anyhow::Result;

pub struct ServerItems {
    pub items: Items,
}

impl ServerItems {
    pub fn new() -> Self {
        Self {
            items: Items::new(),
        }
    }

    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        self.items.init(mods)
    }

    pub fn on_event(
        &mut self,
        event: &Event,
        events: &mut EventManager,
        networking: &mut ServerNetworking,
    ) -> Result<()> {
        if let Some(event) = event.downcast::<BlockBreakEvent>() {
            let broken_block = event.prev_block_id;
            let drop = self.items.get_block_drop(broken_block);
            if let Ok(drop) = drop {
                // spawn item at random chance. drop.chance is a float between 0 and 1
                if rand::random::<f32>() < drop.chance {
                    self.items.spawn_item(
                        events,
                        &self.items.get_item_type(drop.item)?,
                        (event.x * BLOCK_WIDTH) as f32,
                        (event.y * BLOCK_WIDTH) as f32,
                        None,
                    );
                }
            }
        }
        if let Some(event) = event.downcast::<ItemSpawnEvent>() {
            let item = self.items.get_item_by_id(event.id)?;

            let packet = ItemSpawnPacket {
                item_type: item.item_id,
                id: event.id,
                x: item.entity.x,
                y: item.entity.y,
            };

            networking.send_packet_to_all(&Packet::new(packet)?)?;
        }
        Ok(())
    }
}
