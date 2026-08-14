use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::BaseUiElement;
use crate::server::server_ui::{PlayerEventType, ServerState, UiMessageType};

use super::ui_manager;
use super::ui_manager::SCALE;

/// this function makes the string have at least a certain length by padding it with zeroes in the beginning
fn pad_start(string: String, length: usize) -> String {
    let mut string = string;
    while string.len() < length {
        string = format!("0{string}");
    }
    string
}

/// this function formats a number of seconds into a string of format HH:MM:SS
fn format_seconds(seconds: u64) -> String {
    let mut seconds = seconds;
    let hours = seconds / 3600;
    seconds -= hours * 3600;
    let minutes = seconds / 60;
    seconds -= minutes * 60;
    format!("{}:{}:{}", pad_start(hours.to_string(), 1), pad_start(minutes.to_string(), 2), pad_start(seconds.to_string(), 2))
}

pub struct ServerInfo {
    player_count_sprite: ui::Sprite,
    players_count: u32,
    server_state_enum: ServerState,
    server_state_sprite: ui::Sprite,
    mspt_sprite: ui::Sprite,
    mspt: (f64, f64),
    uptime: ui::Sprite,
    container: ui::Container,
    server_start: std::time::Instant,
    last_update: std::time::Instant,
    updated_ui: i32,
    updated_server: i32,
    enabled: bool,
}

impl ServerInfo {
    pub fn new(graphics_context: &gfx::GraphicsContext) -> Self {
        Self {
            player_count_sprite: ui::Sprite::new(),
            players_count: 0,
            server_state_enum: ServerState::Nothing,
            server_state_sprite: ui::Sprite::new(),
            mspt_sprite: ui::Sprite::new(),
            mspt: (0.0, 0.0),
            uptime: ui::Sprite::new(),
            //container math will be redone
            container: ui::Container::new(graphics_context, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), ui::TOP_LEFT, None),
            server_start: std::time::Instant::now(),
            last_update: std::time::Instant::now(),
            updated_ui: 0,
            updated_server: 0,
            enabled: false,
        }
    }

    fn update_state_sprite(&mut self, graphics_context: &gfx::GraphicsContext) {
        let state_str = match self.server_state_enum {
            ServerState::Nothing => "Nothing",
            ServerState::Starting => "Starting",
            ServerState::InitMods => "InitMods",
            ServerState::LoadingWorld => "LoadingWorld",
            ServerState::GeneratingWorld => "GeneratingWorld",
            ServerState::Running => "Running",
            ServerState::Stopping => "Stopping",
            ServerState::Stopped => "Stopped",
        }
        .to_owned();
        self.server_state_sprite
            .set_texture(gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface(&state_str, None)));
    }
}

