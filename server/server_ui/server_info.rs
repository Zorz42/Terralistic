use crate::libraries::graphics as gfx;
use crate::server::server_core::ServerState;
use crate::server::server_core::UiMessageType;

use super::ui_manager;

pub struct ServerInfo {
    _world_name: gfx::Sprite,
    //is it really needed tho?
    _world_seed: gfx::Sprite,
    _players: gfx::Sprite,
    server_state_enum: ServerState,
    //format: state, if running then running:mspt
    server_state_sprite: gfx::Sprite,
    _mspt: gfx::Sprite,
    _clock: gfx::Sprite,
    _container: gfx::Container,
}

impl ServerInfo {
    #[allow(clippy::default_trait_access)]
    pub fn new(graphics_context: &mut gfx::GraphicsContext) -> Self {
        Self {
            _world_name: Default::default(),
            _world_seed: Default::default(),
            _players: Default::default(),
            server_state_enum: ServerState::Nothing,
            server_state_sprite: Default::default(),
            _mspt: Default::default(),
            _clock: Default::default(),
            //container math will be redone
            _container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(0.0, 0.0),
                graphics_context.renderer.get_window_size(),
                gfx::TOP_LEFT,
                None,
            ),
        }
    }
}

impl ui_manager::ModuleTrait for ServerInfo {
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        self.server_state_sprite.texture =
            gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface("test"));
        self.server_state_sprite.color = gfx::WHITE;
        self.server_state_sprite.scale = 2.0;
        self.server_state_sprite.orientation = gfx::CENTER;
        self.server_state_sprite.pos = gfx::FloatPos(0.0, 0.0);
    }

    #[allow(clippy::todo)]
    fn update(&mut self, _delta_time: f32) {
        todo!()
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        //self.server_state_sprite.render(graphics_context, Some(&self.container));
        //rectangle for easier debugging
        let a = gfx::Rect::new(
            gfx::FloatPos(0.0, 0.0),
            graphics_context.renderer.get_window_size(),
        );
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
