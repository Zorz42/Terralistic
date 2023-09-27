use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{Container, GraphicsContext};
use crate::server::server_ui::UiMessageType;

use super::ui_manager;
use super::ui_manager::EDGE_SPACING;

pub struct EmptyModule {
    container: Container,
    name: String,
}

impl EmptyModule {
    pub fn new(graphics_context: &mut GraphicsContext, name: String) -> Self {
        Self {
            container: Container::new(
                graphics_context,
                gfx::FloatPos(EDGE_SPACING, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
            name,
        }
    }
}

impl ui_manager::ModuleTrait for EmptyModule {
    fn update(&mut self, _delta_time_seconds: f32, _graphics_context: &mut GraphicsContext) {}

    fn render(&mut self, _graphics_context: &mut GraphicsContext) {}

    fn on_server_message(
        &mut self,
        _message: &UiMessageType,
        _graphics_context: &mut GraphicsContext,
    ) {
    }

    fn get_container_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
