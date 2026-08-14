#![allow(clippy::unwrap_used)]
#![cfg(test)]
mod tests {
    use crate::client::game::framerate_measurer::FramerateMeasurer;

    #[test]
    fn test_framerate_measurer_starts_at_zero() {
        let measurer = FramerateMeasurer::new();
        assert_eq!(measurer.get_fps(), 0);
        assert!(measurer.get_delta_time().abs() < f32::EPSILON);
        assert!(measurer.get_max_frame_time().abs() < f32::EPSILON);
    }

    #[test]
    fn test_update_records_a_delta_time() {
        let mut measurer = FramerateMeasurer::new();
        std::thread::sleep(std::time::Duration::from_millis(5));
        measurer.update();

        assert!(measurer.get_delta_time() > 0.0, "update should measure time since the previous frame");
    }

    /// The 5ms accumulator hands out one step per call and catches up to real time, which
    /// is what drives client side simulation at a fixed rate.
    #[test]
    fn test_has_5ms_passed_catches_up_one_step_at_a_time() {
        let mut measurer = FramerateMeasurer::new();
        std::thread::sleep(std::time::Duration::from_millis(26));

        let mut steps = 0;
        while measurer.has_5ms_passed() {
            steps += 1;
            assert!(steps < 100, "the accumulator never caught up, it is not advancing");
        }

        assert!(steps >= 5, "about 26ms should yield at least 5 steps, got {steps}");
    }

    #[test]
    fn test_has_5ms_passed_is_false_immediately() {
        let mut measurer = FramerateMeasurer::new();
        // drain whatever time construction took
        while measurer.has_5ms_passed() {}
        assert!(!measurer.has_5ms_passed(), "with no time elapsed there is nothing to step");
    }
}
