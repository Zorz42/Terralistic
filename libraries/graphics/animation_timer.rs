/// Hands out one animation frame per `per_frame` milliseconds of elapsed time.
///
/// It accumulates: after waiting 1000ms with a 100ms duration, `frame_ready` returns true
/// ten times and false on the eleventh.
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

    /// Stops the timer from ever reporting another ready frame, so a golden image does not
    /// depend on how long the run took to reach the widget.
    #[cfg(feature = "render-tests")]
    pub const fn freeze(&mut self) {
        self.ms_passed = i64::MAX;
    }

    #[must_use]
    pub fn frame_ready(&mut self) -> bool {
        let elapsed = self.start_time.elapsed().as_millis() as i64;
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
