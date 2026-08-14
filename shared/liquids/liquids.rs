use std::collections::BTreeSet;

use anyhow::{anyhow, bail, Result};
use serde_derive::{Deserialize, Serialize};
use snap;

use crate::libraries::events::{Event, EventManager};
use crate::libraries::grid::Grid;
use crate::libraries::registry::{Registry, RegistryId};
use crate::libraries::serialization;
use crate::shared::blocks::Blocks;
use crate::shared::liquids::LiquidType;

/// A cell filled to the brim.
///
/// Levels are whole numbers, not the floats the first implementation used. A settled row of
/// liquid has to compare *exactly* equal to stop flowing, and floats only ever got close -
/// the old code papered over that by comparing `level as i32`, which made a cell holding
/// 9.7 and one holding 9.2 both "9" and their average drift downwards forever.
pub const MAX_LIQUID_LEVEL: u8 = 100;

/// `LiquidId` stores id to a type of liquid.
#[derive(Deserialize, Serialize, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct LiquidId {
    pub id: i8,
}

impl LiquidId {
    #[must_use]
    pub const fn undefined() -> Self {
        Self { id: -1 }
    }
}

impl RegistryId for LiquidId {
    const KIND: &'static str = "liquid type";

    fn from_index(index: usize) -> Self {
        Self { id: index as i8 }
    }

    fn index(self) -> Option<usize> {
        usize::try_from(self.id).ok()
    }
}

/// One cell of the liquid grid: which liquid, and how much of the cell it fills.
///
/// An empty cell is the empty liquid type at level 0, never a level of some real liquid -
/// `set_liquid` normalizes that, so rendering and physics can trust the pair.
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Liquid {
    pub id: LiquidId,
    pub level: u8,
}

#[derive(Deserialize, Serialize)]
pub(super) struct LiquidsData {
    pub(super) liquids: Grid<Liquid>,
}

impl LiquidsData {
    pub const fn new() -> Self {
        Self { liquids: Grid::new_empty() }
    }
}

/// A world's liquids: a grid of `Liquid` cells plus the simulation that makes them flow.
///
/// The simulation is driven off a set of scheduled cells rather than a scan of the world.
/// A full scan is 5.28 million cells for the default world, twenty times a second, to move
/// water that is usually nowhere near the player - so instead every change schedules itself
/// and its four neighbours, and a cell that has settled falls out of the set and costs
/// nothing until something disturbs it.
pub struct Liquids {
    pub(super) liquids_data: LiquidsData,
    pub(super) liquid_types: Registry<LiquidId, LiquidType>,

    /// The liquid an empty cell holds. Registered first, so it is always id 0.
    pub empty: LiquidId,

    /// Cells that might still have somewhere to flow. A `BTreeSet` rather than a `HashSet`
    /// because the order cells are processed in decides how a stream splits, and Rust
    /// randomises hash iteration per process - the same class of bug that made
    /// `GameModData.resources` a `BTreeMap`.
    scheduled: BTreeSet<(i32, i32)>,

    /// Milliseconds of simulated time, and the time each liquid type next flows at.
    elapsed_ms: f64,
    next_flow: Vec<f64>,
}

impl Liquids {
    #[must_use]
    pub fn new() -> Self {
        let mut result = Self {
            liquids_data: LiquidsData::new(),
            liquid_types: Registry::new(),

            empty: LiquidId::undefined(),

            scheduled: BTreeSet::new(),

            elapsed_ms: 0.0,
            next_flow: Vec::new(),
        };

        let mut empty = LiquidType::new();
        "empty".clone_into(&mut empty.name);
        empty.flow_time = 0;
        result.empty = result.register_new_liquid_type(empty);

        result
    }

    /// Creates an empty map with the given dimensions.
    ///
    /// Unlike `Walls::create` this fills the grid with a real registered type rather than
    /// an undefined one, so reading a cell of a freshly created world works.
    pub fn create(&mut self, size: (u32, u32)) {
        self.liquids_data.liquids = Grid::filled(size, Liquid { id: self.empty, level: 0 });
        self.scheduled.clear();
    }

    #[must_use]
    pub const fn get_size(&self) -> (u32, u32) {
        self.liquids_data.liquids.get_size()
    }

    /// Returns the whole cell at the given position.
    pub fn get_liquid(&self, x: i32, y: i32) -> Result<Liquid> {
        Ok(*self.liquids_data.liquids.get(x, y)?)
    }

    /// Returns the liquid type id at the given position.
    pub fn get_liquid_id_at(&self, x: i32, y: i32) -> Result<LiquidId> {
        Ok(self.get_liquid(x, y)?.id)
    }

    /// Returns how full the cell at the given position is, from 0 to `MAX_LIQUID_LEVEL`.
    pub fn get_liquid_level(&self, x: i32, y: i32) -> Result<u8> {
        Ok(self.get_liquid(x, y)?.level)
    }

    /// Returns the liquid type of the cell at the given position.
    pub fn get_liquid_type_at(&self, x: i32, y: i32) -> Result<&LiquidType> {
        self.get_liquid_type(self.get_liquid_id_at(x, y)?)
    }

