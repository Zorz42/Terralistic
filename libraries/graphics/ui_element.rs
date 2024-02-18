use crate::libraries::graphics as gfx;

pub trait UiElement {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn UiElement>;
    fn get_sub_elements(&self) -> Vec<&dyn UiElement>;
    fn render_inner(&mut self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn render(&mut self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) {
        self.render_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            //TODO change to immutable
            element.render(graphics, Some(&container));
        }
    }
    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>) {
        self.update_inner(graphics, parent_container);
        let container = self.get_container(graphics, parent_container);
        for element in self.get_sub_elements_mut() {
            element.update(graphics, Some(&container));
        }
    }
    fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: Option<&gfx::Container>);
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> gfx::Container;
}
