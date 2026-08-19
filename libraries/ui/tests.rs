#![allow(clippy::unwrap_used)] // tests assert on results directly
#![allow(clippy::assertions_on_result_states)] // some Ok types are not Debug, so unwrap_err is unavailable
#![allow(clippy::panic)] // a test asserting on the shape of a value has nothing else to say when it is the wrong shape
#![cfg(test)]
mod tests {
    use crate::libraries::graphics as gfx;
    use crate::libraries::graphics::{FloatPos, FloatSize, Rect};
    use crate::libraries::ui;

    /// The real font, so a widget that measures text measures the text it will draw. The
    /// glyph texture upload is skipped - `Font::new_headless` - which is all that needs a GPU.
    const FONT: &[u8] = include_bytes!("../../Build/Resources/font.opa");

    fn font() -> gfx::Font {
        gfx::Font::new_headless(FONT, false).unwrap()
    }

    // ---------------------------------------------------------------------------------
    // Headless UI tests
    //
    // Everything below drives real widgets through a `HeadlessContext` - no window and no
    // GPU. Layout, hit testing and event handling go through `UiContext`, so they behave
    // exactly as in the running client; only drawing is missing.
    // ---------------------------------------------------------------------------------

    use std::cell::Cell;
    use std::rc::Rc;
    use ui::{BaseUiElement, UiContext, UiElement};

    /// The window the whole window is the parent of, i.e. what the client passes at the
    /// top of its element tree.
    fn root_container(graphics: &dyn UiContext) -> ui::Container {
        ui::Container::default(graphics)
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
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(640.0, 480.0));

        let container = root_container(&graphics);
        let rect = container.get_absolute_rect();

