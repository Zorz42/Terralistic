use anyhow::Result;
use hecs::Entity;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use crate::libraries::events::Event;
use crate::server::server_core::networking::{SendTarget, ServerNetworking};
use crate::shared::entities::{Entities, EntityDespawnEvent, EntityDespawnPacket, EntityState, EntitySyncPacket, PhysicsComponent, PositionComponent};
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

    pub fn get_entities(&self) -> MutexGuard<'_, Entities> {
        self.entities.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn get_entities_arc(&self) -> Arc<Mutex<Entities>> {
        self.entities.clone()
    }

    /// Sends every entity's state as of `tick`, as one packet.
    pub fn sync_entities(&self, tick: u64, networking: &mut ServerNetworking) -> Result<()> {
        let mut entity_list = Vec::new();
        for (entity, position, physics) in &mut self.get_entities().ecs.query::<(Entity, &PositionComponent, &PhysicsComponent)>() {
            entity_list.push((entity, *position, *physics));
        }

        let mut entities = Vec::with_capacity(entity_list.len());
        for (entity, position, physics) in entity_list {
            entities.push(EntityState {
                id: self.get_entities().get_id_from_entity(entity)?,
                x: position.x(),
                y: position.y(),
                velocity_x: physics.velocity_x,
                velocity_y: physics.velocity_y,
            });
        }

        if entities.is_empty() {
            return Ok(());
        }

        networking.send_packet(&Packet::new(EntitySyncPacket { tick, entities })?, SendTarget::All)
    }

    pub fn on_event(event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<EntityDespawnEvent>() {
            let packet = EntityDespawnPacket { id: event.id };
            networking.send_packet(&Packet::new(packet)?, SendTarget::All)?;
        }
        Ok(())
    }
}
