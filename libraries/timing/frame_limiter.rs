/// How long the frames so far *should* have taken against how long they did, in milliseconds.
///
/// The limit is an **average**, not a per-frame cap: the sleep is the difference between the
/// two sides, so an overrun is made up by the next frames rather than pushing the session
/// behind. Both sides only grow, hence `f64`: as `f32` a 16ms increment stops being
/// representable after four hours, the elapsed side stalls, and the sleep grows without bound.
#[derive(Default, Debug)]
pub struct FrameLimiter {
    /// Zero means unlimited.
    min_ms_per_frame: f64,
    frames_so_far: u64,
    ms_so_far: f64,
}

impl FrameLimiter {
    /// Caps the frame rate at `fps`; non-positive means no limit rather than a division by
    /// zero and a sleep measured in centuries.
    ///
    /// The ledger is cleared only when the limit changes. It means nothing across a change of
    /// target, but re-setting the *same* limit has to be free - a settings menu applies every
    /// setting on every event, and clearing each time caps frames instead of averaging them.
    pub fn set_fps_limit(&mut self, fps: f32) {
        let min_ms_per_frame = if fps > 0.0 { 1000.0 / f64::from(fps) } else { 0.0 };
        if (min_ms_per_frame - self.min_ms_per_frame).abs() > f64::EPSILON {
            *self = Self {
                min_ms_per_frame,
                frames_so_far: 0,
                ms_so_far: 0.0,
            };
        }
    }

    /// Books a frame that took `elapsed_ms` and answers how long to sleep for.
    ///
    /// **The debt is capped at one frame.** Otherwise anything that stalls the loop - a world
    /// loading, a laptop waking, a breakpoint - buys that many frames of uncapped rendering
    /// afterwards, which for a five minute pause at 60 fps is eighteen thousand of them.
    ///
    /// Not `#[must_use]`: booking the frame is half the point, so a caller that only wants the
    /// ledger kept is not making a mistake by ignoring the answer.
    pub fn owed_ms(&mut self, elapsed_ms: f64) -> f64 {
        if self.min_ms_per_frame <= 0.0 {
            return 0.0;
        }

        self.frames_so_far += 1;
        self.ms_so_far += elapsed_ms;

        let owed = self.min_ms_per_frame * self.frames_so_far as f64 - self.ms_so_far;
        if owed < -self.min_ms_per_frame {
            self.ms_so_far = self.min_ms_per_frame * (self.frames_so_far as f64 + 1.0);
        }
        f64::max(owed, 0.0)
    }
}
