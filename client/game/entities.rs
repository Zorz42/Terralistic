use crate::libraries::events::Event;
use crate::shared::entities::{Entities, EntityPositionPacket, IdComponent, PositionComponent};
use crate::shared::packet::Packet;

pub struct ClientEntities {
    pub entities: Entities,
}

impl ClientEntities {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<EntityPositionPacket>() {
                for (_entity, (id, position)) in self
                    .entities
                    .ecs
                    .query_mut::<(&mut IdComponent, &mut PositionComponent)>()
                {
                    if id.id == packet.id {
                        position.x = packet.x;
                        position.y = packet.y;
                    }
                }
            }
        }
    }
}