    /// Returns the liquid type with the given id.
    pub fn get_liquid_type(&self, id: LiquidId) -> Result<&LiquidType> {
        self.liquid_types.get(id)
    }

    /// Sets what a cell holds, and schedules everything the change could make flow.
    ///
    /// A level of 0 always means the empty liquid, whatever id was asked for, and a level
    /// above the maximum is clamped rather than rejected: callers are handing over a
    /// physical amount, not an index.
    pub fn set_liquid(&mut self, x: i32, y: i32, liquid_id: LiquidId, level: u8, events: &mut EventManager) -> Result<()> {
        if self.get_liquid_type(liquid_id).is_err() {
            bail!("Liquid type not found");
        }

        let level = level.min(MAX_LIQUID_LEVEL);
        let new = if level == 0 { Liquid { id: self.empty, level: 0 } } else { Liquid { id: liquid_id, level } };

        let old = *self.liquids_data.liquids.get(x, y)?;

        if old == new {
            return Ok(());
        }

        self.liquids_data.liquids.set(x, y, new)?;

        self.schedule_update(x, y);
        events.push_event(Event::new(LiquidChangeEvent { x, y }));

        Ok(())
    }

    /// Marks a cell and its four neighbours as worth looking at on the next flow step.
    ///
    /// Public because a liquid does not only move when another liquid moves: breaking the
    /// block under a pool has to wake it up too, which is what `ServerLiquids` does with
    /// `BlockChangeEvent`.
    pub fn schedule_update(&mut self, x: i32, y: i32) {
        for (x, y) in [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if self.liquids_data.liquids.contains(x, y) {
                self.scheduled.insert((x, y));
            }
        }
    }

    /// Schedules every cell that still has somewhere to flow.
    ///
    /// A world that was saved mid-splash has to carry on flowing when it is loaded, but the
    /// scheduled set is not saved - it is derived state, and a settled ocean would be
    /// millions of entries of it. So this scans once on load and keeps only the cells that
    /// actually have room next to them, which for a settled world is none of them.
    ///
    /// It **replaces** the schedule rather than adding to it, which is what makes it right
    /// for a freshly generated world too: filling an ocean a cell at a time schedules every
    /// cell of it and all their neighbours, and almost none of that has anywhere to go.
    pub fn schedule_all_unsettled(&mut self, blocks: &Blocks) -> Result<()> {
        self.scheduled.clear();

        let (width, height) = self.get_size();
        for x in 0..width as i32 {
            for y in 0..height as i32 {
                let liquid = self.get_liquid(x, y)?;
                if liquid.level == 0 {
                    continue;
                }

                let has_room = [(x, y + 1), (x - 1, y), (x + 1, y)]
                    .into_iter()
                    .any(|(nx, ny)| self.can_flow_into(nx, ny, liquid, blocks).unwrap_or(false));

                if has_room || !blocks.get_block_type_at(x, y)?.ghost {
                    self.scheduled.insert((x, y));
                }
            }
        }
        Ok(())
    }

    /// Whether `liquid` can move into the cell at the given position: it has to be in the
    /// world, not inside a solid block, hold either nothing or the same liquid, and have
    /// room left.
    fn can_flow_into(&self, x: i32, y: i32, liquid: Liquid, blocks: &Blocks) -> Result<bool> {
        if !self.liquids_data.liquids.contains(x, y) {
            return Ok(false);
        }

        if !blocks.get_block_type_at(x, y)?.ghost {
            return Ok(false);
        }

        let target = self.get_liquid(x, y)?;
        Ok((target.level == 0 || target.id == liquid.id) && target.level < MAX_LIQUID_LEVEL)
    }

    /// Advances the simulation by `frame_length` milliseconds.
    ///
    /// Each liquid type flows on its own schedule, so a slow liquid next to a fast one moves
    /// at its own pace without either of them being stepped more often than they should.
    /// A type that falls behind - a server that stalled, a world that was paused - is not
    /// owed the steps it missed, for the same reason `AnimationTimer` gives up after
    /// `MAX_CATCHUP_FRAMES`: nobody was watching, and catching up would be a burst of
    /// hundreds of steps in one tick.
    pub fn update_liquids(&mut self, blocks: &Blocks, events: &mut EventManager, frame_length: f32) -> Result<()> {
        self.elapsed_ms += f64::from(frame_length);

        let mut due = vec![false; self.liquid_types.len()];
        for (id, liquid_type) in self.liquid_types.iter().enumerate() {
            if liquid_type.flow_time <= 0 {
                continue;
            }

            let next = self.next_flow.get_mut(id).ok_or_else(|| anyhow!("Liquid type has no flow schedule"))?;
            if self.elapsed_ms >= *next {
                *due.get_mut(id).ok_or_else(|| anyhow!("Liquid type has no flow schedule"))? = true;
                *next = self.elapsed_ms + f64::from(liquid_type.flow_time);
            }
        }

        if !due.contains(&true) {
            return Ok(());
        }

        // taken rather than iterated, because flowing schedules more cells and those belong
        // to the next step - otherwise a stream of water would run its whole length in one
        for (x, y) in std::mem::take(&mut self.scheduled) {
            let liquid = self.get_liquid(x, y)?;
            let Some(is_due) = usize::try_from(liquid.id.id).ok().and_then(|id| due.get(id).copied()) else {
                // a type this build does not have, out of a save written by one that did.
                // Dropping it from the schedule is what keeps it from being retried forever
                continue;
            };

            if !is_due {
                // not this liquid's turn, but it still has somewhere to go
                if liquid.level != 0 {
                    self.scheduled.insert((x, y));
                }
                continue;
            }

            self.flow_cell(x, y, blocks, events)?;
        }

        Ok(())
    }

