use std::cmp::Ordering;

use anyhow::Result;

use crate::client::game::networking::ClientNetworking;
use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::shared::chat::ChatPacket;
use crate::shared::packet::Packet;
use gfx::UiElement;

pub struct ChatLine {
    texture: gfx::Texture,
    back_rect: gfx::RenderRect,
    creation_time: std::time::Instant,
    transparency: i32,
}

impl ChatLine {
    pub fn new(graphics: &gfx::GraphicsContext, text: &str, pos: gfx::FloatPos) -> Self {
        let texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(text, None));
        let mut back_rect = gfx::RenderRect::new(pos + gfx::FloatPos(-texture.get_texture_size().0 * 3.0, -texture.get_texture_size().1 * 3.0), gfx::FloatSize(0.0, 0.0));
        back_rect.smooth_factor = 60.0;

        Self {
            texture,
            back_rect,
            creation_time: std::time::Instant::now(),
            transparency: 255,
        }
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, focused: bool) {
        let target_transparency = if focused || (self.creation_time.elapsed().as_millis() as i32) < 5000 { 255 } else { 0 };

        match self.transparency.cmp(&target_transparency) {
            Ordering::Greater => self.transparency -= 10,
            Ordering::Less => self.transparency += 10,
            Ordering::Equal => {}
        }

        self.transparency = self.transparency.clamp(0, 255);

        if self.transparency == 0 {
            return;
        }

        self.back_rect.render(graphics, None);
        let pos = self.back_rect.get_container(graphics, None).rect.pos;
        self.texture.render(graphics, 3.0, pos, None, false, Some(gfx::Color::new(255, 255, 255, self.transparency as u8)));
    }

    pub fn set_pos(&mut self, pos: gfx::FloatPos) {
        self.back_rect.pos = pos;
    }

    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.texture.get_texture_size().0 * 3.0, self.texture.get_texture_size().1 * 3.0)
    }
}

pub struct ClientChat {
    back_rect: gfx::RenderRect,
    text_input: gfx::TextInput,
    chat_lines: Vec<ChatLine>,
    waiting_for_t: bool,
}

impl ClientChat {
    pub fn new(graphics: &gfx::GraphicsContext) -> Self {
        Self {
            text_input: gfx::TextInput::new(graphics),
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
            chat_lines: Vec::new(),
            waiting_for_t: false,
        }
    }

    pub fn init(&mut self) {
        self.text_input.orientation = gfx::BOTTOM_LEFT;
        self.text_input.pos = gfx::FloatPos(gfx::SPACING, -gfx::SPACING);
        self.text_input.scale = 3.0;
        self.text_input.border_color = gfx::BORDER_COLOR;

        self.back_rect.fill_color = gfx::TRANSPARENT;
        self.back_rect.orientation = gfx::BOTTOM_LEFT;
        self.back_rect.pos = gfx::FloatPos(gfx::SPACING, -gfx::SPACING);
        self.back_rect.size.1 = self.text_input.get_size().1;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.smooth_factor = 60.0;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext) {
        if self.text_input.selected {
            self.back_rect.size.0 = gfx::TEXT_INPUT_WIDTH * self.text_input.scale;
        } else {
            self.back_rect.size.0 = gfx::TEXT_INPUT_WIDTH * self.text_input.scale * 0.6;
        }

        self.back_rect.render(graphics, None);

        self.text_input.width = self.back_rect.get_container(graphics, None).rect.size.0 / self.text_input.scale;
        self.text_input.render(graphics, None);

        let mut curr_y = graphics.get_window_size().1 - gfx::SPACING - self.text_input.get_size().1;
        for line in self.chat_lines.iter_mut().rev() {
            curr_y -= line.get_size().1;
            line.set_pos(gfx::FloatPos(gfx::SPACING, curr_y));
            line.render(graphics, self.text_input.selected);
        }
    }

    pub fn on_event(&mut self, event: &Event, graphics: &mut gfx::GraphicsContext, networking: &mut ClientNetworking) -> Result<bool> {
        if let Some(event) = event.downcast::<gfx::Event>() {
            if let gfx::Event::TextInput(..) = event {
                if self.waiting_for_t {
                    self.waiting_for_t = false;
                    return Ok(true);
                }
            }

            self.text_input.on_event(graphics, event, None);

            if let gfx::Event::KeyPress(gfx::Key::Enter, ..) = event {
                if self.text_input.selected && !self.text_input.get_text().is_empty() {
                    networking.send_packet(Packet::new(ChatPacket {
                        message: self.text_input.get_text().clone(),
                    })?)?;

                    self.text_input.set_text(String::new());
                }
            } else if let gfx::Event::KeyPress(gfx::Key::T, ..) = event {
                if !self.is_selected() {
                    self.text_input.selected = true;
                    self.waiting_for_t = true;
                }
            } else if let gfx::Event::KeyPress(gfx::Key::Escape, ..) = event {
                if self.is_selected() {
                    self.text_input.selected = false;
                    return Ok(true);
                }
            }
        } else if let Some(event) = event.downcast::<Packet>() {
            if let Some(packet) = event.try_deserialize::<ChatPacket>() {
                self.chat_lines.push(ChatLine::new(
                    graphics,
                    &packet.message,
                    gfx::FloatPos(0.0, graphics.get_window_size().1 - gfx::SPACING - self.text_input.get_size().1),
                ));
            }
        }
        Ok(self.is_selected() && event.downcast::<gfx::Event>().is_some())
    }

    pub const fn is_selected(&self) -> bool {
        self.text_input.selected
    }
}
