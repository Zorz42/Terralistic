/// How much of a backlog the timer will walk before it gives up and skips to the present.
///
/// Frames are owed for *elapsed* time, not for time the owner spent looking, so a widget that
/// exists but is not stepped for a while comes back owing one frame per millisecond. That is
/// not hypothetical: `client/game/pause_menu.rs` builds its buttons when the world loads and
/// first draws them when the player opens the menu, so after an hour of play the first frame
/// of the pause menu owed 3.6 million animation steps per button.
///
/// Skipping them changes nothing that could be seen. Every animation in the toolkit is
/// `approach`, which closes the remaining distance geometrically and snaps once it is inside
/// its epsilon, so all of them have settled on their target in well under this many steps -
/// the slowest, `Scrollable`'s velocity decay, takes about 850.
pub(super) const MAX_CATCHUP_FRAMES: i64 = 2000;

/// Hands out one animation frame per `per_frame` milliseconds of elapsed time.
///
/// It accumulates: after waiting 1000ms with a 100ms duration, `frame_ready` returns true
/// ten times and false on the eleventh. Only up to `MAX_CATCHUP_FRAMES` of that backlog is
/// ever handed out at once.
///
/// The counters are absolute milliseconds since construction, not deltas, and are 64 bit for
/// that reason - as `i32` they overflowed after 24.8 days of uptime, which stopped every
/// animation in the game for good.
#[derive(Debug)]
pub struct AnimationTimer {
    per_frame: i64,
    start_time: std::time::Instant,
    ms_passed: i64,
}

impl AnimationTimer {
    #[must_use]
    pub fn new(per_frame: i64) -> Self {
        Self {
            per_frame,
            start_time: std::time::Instant::now(),
            ms_passed: 0,
        }
    }

    /// A timer that has already been running for `ms`, so a test can ask what a widget does
    /// after an hour of not being drawn without waiting an hour for it.
    #[cfg(test)]
    #[must_use]
    pub fn new_started_ago(per_frame: i64, ms: u64) -> Self {
        Self {
            per_frame,
            // A clock that cannot go back that far is one no test is measuring against anyway.
            start_time: std::time::Instant::now().checked_sub(std::time::Duration::from_millis(ms)).unwrap_or_else(std::time::Instant::now),
            ms_passed: 0,
        }
    }

    /// Stops the timer from ever reporting another ready frame, so a golden image does not
    /// depend on how long the run took to reach the widget.
    #[cfg(feature = "render-tests")]
    pub const fn freeze(&mut self) {
        self.ms_passed = i64::MAX;
    }

    #[must_use]
    pub fn frame_ready(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i64;
        // Drop whatever backlog is older than the bound instead of walking it. A `max` rather
        // than an assignment, so a frozen timer stays frozen and a timer that is keeping up is
        // untouched.
        self.ms_passed = self.ms_passed.max(elapsed.saturating_sub(MAX_CATCHUP_FRAMES.saturating_mul(self.per_frame)));

        if elapsed > self.ms_passed {
            self.ms_passed += self.per_frame;
            true
        } else {
            false
        }
    }
}

/// Moves `value` a `1 / smooth_factor` fraction of the way to `target`, snapping onto it once
/// the two are within `epsilon`. Every fade and slide in the toolkit is one of these per
/// ready frame.
#[must_use]
pub fn approach(value: f32, target: f32, smooth_factor: f32, epsilon: f32) -> f32 {
    let stepped = value + (target - value) / smooth_factor;
    if (target - stepped).abs() <= epsilon {
        target
    } else {
        stepped
    }
}
