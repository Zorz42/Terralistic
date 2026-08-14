#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::panic)] // a wrong length or an impossible branch is a test failure
#[cfg(test)]
mod tests {
    use noise::Perlin;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use crate::libraries::procgen::{convolve, pick_weighted, turbulence};

    // --- turbulence ---

    #[test]
    fn test_turbulence_is_reproducible_for_a_seed() {
        let noise = Perlin::new(42);

        assert!((turbulence(&noise, 1.5, 2.5) - turbulence(&noise, 1.5, 2.5)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_turbulence_stays_in_range() {
        let noise = Perlin::new(7);

        for x in 0..50 {
            for y in 0..50 {
                let value = turbulence(&noise, x as f32 / 3.0, y as f32 / 3.0);
                assert!((-1.5..=1.5).contains(&value), "turbulence went to {value}");
            }
        }
    }

    /// Two different seeds must not produce the same field, or a world seed would mean
    /// nothing.
    #[test]
    fn test_different_seeds_give_different_noise() {
        let first = Perlin::new(1);
        let second = Perlin::new(2);

        let differences = (0..50)
            .filter(|x| (turbulence(&first, *x as f32 / 3.0, 0.5) - turbulence(&second, *x as f32 / 3.0, 0.5)).abs() > 0.001)
            .count();

        assert!(differences > 25, "only {differences} of 50 samples differed between two seeds");
    }

    // --- convolve ---

    #[test]
    fn test_convolve_of_a_constant_is_that_constant() {
        let smoothed = convolve(&[5.0; 20], 5);

        for value in smoothed {
            assert!((value - 5.0).abs() < 0.001, "a constant array should smooth to itself, got {value}");
        }
    }

    #[test]
    fn test_convolve_keeps_the_length() {
        assert_eq!(convolve(&[1.0, 2.0, 3.0, 4.0], 3).len(), 4);
        assert!(convolve(&[], 5).is_empty());
    }

    /// The point of it: a step becomes a ramp, so a boundary between two regions is a
    /// transition rather than a cliff.
    #[test]
    fn test_convolve_smooths_a_step() {
        let mut step = vec![0.0; 20];
        for value in step.iter_mut().skip(10) {
            *value = 10.0;
        }

        let smoothed = convolve(&step, 9);

        assert!(smoothed[9] > 0.0, "the low side of the step should be pulled up");
        assert!(smoothed[10] < 10.0, "the high side of the step should be pulled down");
        // and it is still monotonic across the boundary
        for i in 1..smoothed.len() {
            assert!(smoothed[i] >= smoothed[i - 1] - 0.001, "smoothing a step should not introduce a dip at {i}");
        }
    }

    /// The window is clipped at the ends rather than padded with zeroes, so an array of
    /// tens smooths to tens all the way to the edge.
    #[test]
    fn test_convolve_does_not_fade_at_the_edges() {
        let smoothed = convolve(&[10.0; 30], 11);

        assert!((smoothed[0] - 10.0).abs() < 0.001, "the first element faded to {}", smoothed[0]);
        assert!((smoothed[29] - 10.0).abs() < 0.001, "the last element faded to {}", smoothed[29]);
    }

    // --- pick_weighted ---

    #[test]
    fn test_pick_weighted_returns_nothing_when_there_is_nothing() {
        let mut rng = StdRng::seed_from_u64(1);

        assert!(pick_weighted::<i32, _>(&[], &mut rng).is_none());
        assert!(pick_weighted(&[(0, "never"), (0, "also never")], &mut rng).is_none());
        assert!(pick_weighted(&[(-5, "negative")], &mut rng).is_none());
    }

    #[test]
    fn test_a_single_option_is_always_picked() {
        let mut rng = StdRng::seed_from_u64(2);

        for _ in 0..20 {
            assert_eq!(pick_weighted(&[(3, "only")], &mut rng), Some(&"only"));
        }
    }

    /// Weights are proportions, and a zero-weighted item is never picked - which is what
    /// lets a caller leave an edge in the list but switched off.
    #[test]
    fn test_weights_are_proportions() {
        let mut rng = StdRng::seed_from_u64(3);
        let options = [(9, "common"), (1, "rare"), (0, "never")];

        let mut common = 0;
        let mut rare = 0;
        for _ in 0..1000 {
            match pick_weighted(&options, &mut rng) {
                Some(&"common") => common += 1,
                Some(&"rare") => rare += 1,
                other => panic!("picked {other:?}, which should never happen"),
            }
        }

        assert!(common > rare * 4, "9:1 weights gave {common}:{rare}");
        assert!(rare > 20, "the rare option should still come up sometimes, got {rare}");
    }
}
