use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::libraries::fixed::Fixed;
use crate::shared::blocks::Blocks;
use crate::shared::liquids::{Liquids, MAX_LIQUID_LEVEL};
use crate::shared::TICKS_PER_SECOND;

/// Downward acceleration, in blocks per second per second.
pub const DEFAULT_GRAVITY: Fixed = Fixed::from_int(80);
/// How much of the velocity along one axis is carried into the other when a collision stops it.
pub const FRICTION_COEFFICIENT: Fixed = Fixed::from_num(1, 5);
/// Fraction of its velocity an entity loses per tick to the air. Terminal speed is whatever
/// acceleration per tick divided by this comes to, so it sets how fast anything can ever go.
pub const AIR_RESISTANCE_COEFFICIENT: Fixed = Fixed::from_num(1, 200);
/// How much of its velocity an entity loses per tick to a liquid that stops it completely
/// (`speed_multiplier` of 0). A liquid's own multiplier scales this down.
pub const LIQUID_RESISTANCE_COEFFICIENT: Fixed = Fixed::from_num(1, 20);
/// How much of gravity a liquid holds an entity up against when it is fully submerged.
/// Below 1.0, so an entity in a liquid still sinks - slowly.
pub const BUOYANCY_COEFFICIENT: Fixed = Fixed::from_num(3, 4);
/// The step the collision march advances by. Small enough to not tunnel through a block at
/// terminal speed, large enough that the march is tens of iterations rather than thousands.
const DIRECTION_SIZE: Fixed = Fixed::from_num(1, 100);
/// Velocity change in one tick above which the landing hurts, and how much per block over.
const FALL_DAMAGE_THRESHOLD: Fixed = Fixed::from_int(40);
const FALL_DAMAGE_PER_BLOCK: i32 = 4;

#[must_use]
pub fn liquid_submersion(position: &PositionComponent, physics: &PhysicsComponent, liquids: &Liquids) -> (Fixed, Fixed) {
    let x = (position.x + physics.collision_width / 2).floor_to_int();
    let y = (position.y + physics.collision_height / 2).floor_to_int();

    let Ok(liquid) = liquids.get_liquid(x, y) else {
        return (Fixed::ZERO, Fixed::ONE);
    };

    if liquid.level == 0 {
        return (Fixed::ZERO, Fixed::ONE);
    }

    let speed_multiplier = liquids.get_liquid_type(liquid.id).map_or(Fixed::ONE, |liquid_type| liquid_type.speed_multiplier);

    (Fixed::from_num(i32::from(liquid.level), i32::from(MAX_LIQUID_LEVEL)), speed_multiplier)
}

#[must_use]
pub fn collides_with_blocks(position: &PositionComponent, physics: &PhysicsComponent, blocks: &Blocks) -> bool {
    let block_x = position.x.floor_to_int();
    let block_y = position.y.floor_to_int();

    let block_x2 = (position.x + physics.collision_width - DIRECTION_SIZE * 2).ceil_to_int();
    let block_y2 = (position.y + physics.collision_height - DIRECTION_SIZE * 2).ceil_to_int();

    for x in block_x..block_x2 {
        for y in block_y..block_y2 {
            let block = blocks.get_block_type_at(x, y);
            if let Ok(block) = block {
                if !block.ghost {
                    return true;
                }
            }
        }
    }

    false
}

#[must_use]
pub fn is_touching_ground(position: &PositionComponent, physics: &PhysicsComponent, blocks: &Blocks) -> bool {
    collides_with_blocks(
        &PositionComponent {
            x: position.x,
            y: position.y + DIRECTION_SIZE * 2,
        },
        physics,
        blocks,
    ) && physics.velocity_y.abs() <= DIRECTION_SIZE
}

