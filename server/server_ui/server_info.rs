use crate::server::server_core::UiMessageType;
use super::ui_manager;
use crate::server::server_core::ServerState;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{IntPos};

pub struct ServerInfo {
    world_name: gfx::Sprite,//is it really needed tho?
    world_seed: gfx::Sprite,
    players: gfx::Sprite,
    server_state_enum: ServerState,//format: state, if running then running:mspt
    server_state_sprite: gfx::Sprite,
    mspt: gfx::Sprite,
    clock: gfx::Sprite,
    container: gfx::Container
}

impl ServerInfo {
    pub fn new(graphics_context: &mut gfx::GraphicsContext) -> Self {
        Self {
            world_name: Default::default(),
            world_seed: Default::default(),
            players: Default::default(),
            server_state_enum: ServerState::Nothing,
            server_state_sprite: Default::default(),
            mspt: Default::default(),
            clock: Default::default(),
            //container math will be redone
            container: gfx::Container::new(&graphics_context, gfx::FloatPos{0: 0.0, 1: 0.0 }, graphics_context.renderer.get_window_size(), gfx::TOP_LEFT, None),
        }
    }
}

impl ui_manager::ModuleTrait for ServerInfo {
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        self.server_state_sprite.texture = gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface("test"));
        self.server_state_sprite.color = gfx::WHITE;
        self.server_state_sprite.scale = 2.0;
        self.server_state_sprite.orientation = gfx::CENTER;
        self.server_state_sprite.pos = gfx::FloatPos(0.0, 0.0);
    }

    fn update(&mut self, delta_time: f32) {
        todo!()
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        //self.server_state_sprite.render(graphics_context, Some(&self.container));
        //rectangle for easier debugging
        let a = gfx::Rect::new(gfx::FloatPos{0: 0.0, 1: 0.0 }, graphics_context.renderer.get_window_size());
        a.render(graphics_context, gfx::WHITE);
    }

    fn on_server_message(&mut self, message: &UiMessageType) {
        match message {
            UiMessageType::ServerState(state) => {
                self.server_state_enum = *state;
            }
            _ => {}
        }
    }
}