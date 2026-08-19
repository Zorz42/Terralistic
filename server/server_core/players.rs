use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;

use anyhow::{anyhow, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::libraries::fixed::Fixed;
use crate::libraries::serialization;
use crate::server::server_core::blocks::ServerBlocks;
use crate::server::server_core::networking::{Connection, DisconnectEvent, NewConnectionWelcomedEvent, PacketFromClientEvent, SendTarget, ServerNetworking};
use crate::server::server_core::print_to_console;
use crate::shared::blocks::Blocks;
use crate::shared::entities::{Entities, HealthChangeEvent, PhysicsComponent, PositionComponent};
use crate::shared::entities::{HealthChangePacket, HealthComponent};
use crate::shared::inventory::{Inventory, InventoryCraftPacket, InventoryPacket, InventorySelectPacket, InventorySwapPacket, Slot};
use crate::shared::items::Items;
use crate::shared::liquids::Liquids;
use crate::shared::packet::Packet;
use crate::shared::players::{
    attract_items_to_players, remove_all_picked_items, spawn_player, update_players_ms, PlayerComponent, PlayerInput, PlayerInputPacket, PlayerInputPacketToClient, PlayerSpawnPacket, RespawnPacket,
    PLAYER_HEIGHT, PLAYER_INVENTORY_SIZE, PLAYER_MAX_HEALTH, PLAYER_WIDTH,
};

#[derive(Serialize, Deserialize)]
pub struct SavedPlayerData {
    pub inventory: Inventory,
    pub position: PositionComponent,
    pub health: HealthComponent,
}

/// What a client has told the server it is doing, and when it said it was doing it.
///
/// Inputs arrive stamped with a tick the server has usually not reached yet - the client runs
/// `INPUT_LEAD_TICKS` ahead precisely so they do - and are held here until it does. An input
/// is a *state*, not an edit, so the last one applied stays in force with no packets at all
/// while a key is held.
#[derive(Default)]
pub(super) struct InputQueue {
    pending: BTreeMap<u64, PlayerInput>,
    current: PlayerInput,
    /// Inputs that arrived after the server had already simulated their tick. The server
    /// cannot un-simulate, so it applies them late and the client is corrected instead;
    /// a count that climbs means `INPUT_LEAD_TICKS` is too small for this connection.
    pub(super) late: u64,
}

impl InputQueue {
    /// Whatever the client said to do at or before `tick`, with the newest winning.
    /// Returns the input in force, changed or not.
    pub(super) fn advance_to(&mut self, tick: u64) -> PlayerInput {
        let future = self.pending.split_off(&(tick + 1));
        if let Some((_, input)) = std::mem::replace(&mut self.pending, future).into_iter().next_back() {
            self.current = input;
        }
        self.current
    }

    pub(super) fn accept(&mut self, tick: u64, input: PlayerInput, now: u64) {
        if tick <= now {
            self.late += 1;
        }
        self.pending.insert(tick, input);
    }
}

pub struct ServerPlayers {
    conns_to_players: HashMap<Connection, Option<Entity>>,
    players_to_conns: HashMap<Entity, Connection>,
    saved_players: HashMap<String, SavedPlayerData>,
    inputs: HashMap<Entity, InputQueue>,
    /// The tick the simulation is on, so an arriving input can be told it is already late.
    current_tick: u64,
}

impl ServerPlayers {
    pub fn new() -> Self {
        Self {
            conns_to_players: HashMap::new(),
            players_to_conns: HashMap::new(),
            saved_players: HashMap::new(),
            inputs: HashMap::new(),
            current_tick: 0,
        }
    }

    fn get_spawn_coords(blocks: &Blocks) -> (Fixed, Fixed) {
        let spawn_column = blocks.get_size().0 as i32 / 2;
        let mut spawn_y = Fixed::ZERO;
        // find a spawn point
        // iterate from the top of the map to the bottom
        for y in (0..blocks.get_size().1 as i32).rev() {
            for x in 0..PLAYER_WIDTH.ceil_to_int() {
                let block_type = blocks.get_block_type(blocks.get_block(spawn_column + x, y).unwrap_or_else(|_| blocks.air()));

                let is_ghost = block_type.ok().is_some_and(|block_type| block_type.ghost);

                if !is_ghost {
                    spawn_y = Fixed::from_int(y) - PLAYER_HEIGHT;
                    break;
                }
            }
        }

        (Fixed::from_int(spawn_column), spawn_y)
    }

    pub fn handle_client_packet(
        &mut self,
        packet_event: &PacketFromClientEvent,
        entities: &mut Entities,
        networking: &mut ServerNetworking,
        blocks: &ServerBlocks,
        events: &mut EventManager,
        items: &Items,
    ) -> Result<()> {
        let player_entity = self.get_player_from_connection(&packet_event.conn)?;
        if let Some(player_entity) = player_entity {
            if let Some(packet) = packet_event.packet.try_deserialize::<PlayerInputPacket>() {
                // queued rather than applied: it is stamped for a tick the server has not
                // reached, and applying it now would run it early by the whole lead
                self.inputs.entry(player_entity).or_default().accept(packet.tick, packet.input, self.current_tick);

                let id = entities.get_id_from_entity(player_entity)?;
                let relayed = Packet::new(PlayerInputPacketToClient {
                    tick: packet.tick,
                    player_id: id,
                    input: packet.input,
                })?;
                networking.send_packet(&relayed, SendTarget::AllExcept(packet_event.conn.clone()))?;
            } else if let Some(packet) = packet_event.packet.try_deserialize::<InventorySelectPacket>() {
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;
                inventory.selected_slot = packet.slot;
            } else if let Some(packet) = packet_event.packet.try_deserialize::<InventorySwapPacket>() {
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;

                if let Slot::Inventory(slot) = packet.slot {
                    inventory.swap_with_selected_item(slot)?;
                }

                if let Slot::Block(x, y, slot) = packet.slot {
                    let mut inventory_data = blocks.get_blocks().get_block_inventory_data(x, y)?;

                    let block_item = inventory_data.get(slot).ok_or_else(|| anyhow!("Block at ({x}, {y}) doesn't have slot {slot}"))?.clone();
                    let selected_item = inventory.get_selected_item();

                    *inventory_data.get_mut(slot).ok_or_else(|| anyhow!("Block at ({x}, {y}) doesn't have slot {slot}"))? = selected_item;

                    let selected_slot = inventory.selected_slot.ok_or_else(|| anyhow!("Player doesn't have selected slot"))?;

                    inventory.set_item(selected_slot, block_item)?;

                    blocks.get_blocks().set_block_inventory_data(x, y, inventory_data, events)?;
                    blocks.update_block(x, y, events)?;
                }
            } else if let Some(packet) = packet_event.packet.try_deserialize::<InventoryCraftPacket>() {
                let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?.clone();
                let position_x = entities.ecs.get::<&PositionComponent>(player_entity)?.x();
                let position_y = entities.ecs.get::<&PositionComponent>(player_entity)?.y();
                let recipe = items.get_recipe(packet.recipe)?.clone();

                inventory.craft(&recipe, (position_x, position_y), items, entities, events)?;

                *entities.ecs.get::<&mut Inventory>(player_entity)? = inventory;
            }
        }

        if packet_event.packet.try_deserialize::<RespawnPacket>().is_some() {
            self.spawn_player(&networking.get_connection_name(&packet_event.conn), &blocks.get_blocks(), entities, networking, &packet_event.conn)?;
        }

        Ok(())
    }

    fn spawn_player(&mut self, name: &String, blocks: &Blocks, entities: &mut Entities, networking: &mut ServerNetworking, connection: &Connection) -> Result<()> {
        let player_data = self.saved_players.get(name);

        let (spawn_x, spawn_y) = player_data.map_or_else(|| Self::get_spawn_coords(blocks), |player_data| (player_data.position.x(), player_data.position.y()));

        for (entity, player, position) in &mut entities.ecs.query::<(Entity, &PlayerComponent, &PositionComponent)>() {
            let id = entities.get_id_from_entity(entity)?;
            let spawn_packet = Packet::new(PlayerSpawnPacket {
                id,
                x: position.x(),
                y: position.y(),
                name: player.get_name().to_owned(),
            })?;
            networking.send_packet(&spawn_packet, SendTarget::Connection(connection.clone()))?;
        }

        let health_component = player_data.map_or_else(|| HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH), |player_data| player_data.health.clone());

        let player_id = entities.new_id();
        let player_entity = spawn_player(entities, spawn_x, spawn_y, name, player_id, health_component)?;
        self.conns_to_players.insert(connection.clone(), Some(player_entity));
        self.players_to_conns.insert(player_entity, connection.clone());

        let player_spawn_packet = Packet::new(PlayerSpawnPacket {
            id: entities.get_id_from_entity(player_entity)?,
            x: spawn_x,
            y: spawn_y,
            name: name.clone(),
        })?;

        let health_component = entities.ecs.get::<&mut HealthComponent>(player_entity)?;
        let health_packet = Packet::new(HealthChangePacket {
            health: health_component.health(),
            max_health: health_component.max_health(),
        })?;
        networking.send_packet(&health_packet, SendTarget::Connection(connection.clone()))?;

        if let Some(player_data) = player_data {
            let mut inventory = entities.ecs.get::<&mut Inventory>(player_entity)?;
            *inventory = player_data.inventory.clone();
            inventory.has_changed = true;
        }

        networking.send_packet(&player_spawn_packet, SendTarget::All)?;

        Ok(())
    }

    fn save_player(&mut self, name: &str, entities: &Entities) -> Result<()> {
        let player_entity = self.get_player_entity_from_name(name, entities)?;
        let position = entities.ecs.get::<&PositionComponent>(*player_entity)?.clone();
        let inventory = entities.ecs.get::<&Inventory>(*player_entity)?.clone();
        let health = entities.ecs.get::<&HealthComponent>(*player_entity)?.clone();
        self.saved_players.insert(
            name.to_owned(),
            SavedPlayerData {
                position: *position,
                inventory: inventory.deref().clone(),
                health: health.deref().clone(),
            },
        );
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn on_event(&mut self, event: &Event, entities: &mut Entities, blocks: &ServerBlocks, networking: &mut ServerNetworking, events: &mut EventManager, items: &Items) -> Result<()> {
        if let Some(packet_event) = event.downcast::<PacketFromClientEvent>() {
            self.handle_client_packet(packet_event, entities, networking, blocks, events, items)?;
        }

        if let Some(new_connection_event) = event.downcast::<NewConnectionWelcomedEvent>() {
            let name = networking.get_connection_name(&new_connection_event.conn);
            self.spawn_player(&name, &blocks.get_blocks(), entities, networking, &new_connection_event.conn)?;
        }

        if let Some(disconnect_event) = event.downcast::<DisconnectEvent>() {
            if let Ok(player_entity) = self.get_player_from_connection(&disconnect_event.conn) {
                let name = networking.get_connection_name(&disconnect_event.conn);
                print_to_console(format!("[\"{name}\"] left the game").as_str(), 0);

                if let Some(player_entity) = player_entity {
                    self.save_player(&name, entities)?;

                    self.players_to_conns.remove(&player_entity);
                    self.inputs.remove(&player_entity);
                    let player_id = entities.get_id_from_entity(player_entity)?;
                    entities.despawn_entity(player_id, events)?;
                }

                self.conns_to_players.remove(&disconnect_event.conn);
            }
        }

        if let Some(health_change_event) = event.downcast::<HealthChangeEvent>() {
            let entity = entities.get_entity_from_id(health_change_event.entity)?;
            let player_conn = self.players_to_conns.get(&entity).cloned();
            if let Some(player_conn) = player_conn {
                let health_component = entities.ecs.query_one_mut::<&HealthComponent>(entity)?;
                networking.send_packet(
                    &Packet::new(HealthChangePacket {
                        health: health_component.health(),
                        max_health: health_component.max_health(),
                    })?,
                    SendTarget::Connection(player_conn.clone()),
                )?;

                if health_component.health() == 0 {
                    let inventory = entities.ecs.get::<&Inventory>(entity)?.deref().clone();
                    for item in inventory.iter().flatten() {
                        for _ in 0..item.count {
                            let x = entities.ecs.get::<&PositionComponent>(entity)?.x();
                            let y = entities.ecs.get::<&PositionComponent>(entity)?.y();

                            items.drop_item(events, entities, item.item, x, y)?;
                        }
                    }

                    let name = networking.get_connection_name(&player_conn);
                    self.save_player(&name, entities)?;

                    let spawn_coord = Self::get_spawn_coords(&blocks.get_blocks());
                    let saved_player = self.saved_players.get_mut(&name).ok_or_else(|| anyhow!("Player not found"))?;
                    saved_player.position = PositionComponent::new(spawn_coord.0, spawn_coord.1);
                    saved_player.inventory = Inventory::new(PLAYER_INVENTORY_SIZE);
                    saved_player.health = HealthComponent::new(PLAYER_MAX_HEALTH, PLAYER_MAX_HEALTH);

                    entities.despawn_entity(health_change_event.entity, events)?;
                    self.conns_to_players.insert(player_conn, None);
                    self.players_to_conns.remove(&entity);
                    self.inputs.remove(&entity);
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self, tick: u64, entities: &mut Entities, blocks: &Blocks, liquids: &Liquids, events: &mut EventManager, items: &Items, networking: &mut ServerNetworking) -> Result<()> {
        self.current_tick = tick;

        // Each player's controls for *this* tick, taken from what its client said it would
        // be doing. A player whose client has gone quiet keeps the last input it sent, which
        // is what makes a held key cost no packets.
        for (entity, queue) in &mut self.inputs {
            let input = queue.advance_to(tick);
            if let Ok((player, physics)) = entities.ecs.query_one_mut::<(&mut PlayerComponent, &mut PhysicsComponent)>(*entity) {
                if player.get_input() != input {
                    let mut owned = *physics;
                    player.apply_input(input, &mut owned);
                    *physics = owned;
                }
            }
        }

        update_players_ms(entities, blocks, liquids);
        // server side only: it reads every other entity, so no client could agree with it
        attract_items_to_players(entities);
        remove_all_picked_items(entities, events, items)?;

        for (conn, player) in &self.conns_to_players {
            if let Some(player) = player {
                let mut inventory = entities.ecs.get::<&mut Inventory>(*player)?;
                if inventory.has_changed {
                    inventory.has_changed = false;
                    networking.send_packet(&Packet::new(InventoryPacket { inventory: inventory.clone() })?, SendTarget::Connection(conn.clone()))?;
                }
            }
        }
        Ok(())
    }

    pub fn get_player_from_connection(&self, conn: &Connection) -> Result<Option<Entity>> {
        self.conns_to_players.get(conn).ok_or_else(|| anyhow!("Received PlayerMovingPacket from unknown connection")).cloned()
    }

    pub fn get_player_entity_from_name(&self, name: &str, entities: &Entities) -> Result<&Entity> {
        for entity in &mut self.conns_to_players.values().flatten() {
            let e_component = entities.ecs.get::<&mut PlayerComponent>(*entity)?;
            if e_component.get_name() == name {
                return Ok(entity);
            }
        }
        Err(anyhow!("Player not found"))
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        serialization::serialize(&self.saved_players)
    }

    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        self.saved_players = serialization::deserialize(data)?;
        Ok(())
    }
}
