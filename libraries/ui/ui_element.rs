use crate::libraries::graphics as gfx;

/// The recursion into child elements. Blanket-implemented, so **implement `UiElement`, call
/// `BaseUiElement`.**
pub trait BaseUiElement: UiElement {
    fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &super::Container) {
        self.update_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.update(graphics, &container);
        }
    }

    fn render(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &super::Container) {
        self.render_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.render(graphics, &container);
        }
    }

    /// Offers the event to every child in this element's coordinates, then to the element
    /// itself. True if anything consumed it.
    fn on_event(&mut self, graphics: &mut dyn super::UiContext, event: &gfx::Event, parent_container: &super::Container) -> bool {
        let container = self.get_container(graphics, parent_container);
        let mut event_detected = false;
        for element in self.get_sub_elements_mut() {
            event_detected |= element.on_event(graphics, event, &container);
        }
        self.on_event_inner(graphics, event, parent_container) || event_detected
    }
}

impl<T: UiElement> BaseUiElement for T {}

/// What a widget implements: where it sits, what it draws, what it does with input.
///
/// `get_container` is the only required method, so a leaf writes only what it does. **An
/// element with children has to override both `get_sub_elements` methods** - the recursion
/// above finds them nowhere else, and one that forgets is drawn while its children are not.
pub trait UiElement {
    /// The children to recurse into, if any.
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }
    /// The same, for the callers that only read - a menu delegating to the element it wraps.
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }
    /// Records what to draw. The one place a widget is allowed to touch the GPU.
    fn render_inner(&mut self, _: &mut gfx::GraphicsContext, _: &super::Container) {}
    /// Advances animations and other per-frame state.
    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &super::Container) {}
    /// Takes a `UiContext`, not a `GraphicsContext`, so event handling is testable without a
    /// window. Anything here that needs to draw belongs in `render_inner`.
    fn on_event_inner(&mut self, _: &mut dyn super::UiContext, _: &gfx::Event, _: &super::Container) -> bool {
        false
    }
    /// Layout only, so this also takes a `UiContext`.
    fn get_container(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> super::Container;

    /// Whether the pointer is inside this element's rectangle - hit testing is layout, so it
    /// lives here rather than in every widget. Override it where an element is hoverable on
    /// other terms: a disabled `Button` is never hovered, which is what stops it clicking.
    fn is_hovered(&self, graphics: &dyn super::UiContext, parent_container: &super::Container) -> bool {
        self.get_container(graphics, parent_container).get_absolute_rect().contains(graphics.get_mouse_pos())
    }
}

/// What makes a click a click: a press **and** a release on the same widget.
///
/// On the release alone, a press that landed on the menu behind activates whatever the pointer is
/// over when the button comes up; both halves also let a user back out by dragging off first.
/// `Button` and `Toggle` each own one.
#[derive(Default)]
pub struct ClickTracker {
    /// Whether the last press landed here. **Not** cleared by the release that consumes it:
    /// several menus hold their buttons as sub-elements *and* dispatch to them again from
    /// `on_event_inner`, so one release reaches a widget twice and the menu reads the second.
    pressed_inside: bool,
}

impl ClickTracker {
    /// Feeds one event in and answers whether it completed a click. `hovered` is the widget's
    /// own question to answer: a disabled `Button` says no and so is never clicked.
    pub const fn completes_a_click(&mut self, event: &gfx::Event, hovered: bool) -> bool {
        match event {
            gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) => {
                self.pressed_inside = hovered;
                false
            }
            gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) => self.pressed_inside && hovered,
            _ => false,
        }
    }
}
