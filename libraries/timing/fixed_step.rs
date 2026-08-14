/// How much of a backlog an animating timer will walk before it gives up and skips to the
/// present.
///
/// Steps are owed for *elapsed* time, not for time the owner spent looking, so a widget that
/// exists but is not stepped for a while comes back owing one step per millisecond. That is
/// not hypothetical: a pause menu whose buttons are built when the world loads and first
/// drawn when the player opens the menu owed 3.6 million animation steps per button after an
/// hour of play.
///
/// Skipping them changes nothing that could be seen. An animation that closes the remaining
/// distance geometrically and snaps once it is inside an epsilon has settled on its target in
/// well under this many steps.
pub const MAX_CATCHUP_FRAMES: i64 = 2000;

/// Hands out one step per `step_ms` milliseconds of elapsed time.
///
/// It accumulates: after waiting 1000ms with a 100ms step, `step` returns true ten times and
/// false on the eleventh. That is what makes a fixed rate simulation independent of how often
/// it is driven - the caller loops `while timer.step()` and gets the same number of steps per
/// second whatever the frame rate.
///
/// # Catching up is a policy, and it is chosen at construction
///
/// `new` owes every step: a simulation that skips one has silently run slower than it should
/// have, and nothing downstream can tell. `for_animation` caps the backlog at
/// `MAX_CATCHUP_FRAMES`, because the frames nobody was there to see are worth nothing and
/// paying them all back at once is a burst of thousands of steps in one frame.
///
/// Both count **absolute milliseconds since construction**, not deltas, and are 64 bit for
/// that reason - as `i32` they overflowed after 24.8 days of uptime, which stopped every
/// animation in the game for good.
#[derive(Debug)]
pub struct FixedStep {
    step_ms: i64,
    start_time: std::time::Instant,
    /// How much simulated time has already been handed out, in milliseconds.
    stepped_ms: i64,
    /// Largest backlog to walk, or `None` to owe every step.
    catchup_limit_ms: Option<i64>,
}

impl FixedStep {
    /// A timer that owes every step that elapsed. For simulation.
    #[must_use]
    pub fn new(step_ms: i64) -> Self {
        Self {
            step_ms,
            start_time: std::time::Instant::now(),
            stepped_ms: 0,
            catchup_limit_ms: None,
        }
    }

    /// A timer that drops backlog older than `MAX_CATCHUP_FRAMES` steps. For animation.
    #[must_use]
    pub fn for_animation(step_ms: i64) -> Self {
        Self {
            catchup_limit_ms: Some(MAX_CATCHUP_FRAMES.saturating_mul(step_ms)),
            ..Self::new(step_ms)
        }
    }

    /// An animating timer that has already been running for `ms`, so a test can ask what a
    /// widget does after an hour of not being drawn without waiting an hour for it.
    #[cfg(test)]
    #[must_use]
    pub fn for_animation_started_ago(step_ms: i64, ms: u64) -> Self {
        Self {
            // A clock that cannot go back that far is one no test is measuring against anyway.
            start_time: std::time::Instant::now().checked_sub(std::time::Duration::from_millis(ms)).unwrap_or_else(std::time::Instant::now),
            ..Self::for_animation(step_ms)
        }
    }

    /// Stops the timer from ever reporting another step, so a golden image does not depend on
    /// how long the run took to reach the widget.
    #[cfg(feature = "render-tests")]
    pub const fn freeze(&mut self) {
        self.stepped_ms = i64::MAX;
    }

    /// How much simulated time has been handed out so far, in milliseconds.
    #[must_use]
    pub const fn stepped_ms(&self) -> i64 {
        self.stepped_ms
    }

    /// Takes one step if one is owed. Call it in a `while` loop to catch up to real time.
    #[must_use]
    pub fn step(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i64;

        if let Some(limit) = self.catchup_limit_ms {
            // Drop whatever backlog is older than the bound instead of walking it. A `max`
            // rather than an assignment, so a frozen timer stays frozen and a timer that is
            // keeping up is untouched.
            self.stepped_ms = self.stepped_ms.max(elapsed.saturating_sub(limit));
        }

        if elapsed > self.stepped_ms {
            self.stepped_ms += self.step_ms;
            true
        } else {
            false
        }
    }
}
