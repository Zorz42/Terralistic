use crate::client::game::networking::ClientNetworking;
use crate::client::game::players::ClientPlayers;
use crate::libraries::events::{Event, EventManager};
use crate::shared::entities::{Entities, EntityDespawnPacket, EntityPositionVelocityPacket, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;
use crate::shared::players::PlayerPositionPacketToServer;
use anyhow::Result;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

pub struct ClientEntities {
    pub entities: Arc<Mutex<Entities>>,
}

impl ClientEntities {
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

    pub fn on_event(&self, event: &Event, events: &mut EventManager, players: &ClientPlayers, networking: &mut ClientNetworking) -> Result<()> {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<EntityPositionVelocityPacket>() {
                let entity = self.get_entities().get_entity_from_id(packet.id)?;

                if !packet.force && Some(entity) == players.get_main_player() {
                    let position_component = *self.get_entities().ecs.query_one_mut::<&mut PositionComponent>(entity)?;
                    let packet = Packet::new(PlayerPositionPacketToServer {
                        x: position_component.x(),
                        y: position_component.y(),
                    })?;
                    networking.send_packet(packet)?;
                    return Ok(());
                }

                {
                    let mut entities = self.get_entities();
                    let position_component = entities.ecs.query_one_mut::<&mut PositionComponent>(entity)?;
                    position_component.set_x(packet.x);
                    position_component.set_y(packet.y);
                }

                {
                    let mut entities = self.get_entities();
                    let physics_component = entities.ecs.query_one_mut::<&mut PhysicsComponent>(entity)?;
                    physics_component.velocity_x = packet.velocity_x;
                    physics_component.velocity_y = packet.velocity_y;
                }
            }
            if let Some(packet) = packet.try_deserialize::<EntityDespawnPacket>() {
                let entity_to_despawn = self.get_entities().get_entity_from_id(packet.id);
                if let Ok(entity) = entity_to_despawn {
                    let entity_id = self.get_entities().get_id_from_entity(entity)?;
                    self.get_entities().despawn_entity(entity_id, events)?;
                }
            }
        }
        Ok(())
    }
}