    /// Moves the liquid in one cell down, and then sideways.
    fn flow_cell(&mut self, x: i32, y: i32, blocks: &Blocks, events: &mut EventManager) -> Result<()> {
        let liquid = self.get_liquid(x, y)?;
        if liquid.level == 0 {
            return Ok(());
        }

        // a block was placed on top of it
        if !blocks.get_block_type_at(x, y)?.ghost {
            return self.set_liquid(x, y, self.empty, 0, events);
        }

        let mut level = liquid.level;

        if self.can_flow_into(x, y + 1, liquid, blocks)? {
            let below = self.get_liquid(x, y + 1)?;
            let moved = level.min(MAX_LIQUID_LEVEL - below.level);
            self.set_liquid(x, y + 1, liquid.id, below.level + moved, events)?;
            level -= moved;
            self.set_liquid(x, y, liquid.id, level, events)?;
        }

        if level == 0 {
            return Ok(());
        }

        // Sideways is an averaging step over this cell and whichever neighbours hold less
        // than it does. A neighbour holding more is left alone - evening that pair out is
        // its own turn's job, and doing it from both sides is how liquid ends up sloshing
        // back and forth forever.
        let mut targets = Vec::new();
        let mut total = u32::from(level);
        for neighbour_x in [x - 1, x + 1] {
            if self.can_flow_into(neighbour_x, y, liquid, blocks)? {
                let neighbour = self.get_liquid(neighbour_x, y)?;
                if neighbour.level < level {
                    total += u32::from(neighbour.level);
                    targets.push(neighbour_x);
                }
            }
        }

        if targets.is_empty() {
            return Ok(());
        }

        let count = targets.len() as u32 + 1;
        let share = (total / count) as u8;
        // the remainder stays with the cell that had the most, so that a row which cannot be
        // divided evenly settles instead of passing the leftover drop back and forth
        let remainder = (total % count) as u8;

        for neighbour_x in targets {
            self.set_liquid(neighbour_x, y, liquid.id, share, events)?;
        }
        self.set_liquid(x, y, liquid.id, share + remainder, events)?;

        Ok(())
    }

    /// Serializes liquids for saving.
    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(snap::raw::Encoder::new().compress_vec(&serialization::serialize(&self.liquids_data)?)?)
    }

    /// Deserializes liquids from u8 vector.
    pub fn deserialize(&mut self, data: &[u8]) -> Result<()> {
        let decompressed = snap::raw::Decoder::new().decompress_vec(data)?;
        self.liquids_data = serialization::deserialize(&decompressed)?;
        self.scheduled.clear();

        Ok(())
    }

    /// This function adds a new liquid type, and is used by mods.
    pub fn register_new_liquid_type(&mut self, liquid_type: LiquidType) -> LiquidId {
        // its first step is one whole flow time away, not immediately: a type registered
        // with a flow time of a second should not get a free step the moment it appears
        self.next_flow.push(self.elapsed_ms + f64::from(liquid_type.flow_time));
        self.liquid_types.register(liquid_type)
    }

    /// Returns a liquid id with the given name.
    pub fn get_liquid_id_by_name(&self, name: &str) -> Result<LiquidId> {
        self.liquid_types.get_id_by_name(name)
    }

    /// Returns all liquid ids.
    #[must_use]
    pub fn get_all_liquid_ids(&self) -> Vec<LiquidId> {
        self.liquid_types.ids()
    }
}

pub struct LiquidChangeEvent {
    pub x: i32,
    pub y: i32,
}

/// A welcome packet that carries all the information about the world liquids
#[derive(Serialize, Deserialize)]
pub struct LiquidsWelcomePacket {
    pub data: Vec<u8>,
}

/// One cell's new contents, as sent to clients.
#[derive(Serialize, Deserialize)]
pub struct LiquidChange {
    pub x: i32,
    pub y: i32,
    pub liquid: LiquidId,
    pub level: u8,
}

/// Every cell that changed since the last update, in one packet.
///
/// Liquids are the one part of the world that changes on its own, ten times a second, over
/// as many cells as the player has flooded. One packet per cell the way blocks do it turns a
/// bucket of water into hundreds of packets a second, so the server batches a whole flow
/// step instead.
#[derive(Serialize, Deserialize)]
pub struct LiquidChangesPacket {
    pub changes: Vec<LiquidChange>,
}
