//! Clocks, budgets and limiters.
//!
//! Everything here answers one of three questions: *is it time yet* (`FixedStep`,
//! `Interval`), *how long should I sleep* (`FrameLimiter`), or *have I spent my budget*
//! (`Budget`) - plus `FrameStats` and `DeltaTimer`, which measure rather than decide.
//!
//! These were five separate implementations before, no two alike, and two of them had
//! already been fixed for the same class of bug months apart: a counter that overflowed
//! after 24.8 days and a ledger that stalled after four hours. Both were width bugs in an
//! accumulator, and there is now one accumulator.
//!
//! # Not in scope
//!
//! Doing the sleeping. `FrameLimiter` says how long to sleep and the caller sleeps, because
//! whoever owns the loop usually has something better to do with that time - servicing a
//! window, draining a queue - than block in here.
//!
//! Wall-clock dates and formatting. These all measure elapsed time from a start they chose
//! themselves, which is what makes them independent of the system clock being changed under
//! them.

pub use budget::*;
pub use fixed_step::*;
pub use frame_limiter::*;
pub use frame_stats::*;
pub use interval::*;

mod budget;
mod fixed_step;
mod frame_limiter;
mod frame_stats;
mod interval;
mod tests;
