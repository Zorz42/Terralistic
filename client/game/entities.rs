use anyhow::Result;

use crate::libraries::events::{Event, EventManager};
use crate::shared::entities::{Entities, EntityDespawnPacket, EntityPositionVelocityPacket, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;

pub struct ClientEntities {
    pub entities: Entities,
}

impl ClientEntities {
    pub fn new() -> Self {
        Self { entities: Entities::new() }
    }

    pub fn on_event(&mut self, event: &Event, events: &mut EventManager) -> Result<()> {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<EntityPositionVelocityPacket>() {
                let entity = self.entities.get_entity_from_id(packet.id)?;
                {
                    let position_component = self.entities.ecs.query_one_mut::<&mut PositionComponent>(entity)?;
                    position_component.set_x(packet.x);
                    position_component.set_y(packet.y);
                }

                {
                    let physics_component = self.entities.ecs.query_one_mut::<&mut PhysicsComponent>(entity)?;
                    physics_component.velocity_x = packet.velocity_x;
                    physics_component.velocity_y = packet.velocity_y;
                }
            }
            if let Some(packet) = packet.try_deserialize::<EntityDespawnPacket>() {
                let entity_to_despawn = self.entities.get_entity_from_id(packet.id);
                if let Ok(entity) = entity_to_despawn {
                    let entity_id = self.entities.get_id_from_entity(entity)?;
                    self.entities.despawn_entity(entity_id, events)?;
                }
            }
        }
        Ok(())
    }
}
