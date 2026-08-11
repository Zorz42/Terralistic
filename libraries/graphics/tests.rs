#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![cfg(test)]
mod tests {
    use crate::libraries::graphics::transformation::Transformation;
    use crate::libraries::graphics::{interpolate_colors, Color, FloatPos, FloatSize, IntPos, IntSize, Rect, Surface};

    // --- Color ---

    #[test]
    fn test_color_setters_are_independent() {
        let color = Color::new(1, 2, 3, 4);

        assert_eq!(color.set_r(10), Color::new(10, 2, 3, 4));
        assert_eq!(color.set_g(20), Color::new(1, 20, 3, 4));
        assert_eq!(color.set_b(30), Color::new(1, 2, 30, 4));
        assert_eq!(color.set_a(40), Color::new(1, 2, 3, 40));
        // the setters take self by value, so the original is untouched
        assert_eq!(color, Color::new(1, 2, 3, 4));
    }

    #[test]
    fn test_interpolate_colors_at_the_ends() {
        let a = Color::new(0, 0, 0, 0);
        let b = Color::new(200, 100, 50, 255);

        assert_eq!(interpolate_colors(a, b, 0.0), a);
        assert_eq!(interpolate_colors(a, b, 1.0), b);
    }

    #[test]
    fn test_interpolate_colors_midpoint() {
        let a = Color::new(0, 0, 0, 0);
        let b = Color::new(200, 100, 50, 254);

        assert_eq!(interpolate_colors(a, b, 0.5), Color::new(100, 50, 25, 127));
    }

    // --- position and size types ---

    #[test]
    fn test_int_pos_arithmetic() {
        assert_eq!(IntPos(3, 4) + IntPos(1, 2), IntPos(4, 6));
        assert_eq!(IntPos(3, 4) - IntPos(1, 2), IntPos(2, 2));
        // a size can be added to a position
        assert_eq!(IntPos(3, 4) + IntSize(1, 2), IntPos(4, 6));
        assert_eq!(IntPos(3, 4) - IntSize(1, 2), IntPos(2, 2));
    }

    #[test]
    fn test_int_pos_goes_negative() {
        assert_eq!(IntPos(0, 0) - IntPos(5, 7), IntPos(-5, -7));
    }

    #[test]
    fn test_float_pos_arithmetic() {
        assert_eq!(FloatPos(3.0, 4.0) + FloatPos(1.0, 2.0), FloatPos(4.0, 6.0));
        assert_eq!(FloatPos(3.0, 4.0) - FloatPos(1.0, 2.0), FloatPos(2.0, 2.0));
        assert_eq!(FloatPos(3.0, 4.0) + FloatSize(1.0, 2.0), FloatPos(4.0, 6.0));
        assert_eq!(FloatPos(3.0, 4.0) - FloatSize(1.0, 2.0), FloatPos(2.0, 2.0));
    }

    #[test]
    fn test_size_arithmetic() {
        assert_eq!(IntSize(3, 4) + IntSize(1, 2), IntSize(4, 6));
        assert_eq!(IntSize(3, 4) - IntSize(1, 2), IntSize(2, 2));
        assert_eq!(FloatSize(3.0, 4.0) + FloatSize(1.0, 2.0), FloatSize(4.0, 6.0));
    }

    /// Float positions compare with a tolerance rather than exactly, so values that differ
    /// far below a pixel are the same position.
    #[test]
    fn test_float_pos_equality_is_approximate() {
        assert_eq!(FloatPos(1.0, 1.0), FloatPos(1.000_01, 1.000_01));
        assert_ne!(FloatPos(1.0, 1.0), FloatPos(1.01, 1.0));

        assert_eq!(FloatSize(1.0, 1.0), FloatSize(1.000_01, 1.000_01));
        assert_ne!(FloatSize(1.0, 1.0), FloatSize(1.01, 1.0));
    }

