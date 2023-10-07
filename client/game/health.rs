use crate::client::game::floating_text::{FloatingText, FloatingTextManager};
use crate::client::game::players::ClientPlayers;
use anyhow::Result;

use crate::gfx;
use crate::libraries::events::Event;
use crate::shared::entities::{Entities, HealthChangePacket, PositionComponent};
use crate::shared::mod_manager::ModManager;
use crate::shared::packet::Packet;
use crate::shared::players::PLAYER_WIDTH;

const HEART_WIDTH: f32 = 33.0;

pub struct ClientHealth {
    heart_texture: gfx::Texture,
    health: i32,
    max_health: i32,
    hearts_rect_array: gfx::RectArray,
}

impl ClientHealth {
    pub fn new() -> Self {
        Self {
            heart_texture: gfx::Texture::new(),
            health: 0,
            max_health: 0,
            hearts_rect_array: gfx::RectArray::new(),
        }
    }

    fn generate_rect_array(&mut self) {
        self.hearts_rect_array = gfx::RectArray::new();
        let mut health_remaining = self.health;
        for i in 0..self.max_health / 5 {
            let heart_pos =
                gfx::FloatPos(HEART_WIDTH * (i % 10) as f32, HEART_WIDTH * (i / 10) as f32);
            let state = if health_remaining >= 5 {
                health_remaining -= 5;
                0
            } else if health_remaining > 0 {
                let res = 5 - health_remaining;
                health_remaining = 0;
                res
            } else {
                break;
            };

            self.hearts_rect_array.add_rect(
                &gfx::Rect::new(heart_pos, gfx::FloatSize(HEART_WIDTH, HEART_WIDTH)),
                &[
                    gfx::Color::new(255, 255, 255, 255),
                    gfx::Color::new(255, 255, 255, 255),
                    gfx::Color::new(255, 255, 255, 255),
                    gfx::Color::new(255, 255, 255, 255),
                ],
                &gfx::Rect::new(
                    gfx::FloatPos(0.0, 11.0 * state as f32),
                    gfx::FloatSize(11.0, 11.0),
                ),
            );
        }
        self.hearts_rect_array.update();
    }

    pub fn load_resources(&mut self, mods: &ModManager) -> Result<()> {
        let hearts_surface = gfx::Surface::deserialize_from_bytes(
            mods.get_resource("misc:hearts.opa").ok_or_else(|| {
                anyhow::anyhow!("Failed to load misc:hearts.opa from mod manager")
            })?,
        )?;

        self.heart_texture = gfx::Texture::load_from_surface(&hearts_surface);
        self.generate_rect_array();

        Ok(())
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext) {
        let pos_x = graphics.renderer.get_window_size().0 - 10.0 * HEART_WIDTH - gfx::SPACING;
        self.hearts_rect_array.render(
            graphics,
            Some(&self.heart_texture),
            gfx::FloatPos(pos_x, gfx::SPACING),
        );
    }

    pub fn on_event(
        &mut self,
        event: &Event,
        graphics: &gfx::GraphicsContext,
        floating_texts: &mut FloatingTextManager,
        players: &ClientPlayers,
        entities: &Entities,
    ) {
        if let Some(packet) = event.downcast::<Packet>() {
            if let Some(packet) = packet.try_deserialize::<HealthChangePacket>() {
                if self.max_health != 0 {
                    let player_pos = players
                        .get_main_player()
                        .and_then(|player| entities.ecs.get::<&PositionComponent>(player).ok());

                    if let Some(player_pos) = player_pos {
                        let x = player_pos.x() + PLAYER_WIDTH / 2.0;
                        let y = player_pos.y() - 0.5;

                        let (val, color) = match self.health.cmp(&packet.health) {
                            std::cmp::Ordering::Less => {
                                (packet.health - self.health, gfx::Color::new(0, 200, 0, 255))
                            }
                            std::cmp::Ordering::Greater => {
                                (self.health - packet.health, gfx::Color::new(200, 0, 0, 255))
                            }
                            std::cmp::Ordering::Equal => (0, gfx::Color::new(255, 255, 255, 255)),
                        };

                        floating_texts.spawn_text(FloatingText::new(
                            graphics,
                            &(val).to_string(),
                            x,
                            y,
                            3000,
                            color,
                            2.0,
                        ));
                    }
                }

                self.health = packet.health;
                self.max_health = packet.max_health;
                self.generate_rect_array();
            }
        }
    }
}
