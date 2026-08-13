use crate::libraries::graphics as gfx;

/// The recursion into child elements. Blanket-implemented, so **implement `UiElement`, call
/// `BaseUiElement`.**
pub trait BaseUiElement: UiElement {
    fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.update_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.update(graphics, &container);
        }
    }

    fn render(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        self.render_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.render(graphics, &container);
        }
    }

    /// Offers the event to every child in this element's coordinates, then to the element
    /// itself. True if anything consumed it.
    fn on_event(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        let container = self.get_container(graphics, parent_container);
        let mut event_detected = false;
        for element in self.get_sub_elements_mut() {
            event_detected |= element.on_event(graphics, event, &container);
        }
        self.on_event_inner(graphics, event, parent_container) || event_detected
    }
}

impl<T: UiElement> BaseUiElement for T {}

/// What a widget implements: where it sits, what it draws, and what it does with input.
pub trait UiElement {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement>;
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement>;
    /// Records what to draw. The one place a widget is allowed to touch the GPU.
    fn render_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {}
    /// Advances animations and other per-frame state.
    fn update_inner(&mut self, _: &mut gfx::GraphicsContext, _: &gfx::Container) {}
    /// Takes a `UiContext` rather than the full `GraphicsContext`, so event handling can be
    /// exercised without a window. Anything in here that needs to draw belongs in
    /// `render_inner` instead.
    fn on_event_inner(&mut self, _: &mut dyn gfx::UiContext, _: &gfx::Event, _: &gfx::Container) -> bool {
        false
    }
    /// Layout only, so this also takes a `UiContext`.
    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container;

    /// Whether the pointer is inside this element's rectangle.
    ///
    /// Hit testing is layout, so it belongs here rather than being written out again by each
    /// widget that wants it. Override it where an element is hoverable on other terms - a
    /// disabled `Button` is never hovered, which is also what stops it reacting to clicks.
    fn is_hovered(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> bool {
        self.get_container(graphics, parent_container).get_absolute_rect().contains(graphics.get_mouse_pos())
    }
}
