use crate::client::menus::Settings;
use crate::libraries::graphics as gfx;

pub struct SettingsMenu {}

impl SettingsMenu {
    pub fn new() -> Self {
        todo!();
    }

    pub fn init(settings: &Settings) {
        todo!();
    }

    pub fn render(graphics: &mut gfx::GraphicsContext) {
        todo!();
    }

    pub fn on_event(&mut self, event: &gfx::Event, graphics: &mut gfx::GraphicsContext) {
        todo!();
    }
}