impl ui_manager::ModuleTrait for ServerInfo {
    //initializes all the sprites
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        self.server_state_sprite
            .set_texture(gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface("test", None)));
        self.server_state_sprite.color = ui::WHITE;
        self.server_state_sprite.scale = SCALE;
        self.server_state_sprite.orientation = ui::TOP;

        self.uptime.color = ui::WHITE;
        self.uptime.scale = SCALE;
        self.uptime.orientation = ui::TOP_RIGHT;
        self.uptime.pos = gfx::FloatPos(-ui::SPACING, ui::SPACING);

        self.mspt_sprite.color = ui::WHITE;
        self.mspt_sprite.scale = SCALE;
        self.mspt_sprite.orientation = ui::TOP;

        self.player_count_sprite.color = ui::WHITE;
        self.player_count_sprite.scale = SCALE;
        self.player_count_sprite.orientation = ui::TOP_LEFT;
        self.player_count_sprite.pos = gfx::FloatPos(ui::SPACING, ui::SPACING);
        self.player_count_sprite
            .set_texture(gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface("Players: 0", None)));
    }

    fn update(&mut self, _delta_time: f32, graphics_context: &mut gfx::GraphicsContext) {
        //update clock sprite
        let uptime_num = self.server_start.elapsed().as_secs();
        self.uptime.set_texture(gfx::Texture::load_from_surface(
            &graphics_context.font.create_text_surface(&format!("Uptime: {}", format_seconds(uptime_num)), None),
        ));

        //calculate state and mspt_sprite text positions
        let combined_size = self.server_state_sprite.get_texture().get_texture_size().0 + self.mspt_sprite.get_texture().get_texture_size().0;

        //if the server is running, move the running text slightly to the right and add the mspt_sprite text so they are centered together, otherwise keep it in the center and don't show mspt_sprite
        if self.server_state_enum == ServerState::Running {
            //move server state sprite to the left
            self.server_state_sprite.pos = gfx::FloatPos((self.server_state_sprite.get_texture().get_texture_size().0 / 2.0 * SCALE) - (combined_size / 2.0 * SCALE), ui::SPACING);
        } else {
            //move server state sprite to the center
            self.server_state_sprite.pos = gfx::FloatPos(0.0, ui::SPACING);
        }

        //move mspt_sprite sprite to the right
        self.mspt_sprite.pos = gfx::FloatPos((combined_size / 2.0 * SCALE) - (self.mspt_sprite.get_texture().get_texture_size().0 / 2.0 * SCALE), ui::SPACING);
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        //render sprites
        self.uptime.render(graphics_context, &self.container);

        self.server_state_sprite.render(graphics_context, &self.container);
        if self.server_state_enum == ServerState::Running {
            self.mspt_sprite.render(graphics_context, &self.container);
        }

        self.player_count_sprite.render(graphics_context, &self.container);
    }

    fn on_server_message(&mut self, message: &UiMessageType, graphics_context: &mut gfx::GraphicsContext) {
        match message {
            //update server state sprite
            UiMessageType::ServerState(state) => {
                self.server_state_enum = *state;
                self.update_state_sprite(graphics_context);
            }
            //update mspt_sprite sprite
            UiMessageType::MsptUpdate((server_mspt, ui_mspt)) => {
                self.mspt.0 += server_mspt.unwrap_or(0.0);
                self.mspt.1 += *ui_mspt;
                self.updated_ui += 1;
                self.updated_server += i32::from(server_mspt.is_some());
                if self.last_update.elapsed().as_millis() < 1000 {
                    return;
                }
                self.mspt_sprite.set_texture(gfx::Texture::load_from_surface(
                    &graphics_context.font.create_text_surface(
                        format!(
                            " {:.3}/{:.3}mspt (max {:.3}mspt)",
                            (self.mspt.0 / self.updated_server as f64).to_owned(),
                            (self.mspt.1 / self.updated_ui as f64).to_owned(),
                            1000.0 / 20.0
                        )
                        .as_str(),
                        None,
                    ),
                ));
                self.last_update = std::time::Instant::now();
                self.updated_ui = 0;
                self.updated_server = 0;
                self.mspt = (0.0, 0.0);
            }
            UiMessageType::PlayerEvent(event) => match event {
                //update player count sprite
                PlayerEventType::Join((_name, _addr)) => {
                    self.players_count += 1;
                    self.player_count_sprite.set_texture(gfx::Texture::load_from_surface(
                        &graphics_context.font.create_text_surface(&format!("Players: {}", self.players_count), None),
                    ));
                }
                //update player count sprite
                PlayerEventType::Leave(_addr) => {
                    self.players_count -= 1;
                    self.player_count_sprite.set_texture(gfx::Texture::load_from_surface(
                        &graphics_context.font.create_text_surface(&format!("Players: {}", self.players_count), None),
                    ));
                }
            },
            _ => {}
        }
    }
    fn get_container_mut(&mut self) -> &mut ui::Container {
        &mut self.container
    }

    fn get_name(&self) -> &'static str {
        "server_info"
    }

    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}
