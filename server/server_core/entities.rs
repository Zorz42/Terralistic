use anyhow::Result;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crate::libraries::events::Event;
use crate::server::server_core::networking::{SendTarget, ServerNetworking};
use crate::shared::entities::{Entities, EntityDespawnEvent, EntityDespawnPacket, EntityPositionVelocityPacket, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;

pub struct ServerEntities {
    entities: Arc<Mutex<Entities>>,
}

impl ServerEntities {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(Entities::new())),
        }
    }

    pub fn get_entities(&self) -> MutexGuard<Entities> {
        self.entities.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn get_entities_arc(&self) -> Arc<Mutex<Entities>> {
        self.entities.clone()
    }

    pub fn sync_entities(&mut self, networking: &mut ServerNetworking) -> Result<()> {
        for (entity, (position, physics)) in &mut self.get_entities().ecs.query::<(&PositionComponent, &PhysicsComponent)>() {
            let id = self.get_entities().get_id_from_entity(entity)?;
            networking.send_packet(
                &Packet::new(EntityPositionVelocityPacket {
                    id,
                    x: position.x(),
                    y: position.y(),
                    velocity_x: physics.velocity_x,
                    velocity_y: physics.velocity_y,
                })?,
                SendTarget::All,
            )?;
        }
        Ok(())
    }

    pub fn on_event(event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<EntityDespawnEvent>() {
            let packet = EntityDespawnPacket { id: event.id };
            networking.send_packet(&Packet::new(packet)?, SendTarget::All)?;
        }
        Ok(())
    }
}
