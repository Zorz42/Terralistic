use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::packet::Packet;
use crate::shared::players::RespawnPacket;
use anyhow::Result;
use gfx::{BaseUiElement, UiElement};

pub struct RespawnScreen {
    respawn_button: gfx::Button,
    respawn_text: gfx::Sprite,
    back_rect: gfx::RenderRect,
    pub is_shown: bool,
}

impl RespawnScreen {
    pub fn new() -> Self {
        Self {
            respawn_button: gfx::Button::new(|| {}),
            respawn_text: gfx::Sprite::new(),
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            is_shown: false,
        }
    }

    pub fn init(&mut self, graphics: &gfx::GraphicsContext) {
        self.respawn_button.orientation = gfx::CENTER;
        self.respawn_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Respawn", None));
        self.respawn_button.scale = 3.0;

        self.respawn_text.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("You died", None));
        self.respawn_text.pos.1 = -100.0;
        self.respawn_text.orientation = gfx::CENTER;
        self.respawn_text.scale = 3.0;

        self.back_rect.orientation = gfx::CENTER;
        self.back_rect.size.0 = 300.0;
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.border_color = gfx::BORDER_COLOR;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    }

    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext) {
        if self.is_shown {
            gfx::Rect::new(gfx::FloatPos(0.0, 0.0), graphics.get_window_size()).render(graphics, gfx::Color::new(255, 0, 0, 100));

            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.render(graphics, Some(&self.back_rect.get_container(graphics, None)));

            self.respawn_text.render(graphics, Some(&self.back_rect.get_container(graphics, None)), None);

            self.respawn_button.render(graphics, Some(&self.back_rect.get_container(graphics, None)));
        }
    }

    pub fn on_event(&mut self, event: &Event, graphics: &gfx::GraphicsContext, networking: &mut ClientNetworking) -> Result<()> {
        if self.is_shown {
            if let Some(gfx::Event::KeyPress(gfx::Key::MouseLeft, ..)) = event.downcast::<gfx::Event>() {
                if self.respawn_button.is_hovered(graphics, Some(&self.back_rect.get_container(graphics, None))) {
                    let packet = Packet::new(RespawnPacket {})?;
                    networking.send_packet(packet)?;
                }
            }
        }

        Ok(())
    }
}
