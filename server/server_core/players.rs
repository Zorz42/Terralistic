use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::{
    Connection, NewConnectionWelcomedEvent, PacketFromClientEvent, SendTarget, ServerNetworking,
};
use crate::shared::blocks::Blocks;
use crate::shared::entities::{Entities, IdComponent, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;
use crate::shared::players::{
    remove_all_picked_items, spawn_player, update_players_ms, PlayerComponent, PlayerMovingPacket,
    PlayerSpawnPacket,
};
use anyhow::{anyhow, Result};
use hecs::Entity;
use std::collections::HashMap;

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

    pub fn on_event(
        &mut self,
        event: &Event,
        entities: &mut Entities,
        blocks: &Blocks,
        networking: &mut ServerNetworking,
    ) -> Result<()> {
        if let Some(packet_event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = packet_event.packet.try_deserialize::<PlayerMovingPacket>() {
                let player_entity =
                    *self
                        .conns_to_players
                        .get(&packet_event.conn)
                        .ok_or_else(|| {
                            anyhow!("Received PlayerMovingPacket from unknown connection")
                        })?;
                let mut player_component =
                    entities.ecs.get::<&mut PlayerComponent>(player_entity)?;
                let mut physics_component =
                    entities.ecs.get::<&mut PhysicsComponent>(player_entity)?;
                player_component.set_moving_type(packet.moving_type, &mut physics_component);
                player_component.jumping = packet.jumping;
                
                let id = entities.ecs.get::<&IdComponent>(player_entity)?.id();
                let packet = Packet::new(PlayerMovingPacket {
                    moving_type: packet.moving_type,
                    jumping: packet.jumping,
                    player_id: id,
                })?;
                networking.send_packet(&packet, SendTarget::AllExcept(packet_event.conn.clone()))?;
            }
        }

        if let Some(new_connection_event) = event.downcast::<NewConnectionWelcomedEvent>() {
            let spawn_x = blocks.get_width() as f32 / 2.0;
            let spawn_y = 0.0;
            
            for (_entity, (player, position, id)) in entities.ecs.query::<(&PlayerComponent, &PositionComponent, &IdComponent)>().iter() {
                let spawn_packet = Packet::new(PlayerSpawnPacket {
                    id: id.id(),
                    x: position.x(),
                    y: position.y(),
                    name: player.get_name().to_owned(),
                })?;
                networking.send_packet(&spawn_packet, SendTarget::Connection(new_connection_event.conn.clone()))?;
            }

            let name = networking.get_connection_name(&new_connection_event.conn);
            let player_entity = spawn_player(entities, spawn_x, spawn_y, &name, None);
            self.conns_to_players
                .insert(new_connection_event.conn.clone(), player_entity);
            self.players_to_conns
                .insert(player_entity, new_connection_event.conn.clone());

            let player_spawn_packet = Packet::new(PlayerSpawnPacket {
                id: entities.ecs.get::<&IdComponent>(player_entity)?.id(),
                x: spawn_x,
                y: spawn_y,
                name: name.clone(),
            })?;

            networking.send_packet(&player_spawn_packet, SendTarget::All)?;
        }

        Ok(())
    }

    pub fn update(
        entities: &mut Entities,
        blocks: &Blocks,
        events: &mut EventManager,
    ) -> Result<()> {
        update_players_ms(entities, blocks);
        remove_all_picked_items(entities, events)
    }
}