        assert_eq!(rect.pos, FloatPos(0.0, 0.0));
        assert_eq!(rect.size, FloatSize(640.0, 480.0));
    }

    #[test]
    fn test_top_left_container_is_offset_from_the_origin() {
        let graphics = ui::HeadlessContext::new();
        let container = ui::Container::new(&graphics, FloatPos(10.0, 20.0), FloatSize(100.0, 50.0), ui::TOP_LEFT, None);

        assert_eq!(container.get_absolute_rect().pos, FloatPos(10.0, 20.0));
    }

    /// An orientation moves the container to that point of the parent *and* pulls it back
    /// by the same fraction of its own size, so `CENTER` centres the element rather than
    /// putting its top left corner in the middle.
    #[test]
    fn test_center_container_is_centred_on_its_own_size() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        let container = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), ui::CENTER, None);

        // 500 - 50 and 400 - 25
        assert_eq!(container.get_absolute_rect().pos, FloatPos(450.0, 375.0));
    }

    #[test]
    fn test_bottom_right_container_sits_inside_the_corner() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        let container = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), ui::BOTTOM_RIGHT, None);

        assert_eq!(container.get_absolute_rect().pos, FloatPos(900.0, 750.0));
    }

    /// Every orientation constant places the element fully inside a parent of the same
    /// aspect, which is the invariant the whole layout system rests on.
    #[test]
    fn test_all_orientations_stay_inside_the_parent() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));

        for orientation in [ui::TOP_LEFT, ui::TOP, ui::TOP_RIGHT, ui::LEFT, ui::CENTER, ui::RIGHT, ui::BOTTOM_LEFT, ui::BOTTOM, ui::BOTTOM_RIGHT] {
            let container = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 50.0), orientation, None);
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
        let graphics = ui::HeadlessContext::new();

        let parent = ui::Container::new(&graphics, FloatPos(100.0, 200.0), FloatSize(400.0, 300.0), ui::TOP_LEFT, None);
        let child = ui::Container::new(&graphics, FloatPos(10.0, 20.0), FloatSize(50.0, 50.0), ui::TOP_LEFT, Some(&parent));

        assert_eq!(child.get_absolute_rect().pos, FloatPos(110.0, 220.0));
    }

    #[test]
    fn test_child_centred_in_a_parent_ignores_the_window() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(4000.0, 4000.0));

        let parent = ui::Container::new(&graphics, FloatPos(100.0, 100.0), FloatSize(200.0, 200.0), ui::TOP_LEFT, None);
        let child = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(20.0, 20.0), ui::CENTER, Some(&parent));

        // centre of the parent (200, 200), less half the child
        assert_eq!(child.get_absolute_rect().pos, FloatPos(190.0, 190.0));
    }

    /// A resize is not observed until the container is rebuilt, which is why the client
    /// recreates containers every frame instead of caching them.
    #[test]
    fn test_container_follows_the_window_size_when_rebuilt() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));
        let before = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 100.0), ui::BOTTOM, None);

        graphics.set_window_size(FloatSize(1000.0, 400.0));
        let after = ui::Container::new(&graphics, FloatPos(0.0, 0.0), FloatSize(100.0, 100.0), ui::BOTTOM, None);

        assert_close(before.get_absolute_rect().pos.1, 700.0);
        assert_close(after.get_absolute_rect().pos.1, 300.0);
    }

    // --- Button ---

    /// Records how many times a button's closure ran, so a test can tell a real click
    /// from a near miss.
    fn counting_button(size: FloatSize) -> (ui::Button, Rc<Cell<u32>>) {
        let clicks = Rc::new(Cell::new(0));
        let counter = clicks.clone();
        let mut button = ui::Button::new(move || counter.set(counter.get() + 1));
        button.padding = 0.0;
        button.texture = gfx::Texture::new_sized(size);
        (button, clicks)
    }

    #[test]
    fn test_button_size_includes_padding_and_scale() {
        let mut button = ui::Button::new(|| {});
        button.texture = gfx::Texture::new_sized(FloatSize(100.0, 20.0));
        button.padding = 5.0;
        button.scale = 2.0;

        // (100 + 5*2) * 2 and (20 + 5*2) * 2
        assert_eq!(button.get_size(), FloatSize(220.0, 60.0));
    }

    #[test]
    fn test_button_is_hovered_only_inside_its_rect() {
        let mut graphics = ui::HeadlessContext::new();
        let (button, _) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        assert!(button.is_hovered(&graphics, &root));

        graphics.set_mouse_pos(FloatPos(150.0, 25.0));
        assert!(!button.is_hovered(&graphics, &root), "the mouse is past the right edge");
    }

    #[test]
    fn test_disabled_button_is_never_hovered() {
        let mut graphics = ui::HeadlessContext::new();
        let (mut button, _) = counting_button(FloatSize(100.0, 50.0));
        button.disabled = true;
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(50.0, 25.0));
        assert!(!button.is_hovered(&graphics, &root));
    }

    #[test]
    fn test_button_fires_on_mouse_release_while_hovered() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(50.0, 25.0));

        let consumed = button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
        assert!(!consumed);
    }

    #[test]
    fn test_button_does_not_fire_when_the_mouse_is_elsewhere() {
        let mut graphics = ui::HeadlessContext::new();
        let (mut button, clicks) = counting_button(FloatSize(100.0, 50.0));
        let root = root_container(&graphics);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));

        button.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        button.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert_eq!(clicks.get(), 0);
    }

    #[test]
    fn test_disabled_button_does_not_fire() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
    fn click(graphics: &mut ui::HeadlessContext, toggle: &mut ui::Toggle, root: &ui::Container) -> bool {
        toggle.on_event(graphics, &press(gfx::Key::MouseLeft), root);
        toggle.on_event(graphics, &release(gfx::Key::MouseLeft), root)
    }

    #[test]
    fn test_toggle_flips_when_clicked() {
        let mut graphics = ui::HeadlessContext::new();
        let mut toggle = ui::Toggle::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut toggle = ui::Toggle::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut toggle = ui::Toggle::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut toggle = ui::Toggle::new();
        let root = root_container(&graphics);

        graphics.set_mouse_pos(FloatPos(10.0, 10.0));
        toggle.on_event(&mut graphics, &press(gfx::Key::MouseLeft), &root);
        graphics.set_mouse_pos(FloatPos(500.0, 500.0));
        toggle.on_event(&mut graphics, &release(gfx::Key::MouseLeft), &root);

        assert!(!toggle.toggled);
    }

    #[test]
    fn test_toggle_ignores_clicks_outside_itself() {
        let mut graphics = ui::HeadlessContext::new();
        let mut toggle = ui::Toggle::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut scrollable = ui::Scrollable::new();
        let root = root_container(&graphics);

        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(10.0), &root);

        // the velocity is private, but a scroll of the same sign should not increase it
        // further, which is what the max/min in the handler is for
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(1.0), &root);
        assert_close(scrollable.get_scroll_pos(), 0.0); // position only moves while updating, not on the event
    }

    #[test]
    fn test_scrollable_starts_at_the_top() {
        let scrollable = ui::Scrollable::new();
        assert_close(scrollable.get_scroll_pos(), 0.0);
    }

    /// `get_scroll_y` is the scrollable's own position less the scroll offset, which is how the
    /// server and world lists slide their rows.
    #[test]
    fn test_get_scroll_y_is_the_container_position_when_unscrolled() {
        let mut scrollable = ui::Scrollable::new();
        scrollable.rect.pos = FloatPos(30.0, 40.0);

        assert_close(scrollable.get_scroll_y(), 40.0);
    }

    /// Everything about a `Scrollable` is vertical - `scroll_pos` is bounded against
    /// `rect.size.1`, and both menus add the offset to a y coordinate - so the position it is
    /// measured from has to be the vertical one. This used to read `rect.pos.0`, which happened
    /// to work only because both callers leave their x at zero.
    #[test]
    fn test_the_scroll_offset_ignores_the_horizontal_position() {
        let mut scrollable = ui::Scrollable::new();
        scrollable.rect.pos = FloatPos(500.0, 40.0);

        assert_close(scrollable.get_scroll_y(), 40.0);
    }

    /// A flick loses speed until it stops, rather than decaying towards a velocity that is
    /// merely very small - `approach` snaps once it is inside its epsilon.
    #[test]
    fn test_a_flick_comes_to_a_complete_stop() {
        let mut graphics = ui::HeadlessContext::new();
        let mut scrollable = ui::Scrollable::new();
        scrollable.scroll_smooth_factor = 10.0;
        // room to scroll into, so the flick is not fighting the boundary pull
        scrollable.scroll_size = 10000.0;
        scrollable.rect.size.1 = 400.0;
        let root = root_container(&graphics);
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(-10.0), &root);

        let travelled_in_one_frame = |scrollable: &mut ui::Scrollable| {
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

    /// A list flicked past its end is pulled back *onto* it and stops. Subtracting a fraction
    /// of the overshoot only ever approaches the boundary - which is why every animation goes
    /// through `approach`, whose epsilon turns "close enough" into "done".
    #[test]
    #[allow(clippy::float_cmp, reason = "landing exactly on the boundary is what is being asserted")]
    fn test_scrolling_past_the_top_settles_exactly_back_on_it() {
        let mut graphics = ui::HeadlessContext::new();
        let mut scrollable = ui::Scrollable::new();
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

    /// A list flicked past its end and let go: how deep it goes and how long it takes to come
    /// home. A frame is a millisecond, so `frames_to_settle` reads as milliseconds.
    fn bounce(wheel: f32) -> (f32, usize) {
        let mut graphics = ui::HeadlessContext::new();
        let mut scrollable = ui::Scrollable::new();
        // what `ListPage` uses
        scrollable.scroll_smooth_factor = 100.0;
        scrollable.boundary_smooth_factor = 18.0;
        scrollable.scroll_size = 1000.0;
        scrollable.rect.size.1 = 400.0;

        let root = root_container(&graphics);
        scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(wheel), &root);

        let (mut deepest, mut settled_at) = (0.0_f32, 0);
        for frame in 0..2000 {
            scrollable.advance_frame();
            deepest = deepest.max(-scrollable.get_scroll_pos());
            if scrollable.get_scroll_pos() != 0.0 {
                settled_at = frame + 1;
            }
        }
        assert_close(scrollable.get_scroll_pos(), 0.0);
        (deepest, settled_at)
    }

    /// The bounce follows how hard the list was flicked, which is what makes it read as a rubber
    /// band rather than a wall - but it is compressed the further out it goes, so a hard swipe
    /// does not throw the list a whole screen past its end.
    #[test]
    fn test_the_bounce_grows_with_the_flick_and_is_compressed() {
        let (gentle, _) = bounce(1.0);
        let (hard, _) = bounce(25.0);

        assert!(gentle > 1.0, "even a single notch should bounce, went {gentle}");
        assert!(hard > 4.0 * gentle, "a hard flick should bounce much deeper, {hard} against {gentle}");
        assert!(hard < 25.0 * gentle, "and be compressed rather than proportional, {hard} against {gentle}");
    }

    /// Coming home is one movement, not a crawl. Outside the bounds the momentum decays into the
    /// boundary rather than at its own leisurely rate; left on the latter it kept pushing the
    /// list back out for as long as a flick lasts, and a bounce took most of a second to resolve.
    #[test]
    fn test_the_bounce_comes_home_quickly() {
        for wheel in [1.0, 3.0, 10.0, 25.0] {
            let (_, settled_at) = bounce(wheel);
            assert!(settled_at < 250, "a bounce of {wheel} notches took {settled_at} ms to settle");
        }
    }

    // --- ListPage ---

    /// A row that is nothing but a height and the position the page hands it.
    struct TestRow {
        height: f32,
        pos: FloatPos,
    }

    impl UiElement for TestRow {
        fn get_container(&self, graphics: &dyn UiContext, parent_container: &ui::Container) -> ui::Container {
            ui::Container::new(graphics, self.pos, FloatSize(100.0, self.height), ui::TOP_LEFT, Some(parent_container))
        }
    }

    impl ui::ListRow for TestRow {
        fn get_row_height(&self) -> f32 {
            self.height
        }

        fn set_row_pos(&mut self, pos: FloatPos) {
            self.pos = pos;
        }
    }

    /// One frame of the page's layout. `ListPage::update` takes the rows as trait objects, which
    /// a `[TestRow]` needs a pass to become.
    fn lay_out(page: &mut ui::ListPage, rows: &mut [TestRow], graphics: &dyn UiContext, root: &ui::Container) {
        let mut refs: Vec<&mut dyn ui::ListRow> = rows.iter_mut().map(|row| row as &mut dyn ui::ListRow).collect();
        page.update(graphics, root, &mut refs);
    }

    /// The list is inset by `SPACING` below the top bar, so scrolled fully down it has to end
    /// `SPACING` above the bottom bar. The extent used to drop the gap after the last row, which
    /// left that row's bottom `SPACING` underneath the bar with no way to bring it out.
    #[test]
    fn test_the_last_row_clears_the_bottom_bar_at_full_scroll() {
        let mut graphics = ui::HeadlessContext::new();
        graphics.set_window_size(FloatSize(1000.0, 800.0));
        let root = root_container(&graphics);
        let mut page = ui::ListPage::new(500.0, 100.0, 100.0);
        let mut rows = [
            TestRow {
                height: 200.0,
                pos: FloatPos(0.0, 0.0),
            },
            TestRow {
                height: 200.0,
                pos: FloatPos(0.0, 0.0),
            },
            TestRow {
                height: 200.0,
                pos: FloatPos(0.0, 0.0),
            },
            TestRow {
                height: 200.0,
                pos: FloatPos(0.0, 0.0),
            },
            TestRow {
                height: 200.0,
                pos: FloatPos(0.0, 0.0),
            },
        ];

        lay_out(&mut page, &mut rows, &graphics, &root);
        assert!(page.is_scrollable());

        // scroll to the end and let the bounce settle
        page.scrollable.on_event(&mut graphics, &gfx::Event::MouseScroll(-100.0), &root);
        for _ in 0..2000 {
            page.scrollable.advance_frame();
        }
        lay_out(&mut page, &mut rows, &graphics, &root);

        let last_row_bottom = rows[4].pos.1 + rows[4].height;
        assert_close(last_row_bottom, 800.0 - 100.0 - ui::SPACING);
    }

    #[test]
    fn test_scrollable_ignores_unrelated_events() {
        let mut graphics = ui::HeadlessContext::new();
        let mut scrollable = ui::Scrollable::new();
        let root = root_container(&graphics);

        assert!(!scrollable.on_event(&mut graphics, &press(gfx::Key::A), &root));
    }

    // --- TextInput ---

    /// A selected input, which is the state every test below wants to start from.
    fn selected_input() -> ui::TextInput {
        let mut input = ui::TextInput::new_headless();
        input.selected = true;
        input
    }

    /// A selected input already containing `text`, typed the way a user would. Typing rather
    /// than `set_text`, which leaves the cursor where it was - a test using it would be at
    /// offset 0. See `test_set_text_leaves_the_cursor_at_the_start`.
    fn input_containing(graphics: &mut ui::HeadlessContext, text: &str) -> ui::TextInput {
        let mut input = selected_input();
        let root = root_container(graphics);
        input.on_event(graphics, &type_text(text), &root);
        input
    }

    /// Selects the last `count` characters by holding shift and pressing left, which
    /// leaves the cursor pair "backwards" - its second half is before its first.
    fn select_backwards(graphics: &mut ui::HeadlessContext, input: &mut ui::TextInput, count: usize) {
        let root = root_container(graphics);
        graphics.set_key_state(gfx::Key::LeftShift, true);
        for _ in 0..count {
            input.on_event(graphics, &press(gfx::Key::Left), &root);
        }
        graphics.set_key_state(gfx::Key::LeftShift, false);
    }

    #[test]
    fn test_typing_inserts_text() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("hi"), &root);

        assert_eq!(input.get_text(), "hi");
        assert_eq!(input.get_cursor_range(), (2, 2));
    }

    #[test]
    fn test_typing_does_nothing_when_not_selected() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = ui::TextInput::new_headless();
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("hi"), &root);

        assert_eq!(input.get_text(), "");
    }

    /// `text_processing` filters every character as it arrives, which is how the world
    /// seed field stays numeric.
    #[test]
    fn test_text_processing_filters_characters() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = selected_input();
        input.text_processing = Some(Box::new(|c| c.is_numeric().then_some(c)));
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &type_text("a1b2c3"), &root);

        assert_eq!(input.get_text(), "123");
    }

    #[test]
    fn test_backspace_deletes_one_character() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("abc"), &root);

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "ab");
    }

    #[test]
    fn test_backspace_on_empty_text_is_harmless() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("hello world"), &root);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "hello ");
    }

    #[test]
    fn test_delete_removes_the_character_after_the_cursor() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abc");
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);

        input.on_event(&mut graphics, &press(gfx::Key::Delete), &root);

        assert_eq!(input.get_text(), "ab");
    }

    #[test]
    fn test_arrows_move_the_cursor_one_character() {
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);

        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        assert_eq!(input.get_cursor_range(), (3, 3));

        input.on_event(&mut graphics, &press(gfx::Key::Right), &root);
        assert_eq!(input.get_cursor_range(), (4, 4));
    }

    #[test]
    fn test_cursor_stops_at_both_ends() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "hello world");
        let root = root_container(&graphics);

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::Left), &root);

        assert_eq!(input.get_cursor_range(), (6, 6), "the cursor should land after the space");
    }

    #[test]
    fn test_control_right_jumps_a_whole_word() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");

        select_backwards(&mut graphics, &mut input, 2);

        assert_eq!(input.get_cursor_range(), (2, 4));
    }

    #[test]
    fn test_typing_replaces_the_selection() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "abcd");
        let root = root_container(&graphics);
        select_backwards(&mut graphics, &mut input, 2);

        input.on_event(&mut graphics, &press(gfx::Key::Backspace), &root);

        assert_eq!(input.get_text(), "ab");
        assert_eq!(input.get_cursor_range(), (2, 2));
    }

    #[test]
    fn test_control_c_copies_the_selection() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "ab");
        let root = root_container(&graphics);
        graphics.set_clipboard_text("XY");

        graphics.set_key_state(gfx::Key::LeftControl, true);
        input.on_event(&mut graphics, &press(gfx::Key::V), &root);

        assert_eq!(input.get_text(), "abXY");
    }

    #[test]
    fn test_control_v_replaces_the_selection() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = ui::TextInput::new_headless();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = selected_input();
        let root = root_container(&graphics);
        input.on_event(&mut graphics, &type_text("a long piece of text"), &root);

        input.set_text("ab".to_owned());

        let (start, end) = input.get_cursor_range();
        assert!(start <= 2 && end <= 2, "cursor {start}..{end} is past the end of the new text");
    }

    /// Text longer than a default `TextInput` is wide, so the view has to crop it.
    fn overflowing_input(graphics: &mut ui::HeadlessContext) -> ui::TextInput {
        let input = input_containing(graphics, "a value far too long to fit inside the box it is being typed into");
        assert!(
            font().get_text_size(input.get_text(), None).0 as f32 > input.get_size().0,
            "the fixture has to overflow for these tests to mean anything"
        );
        input
    }

    /// The visible window into a long value follows the cursor. Pinned to the end of the text
    /// instead, walking the cursor left walked it out of the box - and since the cursor is a
    /// filled rectangle and nothing here clips, it went on painting over the widget beside.
    #[test]
    fn test_the_view_follows_the_cursor_out_of_a_long_value() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = overflowing_input(&mut graphics);
        let font = font();
        let hidden = font.get_text_size(input.get_text(), None).0 as f32 - (input.width - input.padding * 2.0);

        input.selected = false;

        assert_close(input.visible_text_rect(&font).pos.0, hidden);
    }

    /// A selection is as wide as the text it covers, which the box need not be, so the
    /// highlight is clipped to the widget - one let past the left edge paints a bar over the
    /// widget beside it, ~120 pixels of one for a long value selected whole.
    #[test]
    fn test_a_selection_wider_than_the_box_is_clipped_to_it() {
        let mut graphics = ui::HeadlessContext::new();
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

    /// A selection the box has room for has to be visible in full. The view is placed from the
    /// cursor's moving end; holding that end against the *left* edge scrolls everything
    /// shift-and-right-arrow selected off behind it, so making a selection showed none.
    #[test]
    fn test_a_selection_that_fits_is_shown_in_full() {
        let mut graphics = ui::HeadlessContext::new();
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
        let mut graphics = ui::HeadlessContext::new();
        let mut input = input_containing(&mut graphics, "short");
        let root = root_container(&graphics);
        let font = font();

        for _ in 0..10 {
            assert_close(input.visible_text_rect(&font).pos.0, 0.0);
            input.on_event(&mut graphics, &press(gfx::Key::Left), &root);
        }
    }

    // --- RenderRect ---

    #[test]
    fn test_render_rect_starts_at_its_target() {
        let rect = ui::RenderRect::new(FloatPos(10.0, 20.0), FloatSize(30.0, 40.0));

        assert_eq!(rect.render_pos, rect.pos);
        assert_eq!(rect.render_size, rect.size);
    }

    #[test]
    fn test_render_rect_lags_behind_a_moved_target() {
        let mut rect = ui::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));
        rect.pos = FloatPos(100.0, 100.0);

        assert_eq!(rect.render_pos, FloatPos(0.0, 0.0), "the drawn position should not jump with the target");

        rect.jump_to_target();
        assert_eq!(rect.render_pos, FloatPos(100.0, 100.0));
    }

    /// The container is built from `render_pos`, not `pos`, which is what makes the
    /// rectangle appear to slide.
    #[test]
    fn test_render_rect_container_follows_the_drawn_position() {
        let graphics = ui::HeadlessContext::new();
        let root = root_container(&graphics);
        let mut rect = ui::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(10.0, 10.0));
        rect.pos = FloatPos(100.0, 100.0);

        assert_eq!(rect.get_container(&graphics, &root).get_absolute_rect().pos, FloatPos(0.0, 0.0));

        rect.jump_to_target();
        assert_eq!(rect.get_container(&graphics, &root).get_absolute_rect().pos, FloatPos(100.0, 100.0));
    }

    // --- Sprite ---

    #[test]
    fn test_sprite_size_scales_with_the_texture() {
        let mut sprite = ui::Sprite::new();
        sprite.set_texture(gfx::Texture::new_sized(FloatSize(40.0, 20.0)));
        sprite.scale = 2.5;

        assert_eq!(sprite.get_size(), FloatSize(100.0, 50.0));
    }

    /// Setting a texture resets the source rectangle to the whole of it, so a sprite
    /// reused for a new texture does not keep cropping to the old one.
    #[test]
    fn test_setting_a_texture_resets_the_source_rect() {
        let mut sprite = ui::Sprite::new();
        sprite.src_rect = Rect::new(FloatPos(5.0, 5.0), FloatSize(1.0, 1.0));

        sprite.set_texture(gfx::Texture::new_sized(FloatSize(40.0, 20.0)));

        assert_eq!(sprite.src_rect.pos, FloatPos(0.0, 0.0));
        assert_eq!(sprite.src_rect.size, FloatSize(40.0, 20.0));
    }

    // --- Event routing through BaseUiElement ---

    /// A panel that positions one button inside itself, so a test can check that the
    /// button is hit tested against the *panel*, not against the window.
    struct TestPanel {
        button: ui::Button,
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

        fn get_container(&self, graphics: &dyn UiContext, parent_container: &ui::Container) -> ui::Container {
            ui::Container::new(graphics, self.pos, self.size, ui::TOP_LEFT, Some(parent_container))
        }
    }

    /// `BaseUiElement::on_event` builds the parent's container and hands it to each child,
    /// so a child's coordinates are relative to its parent. Getting this wrong would put
    /// every nested button's clickable area in the wrong place.
    #[test]
    fn test_events_reach_children_in_parent_relative_coordinates() {
        let mut graphics = ui::HeadlessContext::new();
        let (button, clicks) = counting_button(FloatSize(50.0, 50.0));
        let mut panel = TestPanel {
            button,
            pos: FloatPos(200.0, 100.0),
            size: FloatSize(400.0, 400.0),
        };
        let root = root_container(&graphics);

        let mut click_at = |graphics: &mut ui::HeadlessContext, pos| {
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
        let mut graphics = ui::HeadlessContext::new();
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

    // --- HeadlessContext itself ---

    /// The test double has to answer the same questions as the real context, or the tests
    /// above are measuring nothing.
    #[test]
    fn test_headless_context_reports_what_was_set() {
        let mut graphics = ui::HeadlessContext::new();

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
        let mut graphics = ui::HeadlessContext::new();
        assert!(graphics.as_graphics_context().is_none());
    }

    // --- Dock ---

    use crate::libraries::ui::{area_at_path, DockArea, DockNode, DockSplit, SplitType};

    fn split(orientation: SplitType, at: f32, first: DockNode, second: DockNode) -> DockNode {
        DockNode::Split(Box::new(DockSplit {
            orientation,
            split_pos: at,
            first,
            second,
        }))
    }

    fn pane(name: &str) -> DockNode {
        DockNode::Pane(name.to_owned())
    }

    #[track_caller]
    fn assert_area(area: DockArea, pos: (f32, f32), size: (f32, f32)) {
        assert_eq!(area.pos, FloatPos(pos.0, pos.1));
        assert_eq!(area.size, FloatSize(size.0, size.1));
    }

    #[test]
    fn test_a_leaf_is_the_whole_area() {
        let (area, depth) = area_at_path(&pane("only"), &[], 3);

        assert_area(area, (0.0, 0.0), (1.0, 1.0));
        assert_eq!(depth, 0, "a leaf is as deep as the walk gets");
    }

    #[test]
    fn test_a_vertical_split_divides_left_and_right() {
        let root = split(SplitType::Vertical, 0.25, pane("left"), pane("right"));

        assert_area(area_at_path(&root, &[false], 1).0, (0.0, 0.0), (0.25, 1.0));
        assert_area(area_at_path(&root, &[true], 1).0, (0.25, 0.0), (0.75, 1.0));
    }

    #[test]
    fn test_a_horizontal_split_divides_top_and_bottom() {
        let root = split(SplitType::Horizontal, 0.4, pane("top"), pane("bottom"));

        assert_area(area_at_path(&root, &[false], 1).0, (0.0, 0.0), (1.0, 0.4));
        assert_area(area_at_path(&root, &[true], 1).0, (0.0, 0.4), (1.0, 0.6));
    }

    /// Nesting multiplies: the right half of the bottom half is a quarter of the window in
    /// the far corner. Getting this wrong is what makes a nested pane draw over its sibling.
    #[test]
    fn test_nested_splits_multiply() {
        let inner = split(SplitType::Vertical, 0.5, pane("bottom left"), pane("bottom right"));
        let root = split(SplitType::Horizontal, 0.5, pane("top"), inner);

        let (area, depth) = area_at_path(&root, &[true, true], 2);

        assert_area(area, (0.5, 0.5), (0.5, 0.5));
        assert_eq!(depth, 2);
    }

    /// A path longer than the tree stops where the tree does, and says so - which is what
    /// keeps a selection from claiming a depth that does not exist.
    #[test]
    fn test_a_path_past_the_end_stops_at_the_leaf() {
        let root = split(SplitType::Vertical, 0.5, pane("left"), pane("right"));

        let (area, depth) = area_at_path(&root, &[true, true, true], 5);

        assert_eq!(depth, 1, "the tree is only one split deep");
        assert_area(area, (0.5, 0.0), (0.5, 1.0));
    }

    #[test]
    fn test_a_path_finds_its_node() {
        let inner = split(SplitType::Vertical, 0.5, pane("a"), pane("b"));
        let mut root = split(SplitType::Horizontal, 0.5, pane("top"), inner);

        assert!(matches!(root.at_path(&[false]), DockNode::Pane(name) if name == "top"));
        assert!(matches!(root.at_path(&[true, true]), DockNode::Pane(name) if name == "b"));
        assert!(matches!(root.at_path_mut(&[true, false]), DockNode::Pane(name) if name == "a"));
    }

    #[test]
    fn test_find_pane_searches_the_whole_tree() {
        let inner = split(SplitType::Vertical, 0.5, pane("buried"), DockNode::Nothing);
        let mut root = split(SplitType::Horizontal, 0.5, pane("top"), inner);

        assert!(root.find_pane_mut("buried").is_some());
        assert!(root.find_pane_mut("top").is_some());
        assert!(root.find_pane_mut("not here").is_none());
    }

    #[test]
    fn test_an_area_becomes_pixels() {
        let area = DockArea {
            pos: FloatPos(0.5, 0.25),
            size: FloatSize(0.5, 0.75),
        };

        let rect = area.to_rect(FloatSize(800.0, 400.0));

        assert_eq!(rect.pos, FloatPos(400.0, 100.0));
        assert_eq!(rect.size, FloatSize(400.0, 300.0));
    }
}