/// Advances one entity by one tick: gravity, buoyancy, a collision march along each axis, then
/// drag. Returns how much its velocity changed, which is what a landing is judged by.
///
/// **Pure in everything it touches.** The result depends only on the two components, the block
/// grid and the liquid grid - no clock, no randomness, and nothing about any *other* entity.
/// That is what lets the client store a tick's input, replay it later against a corrected
/// state and arrive at the same answer the server did.
pub fn step_entity(position: &mut PositionComponent, physics: &mut PhysicsComponent, blocks: &Blocks, liquids: &Liquids) -> Fixed {
    let velocity_x_before = physics.velocity_x;
    let velocity_y_before = physics.velocity_y;

    let (submersion, speed_multiplier) = liquid_submersion(position, physics, liquids);

    physics.velocity_x += physics.acceleration_x / TICKS_PER_SECOND;
    physics.velocity_y += physics.acceleration_y / TICKS_PER_SECOND;

    // buoyancy cancels most of the gravity the entity was just given, rather than
    // being a force of its own, so an entity in a liquid sinks slowly instead of
    // fighting a second constant that has to be kept in step with `DEFAULT_GRAVITY`
    physics.velocity_y -= submersion * BUOYANCY_COEFFICIENT * physics.acceleration_y / TICKS_PER_SECOND;

    let target_x = position.x + physics.velocity_x / TICKS_PER_SECOND;
    let target_y = position.y + physics.velocity_y / TICKS_PER_SECOND;

    let direction_x = if physics.velocity_x > Fixed::ZERO { DIRECTION_SIZE } else { -DIRECTION_SIZE };
    loop {
        if (direction_x > Fixed::ZERO && position.x > target_x + direction_x) || (direction_x < Fixed::ZERO && position.x < target_x + direction_x) {
            position.x = target_x;
            break;
        }

        position.x += direction_x;

        if collides_with_blocks(position, physics, blocks) {
            position.x -= direction_x;
            reduce_by(&mut physics.velocity_y, physics.velocity_x * FRICTION_COEFFICIENT);
            physics.velocity_x = Fixed::ZERO;
            break;
        }
    }

    let direction_y = if physics.velocity_y > Fixed::ZERO { DIRECTION_SIZE } else { -DIRECTION_SIZE };
    loop {
        if (direction_y > Fixed::ZERO && position.y > target_y + direction_y) || (direction_y < Fixed::ZERO && position.y < target_y + direction_y) {
            position.y = target_y;
            break;
        }

        position.y += direction_y;

        if collides_with_blocks(position, physics, blocks) {
            position.y -= direction_y;
            reduce_by(&mut physics.velocity_x, physics.velocity_y * FRICTION_COEFFICIENT);
            physics.velocity_y = Fixed::ZERO;
            break;
        }
    }

    let resistance = AIR_RESISTANCE_COEFFICIENT + submersion * LIQUID_RESISTANCE_COEFFICIENT * (Fixed::ONE - speed_multiplier).clamp(Fixed::ZERO, Fixed::ONE);
    physics.velocity_x *= Fixed::ONE - resistance;
    physics.velocity_y *= Fixed::ONE - resistance;

    let velocity_x_change = physics.velocity_x - velocity_x_before;
    let velocity_y_change = physics.velocity_y - velocity_y_before;

    // hypot would do this, but it is libm rather than an IEEE-specified operation and so is
    // one of the few float calls that genuinely differs between platforms. This is exact.
    (velocity_x_change * velocity_x_change + velocity_y_change * velocity_y_change).sqrt()
}

/// A checksum of one entity's simulation state.
///
/// FNV-1a over the raw fixed-point words, written out rather than reached for from the
/// standard library: this number crosses the network and has to mean the same thing at both
/// ends, and `DefaultHasher` promises nothing about staying the same between builds.
///
/// It is only meaningful because the state is integers. Two float simulations that agree to
/// within a rounding error hash differently, so the check would report a desync every tick
/// and tell nobody anything.
#[must_use]
pub fn state_hash(position: &PositionComponent, physics: &PhysicsComponent) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = FNV_OFFSET;
    for word in [
        position.x().raw(),
        position.y().raw(),
        physics.velocity_x.raw(),
        physics.velocity_y.raw(),
        physics.acceleration_x.raw(),
        physics.acceleration_y.raw(),
    ] {
        for byte in word.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }
    hash
}

pub struct Entities {
    pub ecs: hecs::World,
    current_id: u32,
    id_to_entity: HashMap<EntityId, Entity>,
    entity_to_id: HashMap<Entity, EntityId>,
}

/// Reduce a by b, but never go below 0. if a is negative, increase it by b but never go above 0.
pub fn reduce_by(a: &mut Fixed, b: Fixed) {
    let b = b.abs();
    if *a > Fixed::ZERO {
        *a -= b;
        if *a < Fixed::ZERO {
            *a = Fixed::ZERO;
        }
    } else {
        *a += b;
        if *a > Fixed::ZERO {
            *a = Fixed::ZERO;
        }
    }
}

