use crate::libraries::events::{Event, EventManager};
use crate::server::server_core::networking::{
    Connection, DisconnectEvent, NewConnectionWelcomedEvent, PacketFromClientEvent, SendTarget,
    ServerNetworking,
};
use crate::shared::blocks::Blocks;
use crate::shared::entities::{Entities, IdComponent, PhysicsComponent, PositionComponent};
use crate::shared::inventory::{
    Inventory, InventoryPacket, InventorySelectPacket, InventorySwapPacket,
};
use crate::shared::items::Items;
use crate::shared::packet::Packet;
use crate::shared::players::{
    remove_all_picked_items, spawn_player, update_players_ms, PlayerComponent, PlayerMovingPacket,
    PlayerSpawnPacket, PLAYER_HEIGHT, PLAYER_WIDTH,
};
use anyhow::{anyhow, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SavedPlayerData {
    pub inventory: Inventory,
    pub position: PositionComponent,
}

pub struct ServerPlayers {
    conns_to_players: HashMap<Connection, Entity>,
    players_to_conns: HashMap<Entity, Connection>,
    saved_players: HashMap<String, SavedPlayerData>,
}

impl ServerPlayers {
    pub fn new() -> Self {
        Self {
            conns_to_players: HashMap::new(),
            players_to_conns: HashMap::new(),
            saved_players: HashMap::new(),
        }
    }

    fn get_spawn_coords(blocks: &Blocks) -> (f32, f32) {
        let spawn_x = blocks.get_width() as f32 / 2.0;
        let mut spawn_y = 0.0;
        // find a spawn point
        // iterate from the top of the map to the bottom
        for y in (0..blocks.get_height()).rev() {
            for x in 0..(PLAYER_WIDTH.ceil() as i32) {
                let block_type = blocks.get_block_type(
                    blocks
                        .get_block(spawn_x as i32 + x, y as i32)
                        .unwrap_or(blocks.air),
                );

                let is_ghost = match block_type.ok() {
                    Some(block_type) => block_type.ghost,
                    None => false,
                };

                if !is_ghost {
                    spawn_y = y as f32 - PLAYER_HEIGHT;
                    break;
                }
            }
        }

        (spawn_x, spawn_y)
    }

    pub fn on_event(
        &mut self,
        event: &Event,
        entities: &mut Entities,
        blocks: &Blocks,
        networking: &mut ServerNetworking,
        events: &mut EventManager,
    ) -> Result<()> {
        if let Some(packet_event) = event.downcast::<PacketFromClientEvent>() {
            if let Some(packet) = packet_event.packet.try_deserialize::<PlayerMovingPacket>() {
                let player_entity = self.get_player_from_connection(&packet_event.conn)?;
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
                networking
                    .send_packet(&packet, SendTarget::AllExcept(packet_event.conn.clone()))?;
            } else if let Some(packet) = packet_event
                .packet
                .try_deserialize::<InventorySelectPacket>()
            {
                let player_entity = self.get_player_from_connection(&packet_event.conn)?;
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;
                inventory.selected_slot = packet.slot;
            } else if let Some(packet) =
                packet_event.packet.try_deserialize::<InventorySwapPacket>()
            {
                let player_entity = self.get_player_from_connection(&packet_event.conn)?;
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;

                inventory.swap_with_selected_item(packet.slot)?;
            }
        }

        if let Some(new_connection_event) = event.downcast::<NewConnectionWelcomedEvent>() {
            let name = networking.get_connection_name(&new_connection_event.conn);
            let player_data = self.saved_players.get(&name);

            let (spawn_x, spawn_y) = player_data.map_or_else(
                || Self::get_spawn_coords(blocks),
                |player_data| (player_data.position.x(), player_data.position.y()),
            );

            for (_entity, (player, position, id)) in entities
                .ecs
                .query::<(&PlayerComponent, &PositionComponent, &IdComponent)>()
                .iter()
            {
                let spawn_packet = Packet::new(PlayerSpawnPacket {
                    id: id.id(),
                    x: position.x(),
                    y: position.y(),
                    name: player.get_name().to_owned(),
                })?;
                networking.send_packet(
                    &spawn_packet,
                    SendTarget::Connection(new_connection_event.conn.clone()),
                )?;
            }

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

            if let Some(player_data) = player_data {
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;
                *inventory = player_data.inventory.clone();
                inventory.has_changed = true;
            }

            networking.send_packet(&player_spawn_packet, SendTarget::All)?;
        }

        if let Some(disconnect_event) = event.downcast::<DisconnectEvent>() {
            if let Ok(player_entity) = self.get_player_from_connection(&disconnect_event.conn) {
                self.saved_players.insert(
                    networking.get_connection_name(&disconnect_event.conn),
                    SavedPlayerData {
                        position: (*entities.ecs.get::<&PositionComponent>(player_entity)?).clone(),
                        inventory: (*entities.ecs.get::<&Inventory>(player_entity)?).clone(),
                    },
                );

                self.conns_to_players.remove(&disconnect_event.conn);
                self.players_to_conns.remove(&player_entity);
                let player_id = entities.ecs.get::<&IdComponent>(player_entity)?.id();
                entities.despawn_entity(player_id, events)?;
            }
        }

        Ok(())
    }

    pub fn update(
        &mut self,
        entities: &mut Entities,
        blocks: &Blocks,
        events: &mut EventManager,
        items: &mut Items,
        networking: &mut ServerNetworking,
    ) -> Result<()> {
        update_players_ms(entities, blocks);
        remove_all_picked_items(entities, events, items)?;

        for (conn, player) in &self.conns_to_players {
            let mut inventory = entities.ecs.get::<&mut Inventory>(*player)?;
            if inventory.has_changed {
                inventory.has_changed = false;
                networking.send_packet(
                    &Packet::new(InventoryPacket {
                        inventory: inventory.clone(),
                    })?,
                    SendTarget::Connection(conn.clone()),
                )?;
            }
        }
        Ok(())
    }

    pub fn get_player_from_connection(&self, conn: &Connection) -> Result<Entity> {
        self.conns_to_players
            .get(conn)
            .ok_or_else(|| anyhow!("Received PlayerMovingPacket from unknown connection"))
            .cloned()
    }

    pub fn get_player_from_name(&mut self, name: &str) -> Result<&mut SavedPlayerData> {
        self.saved_players
            .get_mut(name)
            .ok_or_else(|| anyhow!("Player with name {} not found", name))
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(&self.saved_players)?)
    }

    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        self.saved_players = bincode::deserialize(data)?;
        Ok(())
    }
}
