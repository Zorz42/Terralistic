use crate::libraries::graphics as gfx;
use crate::server::server_core::ServerState;
use crate::server::server_core::UiMessageType;

use super::ui_manager;
const SCALE: f32 = 2.0;

//this function makes the string have at least a certain length by padding it with zeroes
fn pad_start(string: String, length: usize) -> String {
    let mut string = string;
    while string.len() < length {
        string = format!("0{string}");
    }
    string
}

//this function formats a number of seconds into a string of format HH:MM:SS
fn format_seconds(seconds: u64) -> String {
    let mut seconds = seconds;
    let hours = seconds / 3600;
    seconds -= hours * 3600;
    let minutes = seconds / 60;
    seconds -= minutes * 60;
    format!(
        "{}:{}:{}",
        pad_start(hours.to_string(), 1),
        pad_start(minutes.to_string(), 2),
        pad_start(seconds.to_string(), 2)
    )
}

pub struct ServerInfo {
    _world_name: gfx::Sprite,
    //is it really needed tho?
    _world_seed: gfx::Sprite,
    _players: gfx::Sprite,
    server_state_enum: ServerState,
    //format: state, if running then running:mspt
    server_state_sprite: gfx::Sprite,
    mspt: gfx::Sprite,
    uptime: gfx::Sprite,
    container: gfx::Container,
    server_start: std::time::Instant,
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
            mspt: Default::default(),
            uptime: Default::default(),
            //container math will be redone
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(0.0, 0.0),
                graphics_context.renderer.get_window_size(),
                gfx::TOP_LEFT,
                None,
            ),
            server_start: std::time::Instant::now(),
        }
    }
}

impl ui_manager::ModuleTrait for ServerInfo {
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        self.server_state_sprite.texture =
            gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface("test"));
        self.server_state_sprite.color = gfx::WHITE;
        self.server_state_sprite.scale = SCALE;
        self.server_state_sprite.orientation = gfx::TOP;

        self.uptime.color = gfx::WHITE;
        self.uptime.scale = SCALE;
        self.uptime.orientation = gfx::TOP_RIGHT;
        self.uptime.pos = gfx::FloatPos(-gfx::SPACING, gfx::SPACING);

        self.mspt.color = gfx::WHITE;
        self.mspt.scale = SCALE;
        self.mspt.orientation = gfx::TOP;
    }

    fn update(&mut self, _delta_time: f32, graphics_context: &mut gfx::GraphicsContext) {
        //update the container
        self.container.rect.pos = gfx::FloatPos(0.0, 0.0);
        self.container.rect.size = graphics_context.renderer.get_window_size();

        //update clock sprite
        let uptime_num = self.server_start.elapsed().as_secs();
        self.uptime.texture = gfx::Texture::load_from_surface(
            &graphics_context
                .font
                .create_text_surface(&format!("Uptime: {}", format_seconds(uptime_num))),
        );

        //calculate state and mspt text positions
        let combined_size = self.server_state_sprite.texture.get_texture_size().0
            + self.mspt.texture.get_texture_size().0;

        if self.server_state_enum == ServerState::Running {
            //move server state sprite to the left
            self.server_state_sprite.pos = gfx::FloatPos(
                (self.server_state_sprite.texture.get_texture_size().0 / 2.0 * SCALE)
                    - (combined_size / 2.0 * SCALE),
                gfx::SPACING,
            );
        } else {
            self.server_state_sprite.pos = gfx::FloatPos(0.0, gfx::SPACING);
        }
        self.mspt.pos = gfx::FloatPos(
            (combined_size / 2.0 * SCALE) - (self.mspt.texture.get_texture_size().0 / 2.0 * SCALE),
            gfx::SPACING,
        );
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        gfx::Rect::new(
            gfx::FloatPos(0.0, 0.0),
            graphics_context.renderer.get_window_size(),
        )
        .render(graphics_context, gfx::GREY);

        self.uptime.render(graphics_context, Some(&self.container));

        self.server_state_sprite
            .render(graphics_context, Some(&self.container));
        if self.server_state_enum == ServerState::Running {
            self.mspt.render(graphics_context, Some(&self.container));
        }
    }

    #[allow(clippy::single_match)]
    #[allow(clippy::match_wildcard_for_single_variants)]
    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    ) {
        match message {
            UiMessageType::ServerState(state) => {
                self.server_state_enum = *state;
                self.update_state_sprite(graphics_context);
            }
            UiMessageType::MsptUpdate(mspt) => {
                self.mspt.texture =
                    gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface(
                        format!(" ({}ms)", (*mspt as f64 / 1000.0).to_owned()).as_str(),
                    ));
            }
            _ => {}
        }
    }
}

impl ServerInfo {
    fn update_state_sprite(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        let state_str = match self.server_state_enum {
            ServerState::Nothing => "Nothing",
            ServerState::Starting => "Starting",
            ServerState::InitMods => "InitMods",
            ServerState::LoadingWorld => "LoadingWorld",
            ServerState::Running => "Running",
            ServerState::Stopping => "Stopping",
            ServerState::Stopped => "Stopped",
        }
        .to_owned();
        self.server_state_sprite.texture =
            gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface(&state_str));
    }
}
