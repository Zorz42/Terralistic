use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::{SendTarget, ServerNetworking};
use crate::shared::blocks::BlockBreakEvent;
use crate::shared::entities::{Entities, IdComponent, PositionComponent};
use crate::shared::items::{ItemComponent, ItemSpawnEvent, ItemSpawnPacket, Items};
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
        entities: &mut Entities,
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
                        entities,
                        drop.item,
                        event.x as f32,
                        event.y as f32,
                        None,
                    );
                }
            }
        }
        if let Some(event) = event.downcast::<ItemSpawnEvent>() {
            let entity = event.entity;
            let item = entities.ecs.get::<&ItemComponent>(entity)?;
            let id = *entities.ecs.get::<&IdComponent>(entity)?;
            let position = entities.ecs.get::<&PositionComponent>(entity)?;

            let packet = ItemSpawnPacket {
                item_type: item.get_item_type(),
                id,
                x: position.x(),
                y: position.y(),
            };

            networking.send_packet(&Packet::new(packet)?, SendTarget::All)?;
        }
        Ok(())
    }
}