impl Entities {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ecs: hecs::World::new(),
            current_id: 0,
            id_to_entity: HashMap::new(),
            entity_to_id: HashMap::new(),
        }
    }

    pub fn update_entities_ms(&mut self, blocks: &Blocks, liquids: &Liquids, events: &mut EventManager) -> Result<()> {
        let mut vec = Vec::new();

        for (entity, position, physics) in self.ecs.query_mut::<(Entity, &mut PositionComponent, &mut PhysicsComponent)>() {
            vec.push((entity, step_entity(position, physics, blocks, liquids)));
        }

        for (entity, velocity_change) in vec {
            let id = self.get_id_from_entity(entity)?;
            if let Ok(health_component) = self.ecs.query_one_mut::<&mut HealthComponent>(entity) {
                if velocity_change > FALL_DAMAGE_THRESHOLD {
                    health_component.increase_health(-(velocity_change - FALL_DAMAGE_THRESHOLD).to_int() * FALL_DAMAGE_PER_BLOCK, events, id);
                }
            }
        }

        Ok(())
    }

    pub fn assign_id(&mut self, entity: Entity, id: EntityId) -> Result<()> {
        if self.id_to_entity.contains_key(&id) {
            bail!("id already assigned");
        }
        if self.entity_to_id.contains_key(&entity) {
            bail!("entity already has assigned id");
        }

        self.id_to_entity.insert(id, entity);
        self.entity_to_id.insert(entity, id);

        Ok(())
    }

    pub fn get_entity_from_id(&self, id: EntityId) -> Result<Entity> {
        self.id_to_entity.get(&id).ok_or_else(|| anyhow!("invalid id")).copied()
    }

    pub fn get_id_from_entity(&self, entity: Entity) -> Result<EntityId> {
        self.entity_to_id.get(&entity).ok_or_else(|| anyhow!("invalid entity")).copied()
    }

    pub const fn new_id(&mut self) -> EntityId {
        self.current_id += 1;
        EntityId::new(self.current_id)
    }

    pub fn despawn_entity(&mut self, id: EntityId, events: &mut EventManager) -> Result<()> {
        let entity_to_despawn = self.get_entity_from_id(id);

        if let Ok(entity) = entity_to_despawn {
            self.ecs.despawn(entity)?;
            // the id maps are not part of the ecs, so despawning does not touch them.
            // Leaving the entries behind meant every id ever handed out stayed resolvable
            // to an entity that no longer exists - a lookup that succeeds and then fails
            // at the query, and two maps that only ever grow on a server that spawns and
            // drops items all day.
            self.id_to_entity.remove(&id);
            self.entity_to_id.remove(&entity);
        } else {
            bail!("Could not find entity with id");
        }

        events.push_event(Event::new(EntityDespawnEvent { id }));

        Ok(())
    }
}

/// One entity's authoritative state, as of the tick its packet names.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct EntityState {
    pub id: EntityId,
    pub x: Fixed,
    pub y: Fixed,
    pub velocity_x: Fixed,
    pub velocity_y: Fixed,
}

/// Every entity's state as of one tick, in one packet.
///
/// One packet per entity to every client was a burst of hundreds of small packets on the
/// same tick for a world with items scattered about it - the same shape of problem
/// `LiquidChangesPacket` already avoids by batching a tick's worth of cells.
///
/// The tick matters as much as the states do. A client that knows *when* a state was true can
/// look up what it thought at that same moment and correct itself against it; a client handed
/// a state of unknown age can only snap to it, which is what made a correction throw away
/// everything the player had done since.
#[derive(Serialize, Deserialize)]
pub struct EntitySyncPacket {
    pub tick: u64,
    pub entities: Vec<EntityState>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDespawnPacket {
    pub id: EntityId,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct EntityId {
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EntityDespawnEvent {
    pub id: EntityId,
}

impl EntityId {
    #[must_use]
    const fn new(id: u32) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct PositionComponent {
    x: Fixed,
    y: Fixed,
}

impl PositionComponent {
    #[must_use]
    pub const fn new(x: Fixed, y: Fixed) -> Self {
        Self { x, y }
    }

    /// For spawn points and other whole-block coordinates.
    #[must_use]
    pub const fn from_blocks(x: i32, y: i32) -> Self {
        Self {
            x: Fixed::from_int(x),
            y: Fixed::from_int(y),
        }
    }

    #[must_use]
    pub const fn x(&self) -> Fixed {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> Fixed {
        self.y
    }

    pub const fn set_x(&mut self, x: Fixed) {
        self.x = x;
    }

    pub const fn set_y(&mut self, y: Fixed) {
        self.y = y;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PhysicsComponent {
    pub velocity_x: Fixed,
    pub velocity_y: Fixed,
    pub acceleration_x: Fixed,
    pub acceleration_y: Fixed,
    collision_width: Fixed,
    collision_height: Fixed,
}

impl PhysicsComponent {
    #[must_use]
    pub const fn new(collision_width: Fixed, collision_height: Fixed) -> Self {
        Self {
            velocity_x: Fixed::ZERO,
            velocity_y: Fixed::ZERO,
            acceleration_x: Fixed::ZERO,
            acceleration_y: DEFAULT_GRAVITY,
            collision_width,
            collision_height,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct HealthChangePacket {
    pub health: i32,
    pub max_health: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthComponent {
    health: i32,
    max_health: i32,
}

pub struct HealthChangeEvent {
    pub entity: EntityId,
}

impl HealthComponent {
    #[must_use]
    pub const fn new(health: i32, max_health: i32) -> Self {
        Self { health, max_health }
    }

    #[must_use]
    pub const fn health(&self) -> i32 {
        self.health
    }

    #[must_use]
    pub const fn max_health(&self) -> i32 {
        self.max_health
    }

    pub fn set_health(&mut self, health: i32, events: &mut EventManager, entity_id: EntityId) {
        let health = health.clamp(0, self.max_health);
        if self.health != health {
            self.health = health;
            events.push_event(Event::new(HealthChangeEvent { entity: entity_id }));
        }
    }

    pub fn increase_health(&mut self, health: i32, events: &mut EventManager, entity_id: EntityId) {
        self.set_health(self.health + health, events, entity_id);
    }
}

crate::script_handle!(EntityId);