    #[test]
    fn test_position_conversions_truncate() {
        assert_eq!(IntPos::from(FloatPos(3.9, -1.9)), IntPos(3, -1));
        assert_eq!(FloatPos::from(IntPos(3, -1)), FloatPos(3.0, -1.0));
        assert_eq!(IntSize::from(FloatSize(3.9, 1.2)), IntSize(3, 1));
        assert_eq!(FloatSize::from(IntSize(3, 1)), FloatSize(3.0, 1.0));
    }

    // --- Rect ---

    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(100.0, 50.0));

        assert!(rect.contains(FloatPos(50.0, 40.0)), "a point inside should be contained");
        // the bounds are inclusive on both edges
        assert!(rect.contains(FloatPos(10.0, 20.0)));
        assert!(rect.contains(FloatPos(110.0, 70.0)));

        assert!(!rect.contains(FloatPos(9.0, 40.0)));
        assert!(!rect.contains(FloatPos(111.0, 40.0)));
        assert!(!rect.contains(FloatPos(50.0, 19.0)));
        assert!(!rect.contains(FloatPos(50.0, 71.0)));
    }

    #[test]
    fn test_zero_sized_rect_contains_only_its_corner() {
        let rect = Rect::new(FloatPos(5.0, 5.0), FloatSize(0.0, 0.0));
        assert!(rect.contains(FloatPos(5.0, 5.0)));
        assert!(!rect.contains(FloatPos(5.1, 5.0)));
    }

    // --- Surface ---

    #[test]
    fn test_new_surface_is_transparent() {
        let surface = Surface::new(IntSize(4, 3));
        assert_eq!(surface.get_size(), IntSize(4, 3));

        for x in 0..4 {
            for y in 0..3 {
                assert_eq!(*surface.get_pixel(IntPos(x, y)).unwrap(), Color::new(0, 0, 0, 0));
            }
        }
    }

    #[test]
    fn test_surface_pixel_out_of_bounds() {
        let mut surface = Surface::new(IntSize(4, 3));

        surface.get_pixel(IntPos(4, 0)).unwrap_err();
        surface.get_pixel(IntPos(0, 3)).unwrap_err();
        surface.get_pixel(IntPos(-1, 0)).unwrap_err();
        surface.get_pixel(IntPos(0, -1)).unwrap_err();
        surface.get_pixel_mut(IntPos(9, 9)).unwrap_err();
    }

    /// Pixels are stored row by row, so writing one does not disturb its neighbours.
    #[test]
    fn test_surface_set_pixel() {
        let mut surface = Surface::new(IntSize(4, 3));
        *surface.get_pixel_mut(IntPos(1, 2)).unwrap() = Color::new(1, 2, 3, 4);

        assert_eq!(*surface.get_pixel(IntPos(1, 2)).unwrap(), Color::new(1, 2, 3, 4));
        assert_eq!(*surface.get_pixel(IntPos(2, 2)).unwrap(), Color::new(0, 0, 0, 0));
        assert_eq!(*surface.get_pixel(IntPos(1, 1)).unwrap(), Color::new(0, 0, 0, 0));
    }

    #[test]
    fn test_surface_iterates_every_pixel_row_by_row() {
        let surface = Surface::new(IntSize(3, 2));
        let visited: Vec<IntPos> = surface.iter().map(|(pos, _)| pos).collect();

        assert_eq!(visited, vec![IntPos(0, 0), IntPos(1, 0), IntPos(2, 0), IntPos(0, 1), IntPos(1, 1), IntPos(2, 1)]);
    }

    /// `draw` copies a surface in at an offset, multiplying by the given colour.
    #[test]
    fn test_surface_draw_copies_and_tints() {
        let mut target = Surface::new(IntSize(4, 4));
        let mut source = Surface::new(IntSize(2, 2));
        for (_pos, color) in source.iter_mut() {
            *color = Color::new(200, 100, 50, 255);
        }

        // white leaves the colours alone
        target.draw(IntPos(1, 1), &source, Color::new(255, 255, 255, 255)).unwrap();
        assert_eq!(*target.get_pixel(IntPos(1, 1)).unwrap(), Color::new(200, 100, 50, 255));
        assert_eq!(*target.get_pixel(IntPos(2, 2)).unwrap(), Color::new(200, 100, 50, 255));
        // and does not touch pixels outside the copied area
        assert_eq!(*target.get_pixel(IntPos(0, 0)).unwrap(), Color::new(0, 0, 0, 0));
        assert_eq!(*target.get_pixel(IntPos(3, 3)).unwrap(), Color::new(0, 0, 0, 0));
    }

    #[test]
    fn test_surface_draw_out_of_bounds_is_an_error() {
        let mut target = Surface::new(IntSize(2, 2));
        let source = Surface::new(IntSize(2, 2));

        target.draw(IntPos(1, 1), &source, Color::new(255, 255, 255, 255)).unwrap_err();
    }

    /// This round trip is the `.opa` format, which the build script writes and the game
    /// reads back through `include_bytes!`.
    #[test]
    fn test_surface_serialize_round_trip() {
        let mut surface = Surface::new(IntSize(5, 4));
        *surface.get_pixel_mut(IntPos(0, 0)).unwrap() = Color::new(255, 0, 0, 255);
        *surface.get_pixel_mut(IntPos(4, 3)).unwrap() = Color::new(0, 255, 128, 64);

        let bytes = surface.serialize_to_bytes().unwrap();
        let restored = Surface::deserialize_from_bytes(&bytes).unwrap();

        assert_eq!(restored.get_size(), surface.get_size());
        for (pos, color) in surface.iter() {
            assert_eq!(*restored.get_pixel(pos).unwrap(), *color, "pixel {pos:?} differs");
        }
    }

    #[test]
    fn test_surface_deserialize_rejects_garbage() {
        assert!(Surface::deserialize_from_bytes(&[1, 2, 3, 4, 5]).is_err());
    }

    // --- Transformation ---

    #[test]
    fn test_transformation_starts_as_identity() {
        let transform = Transformation::new();
        let identity = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (actual, expected) in transform.matrix.iter().zip(identity.iter()) {
            assert!((actual - expected).abs() < f32::EPSILON, "expected identity, got {:?}", transform.matrix);
        }
    }

    #[test]
    fn test_transformation_translate() {
        let mut transform = Transformation::new();
        transform.translate(FloatPos(3.0, 5.0));

        assert!((transform.matrix[6] - 3.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_transformation_stretch() {
        let mut transform = Transformation::new();
        transform.stretch((2.0, 3.0));

        assert!((transform.matrix[0] - 2.0).abs() < f32::EPSILON);
        assert!((transform.matrix[4] - 3.0).abs() < f32::EPSILON);
    }

    /// Stretching after translating scales the axes but leaves the existing offset alone,
    /// which is the order `Rect::render` relies on.
    #[test]
    fn test_transformation_translate_then_stretch() {
        let mut transform = Transformation::new();
        transform.translate(FloatPos(10.0, 20.0));
        transform.stretch((2.0, 4.0));

        assert!((transform.matrix[6] - 10.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 20.0).abs() < f32::EPSILON);
        assert!((transform.matrix[0] - 2.0).abs() < f32::EPSILON);
        assert!((transform.matrix[4] - 4.0).abs() < f32::EPSILON);
    }

    /// Translating after stretching moves in the already scaled space.
    #[test]
    fn test_transformation_stretch_then_translate() {
        let mut transform = Transformation::new();
        transform.stretch((2.0, 4.0));
        transform.translate(FloatPos(10.0, 20.0));

        assert!((transform.matrix[6] - 20.0).abs() < f32::EPSILON);
        assert!((transform.matrix[7] - 80.0).abs() < f32::EPSILON);
    }
}
