use std::collections::HashMap;
use hecs::Entity;
use crate::libraries::events::Event;
use crate::server::server_core::networking::{Connection, NewConnectionEvent, PacketFromClientEvent, ServerNetworking};
use crate::shared::blocks::Blocks;
use crate::shared::entities::{Entities, IdComponent};
use crate::shared::packet::Packet;
use crate::shared::players::{PlayerSpawnPacket, spawn_player};
use anyhow::Result;

pub struct ServerPlayers {
    conns_to_players: HashMap<Connection, Entity>,
    players_to_conns: HashMap<Entity, Connection>,
}

impl ServerPlayers {
    pub fn new() -> Self {
        Self {
            conns_to_players: HashMap::new(),
            players_to_conns: HashMap::new(),
        }
    }
    
    pub fn on_event(&mut self, event: &Event, entities: &mut Entities, blocks: &Blocks, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(packet_event) = event.downcast::<PacketFromClientEvent>() {}

        if let Some(new_connection_event) = event.downcast::<NewConnectionEvent>() {
            let spawn_x = blocks.get_width() as f32 / 2.0;
            let spawn_y = 0.0;
            
            let player_entity = spawn_player(entities, spawn_x, spawn_y, None);
            self.conns_to_players.insert(new_connection_event.conn.clone(), player_entity);
            self.players_to_conns.insert(player_entity, new_connection_event.conn.clone());
            
            let player_spawn_packet = Packet::new(PlayerSpawnPacket {
                id: entities.ecs.get::<&IdComponent>(player_entity)?.id(),
                x: spawn_x,
                y: spawn_y,
            })?;
            
            networking.send_packet_to_all(&player_spawn_packet)?;
        }
        
        Ok(())
    }
}
