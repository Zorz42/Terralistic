#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![allow(clippy::panic)] // a test asserting on the shape of a value has nothing else to say when it is the wrong shape
#![cfg(test)]
mod tests {
    use crate::libraries::graphics::transformation::Transformation;
    use crate::libraries::graphics::{interpolate_colors, Color, FloatPos, FloatSize, IntPos, IntSize, Rect, Surface};

    // --- Color ---

    #[test]
    fn test_set_a_leaves_the_colour_it_was_called_on_alone() {
        let color = Color::new(1, 2, 3, 4);

        assert_eq!(color.set_a(40), Color::new(1, 2, 3, 40));
        // it takes self by value, so the original is untouched
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

    /// A surface's serialized form is its pixels and its size, with nothing tying the two
    /// together, so the bytes can perfectly well describe a 64x64 image holding four pixels.
    ///
    /// Everything downstream takes `get_size` at its word, and `GpuDevice::create_texture` in
    /// particular tells wgpu the texture is that big and hands it the short buffer - which is a
    /// validation error, which wgpu turns into a panic ("Copy at offset 0 for 16384 bytes would
    /// end up overrunning the bounds of the Source buffer of size 16"). Surfaces are read out
    /// of `.mod` files, which are ordinary files on disk, so this has to be caught at the door.
    #[test]
    fn test_a_surface_that_lies_about_its_size_is_rejected() {
        /// The same shape as `Surface`, which is all postcard encodes: fields in order, no names.
        #[derive(serde_derive::Serialize)]
        struct MismatchedSurface {
            pixels: Vec<Color>,
            size: IntSize,
        }

        let bytes = crate::libraries::serialization::serialize(&MismatchedSurface {
            pixels: std::vec![Color::new(1, 2, 3, 4); 4],
            size: IntSize(64, 64),
        })
        .unwrap();

        assert!(Surface::deserialize_from_bytes(&snap::raw::Encoder::new().compress_vec(&bytes).unwrap()).is_err());
    }

    /// The other direction, and the reason the check is an equality rather than a lower bound:
    /// a surface with pixels to spare would upload fine and then be indexed by a size that does
    /// not reach them.
    #[test]
    fn test_a_surface_with_pixels_to_spare_is_rejected_too() {
        #[derive(serde_derive::Serialize)]
        struct MismatchedSurface {
            pixels: Vec<Color>,
            size: IntSize,
        }

        let bytes = crate::libraries::serialization::serialize(&MismatchedSurface {
            pixels: std::vec![Color::new(1, 2, 3, 4); 100],
            size: IntSize(2, 2),
        })
        .unwrap();

        assert!(Surface::deserialize_from_bytes(&snap::raw::Encoder::new().compress_vec(&bytes).unwrap()).is_err());
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
    // GPU device, no window. Layout, hit testing and event handling all go through
    // `UiContext`, so they behave exactly as they do in the running client; only drawing
    // is missing. See `libraries/graphics/ui_context.rs`.
    // ---------------------------------------------------------------------------------

    use crate::libraries::graphics as gfx;
    use crate::libraries::graphics::animation_timer::MAX_CATCHUP_FRAMES;
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

        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        let consumed = button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 1);
        assert!(consumed, "a handled click should be reported as consumed");
    }

