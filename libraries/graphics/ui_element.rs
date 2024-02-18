use crate::libraries::graphics as gfx;
use gfx::Container;
use gfx::Event;
use gfx::GraphicsContext;

pub trait UiElement {
    fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>);
    fn update(&mut self, graphics: &mut GraphicsContext, parent_container: Option<&Container>);
    fn on_event(&mut self, graphics: &mut GraphicsContext, event: &Event, parent_container: Option<&Container>);
    fn get_container(&self, graphics: &GraphicsContext, parent_container: Option<&Container>) -> Container;
}
