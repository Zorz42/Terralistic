use crate::libraries::events::Event;
use crate::server::server_core::networking::{SendTarget, ServerNetworking};
use crate::shared::entities::{
    Entities, EntityDespawnEvent, EntityDespawnPacket, EntityPositionVelocityPacket, IdComponent,
    PhysicsComponent, PositionComponent,
};
use crate::shared::packet::Packet;
use anyhow::Result;

pub struct ServerEntities {
    pub entities: Entities,
}

impl ServerEntities {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
        }
    }

    pub fn sync_entities(&mut self, networking: &mut ServerNetworking) -> Result<()> {
        for (_entity, (id, position, physics)) in
            self.entities
                .ecs
                .query_mut::<(&IdComponent, &PositionComponent, &PhysicsComponent)>()
        {
            networking.send_packet(
                &Packet::new(EntityPositionVelocityPacket {
                    id: *id,
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
