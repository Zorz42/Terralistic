use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{DARK_GREY, GREY};
use crate::server::server_ui::{PlayerEventType, UiMessageType};

use super::ui_manager;
use super::ui_manager::{SCALE, EDGE_SPACING};

//this struct is a player card, containing the name, connection and other data
pub struct PlayerCard {
    //the name of the player
    name_sprite: gfx::Sprite,
    name_string: String,
    //the connection of the player
    connection: SocketAddr,
    //the card container
    container: gfx::Container,
    target_y: f32,
    timer: f32,
}

impl PlayerCard {
    pub fn new(
        graphics_context: &mut gfx::GraphicsContext,
        name: String,
        connection: SocketAddr
    ) -> Self {
        let mut a = Self {
            name_sprite: gfx::Sprite::new(),
            name_string: name,
            connection,
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(EDGE_SPACING, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
            target_y: 0.0,
            timer: 0.0,
        };
        a.name_sprite.texture = gfx::Texture::load_from_surface(
            &graphics_context.font.create_text_surface(&a.name_string)
        );
        a.name_sprite.scale = SCALE;
        a.name_sprite.orientation = gfx::LEFT;
        a.name_sprite.color = gfx::WHITE;
        a.name_sprite.pos = gfx::FloatPos(gfx::SPACING, 0.0);
        a
    }

    pub fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        //background
        let mut rect = gfx::RenderRect::new(
            gfx::FloatPos(0.0, 0.0),
            self.container.rect.size
        );
        rect.fill_color = DARK_GREY;
        rect.render(graphics_context, Some(&self.container));

        self.name_sprite.render(graphics_context, Some(&self.container));

        if self.timer < 1.0 {
            //fade in
            let mut rect = gfx::RenderRect::new(
                gfx::FloatPos(0.0, 0.0),
                self.container.rect.size
            );
            rect.fill_color = GREY;
            rect.fill_color.a = (255.0 - self.timer * 255.0) as u8;
            rect.render(graphics_context, Some(&self.container));
        }
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
                gfx::FloatPos(EDGE_SPACING, 0.0),
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

    fn update(&mut self, delta_time: f32, graphics_context: &mut gfx::GraphicsContext) {
        //ideally there should be a on_window_resize function

        //loop through all the player cards and resize them
        let mut y = EDGE_SPACING;
        for card in &mut self.player_cards {
            card.container.rect.size = gfx::FloatSize(
                self.container.rect.size.0 - EDGE_SPACING * 2.0,
                card.name_sprite.texture.get_texture_size().1 * SCALE + 2.0 * gfx::SPACING,
            );

            card.target_y = y;
            y += card.container.rect.size.1 + 2.0 * EDGE_SPACING;

            if (card.target_y - card.container.rect.pos.1).abs() > 0.001 || card.timer < 1.0 {
                card.timer += delta_time;

                card.container.rect.pos.1 = card.target_y * 0.1 + card.container.rect.pos.1 * 0.9;
                card.container.update(graphics_context, Some(&self.container));
            }
        }
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        for card in &mut self.player_cards {
            card.render(graphics_context);
        }
    }

    fn on_server_message(&mut self, message: &UiMessageType, graphics_context: &mut gfx::GraphicsContext) {
        if let UiMessageType::PlayerEvent(event) = message {
            match event {
                PlayerEventType::Join((name, connection)) => {
                    self.player_cards.push(PlayerCard::new(graphics_context, name.clone(), *connection));
                    if let Some(card) = self.player_cards.last_mut() {
                        card.target_y = self.container.rect.size.1;
                        card.container.rect.pos.1 = card.target_y;
                        card.container.update(graphics_context, Some(&self.container));
                    }
                },
                PlayerEventType::Leave(connection) => {
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

    fn get_name(&self) -> &str {
        "PlayerList"
    }
}