use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use hecs::Entity;
use serde_derive::{Deserialize, Serialize};

use crate::libraries::events::{Event, EventManager};
use crate::shared::blocks::Blocks;
use crate::shared::liquids::{Liquids, MAX_LIQUID_LEVEL};

pub const DEFAULT_GRAVITY: f32 = 80.0;
pub const FRICTION_COEFFICIENT: f32 = 0.2;
pub const AIR_RESISTANCE_COEFFICIENT: f32 = 0.005;
/// How much of its velocity an entity loses per tick to a liquid that stops it completely
/// (`speed_multiplier` of 0). A liquid's own multiplier scales this down.
pub const LIQUID_RESISTANCE_COEFFICIENT: f32 = 0.05;
/// How much of gravity a liquid holds an entity up against when it is fully submerged.
/// Below 1.0, so an entity in water still sinks - slowly.
pub const BUOYANCY_COEFFICIENT: f32 = 0.75;
const DIRECTION_SIZE: f32 = 0.01;

/// How deeply an entity sits in liquid, from 0.0 to 1.0, and how much that liquid slows it.
///
/// Measured from the cell the entity's middle is in, so wading through a puddle barely
/// counts and being under the surface counts fully. Partly filled cells scale everything by
/// how full they are, which is what stops a splash of liquid a hundredth of a cell deep from
/// braking a falling player as hard as an ocean would.
#[must_use]
pub fn liquid_submersion(position: &PositionComponent, physics: &PhysicsComponent, liquids: &Liquids) -> (f32, f32) {
    let x = (position.x + physics.collision_width / 2.0) as i32;
    let y = (position.y + physics.collision_height / 2.0) as i32;

    let Ok(liquid) = liquids.get_liquid(x, y) else {
        return (0.0, 1.0);
    };

    if liquid.level == 0 {
        return (0.0, 1.0);
    }

    let speed_multiplier = liquids.get_liquid_type(liquid.id).map_or(1.0, |liquid_type| liquid_type.speed_multiplier);

    (f32::from(liquid.level) / f32::from(MAX_LIQUID_LEVEL), speed_multiplier)
}

#[must_use]
pub fn collides_with_blocks(position: &PositionComponent, physics: &PhysicsComponent, blocks: &Blocks) -> bool {
    let block_x = position.x as i32;
    let block_y = position.y as i32;

    let block_x2 = (position.x + physics.collision_width - 2.0 * DIRECTION_SIZE).ceil() as i32;
    let block_y2 = (position.y + physics.collision_height - 2.0 * DIRECTION_SIZE).ceil() as i32;

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
            y: position.y + DIRECTION_SIZE * 2.0,
        },
        physics,
        blocks,
    ) && physics.velocity_y.abs() <= 0.01
}

pub struct Entities {
    pub ecs: hecs::World,
    current_id: u32,
    id_to_entity: HashMap<EntityId, Entity>,
    entity_to_id: HashMap<Entity, EntityId>,
}

/// Reduce a by b, but never go below 0. if a is negative, increase it by b but never go above 0.
pub fn reduce_by(a: &mut f32, b: f32) {
    let b = b.abs();
    if *a > 0.0 {
        *a -= b;
        if *a < 0.0 {
            *a = 0.0;
        }
    } else {
        *a += b;
        if *a > 0.0 {
            *a = 0.0;
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
            let velocity_x_before = physics.velocity_x;
            let velocity_y_before = physics.velocity_y;

            let (submersion, speed_multiplier) = liquid_submersion(position, physics, liquids);

            physics.velocity_x += physics.acceleration_x / 200.0;
            physics.velocity_y += physics.acceleration_y / 200.0;

            // buoyancy cancels most of the gravity the entity was just given, rather than
            // being a force of its own, so an entity in a liquid sinks slowly instead of
            // fighting a second constant that has to be kept in step with `DEFAULT_GRAVITY`
            physics.velocity_y -= submersion * BUOYANCY_COEFFICIENT * physics.acceleration_y / 200.0;

            let target_x = position.x + physics.velocity_x / 200.0;
            let target_y = position.y + physics.velocity_y / 200.0;

            let direction_x = if physics.velocity_x > 0.0 { 1.0 } else { -1.0 } * DIRECTION_SIZE;
            loop {
                if (direction_x > 0.0 && position.x > target_x + direction_x) || (direction_x < 0.0 && position.x < target_x + direction_x) {
                    position.x = target_x;
                    break;
                }

                position.x += direction_x;

                if collides_with_blocks(position, physics, blocks) {
                    position.x -= direction_x;
                    reduce_by(&mut physics.velocity_y, physics.velocity_x * FRICTION_COEFFICIENT);
                    physics.velocity_x = 0.0;
                    break;
                }
            }

            let direction_y = if physics.velocity_y > 0.0 { 1.0 } else { -1.0 } * DIRECTION_SIZE;
            loop {
                if (direction_y > 0.0 && position.y > target_y + direction_y) || (direction_y < 0.0 && position.y < target_y + direction_y) {
                    position.y = target_y;
                    break;
                }

                position.y += direction_y;

                if collides_with_blocks(position, physics, blocks) {
                    position.y -= direction_y;
                    reduce_by(&mut physics.velocity_x, physics.velocity_y * FRICTION_COEFFICIENT);
                    physics.velocity_y = 0.0;
                    break;
                }
            }

            let resistance = AIR_RESISTANCE_COEFFICIENT + submersion * LIQUID_RESISTANCE_COEFFICIENT * (1.0 - speed_multiplier).clamp(0.0, 1.0);
            physics.velocity_x *= 1.0 - resistance;
            physics.velocity_y *= 1.0 - resistance;

            let velocity_x_change = physics.velocity_x - velocity_x_before;
            let velocity_y_change = physics.velocity_y - velocity_y_before;
            let velocity_change = f32::hypot(velocity_x_change, velocity_y_change);

            vec.push((entity, velocity_change));
        }

        for (entity, velocity_change) in vec {
            let id = self.get_id_from_entity(entity)?;
            if let Ok(health_component) = self.ecs.query_one_mut::<&mut HealthComponent>(entity) {
                if velocity_change > 40.0 {
                    health_component.increase_health(-((velocity_change - 40.0) * 4.0) as i32, events, id);
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

#[derive(Serialize, Deserialize)]
pub struct EntityPositionVelocityPacket {
    pub id: EntityId,
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub force: bool,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct PositionComponent {
    x: f32,
    y: f32,
}

impl PositionComponent {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn x(&self) -> f32 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f32 {
        self.y
    }

    pub const fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub const fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

#[derive(Clone)]
pub struct PhysicsComponent {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    collision_width: f32,
    collision_height: f32,
}

impl PhysicsComponent {
    #[must_use]
    pub const fn new(collision_width: f32, collision_height: f32) -> Self {
        Self {
            velocity_x: 0.0,
            velocity_y: 0.0,
            acceleration_x: 0.0,
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

impl rlua::FromLua<'_> for EntityId {
    fn from_lua(value: rlua::Value, _lua: &rlua::Lua) -> rlua::Result<Self> {
        match value {
            rlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}

/// make `EntityId` Lua compatible
impl rlua::UserData for EntityId {}
