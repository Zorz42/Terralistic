use std::net::IpAddr;
use crate::libraries::graphics as gfx;
use crate::server::server_ui::{PlayerEventType, UiMessageType};

use super::ui_manager;
use super::ui_manager::SCALE;

//this struct is a player card, containing the name, connection and other data
pub struct PlayerCard {
    //the name of the player
    name_sprite: gfx::Sprite,
    name_string: String,
    //the connection of the player
    connection: IpAddr,
    //the card container
    container: gfx::Container,
}

impl PlayerCard {
    pub fn new(
        graphics_context: &mut gfx::GraphicsContext,
        name: String,
        connection: IpAddr
    ) -> Self {
        let mut a = Self {
            name_sprite: gfx::Sprite::new(),
            name_string: name,
            connection,
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
        };
        a.name_sprite.texture = gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface(&a.name_string));
        a.name_sprite.scale = SCALE;
        a.name_sprite.orientation = gfx::TOP_LEFT;
        a.name_sprite.color = gfx::WHITE;
        a
    }

    pub fn render(&mut self, graphics_context: &mut gfx::GraphicsContext, container: &gfx::Container) {
        self.name_sprite.render(graphics_context, Some(container));
    }
}

//this struct contains all the player cards
pub struct PlayerList {
    //the list of player cards
    player_cards: Vec<PlayerCard>,
    //the container that contains all the player cards
    container: gfx::Container,
}

impl PlayerList {
    pub fn new(graphics_context: &mut gfx::GraphicsContext) -> Self {
        Self {
            player_cards: Vec::new(),
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
        }
    }
}

impl ui_manager::ModuleTrait for PlayerList {
    fn init(&mut self, _graphics_context: &mut gfx::GraphicsContext) {
        //empty, nothing to do
    }

    fn update(&mut self, _delta_time: f32, _graphics_context: &mut gfx::GraphicsContext) {
        //empty, nothing to do
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        let mut y = 0.0;
        for card in &mut self.player_cards {
            card.container.rect.pos = gfx::FloatPos(0.0, y);
            card.render(graphics_context, &self.container);
            y += card.container.rect.size.1;
        }
    }

    fn on_server_message(&mut self, message: &UiMessageType, graphics_context: &mut gfx::GraphicsContext) {
        if let UiMessageType::PlayerEvent(event) = message {
            match event {
                PlayerEventType::Join((name, connection)) => {
                    self.player_cards.push(PlayerCard::new(graphics_context, name.clone(), *connection));
                },
                PlayerEventType::Leave(connection) => {
                    #[allow(clippy::unreadable_literal)]
                    for (i, card) in self.player_cards.iter().enumerate() {
                        if card.connection == *connection {
                            self.player_cards.remove(i);
                            break;
                        }
                    }
                },
            }
        }
    }

    fn get_container_mut(&mut self) -> &mut gfx::Container {
        &mut self.container
    }
}