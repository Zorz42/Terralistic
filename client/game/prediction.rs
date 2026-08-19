use std::collections::VecDeque;

use crate::libraries::fixed::Fixed;
use crate::shared::blocks::Blocks;
use crate::shared::entities::{step_entity, PhysicsComponent, PositionComponent};
use crate::shared::liquids::Liquids;
use crate::shared::players::{step_player, PlayerComponent, PlayerInput};

/// How many ticks of the local player's history to keep, at 5ms each - two seconds, which is
/// far more round trip than a playable connection has. Each frame is a few dozen bytes, so
/// the whole buffer is single-digit kilobytes.
const HISTORY_TICKS: usize = 400;

/// How much of a correction's visible error is worked off per tick. The simulation snaps to
/// the server's answer immediately; only the *drawn* position eases across, so a correction
/// that does happen looks like the player sliding a little rather than teleporting.
const ERROR_DECAY: Fixed = Fixed::from_num(9, 10);
/// Below this the remaining error is dropped rather than approached forever.
const ERROR_EPSILON: Fixed = Fixed::from_num(1, 1000);
/// An error larger than this is not smoothed at all. A respawn or a teleport is meant to look
/// like one, and sliding a player across the world would be worse than the jump.
const MAX_SMOOTHED_ERROR: Fixed = Fixed::from_int(8);

#[cfg(test)]
impl Prediction {
    pub fn history_len(&self) -> usize {
        self.frames.len()
    }

    pub fn state_at(&self, tick: u64) -> Option<(PositionComponent, PhysicsComponent)> {
        self.frames.iter().find(|frame| frame.tick == tick).map(|frame| (frame.position, frame.physics))
    }
}

/// The server reports a position and a velocity. The rest of the physics - the acceleration
/// the current input implies, the collision box - is this client's own and never goes on the
/// wire, so it is taken from the tick being corrected rather than from the present. Comparing
/// the present acceleration against a past tick's fired a correction every time the player
/// changed direction between the two, on a state the server had not disagreed with at all.
const fn with_velocity(mut physics: PhysicsComponent, velocity: (Fixed, Fixed)) -> PhysicsComponent {
    physics.velocity_x = velocity.0;
    physics.velocity_y = velocity.1;
    physics
}

/// One tick of the local player: what it was told to do, and where that put it.
#[derive(Clone, Copy)]
struct Frame {
    tick: u64,
    input: PlayerInput,
    position: PositionComponent,
    physics: PhysicsComponent,
}

/// The local player's recent history, and the machinery to correct it against the server.
///
/// This client simulates its own player the instant a key goes down, because waiting for a
/// round trip is what input lag *is*. The server simulates the same player from the same
/// inputs through the same deterministic step, so the two normally agree exactly and there is
/// nothing to do - `reconcile` compares and returns.
///
/// When they do disagree - a block broke underfoot, another player collided, the server
/// teleported someone - the fix is not to snap to a state that is already a round trip old.
/// It is to rewind to the tick the server is talking about, put its answer there, and replay
/// everything this client has done since. The player ends up somewhere that accounts for both.
pub struct Prediction {
    frames: VecDeque<Frame>,
    /// What the last correction moved the drawn player by, decayed towards zero each tick.
    error: (Fixed, Fixed),
    /// How many corrections have been applied. Zero is the healthy number; a count that
    /// climbs while nothing unusual is happening means the two sides are diverging.
    pub corrections: u64,
}

