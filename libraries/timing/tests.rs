#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::libraries::timing::{Budget, DeltaTimer, FixedStep, FrameLimiter, FrameStats, Interval, MAX_CATCHUP_FRAMES};

    // --- FixedStep ---

    /// The timer hands out one step per `step_ms` milliseconds elapsed and then stops, which
    /// is what keeps a simulation running at a fixed rate independent of frame rate.
    #[test]
    fn test_fixed_step_hands_out_steps_for_elapsed_time() {
        let mut timer = FixedStep::for_animation(5);
        std::thread::sleep(std::time::Duration::from_millis(20));

        let mut steps = 0;
        while timer.step() {
            steps += 1;
            assert!(steps < 100, "the timer should stop handing out steps");
        }

        // 20ms of backlog at 5ms per step, give or take the sleep's precision
        assert!((3..=6).contains(&steps), "expected about 4 steps, got {steps}");
    }

    #[test]
    fn test_fixed_step_has_nothing_ready_immediately() {
        let mut timer = FixedStep::for_animation(1000);
        assert!(!timer.step());
    }

    /// The simulation accumulator, which is the same mechanism the client and the server
    /// both drive their 5ms tick from.
    #[test]
    fn test_fixed_step_catches_up_one_step_at_a_time() {
        let mut timer = FixedStep::new(5);
        std::thread::sleep(std::time::Duration::from_millis(26));

        let mut steps = 0;
        while timer.step() {
            steps += 1;
            assert!(steps < 100, "the accumulator never caught up, it is not advancing");
        }

        assert!(steps >= 5, "about 26ms should yield at least 5 steps, got {steps}");
    }

    #[test]
    fn test_stepped_ms_counts_the_simulated_time_handed_out() {
        let mut timer = FixedStep::new(5);
        std::thread::sleep(std::time::Duration::from_millis(20));

        let mut steps = 0_i64;
        while timer.step() {
            steps += 1;
        }

        assert_eq!(timer.stepped_ms(), steps * 5, "stepped_ms must agree with how many steps were taken");
    }

    /// A widget not stepped for a long time owes a step per millisecond, and a pause menu's
    /// buttons really do sit unrendered for a whole session. Walking that is an hour of
    /// animation in one frame; skipping it lands on the same value, every animation having
    /// settled long before the bound.
    #[test]
    fn test_an_animating_timer_skips_a_backlog_it_could_never_walk() {
        let hour_ms = 60 * 60 * 1000;
        let mut timer = FixedStep::for_animation_started_ago(1, hour_ms);

        let mut steps = 0_i64;
        while timer.step() {
            steps += 1;
            assert!(steps < hour_ms as i64, "the timer walked the whole backlog");
        }

        assert!(steps > 0, "the bound should still hand out the steps it caps at");
        // A few more than the bound: the clock keeps running while the backlog is handed out.
        assert!(steps <= MAX_CATCHUP_FRAMES + 100, "expected about {MAX_CATCHUP_FRAMES} steps, got {steps}");
    }

    /// The bound is a floor on how far behind the timer may be, not a reset: a timer that is
    /// keeping up must still hand out exactly the steps it owes.
    #[test]
    fn test_the_backlog_bound_leaves_a_timer_that_is_keeping_up_alone() {
        let mut timer = FixedStep::for_animation_started_ago(5, 20);

        let mut steps = 0;
        while timer.step() {
            steps += 1;
            assert!(steps < 100, "the timer should stop handing out steps");
        }

        assert!((4..=6).contains(&steps), "expected about 4 steps, got {steps}");
    }

    /// The difference between the two constructors: a simulation is owed every step it
    /// missed, because nothing downstream can tell that one did not happen.
    #[test]
    fn test_a_simulation_timer_owes_the_whole_backlog() {
        let mut timer = FixedStep::new(1);
        std::thread::sleep(std::time::Duration::from_millis(30));

        let mut steps = 0;
        while timer.step() {
            steps += 1;
            assert!(steps < 10_000, "the timer is not advancing");
        }

        assert!(steps >= 30, "every elapsed millisecond should be owed a step, got {steps}");
    }

    // --- Budget ---

    #[test]
    fn test_a_fresh_budget_has_time_left() {
        assert!(Budget::of_ms(50).has_time_left());
    }

    #[test]
    fn test_a_spent_budget_has_no_time_left() {
        let budget = Budget::of_ms(5);
        std::thread::sleep(std::time::Duration::from_millis(10));

        assert!(!budget.has_time_left());
        assert!(budget.elapsed().as_millis() >= 10);
    }

    /// A caller that wants the optional work off entirely does not have to special-case it.
    #[test]
    fn test_an_exhausted_budget_never_has_time_left() {
        assert!(!Budget::exhausted().has_time_left());
    }

    // --- Interval ---

    #[test]
    fn test_an_interval_is_not_due_before_its_first_period() {
        let mut interval = Interval::starting_at(0.0, 100.0);

        assert!(!interval.is_due(0.0), "an interval must not fire the moment it is created");
        assert!(!interval.is_due(99.0));
        assert!(interval.is_due(100.0));
    }

    #[test]
    fn test_an_interval_repeats() {
        let mut interval = Interval::starting_at(0.0, 10.0);

        let mut times_due = 0;
        for now in 0..=100 {
            if interval.is_due(f64::from(now)) {
                times_due += 1;
            }
        }

        assert_eq!(times_due, 10, "10ms apart over 100ms should come due ten times");
    }

    /// **A missed interval is not owed.** Something that fell behind gets one late
    /// occurrence, not a burst of every one it skipped.
    #[test]
    fn test_a_missed_interval_is_not_owed() {
        let mut interval = Interval::starting_at(0.0, 10.0);

        assert!(interval.is_due(1000.0), "the long gap comes due once");
        assert!(!interval.is_due(1001.0), "and then waits a whole period again");
        assert!(interval.is_due(1010.0));
    }

    /// A period of zero is how a caller says "this never happens on its own", which the
    /// empty liquid type uses. It must not divide, loop or fire.
    #[test]
    fn test_an_interval_with_no_period_never_comes_due() {
        let mut interval = Interval::starting_at(0.0, 0.0);

        assert!(!interval.is_due(0.0));
        assert!(!interval.is_due(1_000_000.0));
    }

    // --- DeltaTimer ---

    /// The first call has no previous call to measure against, and the time since
    /// construction is however long the caller spent setting up rather than a frame anybody
    /// rendered. The server skips its first update for exactly this reason.
    #[test]
    fn test_the_first_tick_has_nothing_to_measure() {
        let mut timer = DeltaTimer::new();

        assert!(timer.tick().is_none());
    }

    #[test]
    fn test_a_later_tick_measures_the_gap() {
        let mut timer = DeltaTimer::new();
        timer.tick();
        std::thread::sleep(std::time::Duration::from_millis(5));

        let delta = timer.tick().unwrap();

        assert!(delta >= 4.0, "expected about 5ms, got {delta}ms");
    }

    // --- FrameStats ---

    #[test]
    fn test_frame_stats_start_at_zero() {
        let stats = FrameStats::new();

        assert_eq!(stats.get_fps(), 0);
        assert!(stats.get_delta_time().abs() < f32::EPSILON);
        assert!(stats.get_max_frame_time().abs() < f32::EPSILON);
    }

    /// The delta is measured from one frame's start to the next, so the very first frame has
    /// no previous one and reports zero rather than the setup time before it.
    #[test]
    fn test_the_first_frame_has_no_delta_time() {
        let mut stats = FrameStats::new();
        std::thread::sleep(std::time::Duration::from_millis(5));

        stats.begin_frame();

        assert!(stats.get_delta_time().abs() < f32::EPSILON, "the first frame has no previous frame to measure against");
    }

    #[test]
    fn test_a_later_frame_records_a_delta_time() {
        let mut stats = FrameStats::new();
        stats.begin_frame();
        std::thread::sleep(std::time::Duration::from_millis(5));
        stats.begin_frame();

        assert!(stats.get_delta_time() > 0.0, "begin_frame should measure time since the previous frame");
    }

    // --- FrameLimiter ---

    /// 60 fps, so a frame's share of the clock is 16.67ms.
    fn limiter_at_60() -> FrameLimiter {
        let mut limiter = FrameLimiter::default();
        limiter.set_fps_limit(60.0);
        limiter
    }

    /// Milliseconds, so a hundredth is far below anything anyone could see.
    #[track_caller]
    fn assert_close_ms(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 0.01, "expected {expected}ms, got {actual}ms");
    }

    /// A non-positive limit means no limit, rather than a division by zero and a sleep measured
    /// in centuries.
    #[test]
    fn test_an_unlimited_frame_never_sleeps() {
        assert_close_ms(FrameLimiter::default().owed_ms(0.0), 0.0);

        for fps in [0.0, -1.0] {
            let mut limiter = limiter_at_60();
            limiter.set_fps_limit(fps);
            assert_close_ms(limiter.owed_ms(0.0), 0.0);
        }
    }

    /// A frame that finished early sleeps out the rest of its share.
    #[test]
    fn test_a_fast_frame_sleeps_out_the_rest_of_its_share() {
        assert_close_ms(limiter_at_60().owed_ms(4.0), 1000.0 / 60.0 - 4.0);
    }

    /// The limit is an average, so a frame that overran is made up by the next one rather than
    /// leaving the whole session behind by that much.
    #[test]
    fn test_an_overrunning_frame_is_made_up_by_the_next() {
        let mut limiter = limiter_at_60();

        assert_close_ms(limiter.owed_ms(20.0), 0.0);
        assert_close_ms(limiter.owed_ms(0.0), 2.0 * 1000.0 / 60.0 - 20.0);
    }

    /// **The debt is capped at one frame.** A loop that stopped for a while - a world loading, a
    /// laptop waking, a breakpoint - would otherwise be owed every frame of it, and repay them
    /// all at full speed: a five minute pause at 60 fps is eighteen thousand uncapped frames.
    #[test]
    fn test_a_long_stall_does_not_buy_uncapped_frames_afterwards() {
        let mut limiter = limiter_at_60();
        limiter.owed_ms(5.0 * 60.0 * 1000.0);

        // one frame of the stall is made up, and then the limiter is back to capping
        assert_close_ms(limiter.owed_ms(0.0), 0.0);
        assert_close_ms(limiter.owed_ms(0.0), 1000.0 / 60.0);
    }

    /// Re-setting the same limit has to be free: the settings menu applies every setting on
    /// every event it sees, and clearing the ledger each time would leave the limiter capping
    /// each frame on its own rather than averaging over them.
    #[test]
    fn test_setting_the_same_limit_again_keeps_the_ledger() {
        let mut limiter = limiter_at_60();
        limiter.owed_ms(20.0);

        limiter.set_fps_limit(60.0);

        assert_close_ms(limiter.owed_ms(0.0), 2.0 * 1000.0 / 60.0 - 20.0);
    }

    #[test]
    fn test_changing_the_limit_starts_a_new_ledger() {
        let mut limiter = limiter_at_60();
        limiter.owed_ms(20.0);

        limiter.set_fps_limit(30.0);

        // a whole frame at the new rate, with nothing carried over
        assert_close_ms(limiter.owed_ms(0.0), 1000.0 / 30.0);
    }
}
