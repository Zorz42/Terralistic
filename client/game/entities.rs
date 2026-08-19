use crate::client::game::players::ClientPlayers;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::fixed::Fixed;
use crate::shared::entities::{Entities, EntityDespawnPacket, EntityId, EntityState, EntitySyncPacket, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;
use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// How much of the disagreement with the server to close on each snapshot. The rest is left
/// for the next one, so a difference converges over a couple of tenths of a second instead of
/// arriving as a jump.
const SNAPSHOT_BLEND: Fixed = Fixed::from_num(1, 2);
/// A disagreement larger than this is taken whole rather than eased across.
const SNAPSHOT_SNAP_DISTANCE: Fixed = Fixed::from_int(4);
/// How many ticks of every entity to keep. A second, which covers the sync interval several
/// times over plus any flight time a playable connection has.
const HISTORY_TICKS: usize = 200;

pub struct ClientEntities {
    pub entities: Arc<Mutex<Entities>>,
    /// What this client had for every entity on each of the last `HISTORY_TICKS` ticks.
    history: HashMap<EntityId, VecDeque<(u64, PositionComponent, PhysicsComponent)>>,
}

impl ClientEntities {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(Entities::new())),
            history: HashMap::new(),
        }
    }

    /// How many entities have a history. Used to pin that it is dropped with the entity.
    #[cfg(test)]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    pub fn get_entities(&self) -> MutexGuard<'_, Entities> {
        self.entities.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn get_entities_arc(&self) -> Arc<Mutex<Entities>> {
        self.entities.clone()
    }

    /// Remembers where this tick left every entity, so a snapshot naming an earlier tick can be
    /// compared against what this client actually had then.
    ///
    /// Called after the physics, so a recorded tick is a finished one - the same moment the
    /// server reports.
    pub fn record_tick(&mut self, tick: u64) {
        let states: Vec<_> = {
            let mut entities = self.get_entities();
            let live: Vec<_> = entities
                .ecs
                .query_mut::<(hecs::Entity, &PositionComponent, &PhysicsComponent)>()
                .into_iter()
                .map(|(entity, position, physics)| (entity, *position, *physics))
                .collect();
            live.into_iter()
                .filter_map(|(entity, position, physics)| Some((entities.get_id_from_entity(entity).ok()?, position, physics)))
                .collect()
        };

        for (id, position, physics) in states {
            let frames = self.history.entry(id).or_default();
            if frames.len() >= HISTORY_TICKS {
                frames.pop_front();
            }
            frames.push_back((tick, position, physics));
        }

        // an entity that was not recorded this tick no longer exists, so its history goes with
        // it - otherwise every item ever picked up stays in the map for the session
        self.history.retain(|_, frames| frames.back().is_some_and(|frame| frame.0 == tick));
    }

    /// Brings one entity into line with the server.
    ///
    /// **The comparison is against what this client had at `tick`, not against where it is
    /// now.** Both sides run the same deterministic physics, so a dead-reckoned entity usually
    /// agreed at that tick and this does nothing. Measuring against the current position
    /// instead would mostly measure how far the entity moved while the packet was in flight,
    /// and drag it back by that much on every snapshot - which is an item being pulled toward
    /// a player faster than the sync rate looking like it trails the player and never arrives.
    ///
    /// The difference is applied to the current state rather than replacing it, so the ticks
    /// this client has simulated since are kept.
    pub(super) fn apply_state(&self, tick: u64, state: &EntityState) -> Result<()> {
        let mut entities = self.get_entities();
        let entity = entities.get_entity_from_id(state.id)?;
        let (position, physics) = entities.ecs.query_one_mut::<(&mut PositionComponent, &mut PhysicsComponent)>(entity)?;

        // No record of that tick means the entity is newer than the snapshot or the snapshot
        // older than the history. There is nothing to take a difference against, so the
        // server's answer is taken as given.
        let Some(&(_, was_position, was_physics)) = self.history.get(&state.id).and_then(|frames| frames.iter().find(|frame| frame.0 == tick)) else {
            position.set_x(state.x);
            position.set_y(state.y);
            physics.velocity_x = state.velocity_x;
            physics.velocity_y = state.velocity_y;
            return Ok(());
        };

        let error = (state.x - was_position.x(), state.y - was_position.y());
        let blend = if error.0.abs() > SNAPSHOT_SNAP_DISTANCE || error.1.abs() > SNAPSHOT_SNAP_DISTANCE {
            Fixed::ONE
        } else {
            SNAPSHOT_BLEND
        };

        position.set_x(position.x() + error.0 * blend);
        position.set_y(position.y() + error.1 * blend);
        physics.velocity_x += (state.velocity_x - was_physics.velocity_x) * blend;
        physics.velocity_y += (state.velocity_y - was_physics.velocity_y) * blend;

        Ok(())
    }

    pub fn on_event(&self, event: &Event, events: &mut EventManager, players: &ClientPlayers) -> Result<()> {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<EntitySyncPacket>() {
                for state in packet.entities {
                    // This client's own player is corrected by `ClientPlayers`, which can
                    // replay the inputs since the tick this state is for. Everything else
                    // has no inputs to replay and is simply brought into line.
                    let entity = self.get_entities().get_entity_from_id(state.id)?;
                    if Some(entity) == players.get_main_player() {
                        continue;
                    }
                    self.apply_state(packet.tick, &state)?;
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