impl Prediction {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::with_capacity(HISTORY_TICKS),
            error: (Fixed::ZERO, Fixed::ZERO),
            corrections: 0,
        }
    }

    /// Remembers where a tick ended up, so a later correction has something to rewind to.
    pub fn record(&mut self, tick: u64, input: PlayerInput, position: PositionComponent, physics: PhysicsComponent) {
        if self.frames.len() >= HISTORY_TICKS {
            self.frames.pop_front();
        }
        self.frames.push_back(Frame { tick, input, position, physics });
    }

    /// Compares the server's answer for `tick` against what this client had, and if they
    /// differ, rewinds and replays. Returns the corrected state, or `None` when there was
    /// nothing to correct.
    ///
    /// Three cases, and the difference between them matters:
    ///
    /// - **A tick in the buffer**: compare, and replay if it differs. The normal path.
    /// - **A tick newer than anything in the buffer**: this client has fallen behind the
    ///   server, so there is nothing to replay and its answer is taken whole. Should not
    ///   happen - the client runs `INPUT_LEAD_TICKS` ahead - but a long stall would do it,
    ///   and without this the player would stay wrong for good.
    /// - **A tick older than the buffer**: ignored. There is nothing to compare and nothing
    ///   to replay, and snapping to it would undo every input since, which is the
    ///   rubber-band this exists to remove.
    pub fn reconcile(
        &mut self,
        tick: u64,
        server_position: PositionComponent,
        server_velocity: (Fixed, Fixed),
        player: &mut PlayerComponent,
        blocks: &Blocks,
        liquids: &Liquids,
    ) -> Option<(PositionComponent, PhysicsComponent)> {
        let Some(index) = self.frames.iter().position(|frame| frame.tick == tick) else {
            let newest = *self.frames.back()?;
            if tick <= newest.tick {
                return None;
            }
            // fallen behind the server: nothing to replay, so start again from its answer
            self.corrections += 1;
            self.forget();
            return Some((server_position, with_velocity(newest.physics, server_velocity)));
        };
        let frame = *self.frames.get(index)?;
        let server_physics = with_velocity(frame.physics, server_velocity);

        if frame.position == server_position && frame.physics == server_physics {
            return None;
        }

        self.corrections += 1;
        let before = frame.position;

        // rewind: the server's answer replaces what this client thought, and the player's
        // controls go back to what they were on that tick so the replayed transitions match
        let mut position = server_position;
        let mut physics = server_physics;
        player.restore_input(frame.input);

        if let Some(corrected) = self.frames.get_mut(index) {
            corrected.position = position;
            corrected.physics = physics;
        }

        // replay: everything this client has done since, against the corrected state
        let replayed: Vec<Frame> = self.frames.iter().skip(index + 1).copied().collect();
        for mut frame in replayed {
            player.apply_input(frame.input, &mut physics);
            step_player(&position, &mut physics, player, blocks, liquids);
            step_entity(&mut position, &mut physics, blocks, liquids);

            frame.position = position;
            frame.physics = physics;
            if let Some(stored) = self.frames.iter_mut().find(|stored| stored.tick == frame.tick) {
                *stored = frame;
            }
        }

        self.note_visual_error(before, position);
        Some((position, physics))
    }

    /// Remembers how far the correction moved the player on screen, so the jump can be drawn
    /// as a slide. A correction big enough to be a teleport is shown as one.
    fn note_visual_error(&mut self, before: PositionComponent, after: PositionComponent) {
        let error = (self.error.0 + before.x() - after.x(), self.error.1 + before.y() - after.y());
        self.error = if error.0.abs() > MAX_SMOOTHED_ERROR || error.1.abs() > MAX_SMOOTHED_ERROR {
            (Fixed::ZERO, Fixed::ZERO)
        } else {
            error
        };
    }

    /// Drops the history. Used when the server places the player rather than reporting it -
    /// a respawn or a teleport - where replaying what the client did before would undo the
    /// decision.
    pub fn forget(&mut self) {
        self.frames.clear();
        self.error = (Fixed::ZERO, Fixed::ZERO);
    }

    /// Works the correction off a little each tick.
    pub fn decay_visual_error(&mut self) {
        self.error = (self.error.0 * ERROR_DECAY, self.error.1 * ERROR_DECAY);
        if self.error.0.abs() < ERROR_EPSILON {
            self.error.0 = Fixed::ZERO;
        }
        if self.error.1.abs() < ERROR_EPSILON {
            self.error.1 = Fixed::ZERO;
        }
    }

    /// What to add to the drawn position, so the picture lags the correction the simulation
    /// already took.
    pub const fn visual_offset(&self) -> (Fixed, Fixed) {
        self.error
    }
}
