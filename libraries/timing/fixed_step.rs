/// How much backlog an animating timer walks before skipping to the present.
///
/// Steps are owed for *elapsed* time, not for time the owner spent looking, so a widget that
/// is not stepped for a while comes back owing one step per millisecond - a pause menu's
/// buttons, built when the world loads and first drawn an hour later, owed 3.6 million each.
/// Nothing visible is lost: an animation settles well inside this many steps.
pub const MAX_CATCHUP_FRAMES: i64 = 2000;

/// Hands out one step per `step_ms` milliseconds of elapsed time, accumulating - after 1000ms with
/// a 100ms step, `step` answers true ten times.
///
/// A `while timer.step()` loop therefore runs at the same rate whatever the frame rate.
///
/// **Catching up is a policy chosen at construction.** `new` owes every step, because a
/// simulation that skips one has silently run slower and nothing downstream can tell.
/// `for_animation` caps the backlog at `MAX_CATCHUP_FRAMES`, because frames nobody saw are
/// worth nothing and repaying them is a burst of thousands of steps in one frame.
///
/// Both count **absolute milliseconds since construction** and are 64 bit for it: as `i32`
/// they overflowed after 24.8 days of uptime and every animation in the game stopped for good.
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

    /// An animating timer already `ms` old, so a test can ask what a widget does after an hour
    /// of not being drawn without waiting an hour.
    #[cfg(test)]
    #[must_use]
    pub fn for_animation_started_ago(step_ms: i64, ms: u64) -> Self {
        Self {
            // A clock that cannot go back that far is one no test is measuring against anyway.
            start_time: std::time::Instant::now().checked_sub(std::time::Duration::from_millis(ms)).unwrap_or_else(std::time::Instant::now),
            ..Self::for_animation(step_ms)
        }
    }

    /// Stops the timer reporting another step, so a golden does not depend on how long the run
    /// took to reach the widget.
    #[cfg(feature = "render-tests")]
    pub const fn freeze(&mut self) {
        self.stepped_ms = i64::MAX;
    }

    /// How much simulated time has been handed out so far, in milliseconds.
    #[must_use]
    pub const fn stepped_ms(&self) -> i64 {
        self.stepped_ms
    }

    /// How far real time has got through the step the simulation has already taken, from 0 at
    /// its start to just under 1 at its end.
    ///
    /// A renderer draws on the display's clock and the simulation moves in whole steps, so a
    /// 5ms tick drawn at 60fps advances three ticks on one frame and four on the next - a
    /// steady walk drawn as a stutter. This is the fraction to draw at, in between.
    ///
    /// Measured in microseconds, since a millisecond is a fifth of the tick it divides.
    #[must_use]
    pub fn fraction_of_step(&self) -> f32 {
        let into_step = self.start_time.elapsed().as_micros() as i64 - (self.stepped_ms - self.step_ms) * 1000;
        (into_step as f32 / (self.step_ms * 1000) as f32).clamp(0.0, 1.0)
    }

    /// Takes one step if one is owed. Call it in a `while` loop to catch up to real time.
    #[must_use]
    pub fn step(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i64;

        if let Some(limit) = self.catchup_limit_ms {
            // Drop backlog older than the bound rather than walking it. A `max`, so a frozen
            // timer stays frozen and one that is keeping up is untouched.
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
