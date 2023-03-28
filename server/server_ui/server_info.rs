use chrono::Timelike;
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
    clock: gfx::Sprite,
    container: gfx::Container,
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
            clock: Default::default(),
            //container math will be redone
            container: gfx::Container::new(
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
        self.server_state_sprite.scale = 3.0;
        self.server_state_sprite.orientation = gfx::CENTER;
        self.server_state_sprite.pos = gfx::FloatPos(0.0, 0.0);

        self.clock.color = gfx::WHITE;
        self.clock.scale = 2.0;
        self.clock.orientation = gfx::TOP_RIGHT;
        self.clock.pos = gfx::FloatPos(-gfx::SPACING, gfx::SPACING);
    }

    fn update(&mut self, _delta_time: f32, graphics_context: &mut gfx::GraphicsContext) {
        //update clock sprite
        let mut clock_str = chrono::Local::now().to_string();
        while !clock_str.ends_with('.') {
            clock_str.pop();
        }
        clock_str.pop();
        self.clock.texture = gfx::Texture::load_from_surface(
            &graphics_context.font.create_text_surface(&clock_str)
        );
    }

    fn render(&mut self, mut graphics_context: &mut gfx::GraphicsContext) {
        gfx::Rect::new(
            gfx::FloatPos(0.0, 0.0),
            graphics_context.renderer.get_window_size(),
        ).render(graphics_context, gfx::GREY);
        self.clock.render(&mut graphics_context, Some(&self.container));
    }

    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    fn on_server_message(&mut self, message: &UiMessageType) {
        match message {
            UiMessageType::ServerState(state) => {
                self.server_state_enum = *state;
            }
            _ => {}
        }
    }
}
