use crate::libraries::graphics as gfx;

pub trait UiElement {
    fn render(&mut self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn update(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: Option<&gfx::Container>);
    fn on_event(&mut self, graphics: &mut gfx::GraphicsContext, event: &gfx::Event, parent_container: Option<&gfx::Container>);
    fn get_container(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> gfx::Container;
}
