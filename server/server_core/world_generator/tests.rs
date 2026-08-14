#![allow(clippy::unwrap_used)] // tests assert on results directly
#![cfg(test)]
mod tests {
    use crate::libraries::procgen::{convolve, turbulence};
    use noise::Perlin;

    // --- convolve ---

    /// Convolving a flat array leaves it flat: every window averages to the same value.
    #[test]
    fn test_convolve_flat_array_is_unchanged() {
        let input = vec![5.0; 20];
        let output = convolve(&input, 6);

        assert_eq!(output.len(), input.len());
        for value in output {
            assert!((value - 5.0).abs() < 0.0001, "expected 5.0, got {value}");
        }
    }

    #[test]
    fn test_convolve_preserves_length() {
        for size in [2, 4, 10, 50] {
            let input = vec![1.0; 30];
            assert_eq!(convolve(&input, size).len(), 30, "length changed for kernel {size}");
        }
    }

    /// A step edge should come out smoothed: the values around the boundary land strictly
    /// between the two levels rather than jumping.
    #[test]
    fn test_convolve_smooths_a_step() {
        let mut input = vec![0.0; 20];
        for value in input.iter_mut().skip(10) {
            *value = 10.0;
        }

        let output = convolve(&input, 6);

        // the sharp boundary is gone: the value just before the step has risen above 0
        assert!(*output.get(9).unwrap() > 0.0, "the low side should be pulled up near the step");
        // and the value just after has not yet reached the top
        assert!(*output.get(10).unwrap() < 10.0, "the high side should be pulled down near the step");
        // far from the step the original levels survive
        assert!(*output.first().unwrap() < *output.last().unwrap());
    }

    #[test]
    fn test_convolve_output_stays_within_input_range() {
        let input = vec![1.0, 9.0, 2.0, 8.0, 3.0, 7.0, 4.0, 6.0, 5.0, 5.0];
        let output = convolve(&input, 4);

        for value in output {
            assert!((1.0..=9.0).contains(&value), "convolved value {value} escaped the input range");
        }
    }

    /// A kernel of 1 halves to 0, so each output is just the running single element.
    #[test]
    fn test_convolve_with_tiny_kernel() {
        let input = vec![1.0, 2.0, 3.0];
        let output = convolve(&input, 1);
        assert_eq!(output.len(), 3);
    }

    // --- turbulence ---

    /// Turbulence is deterministic for a given seed and coordinate, which is what makes
    /// terrain reproducible from the same Perlin instance.
    #[test]
    fn test_turbulence_is_deterministic() {
        let noise = Perlin::new(42);

        let a = turbulence(&noise, 1.5, 2.5);
        let b = turbulence(&noise, 1.5, 2.5);

        assert!((a - b).abs() < f32::EPSILON, "same input gave {a} then {b}");
    }

    #[test]
    fn test_turbulence_same_seed_same_values() {
        let a = turbulence(&Perlin::new(7), 3.0, 4.0);
        let b = turbulence(&Perlin::new(7), 3.0, 4.0);

        assert!((a - b).abs() < f32::EPSILON);
    }

    #[test]
    fn test_turbulence_varies_across_space() {
        let noise = Perlin::new(1);
        let samples: Vec<f32> = (0..20).map(|i| turbulence(&noise, i as f32 * 3.7, i as f32 * 1.3)).collect();

        let first = *samples.first().unwrap();
        assert!(samples.iter().any(|s| (s - first).abs() > 0.001), "turbulence returned a constant across 20 samples");
    }

    /// Three octaves of Perlin, each in [-1, 1], summed with weights 1, 1/2, 1/4 and then
    /// halved, so the result cannot leave [-0.875, 0.875].
    #[test]
    fn test_turbulence_stays_in_range() {
        let noise = Perlin::new(99);

        for i in 0..200 {
            let value = turbulence(&noise, i as f32 * 0.37, i as f32 * 0.71);
            assert!(value.abs() <= 0.875, "turbulence returned {value}, outside the theoretical bound");
        }
    }

    #[test]
    fn test_turbulence_different_seeds_differ() {
        // deliberately off the integer lattice, see the test below
        let a = turbulence(&Perlin::new(1), 5.3, 5.7);
        let b = turbulence(&Perlin::new(2), 5.3, 5.7);

        assert!((a - b).abs() > f32::EPSILON, "two seeds produced the same value");
    }

    /// Perlin noise is exactly zero at integer lattice points, whatever the seed, and the
    /// three octaves in `turbulence` are all sampled at x/1, x/2 and x/4 - so an integer
    /// coordinate is zero for every octave too.
    ///
    /// This is why the generator divides coordinates by 150, 80 and 20 before sampling: on
    /// a plain integer grid every column would get identical noise.
    #[test]
    fn test_turbulence_is_zero_on_the_integer_lattice() {
        for seed in [1, 2, 12345] {
            let noise = Perlin::new(seed);
            for coord in [0.0, 1.0, 5.0, 12.0] {
                let value = turbulence(&noise, coord, coord);
                assert!(value.abs() < f32::EPSILON, "seed {seed} at {coord} gave {value}, expected 0");
            }
        }
    }
}
