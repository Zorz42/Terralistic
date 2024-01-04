use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::{SendTarget, ServerNetworking};
use crate::shared::blocks::BlockBreakEvent;
use crate::shared::entities::{Entities, PhysicsComponent, PositionComponent};
use crate::shared::items::{init_items_mod_interface, ItemComponent, ItemSpawnEvent, ItemSpawnPacket, Items};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;

pub struct ServerItems {
    items: Arc<Mutex<Items>>,
    event_receiver: Option<Receiver<Event>>,
}

impl ServerItems {
    pub fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(Items::new())),
            event_receiver: None,
        }
    }

    pub fn init(&mut self, mods: &mut ModManager, entities: &Arc<Mutex<Entities>>) -> Result<()> {
        self.event_receiver = Some(init_items_mod_interface(&self.items, entities, mods)?);
        Ok(())
    }

    pub fn update(&mut self, events: &mut EventManager) {
        if let Some(receiver) = &self.event_receiver {
            if let Ok(event) = receiver.try_recv() {
                events.push_event(event);
            }
        }
    }

    pub fn on_event(&mut self, event: &Event, entities: &mut Entities, events: &mut EventManager, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<BlockBreakEvent>() {
            let broken_block = event.prev_block_id;
            let drop = self.get_items().get_block_drop(broken_block);
            if let Ok(drop) = drop {
                // spawn item at random chance. drop.chance is a float between 0 and 1
                if rand::random::<f32>() <= drop.chance {
                    self.get_items().drop_item(events, entities, drop.item, event.x as f32, event.y as f32)?;
                }
            }
        }
        if let Some(event) = event.downcast::<ItemSpawnEvent>() {
            let entity = event.entity;
            let physics = entities.ecs.get::<&mut PhysicsComponent>(entity)?;
            let item = entities.ecs.get::<&ItemComponent>(entity)?;
            let id = entities.get_id_from_entity(entity)?;
            let position = entities.ecs.get::<&PositionComponent>(entity)?;

            let packet = ItemSpawnPacket {
                item_type: item.get_item_type(),
                id,
                x: position.x(),
                y: position.y(),
                velocity_x: physics.velocity_x,
                velocity_y: physics.velocity_y,
            };

            networking.send_packet(&Packet::new(packet)?, SendTarget::All)?;
        }
        Ok(())
    }

    pub fn get_items(&self) -> MutexGuard<Items> {
        self.items.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
