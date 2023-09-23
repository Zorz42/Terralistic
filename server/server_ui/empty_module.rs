use super::ui_manager;
use super::ui_manager::{EDGE_SPACING, SCALE};
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{Container, GraphicsContext};
use crate::server::server_ui::UiMessageType;

pub struct EmptyModule {
    container: Container,
}

impl EmptyModule {
    pub fn new(graphics_context: &mut GraphicsContext) -> Self {
        EmptyModule {
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(EDGE_SPACING, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
        }
    }
}

impl ui_manager::ModuleTrait for EmptyModule {
    fn update(&mut self, delta_time_seconds: f32, graphics_context: &mut GraphicsContext) {}

    fn render(&mut self, graphics_context: &mut GraphicsContext) {}

    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut GraphicsContext,
    ) {
    }

    fn get_container_mut(&mut self) -> &mut Container {
        &mut self.container
    }

    fn get_name(&self) -> &str {
        "Empty"
    }
}
