use crate::libraries::graphics as gfx;

pub trait BaseUiElement: UiElement {
    fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>) {
        self.update_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.update(graphics, Some(&container));
        }
    }
    fn render(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>) {
        self.render_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            //TODO change to immutable
            element.render(graphics, Some(&container));
        }
    }
    fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: Option<&gfx::Container>) -> bool {
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            if element.on_event(graphics, event, Some(&container)) {
                return true;
            }
        }
        self.on_event_inner(graphics, event, parent_container)
    }
}

impl<T: UiElement> BaseUiElement for T {}

pub trait UiElement {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement>;
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement>;
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn on_event_inner(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: Option<&gfx::Container>) -> bool;
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> gfx::Container;
}
