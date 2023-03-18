use crate::libraries::events::Event;
use crate::shared::entities::{
    Entities, EntityPositionVelocityPacket, IdComponent, PhysicsComponent, PositionComponent,
};
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
            if let Some(packet) = packet.try_deserialize::<EntityPositionVelocityPacket>() {
                for (_entity, (id, position, physics)) in self.entities.ecs.query_mut::<(
                    &mut IdComponent,
                    &mut PositionComponent,
                    &mut PhysicsComponent,
                )>() {
                    if id.id() == packet.id {
                        position.set_x(packet.x);
                        position.set_y(packet.y);
                        physics.velocity_x = packet.velocity_x;
                        physics.velocity_y = packet.velocity_y;
                    }
                }
            }
        }
    }
}