    /// A click is a press *and* a release on the same button. Only the release used to be
    /// checked, so a press that landed anywhere else - on the menu behind it, or in the menu
    /// this one replaced - activated whatever the pointer was over when it came back up.
    #[test]
    fn test_a_release_the_button_never_saw_the_press_for_does_nothing() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        let consumed = button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
        assert!(!consumed, "a release that completes nothing is not this button's to consume");
    }

    /// The other half of the same rule, and the one it always got right: pressing a button and
    /// dragging off it before letting go is how a user backs out of a click.
    #[test]
    fn test_pressing_a_button_and_letting_go_elsewhere_does_nothing() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));
        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
    }

    /// Backing out of backing out. The press is remembered, not the pointer's whole path.
    #[test]
    fn test_dragging_off_a_pressed_button_and_back_still_fires() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 1);
    }

    /// Several menus hold their buttons as sub-elements *and* dispatch to them again from
    /// `on_event_inner`, so one release reaches a button twice - and the menu reads the second
    /// answer. Remembering the press must not turn into consuming it.
    #[test]
    fn test_a_release_delivered_twice_is_answered_twice() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        assert!(button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root));
        assert!(button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root));

        assert_eq!(clicks.get(), 2);
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

        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
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

        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_button_ignores_other_keys() {
        let mut graphics = gfx::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        for key in [gfx::Key::Enter, gfx::Key::MouseRight] {
            button.on_event(&mut graphics, &press(key), &root);
            button.on_event(&mut graphics, &release(key), &root);
        }

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_button_press_calls_the_closure_directly() {
        let (button, clicks) = counting_button(FloatSize(10.0, 10.0));

        button.press();

        assert_eq!(clicks.get(), 1);
    }

    // --- Toggle ---

    /// A press and a release at the same place, which is what a click is.
    fn click(graphics: &mut gfx::HeadlessContext, toggle: &mut gfx::Toggle, root: &gfx::Container) -> bool {
        toggle.on_event(graphics, &press(gfx::Key::MouseLeft), root);
        toggle.on_event(graphics, &release(gfx::Key::MouseLeft), root)
    }

    #[test]
    fn test_toggle_flips_when_clicked() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(10.0, 10.0));

        assert!(!toggle.toggled);
        let consumed = click(&mut graphics, &mut toggle, &root);

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

        click(&mut graphics, &mut toggle, &root);
        click(&mut graphics, &mut toggle, &root);

        assert!(!toggle.toggled);
    }

    /// The same rule as `Button`, which a `Toggle` used not to follow: only the release was
    /// checked, so a press that landed anywhere else - on the menu behind, or on whatever the
    /// player was actually aiming at - flipped whichever toggle the pointer had wandered onto
    /// by the time the button came back up.
    #[test]
    fn test_a_release_the_toggle_never_saw_the_press_for_does_nothing() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);

        // the press lands somewhere else entirely
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));
        toggle.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        // and the pointer is over the toggle by the time it comes back up
        graphics.set_mouse_pos(FloatPos(10.0, 10.0));
        let consumed = toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(!toggle.toggled);
        assert!(!consumed, "a release that completes nothing is not this toggle's to consume");
    }

    /// And the other half: pressing on a toggle and dragging off it before letting go is how a
    /// user backs out.
    #[test]
    fn test_pressing_a_toggle_and_letting_go_elsewhere_does_nothing() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(10.0, 10.0));
        toggle.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));
        toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(!toggle.toggled);
    }

    #[test]
    fn test_toggle_ignores_clicks_outside_itself() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut toggle = gfx::Toggle::new();
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));

        let consumed = click(&mut graphics, &mut toggle, &root);

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
        assert_close(scrollable.get_scroll_pos(), 0.0); // position only moves while updating, not on the event
    }

    #[test]
    fn test_scrollable_starts_at_the_top() {
        let scrollable = gfx::Scrollable::new();
        assert_close(scrollable.get_scroll_pos(), 0.0);
    }

    /// `get_scroll_y` is the scrollable's own position less the scroll offset, which is how the
    /// server and world lists slide their rows.
    #[test]
    fn test_get_scroll_y_is_the_container_position_when_unscrolled() {
        let mut scrollable = gfx::Scrollable::new();
        scrollable.rect.pos = FloatPos(30.0, 40.0);

        assert_close(scrollable.get_scroll_y(), 40.0);
    }

    /// Everything about a `Scrollable` is vertical - `scroll_pos` is bounded against
    /// `rect.size.1`, and both menus add the offset to a y coordinate - so the position it is
    /// measured from has to be the vertical one. This used to read `rect.pos.0`, which happened
    /// to work only because both callers leave their x at zero.
    #[test]
    fn test_the_scroll_offset_ignores_the_horizontal_position() {
        let mut scrollable = gfx::Scrollable::new();
        scrollable.rect.pos = FloatPos(500.0, 40.0);

        assert_close(scrollable.get_scroll_y(), 40.0);
    }

    /// A flick loses speed until it stops, rather than decaying towards a velocity that is
    /// merely very small - `approach` snaps once it is inside its epsilon.
    #[test]
    fn test_a_flick_comes_to_a_complete_stop() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut scrollable = gfx::Scrollable::new();
        scrollable.scroll_smooth_factor = 10.0;
        // room to scroll into, so the flick is not fighting the boundary pull
        scrollable.scroll_size = 10000.0;
        scrollable.rect.size.1 = 400.0;
        let root = root_container(&graphics);
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(-10.0), &root);

        let travelled_in_one_frame = |scrollable: &mut gfx::Scrollable| {
            let before = scrollable.get_scroll_pos();
            scrollable.advance_frame();
            scrollable.get_scroll_pos() - before
        };

        assert!(travelled_in_one_frame(&mut scrollable) > 0.0, "the flick should move the list");
        for _ in 0..200 {
            scrollable.advance_frame();
        }
        assert_close(travelled_in_one_frame(&mut scrollable), 0.0);
    }

    /// A list flicked past its end is pulled back *onto* the end, and stops there.
    ///
    /// The pull used to subtract a fraction of the overshoot per frame with nothing to finish
    /// it off, so it only ever approached the boundary asymptotically. Too small to see, but
    /// it is the reason the toolkit funnels every animation through `approach`: the epsilon is
    /// what turns "close enough" into "done".
    #[test]
    #[allow(clippy::float_cmp, reason = "landing exactly on the boundary is what is being asserted")]
    fn test_scrolling_past_the_top_settles_exactly_back_on_it() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut scrollable = gfx::Scrollable::new();
        // what both menus use
        scrollable.scroll_smooth_factor = 100.0;
        scrollable.boundary_smooth_factor = 40.0;
        scrollable.scroll_size = 1000.0;
        scrollable.rect.size.1 = 400.0;

        let root = root_container(&graphics);
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(10.0), &root);

        scrollable.advance_frame();
        assert!(scrollable.get_scroll_pos() < 0.0, "scrolling up from the top should overshoot");

        for _ in 0..2000 {
            scrollable.advance_frame();
        }
        assert_eq!(scrollable.get_scroll_pos(), 0.0, "the list should come to rest on the top, not near it");
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

    /// The cursor is a byte offset that moves by characters: stepping it a byte at a time
    /// would land inside a multi-byte character, and the next edit would panic on a range that
    /// is not a char boundary.
    #[test]
    #[allow(clippy::non_ascii_literal, reason = "the character being multi-byte is the point of the test")]
    fn test_the_cursor_steps_over_a_whole_multibyte_character() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "aé");
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        assert_eq!(input.get_cursor_range(), (1, 1), "left should land before the two byte char, not inside it");

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);
        assert_eq!(input.get_text(), "é");
    }

    #[test]
    #[allow(clippy::non_ascii_literal, reason = "the character being multi-byte is the point of the test")]
    fn test_deleting_forwards_over_a_multibyte_character() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "éa");
        let root = root_container(&graphics);

        for _ in 0..2 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
        input.on_event(&mut graphics, &press(gfx::Key::Delete), &root);

        assert_eq!(input.get_text(), "a");
    }

    #[test]
    #[allow(clippy::non_ascii_literal, reason = "the character being multi-byte is the point of the test")]
    fn test_control_left_jumps_words_containing_multibyte_characters() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "über café");
        let root = root_container(&graphics);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);

        // "café" is five bytes, so a byte-counting jump would land mid-character
        assert_eq!(input.get_text().get(input.get_cursor_range().0..), Some("café"));
    }

    /// Each half of the cursor is clamped separately. A lexicographic tuple `min` would leave
    /// `(2, 8)` alone when clamping against `(3, 3)`, and the 8 then indexes past the end.
    #[test]
    fn test_set_text_clamps_both_halves_of_a_forwards_selection() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcdefghij");
        let root = root_container(&graphics);
        for _ in 0..8 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, true);
        for _ in 0..6 {
            input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, false);
        assert_eq!(input.get_cursor_range(), (2, 8));

        input.set_text("abc".to_owned());

        let (start, end) = input.get_cursor_range();
        assert!(end <= 3, "cursor {start}..{end} is past the end of the new text");
        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);
    }

    /// Pasting goes through `text_processing` exactly the way typing does, so Ctrl+V cannot put
    /// a character into a field that typing it would have been rejected from.
    #[test]
    fn test_pasting_is_filtered_the_same_way_typing_is() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.text_processing = Some(Box::new(|c: char| c.is_ascii_digit().then_some(c)));
        graphics.set_clipboard_text("a1b2c3");

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::V), &root);

        assert_eq!(input.get_text(), "123");
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

    /// Text longer than a default `TextInput` is wide, so the view has to crop it.
    fn overflowing_input(graphics: &mut gfx::HeadlessContext) -> gfx::TextInput {
        let input = input_containing(graphics, "a value far too long to fit inside the box it is being typed into");
        assert!(
            font().get_text_size(input.get_text(), None).0 as f32 > input.get_size().0,
            "the fixture has to overflow for these tests to mean anything"
        );
        input
    }

    /// The visible window into a long value follows the cursor.
    ///
    /// It used to be pinned to the end of the text whatever the cursor was doing, so walking
    /// the cursor left through a long value walked it straight out of the left edge of the
    /// box. The cursor is a filled white rectangle and nothing here clips, so it went on being
    /// drawn over whatever sat beside the input.
    #[test]
    fn test_the_view_follows_the_cursor_out_of_a_long_value() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = overflowing_input(&mut graphics);
        let root = root_container(&graphics);
        let font = font();

        assert!(input.cursor_rect(&font, input.get_size()).pos.0 > 0.0, "the cursor starts at the end, which is on screen");

        while input.get_cursor_range().0 > 0 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
            let cursor = input.cursor_rect(&font, input.get_size());
            assert!(
                cursor.pos.0 + cursor.size.0 > 0.0 && cursor.pos.0 < input.get_size().0,
                "the cursor left the box at offset {:?}: {cursor:?}",
                input.get_cursor_range()
            );
        }

        assert_close(input.visible_text_rect(&font).pos.0, 0.0);
    }

    /// A field nobody is typing in still shows the end of what was typed, which is what the
    /// `text_input_overflowing_text` golden records.
    #[test]
    fn test_an_unselected_field_shows_the_end_of_a_long_value() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = overflowing_input(&mut graphics);
        let font = font();
        let hidden = font.get_text_size(input.get_text(), None).0 as f32 - (input.width - input.padding * 2.0);

        input.selected = false;

        assert_close(input.visible_text_rect(&font).pos.0, hidden);
    }

    /// A selection is as wide as the text it covers, and the text is allowed to be wider than
    /// the box - so the highlight has to be clipped to the widget. Nothing here clips, and the
    /// highlight is a filled rectangle, so one let past the left edge paints a bar over
    /// whatever sits beside the input. Selecting the whole of a long value reached ~120 pixels
    /// past it.
    #[test]
    fn test_a_selection_wider_than_the_box_is_clipped_to_it() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = overflowing_input(&mut graphics);
        let root = root_container(&graphics);
        let font = font();
        let size = input.get_size();

        // select the whole value, one character at a time, from each end
        for key in [gfx::Key::Left, gfx::Key::Right] {
            graphics.set_key_state(gfx::Key::LeftShift, true);
            for _ in 0..input.get_text().len() {
                input.on_event(&mut graphics, &press(key), &root);
                let highlight = input.cursor_rect(&font, size);
                assert!(
                    highlight.pos.0 >= 0.0 && highlight.pos.0 + highlight.size.0 <= size.0,
                    "the selection {highlight:?} left the {size:?} box at offset {:?}",
                    input.get_cursor_range()
                );
            }
            graphics.set_key_state(gfx::Key::LeftShift, false);
            // collapse the selection and go back to the other end for the second pass
            input.on_event(&mut graphics, &press(key), &root);
        }
    }

    /// A selection the box has room for has to be visible in full.
    ///
    /// The view is placed from the end of the cursor the user is moving. Holding that end
    /// against the *left* edge - which is what it used to do - scrolls everything selected by
    /// shift and the right arrow off the screen behind it, so making a selection showed no
    /// selection at all.
    #[test]
    fn test_a_selection_that_fits_is_shown_in_full() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = overflowing_input(&mut graphics);
        let root = root_container(&graphics);
        let font = font();

        while input.get_cursor_range().0 > 0 {
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, true);
        for _ in 0..8 {
            input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        }

        let selected = font.get_text_size(input.get_text().get(..input.get_cursor_range().1).unwrap(), None).0 as f32;
        assert!(selected < input.get_size().0, "the fixture has to select less than the box holds");

        // On screen at all, and all of it: the clipping in `cursor_rect` is what makes the
        // first half of that assertion mean anything.
        let highlight = input.cursor_rect(&font, input.get_size());
        assert!(highlight.pos.0 >= 0.0, "the selection starts at {}, outside the box", highlight.pos.0);
        assert!(highlight.size.0 >= selected, "only {} of the selection's {selected} pixels are on screen", highlight.size.0);
    }

    /// Text that fits is never cropped, wherever the cursor is.
    #[test]
    fn test_a_value_that_fits_is_shown_from_the_start() {
        let mut graphics = gfx::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "short");
        let root = root_container(&graphics);
        let font = font();

        for _ in 0..10 {
            assert_close(input.visible_text_rect(&font).pos.0, 0.0);
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
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

    /// A limit narrower than a single glyph cannot be satisfied, so the glyph stays on the line
    /// it is already on. Wrapping instead would leave a blank line above it, count that line's
    /// height, and then do the same again for every character after it.
    #[test]
    fn test_a_limit_narrower_than_a_glyph_does_not_wrap_every_character() {
        let font = font();

        assert_eq!(font.get_text_size("a", Some(1)).1, 16, "one glyph cannot need two lines");
        assert_eq!(font.get_text_size("ab", Some(1)).1, 33, "one wrap between the two, not one before each");
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

    /// Whether a column of a rasterised string has any ink in it.
    fn column_has_ink(surface: &Surface, x: u32) -> bool {
        (0..surface.get_size().1 as i32).any(|y| surface.get_pixel(IntPos(x as i32, y)).is_ok_and(|pixel| pixel.a != 0))
    }

    /// `get_text_size` is an *advance* width, so measuring a prefix has to land exactly where
    /// the next glyph is drawn.
    ///
    /// A space advances `SPACE_WIDTH` further than its (empty) glyph, and the width used to be
    /// sampled before that was added - so `TextInput`, which measures the text before the
    /// cursor this way, put the cursor two pixels left of the character after a space.
    #[test]
    fn test_a_prefix_ending_in_a_space_measures_up_to_the_next_glyph() {
        let font = font();
        let prefix = font.get_text_size("a ", None).0;
        let surface = font.create_text_surface("a b", None);

        let first_ink_after_the_a = (font.get_text_size("a", None).0..surface.get_size().0).find(|&x| column_has_ink(&surface, x));

        assert_eq!(first_ink_after_the_a, Some(prefix), "the cursor after a space belongs where the next glyph starts");
    }

    /// Widths add up: laying two strings out end to end is the same as laying out their
    /// concatenation, which is what makes measuring a prefix meaningful at all.
    #[test]
    fn test_measuring_is_additive() {
        let font = font();
        let width = |text| font.get_text_size(text, None).0;

        assert_eq!(width("ab"), width("a") + width("b"));
        assert_eq!(width("a "), width("a") + width(" "));
        assert_eq!(width("a b"), width("a ") + width("b"));
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

    /// **Including the space.** It is the one character whose advance is not its glyph's width:
    /// trimming leaves a proportional font's space empty, so it is given a width of its own -
    /// and adding that on top of a mono font's padded space made the space two pixels wider
    /// than every other character, which is the whole of what a mono font promises not to do.
    /// The server console is a column of timestamps drawn in this font.
    #[test]
    fn test_a_mono_space_advances_like_every_other_character() {
        let mono = gfx::Font::new_headless(FONT_MONO, true).unwrap();

        assert_eq!(mono.get_text_size("i i", None).0, mono.get_text_size("iii", None).0);
        assert_eq!(mono.get_text_size(" ", None).0, mono.get_text_size("i", None).0);
    }

    /// The proportional font still needs it: its space glyph is trimmed to nothing, so without
    /// a width of its own a space would be a single pixel of character spacing.
    #[test]
    fn test_a_proportional_space_is_wider_than_its_empty_glyph() {
        let font = font();
        assert!(font.get_text_size(" ", None).0 > 1);
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

        assert_eq!(rect.render_pos, rect.pos);
        assert_eq!(rect.render_size, rect.size);
    }

    #[test]
    fn test_render_rect_lags_behind_a_moved_target() {
        let mut rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));
        rect.pos = FloatPos(100.0, 100.0);

        assert_eq!(rect.render_pos, FloatPos(0.0, 0.0), "the drawn position should not jump with the target");

        rect.jump_to_target();
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

        let mut click_at = |graphics: &mut gfx::HeadlessContext, pos| {
            graphics.set_mouse_pos(pos);
            panel.on_event(graphics, &press(gfx::Key::MouseLeft), &root);
            panel.on_event(graphics, &release(gfx::Key::MouseLeft), &root);
        };

        // inside the button once the panel's offset is applied
        click_at(&mut graphics, FloatPos(225.0, 125.0));
        assert_eq!(clicks.get(), 1, "the click should have reached the button");

        // where the button would be if the panel's offset were ignored
        click_at(&mut graphics, FloatPos(25.0, 25.0));
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
        panel.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        assert!(panel.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root));

        graphics.set_mouse_pos(FloatPos(900.0, 700.0));
        panel.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
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

    /// A widget that exists but is not stepped for a long time owes a frame for every
    /// millisecond of it, and the pause menu's buttons really do sit unrendered for a whole
    /// session before their first frame. Walking that backlog is an hour of animation in one
    /// frame; skipping it lands on the same value, because every animation here has settled
    /// long before the bound.
    #[test]
    fn test_animation_timer_skips_a_backlog_it_could_never_walk() {
        let hour_ms = 60 * 60 * 1000;
        let mut timer = gfx::AnimationTimer::new_started_ago(1, hour_ms);

        let mut frames = 0_i64;
        while timer.frame_ready() {
            frames += 1;
            assert!(frames < hour_ms as i64, "the timer walked the whole backlog");
        }

        assert!(frames > 0, "the bound should still hand out the frames it caps at");
        // A few more than the bound: the clock keeps running while the backlog is handed out.
        assert!(frames <= MAX_CATCHUP_FRAMES + 100, "expected about {MAX_CATCHUP_FRAMES} frames, got {frames}");
    }

    /// The bound is a floor on how far behind the timer may be, not a reset: a timer that is
    /// keeping up must still hand out exactly the frames it owes.
    #[test]
    fn test_the_backlog_bound_leaves_a_timer_that_is_keeping_up_alone() {
        let mut timer = gfx::AnimationTimer::new_started_ago(5, 20);

        let mut frames = 0;
        while timer.frame_ready() {
            frames += 1;
            assert!(frames < 100, "the timer should stop handing out frames");
        }

        assert!((4..=6).contains(&frames), "expected about 4 frames, got {frames}");
    }

    // --- FrameLimiter ---

    use crate::libraries::graphics::renderer::FrameLimiter;

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
    // Drawing records a `DrawCommand` rather than issuing a GPU call, so what a primitive
    // draws is assertable without a window. This is the only tier of rendering coverage
    // `cargo test` can run - the golden images need a real context on the main thread.
    //
    // Every primitive reaches here, because building one without a device is a supported
    // state rather than a failure: `RectArray` stages its vertices until an `upload` that
    // never happens, and `ShadowContext` and `Font` get textures that know their size and own
    // nothing. What these cannot see is the pixels - whether a command lands where it says
    // it does is the golden suite's job.
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

    /// An outline is four one pixel quads laid on the rectangle's own edge pixels, which an
    /// empty rectangle does not have: the bottom edge is placed at `pos.1 + size.1 - 1.0`, so
    /// with no height it lands a row *above* the top edge, outside the rectangle entirely.
    ///
    /// A `Button` mid-hover-fade reaches this. Its hover rectangle is the button inset by up
    /// to 30 pixels a side, which is more than a small button has to give, and the border it
    /// is drawn with is part way to opaque by then.
    #[test]
    fn test_an_empty_rect_draws_no_outline() {
        let recorder = DrawRecorder::new();

        Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 0.0)).render_outline(&recorder, WHITE);
        Rect::new(FloatPos(50.0, 50.0), FloatSize(40.0, 0.0)).render_outline(&recorder, WHITE);
        Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 40.0)).render_outline(&recorder, WHITE);

        assert!(recorder.is_empty(), "an empty rectangle has no edge pixels to draw");

        // what it would have drawn: two vertical lines, the right hand one a pixel to the left
        // of a rectangle that has no width at all
        let edges = crate::libraries::graphics::wgpu_backend::outline_edges(Rect::new(FloatPos(50.0, 50.0), FloatSize(0.0, 40.0)));
        assert_eq!(edges[3], Rect::new(FloatPos(49.0, 50.0), FloatSize(1.0, 40.0)));
    }

    /// The outline of a rectangle that does have edges covers its own footprint exactly, in
    /// both directions - a border that spilled outside would be a pixel of another widget.
    #[test]
    fn test_outline_edges_stay_inside_the_rectangle() {
        let rect = Rect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        for edge in crate::libraries::graphics::wgpu_backend::outline_edges(rect) {
            assert!(edge.pos.0 >= rect.pos.0 && edge.pos.1 >= rect.pos.1, "{edge:?} starts outside {rect:?}");
            assert!(
                edge.pos.0 + edge.size.0 <= rect.pos.0 + rect.size.0 && edge.pos.1 + edge.size.1 <= rect.pos.1 + rect.size.1,
                "{edge:?} ends outside {rect:?}"
            );
        }
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

    /// Uploading a surface with no device in the process is not an error: it produces a texture
    /// that knows its size and owns nothing, which is what lets layout run headlessly.
    #[test]
    fn test_a_texture_can_be_built_without_a_device() {
        let texture = gfx::Texture::load_from_surface(&Surface::new(IntSize(6, 9)));

        assert_eq!(texture.get_texture_size(), FloatSize(6.0, 9.0));
        assert_eq!(texture.get_handle(), gfx::Texture::new().get_handle());
    }

    /// One mesh, however many rectangles went into it - drawing them in a single call is the
    /// whole reason `RectArray` exists.
    #[test]
    fn test_a_rect_array_records_one_mesh_for_all_of_its_rectangles() {
        let recorder = DrawRecorder::new();
        let texture = gfx::Texture::new_sized(FloatSize(8.0, 8.0));
        let mut array = gfx::RectArray::new();
        let tex_rect = Rect::new(FloatPos(0.0, 0.0), FloatSize(8.0, 8.0));
        for i in 0..3 {
            array.add_rect(&Rect::new(FloatPos(i as f32 * 10.0, 0.0), FloatSize(10.0, 10.0)), &[WHITE; 4], &tex_rect);
        }

        array.render(&recorder, Some(&texture), FloatPos(4.0, 5.0));

        assert_eq!(recorder.len(), 1);
        let Some(DrawCommand::Mesh { texture: named, pos, .. }) = recorder.get_commands().first().copied() else {
            panic!("a rect array should record a mesh, got {:?}", recorder.get_commands())
        };
        assert_eq!(named, Some((texture.get_handle(), FloatSize(8.0, 8.0))));
        assert_eq!(pos, FloatPos(4.0, 5.0));
    }

    /// A `RectArray` with no texture draws its vertices' own colours, which is how the light
    /// map and the health bar are drawn.
    #[test]
    fn test_an_untextured_rect_array_names_no_texture() {
        let recorder = DrawRecorder::new();
        gfx::RectArray::new().render(&recorder, None, FloatPos(0.0, 0.0));

        assert!(matches!(recorder.get_commands().first(), Some(DrawCommand::Mesh { texture: None, .. })));
    }

    // --- Font drawing ---

    /// Measuring and drawing have to agree to the pixel, because `TextInput` places its cursor
    /// by measuring the text before it and the glyph after it is drawn by `render_text`. The
    /// two share `Font::advance` so that they cannot drift; this is what pins that they don't.
    ///
    /// The space is the character that makes it worth checking: its glyph is trimmed to nothing,
    /// so it records no draw at all and only moves the pen.
    #[test]
    fn test_render_text_draws_each_glyph_where_measuring_says_it_will() {
        let font = font();
        let recorder = DrawRecorder::with_draw_area(FloatSize(1000.0, 100.0));
        let text = "a b";

        font.render_text(&recorder, text, FloatPos(10.0, 20.0), 2.0);

        // `get_text_size` never reports a zero width, so an empty prefix is answered directly -
        // exactly as `TextInput::width_up_to` does it.
        let expected: Vec<f32> = text
            .char_indices()
            .filter(|(_, character)| *character != ' ')
            .map(|(index, _)| {
                if index == 0 {
                    10.0
                } else {
                    10.0 + font.get_text_size(text.get(..index).unwrap(), None).0 as f32 * 2.0
                }
            })
            .collect();
        let drawn: Vec<f32> = recorder
            .get_commands()
            .into_iter()
            .map(|command| match command {
                DrawCommand::Texture { pos, .. } => pos.0,
                other => panic!("text should only draw textures, got {other:?}"),
            })
            .collect();

        assert_eq!(drawn, expected);
    }

    // --- ShadowContext ---

    use crate::libraries::graphics::shadow::{ShadowContext, FADE, TEXTURE_SIZE};

    /// Where every piece of a shadow around `rect` lands, as (destination, source) rectangles.
    /// The pieces are drawn unscaled, so a destination is as big as the region it samples.
    fn shadow_pieces(rect: Rect) -> Vec<(Rect, Rect)> {
        let recorder = DrawRecorder::with_draw_area(FloatSize(4000.0, 4000.0));
        ShadowContext::new().render(&recorder, &rect, 1.0);

        recorder
            .get_commands()
            .into_iter()
            .map(|command| match command {
                DrawCommand::Texture { pos, src_rect, .. } => (Rect::new(pos, src_rect.size), src_rect),
                other => panic!("a shadow should only draw textures, got {other:?}"),
            })
            .collect()
    }

    /// The sizes that matter: below 300 the edge pieces meet in the middle on their own, and
    /// above it they are capped and the gap has to be tiled.
    const SHADOW_SIZES: [FloatSize; 5] = [FloatSize(0.0, 0.0), FloatSize(140.0, 100.0), FloatSize(140.0, 900.0), FloatSize(900.0, 140.0), FloatSize(901.0, 851.0)];

    /// Every piece samples a region of the baked texture, and a region that ran off the edge of
    /// it would be clamped by the sampler into a smear of whatever the last row holds.
    #[test]
    fn test_a_shadow_only_ever_samples_inside_its_own_texture() {
        for size in SHADOW_SIZES {
            for (_, source) in shadow_pieces(Rect::new(FloatPos(300.0, 300.0), size)) {
                assert!(
                    source.pos.0 >= 0.0 && source.pos.1 >= 0.0 && source.pos.0 + source.size.0 <= TEXTURE_SIZE && source.pos.1 + source.size.1 <= TEXTURE_SIZE,
                    "a {size:?} shadow samples {source:?}, outside the {TEXTURE_SIZE}x{TEXTURE_SIZE} texture"
                );
            }
        }
    }

    /// A shadow is drawn *under* an opaque rectangle, so anything it puts inside one is wasted
    /// work - and it is not wasted work if the rectangle turns out to be translucent, it is a
    /// dark smear across it.
    #[test]
    fn test_a_shadow_never_draws_inside_the_rectangle_it_surrounds() {
        for size in SHADOW_SIZES {
            let rect = Rect::new(FloatPos(300.0, 300.0), size);
            for (piece, _) in shadow_pieces(rect) {
                let overlaps = |piece_pos: f32, piece_size: f32, rect_pos: f32, rect_size: f32| piece_pos + piece_size > rect_pos && piece_pos < rect_pos + rect_size;
                assert!(
                    !(overlaps(piece.pos.0, piece.size.0, rect.pos.0, rect.size.0) && overlaps(piece.pos.1, piece.size.1, rect.pos.1, rect.size.1)),
                    "the piece {piece:?} of a {size:?} shadow reaches inside {rect:?}"
                );
            }
        }
    }

    /// And the other direction: the band of `FADE` pixels around the rectangle is covered by
    /// the pieces with no gaps.
    ///
    /// This is what the tiling exists for. A rectangle taller than 300 pixels outgrows the two
    /// edge pieces, which are capped so that opposite corners meet rather than overlapping, and
    /// the middle of the texture is repeated down the gap they leave. An off-by-one there is a
    /// transparent stripe up the side of a menu.
    #[test]
    fn test_a_shadow_covers_the_whole_band_around_the_rectangle() {
        for size in SHADOW_SIZES {
            let rect = Rect::new(FloatPos(300.0, 300.0), size);
            let pieces = shadow_pieces(rect);

            // Sampled on a grid offset off every piece boundary, so a point is covered by a
            // piece rather than merely touching one - `Rect::contains` is inclusive.
            let mut x = rect.pos.0 - FADE + 3.75;
            while x < rect.pos.0 + rect.size.0 + FADE {
                let mut y = rect.pos.1 - FADE + 3.75;
                while y < rect.pos.1 + rect.size.1 + FADE {
                    let inside = rect.contains(FloatPos(x, y));
                    if !inside {
                        assert!(pieces.iter().any(|(piece, _)| piece.contains(FloatPos(x, y))), "a {size:?} shadow leaves ({x}, {y}) uncovered");
                    }
                    y += 7.5;
                }
                x += 7.5;
            }
        }
    }

    /// The shadow fades out, so the pieces are drawn translucent - and the intensity a
    /// `RenderRect` passes down has to reach them.
    #[test]
    fn test_shadow_intensity_scales_the_colour_every_piece_is_drawn_with() {
        let recorder = DrawRecorder::with_draw_area(FloatSize(4000.0, 4000.0));
        ShadowContext::new().render(&recorder, &Rect::new(FloatPos(300.0, 300.0), FloatSize(140.0, 100.0)), 0.5);

        for command in recorder.get_commands() {
            let DrawCommand::Texture { color, .. } = command else {
                panic!("a shadow should only draw textures, got {command:?}")
            };
            assert_eq!(color, Color::new(0, 0, 0, 40));
        }
    }
}
