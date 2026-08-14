use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::ui;
use crate::libraries::ui::{BaseUiElement, UiElement};
use crate::shared::packet::Packet;
use crate::shared::players::RespawnPacket;
use anyhow::Result;

use crate::libraries::ui::UiContext;
pub struct RespawnScreen {
    respawn_button: ui::Button,
    respawn_text: ui::Sprite,
    back_rect: ui::RenderRect,
    pub is_shown: bool,
}

impl RespawnScreen {
    pub fn new() -> Self {
        Self {
            respawn_button: ui::Button::new(|| {}),
            respawn_text: ui::Sprite::new(),
            back_rect: ui::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            is_shown: false,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext) {
        self.respawn_button.orientation = ui::CENTER;
        self.respawn_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Respawn", None));
        self.respawn_button.scale = 3.0;

        self.respawn_text.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("You died", None)));
        self.respawn_text.pos.1 = -100.0;
        self.respawn_text.orientation = ui::CENTER;
        self.respawn_text.scale = 3.0;

        self.back_rect.orientation = ui::CENTER;
        self.back_rect.size.0 = 300.0;
        self.back_rect.fill_color = ui::BLACK;
        self.back_rect.fill_color.a = ui::TRANSPARENCY;
        self.back_rect.border_color = ui::BORDER_COLOR;
        self.back_rect.blur_radius = ui::BLUR;
        self.back_rect.shadow_intensity = ui::SHADOW_INTENSITY;
    }

    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) {
        if self.is_shown {
            gfx::Rect::new(gfx::FloatPos(0.0, 0.0), graphics.get_window_size()).render(graphics, gfx::Color::new(255, 0, 0, 100));

            let window_container = ui::Container::default(graphics);

            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.render(graphics, &self.back_rect.get_container(graphics, &window_container));

            self.respawn_text.render(graphics, &self.back_rect.get_container(graphics, &window_container));

            self.respawn_button.render(graphics, &self.back_rect.get_container(graphics, &window_container));
            //TODO UI element
        }
    }

    pub fn on_event(&self, event: &Event, graphics: &gfx::GraphicsContext, networking: &mut ClientNetworking) -> Result<()> {
        if self.is_shown {
            if let Some(gfx::Event::KeyPress(gfx::Key::MouseLeft, ..)) = event.downcast::<gfx::Event>() {
                if self.respawn_button.is_hovered(graphics, &self.back_rect.get_container(graphics, &ui::Container::default(graphics))) {
                    let packet = Packet::new(RespawnPacket {})?;
                    networking.send_packet(packet)?;
                }
            }
        }

        Ok(())
    }
}
