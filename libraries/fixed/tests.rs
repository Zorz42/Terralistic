#![allow(clippy::unwrap_used)] // tests assert on results directly
#[cfg(test)]
mod tests {
    use crate::libraries::fixed::{Fixed, ONE_RAW};
    use std::collections::HashMap;

    #[test]
    fn test_whole_numbers_round_trip() {
        for value in [-30000, -1, 0, 1, 4400, 30000] {
            assert_eq!(Fixed::from_int(value).to_int(), value);
        }
    }

    #[test]
    fn test_from_num_is_an_exact_ratio() {
        assert_eq!(Fixed::from_num(1, 2), Fixed::ONE / 2);
        assert_eq!(Fixed::from_num(1, 1), Fixed::ONE);
        // 0.005, the air resistance constant, to the nearest representable step
        assert_eq!(Fixed::from_num(1, 200).raw(), (ONE_RAW / 200) as i32);
    }

    #[test]
    fn test_from_num_with_zero_denominator_is_zero() {
        assert_eq!(Fixed::from_num(5, 0), Fixed::ZERO);
    }

    /// The property the module exists for. `>>` would floor, so a leftward value would lose
    /// more per step than the mirrored rightward one and an entity would drift left forever.
    #[test]
    fn test_arithmetic_is_symmetric_about_zero() {
        let decay = Fixed::from_num(995, 1000);
        for raw in [1, 7, 12345, 0xFFFF, 1 << 20] {
            let right = Fixed::from_raw(raw);
            let left = -right;
            assert_eq!(right * decay, -(left * decay), "multiplication drifted at raw {raw}");
            assert_eq!(right / 200, -(left / 200), "division drifted at raw {raw}");
            assert_eq!(right.to_int(), -left.to_int(), "truncation drifted at raw {raw}");
        }
    }

    #[test]
    fn test_floor_and_truncation_differ_on_negatives() {
        let value = Fixed::from_num(-3, 2); // -1.5
        assert_eq!(value.to_int(), -1, "to_int rounds toward zero");
        assert_eq!(value.floor_to_int(), -2, "floor_to_int rounds down");
        assert_eq!(value.ceil_to_int(), -1);
    }

    #[test]
    fn test_floor_and_truncation_agree_on_positives() {
        let value = Fixed::from_num(3, 2);
        assert_eq!(value.to_int(), 1);
        assert_eq!(value.floor_to_int(), 1);
        assert_eq!(value.ceil_to_int(), 2);
    }

    /// A runaway value pins at the end of the range. Wrapping would put it at the opposite
    /// end, which in a world is the far side of the map.
    #[test]
    fn test_narrowing_saturates_rather_than_wrapping() {
        assert_eq!(Fixed::MAX + Fixed::ONE, Fixed::MAX);
        assert_eq!(Fixed::MIN - Fixed::ONE, Fixed::MIN);
        assert_eq!(Fixed::MAX * Fixed::from_int(2), Fixed::MAX);
        assert_eq!(Fixed::MIN * Fixed::from_int(2), Fixed::MIN);
        assert_eq!(Fixed::from_int(30000) * 1000, Fixed::MAX);
    }

    #[test]
    fn test_division_by_zero_pins_instead_of_panicking() {
        assert_eq!(Fixed::ONE / Fixed::ZERO, Fixed::MAX);
        assert_eq!(-Fixed::ONE / Fixed::ZERO, Fixed::MIN);
        assert_eq!(Fixed::ZERO / Fixed::ZERO, Fixed::ZERO);
        assert_eq!(Fixed::ONE / 0, Fixed::MAX);
    }

    #[test]
    fn test_sqrt_of_perfect_squares_is_exact() {
        for value in [0, 1, 4, 9, 16, 144, 10000] {
            assert_eq!(Fixed::from_int(value).sqrt(), Fixed::from_int((value as f64).sqrt() as i32), "sqrt({value})");
        }
    }

    #[test]
    fn test_sqrt_is_close_for_non_squares() {
        let two = Fixed::from_int(2).sqrt();
        assert!((two.to_f32() - std::f32::consts::SQRT_2).abs() < 0.001, "got {two}");
    }

    #[test]
    fn test_sqrt_of_negative_is_zero() {
        assert_eq!(Fixed::from_int(-4).sqrt(), Fixed::ZERO);
    }

    /// What floats could not do, and the reason the simulation can be compared rather than
    /// approximately compared. `gfx::FloatPos` deliberately refuses to implement `Hash`.
    #[test]
    fn test_is_usable_as_an_exact_map_key() {
        let mut map = HashMap::new();
        map.insert(Fixed::from_num(1, 3), "a third");
        assert_eq!(map.get(&(Fixed::ONE / Fixed::from_int(3))), Some(&"a third"));
    }

    #[test]
    fn test_ordering_matches_the_numbers() {
        assert!(Fixed::from_int(-1) < Fixed::ZERO);
        assert!(Fixed::ZERO < Fixed::EPSILON);
        assert!(Fixed::EPSILON < Fixed::ONE);
    }

    /// Values below the resolution vanish, which is what makes a decaying velocity settle.
    /// The same rule liquid levels already follow for the same reason.
    #[test]
    fn test_values_below_the_resolution_reach_zero() {
        let mut velocity = Fixed::from_num(1, 100);
        let decay = Fixed::from_num(9, 10);
        for _ in 0..1000 {
            velocity *= decay;
        }
        assert_eq!(velocity, Fixed::ZERO, "a decaying value must actually stop");
    }

    /// The range has to hold a world a few thousand units across with room to spare.
    #[test]
    fn test_range_covers_a_large_world() {
        let far_corner = Fixed::from_int(4400);
        assert_eq!(far_corner.to_int(), 4400);
        assert!(far_corner < Fixed::MAX / 2, "a world coordinate should be nowhere near the end of the range");
    }

    #[test]
    fn test_float_conversion_round_trips_within_resolution() {
        for value in [-1234.5_f32, -0.001, 0.0, 0.005, 1.0, 3.25, 4400.0] {
            let converted = Fixed::from_f32(value).to_f32();
            assert!((converted - value).abs() < 0.0001, "{value} became {converted}");
        }
    }

    #[test]
    fn test_nan_becomes_zero_rather_than_entering_the_simulation() {
        assert_eq!(Fixed::from_f32(f32::NAN), Fixed::ZERO);
        assert_eq!(Fixed::from_f32(f32::INFINITY), Fixed::MAX);
        assert_eq!(Fixed::from_f32(f32::NEG_INFINITY), Fixed::MIN);
    }

    #[test]
    fn test_serialization_round_trips() {
        let value = Fixed::from_num(1234, 7);
        let bytes = crate::libraries::serialization::serialize(&value).unwrap();
        assert_eq!(crate::libraries::serialization::deserialize::<Fixed>(&bytes).unwrap(), value);
    }
}
