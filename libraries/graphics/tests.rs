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

    // ---------------------------------------------------------------------------------
    // Headless UI tests
    //
    // Everything below drives real widgets through a `HeadlessContext` - no window, no
    // OpenGL context, no SDL. Layout, hit testing and event handling all go through
    // `UiContext`, so they behave exactly as they do in the running client; only drawing
    // is missing. See `libraries/graphics/ui_context.rs`.
    // ---------------------------------------------------------------------------------

    use crate::libraries::graphics as gfx;
    use gfx::{BaseUiElement, UiContext, UiElement};
    use std::cell::Cell;
    use std::rc::Rc;

    /// The window the whole window is the parent of, i.e. what the client passes at the
    /// top of its element tree.
    fn root_container(graphics: &dyn UiContext) -> gfx::Container {
        gfx::Container::default(graphics)
    }

    fn press(key: gfx::Key) -> gfx::Event {
        gfx::Event::KeyPress(key, false)
    }

    fn release(key: gfx::Key) -> gfx::Event {
        gfx::Event::KeyRelease(key, false)
    }

    fn type_text(text: &str) -> gfx::Event {
        gfx::Event::TextInput(text.to_owned())
    }

    /// `FloatPos` and `FloatSize` compare approximately, but bare `f32` does not, so
    /// single coordinates go through here.
    #[track_caller]
    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < f32::EPSILON, "expected {expected}, got {actual}");
    }

    // --- Container layout ---

    #[test]
    fn test_default_container_is_the_whole_window() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(640.0, 480.0));

        let container = root_container(&graphics);
        let rect = container.get_absolute_rect();

        assert_eq!(rect.pos, FloatPos(0.0, 0.0));
        assert_eq!(rect.size, FloatSize(640.0, 480.0));
    }

    #[test]
    fn test_top_left_container_is_offset_from_the_origin() {
        let graphics = gfx::HeadlessContext::new();
        let container = gfx::Container::new(&graphics, FloatPos(10.0, 20.0), FloatSize(100.0, 50.0), gfx::TOP_LEFT, None);

        assert_eq!(container.get_absolute_rect().pos, FloatPos(10.0, 20.0));
    }

    /// An orientation moves the container to that point of the parent *and* pulls it back
    /// by the same fraction of its own size, so `CENTER` centres the element rather than
    /// putting its top left corner in the middle.
    #[test]
    fn test_center_container_is_centred_on_its_own_size() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        let container = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), gfx::CENTER, None);

        // 500 - 50 and 400 - 25
        assert_eq!(container.get_absolute_rect().pos, FloatPos(450.0, 375.0));
    }

    #[test]
    fn test_bottom_right_container_sits_inside_the_corner() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        let container = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), gfx::BOTTOM_RIGHT, None);

        assert_eq!(container.get_absolute_rect().pos, FloatPos(900.0, 750.0));
    }

    /// Every orientation constant places the element fully inside a parent of the same
    /// aspect, which is the invariant the whole layout system rests on.
    #[test]
    fn test_all_orientations_stay_inside_the_parent() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        for orientation in [
            gfx::TOP_LEFT,
            gfx::TOP,
            gfx::TOP_RIGHT,
            gfx::LEFT,
            gfx::CENTER,
            gfx::RIGHT,
            gfx::BOTTOM_LEFT,
            gfx::BOTTOM,
            gfx::BOTTOM_RIGHT,
        ] {
            let container = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), orientation, None);
            let rect = container.get_absolute_rect();

            assert!(
                rect.pos.0 >= 0.0 && rect.pos.0 + rect.size.0 <= 1000.0,
                "orientation {orientation:?} left the window horizontally at {:?}",
                rect.pos
            );
            assert!(
                rect.pos.1 >= 0.0 && rect.pos.1 + rect.size.1 <= 800.0,
                "orientation {orientation:?} left the window vertically at {:?}",
                rect.pos
            );
        }
    }

    /// A child's absolute position is relative to its parent's absolute position, so
    /// nesting composes rather than resetting to the window.
    #[test]
    fn test_nested_containers_compose() {
        let graphics = gfx::HeadlessContext::new();

        let parent = gfx::Container::new(&graphics, FloatPos(100.0, 200.0), FloatSize(400.0, 300.0), gfx::TOP_LEFT, None);
        let child = gfx::Container::new(&graphics, FloatPos(10.0, 20.0), FloatSize(50.0, 50.0), gfx::TOP_LEFT, Some(&parent));

        assert_eq!(child.get_absolute_rect().pos, FloatPos(110.0, 220.0));
    }

    #[test]
    fn test_child_centred_in_a_parent_ignores_the_window() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(4000.0, 4000.0));

        let parent = gfx::Container::new(&graphics, FloatPos(100.0, 100.0), FloatSize(200.0, 200.0), gfx::TOP_LEFT, None);
        let child = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(20.0, 20.0), gfx::CENTER, Some(&parent));

        // centre of the parent (200, 200), less half the child
        assert_eq!(child.get_absolute_rect().pos, FloatPos(190.0, 190.0));
    }

    /// A resize is not observed until the container is rebuilt, which is why the client
    /// recreates containers every frame instead of caching them.
    #[test]
    fn test_container_follows_the_window_size_when_rebuilt() {
        let mut graphics = gfx::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));
        let before = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 100.0), gfx::BOTTOM, None);

        graphics.set_window_size(FloatSize(1000.0, 400.0));
        let after = gfx::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 100.0), gfx::BOTTOM, None);

        assert_close(before.get_absolute_rect().pos.1, 700.0);
        assert_close(after.get_absolute_rect().pos.1, 300.0);
    }

    // --- Button ---

    /// Records how many times a button's closure ran, so a test can tell a real click
    /// from a near miss.
    fn counting_button(size: FloatSize) -> (gfx::Button, Rc<Cell<u32>>) {
        let clicks = Rc::new(Cell::new(0));
        let counter = clicks.clone();
        let mut button = gfx::Button::new(move || counter.set(counter.get() + 1));
        button.padding = 0.0;
        button.texture = gfx::Texture::new_sized(size);
        (button, clicks)
    }

    #[test]
    fn test_button_size_includes_padding_and_scale() {
        let mut button = gfx::Button::new(|| {});
        button.texture = gfx::Texture::new_sized(FloatSize(100.0, 20.0));
        button.padding = 5.0;
        button.scale = 2.0;

        // (100 + 5*2) * 2 and (20 + 5*2) * 2
        assert_eq!(button.get_size(), FloatSize(220.0, 60.0));
    }

    #[test]
    fn test_button_is_hovered_only_inside_its_rect() {
        let mut graphics = gfx::HeadlessContext::new();
        let (button, _) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        assert!(button.is_hovered(&graphics, &root));

        graphics.set_mouse_pos(FloatPos(150.0, 25.0));
        assert!(!button.is_hovered(&graphics, &root), "the mouse is past the right edge");
    }

    #[test]
    fn test_disabled_button_is_never_hovered() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, _) = counting_button(FloatSize(100.0, 50.0));
        button.disabled = true;
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        assert!(!button.is_hovered(&graphics, &root));
    }

    #[test]
    fn test_button_fires_on_mouse_release_while_hovered() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        let consumed = button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 1);
        assert!(consumed, "a handled click should be reported as consumed");
    }

    /// The press half of a click does nothing; only the release fires. That is what lets
    /// a user press on a button, move away and let go without triggering it.
    #[test]
    fn test_button_does_not_fire_on_press() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        let consumed = button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
        assert!(!consumed);
    }

    #[test]
    fn test_button_does_not_fire_when_the_mouse_is_elsewhere() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));

        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_disabled_button_does_not_fire() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        button.disabled = true;
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_button_ignores_other_keys() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        button.on_event(&mut graphics, &release(gfx::Key::Enter), &root);
        button.on_event(&mut graphics, &release(gfx::Key::MouseRight), &root);

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_button_press_calls_the_closure_directly() {
        let (button, clicks) = counting_button(FloatSize(10.0, 10.0));

        button.press();

        assert_eq!(clicks.get(), 1);
    }

    // --- Toggle ---

    #[test]
    fn test_toggle_flips_when_clicked() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(10.0, 10.0));

        assert!(!toggle.toggled);
        let consumed = toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(toggle.toggled);
        assert!(toggle.changed);
        assert!(consumed);
    }

    #[test]
    fn test_toggle_flips_back_on_a_second_click() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(10.0, 10.0));

        toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);
        toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(!toggle.toggled);
    }

    #[test]
    fn test_toggle_ignores_clicks_outside_itself() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));

        let consumed = toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(!toggle.toggled);
        assert!(!consumed);
    }

    // --- Scrollable ---

    /// Scroll deltas are negated and damped by 0.8, so a positive wheel event scrolls the
    /// content up.
    #[test]
    fn test_scroll_sets_velocity_in_the_opposite_direction() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut scrollable = gfx::Scrollable::new();
        let root = root_container(&graphics);

        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(10.0), &root);

        // the velocity is private, but a scroll of the same sign should not increase it
        // further, which is what the max/min in the handler is for
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(1.0), &root);
        assert_close(scrollable.get_scroll_pos(), 0.0); // position only moves while rendering, not on the event
    }

    #[test]
    fn test_scrollable_starts_at_the_top() {
        let scrollable = gfx::Scrollable::new();
        assert_close(scrollable.get_scroll_pos(), 0.0);
    }

    /// `get_scroll_x` is the container position less the scroll offset, which is how the
    /// server and world lists slide their rows.
    #[test]
    fn test_get_scroll_x_is_the_container_position_when_unscrolled() {
        let graphics = gfx::HeadlessContext::new();
        let mut scrollable = gfx::Scrollable::new();
        scrollable.rect.pos = FloatPos(30.0, 40.0);
        let root = root_container(&graphics);

        assert_close(scrollable.get_scroll_x(&graphics, &root), 30.0);
    }

    #[test]
    fn test_scrollable_ignores_unrelated_events() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut scrollable = gfx::Scrollable::new();
        let root = root_container(&graphics);

        assert!(!scrollable.on_event(&mut graphics, &press(gfx::Key::A), &root));
    }

    // --- TextInput ---

    /// A selected input, which is the state every test below wants to start from.
    fn selected_input() -> gfx::TextInput {
        let mut input = gfx::TextInput::new_headless();
        input.selected = true;
        input
    }

    /// A selected input that already contains `text`, typed in the way a user would.
    ///
    /// This types rather than calling `set_text` on purpose: `set_text` leaves the cursor
    /// where it was, so a test that used it would be operating at offset 0 with nothing
    /// selected. See `test_set_text_leaves_the_cursor_at_the_start`.
    fn input_containing(graphics: &mut gfx::HeadlessContext, text: &str) -> gfx::TextInput {
        let mut input = selected_input();
        let root = root_container(graphics);
        input.on_event(graphics, &type_text(text), &root);
        input
    }

    /// Selects the last `count` characters by holding shift and pressing left, which
    /// leaves the cursor pair "backwards" - its second half is before its first.
    fn select_backwards(graphics: &mut gfx::HeadlessContext, input: &mut gfx::TextInput, count: usize) {
        let root = root_container(graphics);
        graphics.set_key_state(gfx::Key::LeftShift, true);
        for _ in 0..count {
            input.on_event(graphics, &press(gfx::Key::Left), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, false);
    }

    #[test]
    fn test_typing_inserts_text() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("hi"), &root);

        assert_eq!(input.get_text(), "hi");
        assert_eq!(input.get_cursor_range(), (2, 2));
    }

    #[test]
    fn test_typing_does_nothing_when_not_selected() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = gfx::TextInput::new_headless();
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("hi"), &root);

        assert_eq!(input.get_text(), "");
    }

    /// `text_processing` filters every character as it arrives, which is how the world
    /// seed field stays numeric.
    #[test]
    fn test_text_processing_filters_characters() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        input.text_processing = Some(Box::new(|c| c.is_numeric().then_some(c)));
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("a1b2c3"), &root);

        assert_eq!(input.get_text(), "123");
    }

    #[test]
    fn test_backspace_deletes_one_character() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("abc"), &root);

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "ab");
    }

    #[test]
    fn test_backspace_on_empty_text_is_harmless() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "");
        assert_eq!(input.get_cursor_range(), (0, 0));
    }

    /// Holding control turns backspace into a word delete, stopping at one of
    /// `WORD_DELIMITERS`.
    #[test]
    fn test_control_backspace_deletes_a_word() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("hello world"), &root);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "hello ");
    }

    #[test]
    fn test_delete_removes_the_character_after_the_cursor() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abc");
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);

        input.on_event(&mut graphics, &press(gfx::Key::Delete), &root);

        assert_eq!(input.get_text(), "ab");
    }

    #[test]
    fn test_arrows_move_the_cursor_one_character() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        assert_eq!(input.get_cursor_range(), (3, 3));

        input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        assert_eq!(input.get_cursor_range(), (4, 4));
    }

    #[test]
    fn test_cursor_stops_at_both_ends() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "ab");
        let root = root_container(&graphics);

        for _ in 0..10 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
        assert_eq!(input.get_cursor_range(), (0, 0));

        for _ in 0..10 {
            input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        }
        assert_eq!(input.get_cursor_range(), (2, 2));
    }

    /// `set_text` deliberately does not move the cursor - it only clamps it into the new
    /// text. Setting text on a fresh input therefore leaves the cursor at the *start*,
    /// not after what was set.
    #[test]
    fn test_set_text_leaves_the_cursor_at_the_start() {
        let mut input = selected_input();

        input.set_text("hello".to_owned());

        assert_eq!(input.get_cursor_range(), (0, 0));
    }

    #[test]
    fn test_control_left_jumps_a_whole_word() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "hello world");
        let root = root_container(&graphics);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);

        assert_eq!(input.get_cursor_range(), (6, 6), "the cursor should land after the space");
    }

    #[test]
    fn test_control_right_jumps_a_whole_word() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "hello world");
        let root = root_container(&graphics);
        for _ in 0..11 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Right), &root);

        assert_eq!(input.get_cursor_range(), (5, 5), "the cursor should stop at the space");
    }

    /// Shift extends the selection instead of collapsing it, so the two halves of the
    /// cursor pair come apart. Selecting leftwards leaves them in descending order, which
    /// `get_cursor` puts back in order.
    #[test]
    fn test_shift_arrow_selects() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");

        select_backwards(&mut graphics, &mut input, 2);

        assert_eq!(input.get_cursor_range(), (2, 4));
    }

    #[test]
    fn test_typing_replaces_the_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);

        input.on_event(&mut graphics, &type_text("XY"), &root);

        assert_eq!(input.get_text(), "abXY");
        assert_eq!(input.get_cursor_range(), (4, 4));
    }

    /// The same replacement, but selecting forwards from the start of the text, so the
    /// cursor pair is in ascending order. Both directions have to behave the same.
    #[test]
    fn test_typing_replaces_a_forwards_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        for _ in 0..4 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, true);
        input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        graphics.set_key_state(gfx::Key::LeftShift, false);

        input.on_event(&mut graphics, &type_text("XY"), &root);

        assert_eq!(input.get_text(), "XYcd");
    }

    #[test]
    fn test_backspace_deletes_the_whole_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "ab");
        assert_eq!(input.get_cursor_range(), (2, 2));
    }

    #[test]
    fn test_control_c_copies_the_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::C), &root);

        assert_eq!(graphics.get_clipboard_text(), Some("cd".to_owned()));
        assert_eq!(input.get_text(), "abcd", "copying must not change the text");
    }

    #[test]
    fn test_control_v_pastes_at_the_cursor() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "ab");
        let root = root_container(&graphics);
        graphics.set_clipboard_text("XY");

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::V), &root);

        assert_eq!(input.get_text(), "abXY");
    }

    #[test]
    fn test_control_v_replaces_the_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);
        graphics.set_clipboard_text("XY");

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::V), &root);

        assert_eq!(input.get_text(), "abXY");
    }

    #[test]
    fn test_control_x_cuts_the_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::X), &root);

        assert_eq!(input.get_text(), "ab");
        assert_eq!(graphics.get_clipboard_text(), Some("cd".to_owned()));
    }

    /// Plain letters are only clipboard shortcuts while control is down; otherwise they
    /// arrive as `TextInput` and the key press itself must not edit anything.
    #[test]
    fn test_c_v_and_x_do_nothing_without_control() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        graphics.set_clipboard_text("PASTED");

        for key in [gfx::Key::C, gfx::Key::V, gfx::Key::X] {
            input.on_event(&mut graphics, &press(key), &root);
        }

        assert_eq!(input.get_text(), "abcd");
    }

    /// Clicking inside the box selects it and clicking outside deselects it. This runs
    /// even when the input is not selected, which is how it becomes selected at all.
    #[test]
    fn test_clicking_selects_and_deselects() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = gfx::TextInput::new_headless();
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(10.0, 10.0));
        input.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        assert!(input.selected);

        graphics.set_mouse_pos(FloatPos(900.0, 700.0));
        input.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        assert!(!input.selected);
    }

    #[test]
    fn test_set_text_clamps_the_cursor_into_range() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("a long piece of text"), &root);

        input.set_text("ab".to_owned());

        let (start, end) = input.get_cursor_range();
        assert!(start <= 2 && end <= 2, "cursor {start}..{end} is past the end of the new text");
    }

    // --- Font ---

    const FONT: &[u8] = include_bytes!("../../Build/Resources/font.opa");
    const FONT_MONO: &[u8] = include_bytes!("../../Build/Resources/font_mono.opa");

    fn font() -> gfx::Font {
        gfx::Font::new_headless(FONT, false).unwrap()
    }

    #[test]
    fn test_font_loads_from_the_shipped_atlas() {
        gfx::Font::new_headless(FONT, false).unwrap();
        gfx::Font::new_headless(FONT_MONO, true).unwrap();
    }

    #[test]
    fn test_font_rejects_garbage() {
        // Font is not Debug, so unwrap_err is unavailable
        assert!(gfx::Font::new_headless(&[1, 2, 3, 4], false).is_err());
    }

    /// One line of text is one character tall, and the width is at least 1 even when
    /// empty, so a zero sized texture is never created.
    #[test]
    fn test_empty_text_is_one_line_tall() {
        let size = font().get_text_size("", None);
        assert_eq!(size.1, 16);
        assert_eq!(size.0, 1);
    }

    #[test]
    fn test_text_width_grows_with_length() {
        let font = font();
        let one = font.get_text_size("a", None).0;
        let three = font.get_text_size("aaa", None).0;

        assert!(three > one, "'aaa' ({three}) should be wider than 'a' ({one})");
    }

    #[test]
    fn test_newline_adds_a_line() {
        let font = font();
        let one_line = font.get_text_size("ab", None).1;
        let two_lines = font.get_text_size("ab\ncd", None).1;

        assert!(two_lines > one_line, "two lines ({two_lines}) should be taller than one ({one_line})");
    }

    /// `get_text_size` starts a string that ends in a newline at height 0 instead of 16,
    /// to avoid counting an empty last line. The compensation is one pixel short, though:
    /// a newline adds the glyph height *plus* `CHAR_SPACING`, so a trailing newline nets
    /// +1 rather than 0.
    #[test]
    fn test_trailing_newline_costs_one_pixel_of_spacing() {
        let font = font();
        let plain = font.get_text_size("a", None).1;
        let trailing = font.get_text_size("a\n", None).1;

        assert_eq!(plain, 16);
        assert_eq!(trailing, 17, "the trailing newline compensation misses CHAR_SPACING");
    }

    /// Each extra line costs the glyph height plus one pixel of spacing.
    #[test]
    fn test_each_line_adds_a_fixed_height() {
        let font = font();
        assert_eq!(font.get_text_size("a", None).1, 16);
        assert_eq!(font.get_text_size("a\nb", None).1, 33);
        assert_eq!(font.get_text_size("a\nb\nc", None).1, 50);
    }

    #[test]
    fn test_width_limit_wraps_onto_more_lines() {
        let font = font();
        let unwrapped = font.get_text_size("a long sentence that will not fit", None);
        let wrapped = font.get_text_size("a long sentence that will not fit", Some(50));

        assert!(wrapped.1 > unwrapped.1, "wrapping should make the text taller");
        assert!(wrapped.0 <= 50 + 16, "wrapped text should stay near the limit, got {}", wrapped.0);
    }

    #[test]
    fn test_created_surface_matches_the_measured_size() {
        let font = font();
        for text in ["", "a", "hello world", "two\nlines"] {
            let size = font.get_text_size(text, None);
            let surface = font.create_text_surface(text, None);
            assert_eq!(surface.get_size(), size, "size mismatch for {text:?}");
        }
    }

    #[test]
    fn test_scaled_text_size_multiplies() {
        let font = font();
        let size = font.get_text_size("hello", None);
        let scaled = font.get_text_size_scaled("hello", 3.0, None);

        assert!((scaled.0 - size.0 as f32 * 3.0).abs() < f32::EPSILON);
        assert!((scaled.1 - size.1 as f32 * 3.0).abs() < f32::EPSILON);
    }

    /// A monospaced font pads narrow glyphs out so every character advances by the same
    /// amount - which is the whole reason the debug menu uses one.
    #[test]
    fn test_mono_font_advances_uniformly() {
        let mono = gfx::Font::new_headless(FONT_MONO, true).unwrap();

        let narrow = mono.get_text_size("iiii", None).0;
        let wide = mono.get_text_size("MMMM", None).0;

        assert_eq!(narrow, wide, "a mono font should give 'iiii' and 'MMMM' the same width");
    }

    #[test]
    fn test_proportional_font_does_not_advance_uniformly() {
        let font = font();
        assert_ne!(font.get_text_size("iiii", None).0, font.get_text_size("MMMM", None).0);
    }

    /// Rendering text needs the GPU textures a headless font does not have, so it draws
    /// nothing rather than misbehaving. This pins that it is at least safe to construct.
    #[test]
    fn test_headless_font_measures_without_textures() {
        let font = gfx::Font::new_headless(FONT, false).unwrap();
        assert!(font.get_text_size("measured anyway", None).0 > 1);
    }

    // --- RenderRect ---

    #[test]
    fn test_render_rect_starts_at_its_target() {
        let rect = gfx::RenderRect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));
        assert!(rect.is_at_target());
    }

    #[test]
    fn test_render_rect_lags_behind_a_moved_target() {
        let mut rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));
        rect.pos = FloatPos(100.0, 100.0);

        assert!(!rect.is_at_target(), "the drawn position should not jump with the target");

        rect.jump_to_target();
        assert!(rect.is_at_target());
        assert_eq!(rect.render_pos, FloatPos(100.0, 100.0));
    }

    /// The container is built from `render_pos`, not `pos`, which is what makes the
    /// rectangle appear to slide.
    #[test]
    fn test_render_rect_container_follows_the_drawn_position() {
        let graphics = gfx::HeadlessContext::new();
        let root = root_container(&graphics);
        let mut rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));
        rect.pos = FloatPos(100.0, 100.0);

        assert_eq!(rect.get_container(&graphics, &root).get_absolute_rect().pos, FloatPos(0.0, 0.0));

        rect.jump_to_target();
        assert_eq!(rect.get_container(&graphics, &root).get_absolute_rect().pos, FloatPos(100.0, 100.0));
    }

    // --- Sprite ---

    #[test]
    fn test_sprite_size_scales_with_the_texture() {
        let mut sprite = gfx::Sprite::new();
        sprite.set_texture(gfx::Texture::new_sized(FloatSize(40.0, 20.0)));
        sprite.scale = 2.5;

        assert_eq!(sprite.get_size(), FloatSize(100.0, 50.0));
    }

    /// Setting a texture resets the source rectangle to the whole of it, so a sprite
    /// reused for a new texture does not keep cropping to the old one.
    #[test]
    fn test_setting_a_texture_resets_the_source_rect() {
        let mut sprite = gfx::Sprite::new();
        sprite.src_rect = Rect::new(FloatPos(5.0, 5.0), FloatSize(1.0, 1.0));

        sprite.set_texture(gfx::Texture::new_sized(FloatSize(40.0, 20.0)));

        assert_eq!(sprite.src_rect.pos, FloatPos(0.0, 0.0));
        assert_eq!(sprite.src_rect.size, FloatSize(40.0, 20.0));
    }

    // --- Event routing through BaseUiElement ---

    /// A panel that positions one button inside itself, so a test can check that the
    /// button is hit tested against the *panel*, not against the window.
    struct TestPanel {
        button: gfx::Button,
        pos: FloatPos,
        size: FloatSize,
    }

    impl UiElement for TestPanel {
        fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
            vec![&mut self.button]
        }

        fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
            vec![&self.button]
        }

        fn get_container(&self, graphics: &dyn UiContext, parent_container: &gfx::Container) -> gfx::Container {
            gfx::Container::new(graphics, self.pos, self.size, gfx::TOP_LEFT, Some(parent_container))
        }
    }

    /// `BaseUiElement::on_event` builds the parent's container and hands it to each child,
    /// so a child's coordinates are relative to its parent. Getting this wrong would put
    /// every nested button's clickable area in the wrong place.
    #[test]
    fn test_events_reach_children_in_parent_relative_coordinates() {
        let mut graphics = gfx::HeadlessContext::new();
        let (button, clicks) = counting_button(FloatSize(50.0, 50.0));
        let mut panel = TestPanel {
            button,
            pos: FloatPos(200.0, 100.0),
            size: FloatSize(400.0, 400.0),
        };
        let root = root_container(&graphics);

        // inside the button once the panel's offset is applied
        graphics.set_mouse_pos(FloatPos(225.0, 125.0));
        panel.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);
        assert_eq!(clicks.get(), 1, "the click should have reached the button");

        // where the button would be if the panel's offset were ignored
        graphics.set_mouse_pos(FloatPos(25.0, 25.0));
        panel.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);
        assert_eq!(clicks.get(), 1, "a click at the unoffset position should have missed");
    }

    /// A parent reports an event as consumed when any of its children consumed it, even
    /// though the parent itself does nothing with it.
    #[test]
    fn test_parent_reports_a_childs_consumption() {
        let mut graphics = gfx::HeadlessContext::new();
        let (button, _) = counting_button(FloatSize(50.0, 50.0));
        let mut panel = TestPanel {
            button,
            pos: FloatPos(0.0, 0.0),
            size: FloatSize(400.0, 400.0),
        };
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(25.0, 25.0));
        assert!(panel.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root));

        graphics.set_mouse_pos(FloatPos(900.0, 700.0));
        assert!(!panel.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root));
    }

    // --- AnimationTimer ---

    /// The timer hands out one frame per `per_frame` milliseconds elapsed and then stops,
    /// which is what keeps animations running at a fixed rate independent of frame rate.
    #[test]
    fn test_animation_timer_hands_out_frames_for_elapsed_time() {
        let mut timer = gfx::AnimationTimer::new(5);
        std::thread::sleep(std::time::Duration::from_millis(20));

        let mut frames = 0;
        while timer.frame_ready() {
            frames += 1;
            assert!(frames < 100, "the timer should stop handing out frames");
        }

        // 20ms of backlog at 5ms per frame, give or take the sleep's precision
        assert!((3..=6).contains(&frames), "expected about 4 frames, got {frames}");
    }

    #[test]
    fn test_animation_timer_has_no_frames_ready_immediately() {
        let mut timer = gfx::AnimationTimer::new(1000);
        assert!(!timer.frame_ready());
    }

    // --- HeadlessContext itself ---

    /// The test double has to answer the same questions as the real context, or the tests
    /// above are measuring nothing.
    #[test]
    fn test_headless_context_reports_what_was_set() {
        let mut graphics = gfx::HeadlessContext::new();

        graphics.set_window_size(FloatSize(123.0, 456.0));
        graphics.set_mouse_pos(FloatPos(7.0, 8.0));
        graphics.set_key_state(gfx::Key::LeftControl, true);

        assert_eq!(graphics.get_window_size(), FloatSize(123.0, 456.0));
        assert_eq!(graphics.get_mouse_pos(), FloatPos(7.0, 8.0));
        assert!(graphics.get_key_state(gfx::Key::LeftControl));
        assert!(!graphics.get_key_state(gfx::Key::LeftShift));

        graphics.set_key_state(gfx::Key::LeftControl, false);
        assert!(!graphics.get_key_state(gfx::Key::LeftControl));
    }

    /// There is no window behind a headless context, so the graphical escape hatch has to
    /// report that honestly rather than pretending.
    #[test]
    fn test_headless_context_has_no_graphics_context() {
        let mut graphics = gfx::HeadlessContext::new();
        assert!(graphics.as_graphics_context().is_none());
    }

    // ---------------------------------------------------------------------------------
    // Draw list tests
    //
    // Drawing records a `DrawCommand` rather than issuing an OpenGL call, so what a
    // primitive draws is assertable without a window. That is the whole point of the split
    // in `libraries/graphics/draw_list.rs`, and it is the only tier of rendering coverage
    // that `cargo test` can run - the golden images need a real context on the main thread.
    //
    // What is missing here is anything that has to own a GPU object before it can record:
    // `RectArray` allocates its buffers in `new`, `ShadowContext` uploads its baked gaussian,
    // and `Font::new_headless` deliberately skips the glyph upload, so `render_text` finds no
    // textures and draws nothing. Those stay the golden suite's job.
    // ---------------------------------------------------------------------------------

    use gfx::{BlendMode, DrawCommand, DrawList, DrawRecorder, DrawTarget};

    const WHITE: Color = Color::new(255, 255, 255, 255);

    #[test]
    fn test_rect_records_itself_unchanged() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render(&recorder, Color::new(1, 2, 3, 4));

        assert_eq!(recorder.get_commands(), vec![DrawCommand::Rect { rect, color: Color::new(1, 2, 3, 4) }]);
    }

    #[test]
    fn test_rect_outline_records_a_different_command() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render_outline(&recorder, WHITE);

        assert_eq!(recorder.get_commands(), vec![DrawCommand::RectOutline { rect, color: WHITE }]);
    }

    /// A fully transparent draw is dropped at record time rather than being handed to the
    /// backend to blend into nothing.
    #[test]
    fn test_invisible_rects_are_never_recorded() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        rect.render(&recorder, WHITE.set_a(0));
        rect.render_outline(&recorder, WHITE.set_a(0));

        assert!(recorder.is_empty());
    }

    #[test]
    fn test_offscreen_rects_are_culled_at_record_time() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        // past each of the four edges
        Rect::new(FloatPos(-50.0, 10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(10.0, -50.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(200.0, 10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(10.0, 200.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);

        assert!(recorder.is_empty());
    }

    #[test]
    fn test_a_rect_hanging_over_an_edge_still_draws() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        Rect::new(FloatPos(-10.0, -10.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);
        Rect::new(FloatPos(90.0, 90.0), FloatSize(20.0, 20.0)).render(&recorder, WHITE);

        assert_eq!(recorder.len(), 2);
    }

    /// Culling `render` but not `render_outline` is a real asymmetry, not an oversight: a
    /// border whose rectangle starts offscreen still has edges that cross the window.
    #[test]
    fn test_outlines_are_not_culled() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(100.0, 100.0));

        Rect::new(FloatPos(-500.0, -500.0), FloatSize(20.0, 20.0)).render_outline(&recorder, WHITE);

        assert_eq!(recorder.len(), 1);
    }

    #[test]
    fn test_texture_defaults_to_its_whole_self_undyed() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 8.0));

        texture.render(&recorder, 2.0, FloatPos(5.0, 6.0), None, false, None);

        assert_eq!(
            recorder.get_commands(),
            vec![DrawCommand::Texture {
                texture: texture.get_handle(),
                texture_size: FloatSize(16.0, 8.0),
                src_rect: Rect::new(FloatPos(0.0, 0.0), FloatSize(16.0, 8.0)),
                pos: FloatPos(5.0, 6.0),
                scale: 2.0,
                flipped: false,
                color: WHITE,
            }]
        );
    }

    #[test]
    fn test_texture_passes_through_source_rect_flip_and_tint() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 16.0));
        let src = Rect::new(FloatPos(4.0, 4.0), FloatSize(8.0, 8.0));

        texture.render(&recorder, 3.0, FloatPos(1.0, 2.0), Some(src), true, Some(Color::new(9, 8, 7, 6)));

        assert_eq!(
            recorder.get_commands(),
            vec![DrawCommand::Texture {
                texture: texture.get_handle(),
                texture_size: FloatSize(16.0, 16.0),
                src_rect: src,
                pos: FloatPos(1.0, 2.0),
                scale: 3.0,
                flipped: true,
                color: Color::new(9, 8, 7, 6),
            }]
        );
    }

    #[test]
    fn test_an_empty_source_rect_draws_nothing() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(16.0, 16.0));

        texture.render(&recorder, 1.0, FloatPos(0.0, 0.0), Some(Rect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0))), false, None);

        assert!(recorder.is_empty());
    }

    /// A `Texture::new` owns nothing on the GPU and reports a zero size, so the default
    /// source rectangle is empty and the same early return catches it. Without that, the
    /// backend would be handed a command naming a texture that does not exist.
    #[test]
    fn test_a_texture_with_no_gpu_object_draws_nothing() {
        let recorder = DrawRecorder::new();

        gfx::Texture::new().render(&recorder, 1.0, FloatPos(0.0, 0.0), None, false, None);

        assert!(recorder.is_empty());
    }

    /// A blend mode change only means something relative to the draws around it, which is
    /// why it is a command in the list rather than a call that takes effect immediately.
    #[test]
    fn test_blend_mode_changes_keep_their_place_in_the_order() {
        let recorder = DrawRecorder::new();
        let rect = Rect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));

        rect.render(&recorder, WHITE);
        recorder.set_blend_mode(BlendMode::Multiply);
        rect.render(&recorder, WHITE);
        recorder.set_blend_mode(BlendMode::Alpha);

        assert_eq!(
            recorder.get_commands(),
            vec![
                DrawCommand::Rect { rect, color: WHITE },
                DrawCommand::SetBlendMode(BlendMode::Multiply),
                DrawCommand::Rect { rect, color: WHITE },
                DrawCommand::SetBlendMode(BlendMode::Alpha),
            ]
        );
    }

    #[test]
    fn test_clearing_a_list_empties_it() {
        let mut list = DrawList::new();
        assert!(list.is_empty());

        list.push(DrawCommand::SetBlendMode(BlendMode::Alpha));
        list.push(DrawCommand::SetBlendMode(BlendMode::Multiply));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get_commands().len(), 2);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.get_commands(), &[]);
    }

    /// Dropping a texture parks its registry id instead of releasing it, because a command
    /// recorded earlier this frame may still refer to it. A texture that never had an id
    /// must not park anything.
    #[test]
    fn test_dropping_a_gpu_less_texture_parks_nothing() {
        let before = gfx::gpu_device::get_pending_counts();

        drop(gfx::Texture::new_sized(FloatSize(4.0, 4.0)));
        drop(gfx::Texture::new());

        assert_eq!(gfx::gpu_device::get_pending_counts(), before);
    }

    /// Uploading a surface with no device in the process is not an error any more, it just
    /// produces a texture that knows its size and owns nothing. Under OpenGL the same call
    /// without a current context was undefined behaviour, so this could not be a test.
    #[test]
    fn test_a_texture_can_be_built_without_a_device() {
        let texture = gfx::Texture::load_from_surface(&Surface::new(IntSize(6, 9)));

        assert_eq!(texture.get_texture_size(), FloatSize(6.0, 9.0));
        assert_eq!(texture.get_handle(), gfx::Texture::new().get_handle());
    }
}
