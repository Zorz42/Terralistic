/// How long the frames so far *should* have taken against how long they did, in milliseconds.
///
/// The limit is an **average** rather than a per-frame cap: the sleep at the end of a frame is
/// the difference between the two sides, so a frame that overran is made up by the next ones
/// instead of pushing the whole session behind.
///
/// Both sides only ever grow, which is why they are `f64`: as `f32` a 16ms increment stops
/// being representable after about four hours, the elapsed side stalls while the target side
/// climbs, and the sleep grows without bound.
#[derive(Default, Debug)]
pub struct FrameLimiter {
    /// Zero means unlimited.
    min_ms_per_frame: f64,
    frames_so_far: u64,
    ms_so_far: f64,
}

impl FrameLimiter {
    /// Caps the frame rate at `fps`. A non-positive `fps` means no limit, rather than a
    /// division by zero and a sleep measured in centuries.
    ///
    /// The ledger is only cleared when the limit actually changes: it means nothing across a
    /// change of target, but re-setting the *same* limit has to be free, because a settings
    /// menu applies every setting on every event it sees. Clearing it each time would leave
    /// the limiter capping each frame on its own rather than averaging over them.
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
    /// **The debt is capped at one frame.** Without that the ledger is repaid at full speed
    /// however long it took to run up, so anything that stops the loop for a while - a world
    /// loading, a laptop waking, a breakpoint - buys that many frames of completely uncapped
    /// rendering afterwards. A five minute pause at a 60 fps limit is eighteen thousand of
    /// them. Making up a frame or two of jitter is the point; making up a stall the caller did
    /// not ask for is not.
    ///
    /// Deliberately not `#[must_use]`: booking the frame is half the point, and a caller
    /// that only wants the ledger kept - a test, or a loop that sleeps some other way - is
    /// not making a mistake by ignoring the answer.
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
