use crate::client::game::players::ClientPlayers;
use crate::libraries::events::{Event, EventManager};
use crate::libraries::fixed::Fixed;
use crate::shared::entities::{Entities, EntityDespawnPacket, EntityState, EntitySyncPacket, PhysicsComponent, PositionComponent};
use crate::shared::packet::Packet;
use anyhow::Result;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// How much of the gap to the server's position to close on each snapshot. The rest is left
/// for the next one, so a small disagreement converges over a couple of tenths of a second
/// instead of arriving as a jump.
const SNAPSHOT_BLEND: Fixed = Fixed::from_num(1, 2);
/// A gap larger than this is taken whole rather than eased across.
const SNAPSHOT_SNAP_DISTANCE: Fixed = Fixed::from_int(4);

pub struct ClientEntities {
    pub entities: Arc<Mutex<Entities>>,
}

impl ClientEntities {
    pub fn new() -> Self {
        Self {
            entities: Arc::new(Mutex::new(Entities::new())),
        }
    }

    pub fn get_entities(&self) -> MutexGuard<'_, Entities> {
        self.entities.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn get_entities_arc(&self) -> Arc<Mutex<Entities>> {
        self.entities.clone()
    }

    /// Brings one entity into line with the server.
    ///
    /// Velocity is taken exactly - it is the server's to decide - but position is walked in
    /// rather than assigned. Both sides run the same deterministic physics from the same
    /// spawn state, so a dead-reckoned item usually agrees already and this does nothing;
    /// where it does not, easing turns a visible jump into a drift that the next snapshot
    /// finishes. A difference too large to walk in is taken whole, since easing across the
    /// world would look worse than arriving.
    fn apply_state(&self, state: &EntityState) -> Result<()> {
        let mut entities = self.get_entities();
        let entity = entities.get_entity_from_id(state.id)?;
        let (position, physics) = entities.ecs.query_one_mut::<(&mut PositionComponent, &mut PhysicsComponent)>(entity)?;

        physics.velocity_x = state.velocity_x;
        physics.velocity_y = state.velocity_y;

        let (dx, dy) = (state.x - position.x(), state.y - position.y());
        if dx.abs() > SNAPSHOT_SNAP_DISTANCE || dy.abs() > SNAPSHOT_SNAP_DISTANCE {
            position.set_x(state.x);
            position.set_y(state.y);
        } else {
            position.set_x(position.x() + dx * SNAPSHOT_BLEND);
            position.set_y(position.y() + dy * SNAPSHOT_BLEND);
        }

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
                    self.apply_state(&state)?;
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
