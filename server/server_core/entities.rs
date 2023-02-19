use crate::server::server_core::networking::ServerNetworking;
use crate::shared::entities::{Entities, EntityPositionPacket, IdComponent, PositionComponent};
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
        for (_entity, (id, position)) in self
            .entities
            .ecs
            .query_mut::<(&IdComponent, &PositionComponent)>()
        {
            networking.send_packet_to_all(&Packet::new(EntityPositionPacket {
                id: id.id,
                x: position.x,
                y: position.y,
            })?)?;
        }
        Ok(())
    }
}
