use crate::libraries::graphics as gfx;
use crate::server::server_ui::{UiMessageType, EDGE_SPACING};
use std::sync::mpsc::Sender;

use super::ui_manager;

pub struct ConsoleLine {
    text: String,
    sprite: gfx::Sprite,
}

impl ConsoleLine {
    pub fn new(graphics_context: &mut gfx::GraphicsContext, text: String) -> Self {
        let mut a = Self {
            text,
            sprite: gfx::Sprite::new(),
        };
        a.sprite.texture =
            gfx::Texture::load_from_surface(&graphics_context.font.create_text_surface(&a.text));
        a.sprite.scale = 1.4;
        a.sprite.orientation = gfx::BOTTOM_LEFT;
        a.sprite.color = gfx::WHITE;
        a.sprite.pos = gfx::FloatPos(gfx::SPACING / 3.0, 0.0);
        a
    }

    pub fn render(
        &mut self,
        graphics_context: &mut gfx::GraphicsContext,
        container: &gfx::Container,
        max_y: f32,
        min_y: f32,
    ) {
        if self.sprite.pos.1 < max_y
            && self.sprite.pos.1 + self.sprite.texture.get_texture_size().1 > min_y
        {
            self.sprite.render(graphics_context, Some(container));
        }
    }
}

pub struct Console {
    text_lines: Vec<ConsoleLine>,
    container: gfx::Container,
    sender: Option<Sender<Vec<u8>>>,
    input: gfx::TextInput,
    scroll: f32,
}

impl Console {
    pub fn new(graphics_context: &mut gfx::GraphicsContext) -> Self {
        Self {
            text_lines: Vec::new(),
            container: gfx::Container::new(
                graphics_context,
                gfx::FloatPos(0.0, 0.0),
                gfx::FloatSize(0.0, 0.0),
                gfx::TOP_LEFT,
                None,
            ),
            sender: None,
            input: gfx::TextInput::new(graphics_context),
            scroll: 0.0,
        }
    }

    fn add_line(&mut self, message: String, graphics_context: &mut gfx::GraphicsContext) {
        //add a line
        let line = ConsoleLine::new(graphics_context, message);
        self.text_lines.push(line);
        self.position_lines();
    }

    fn position_lines(&mut self) {
        let mut y = -self.input.pos.1 - self.input.get_size().1 - gfx::SPACING / 2.0;
        for line in self.text_lines.iter_mut().rev() {
            line.sprite.pos.1 = y + self.scroll;
            y -= line.sprite.texture.get_texture_size().1 + gfx::SPACING / 2.0;
        }
    }
}

impl ui_manager::ModuleTrait for Console {
    fn init(&mut self, _graphics_context: &mut gfx::GraphicsContext) {
        self.input.pos = gfx::FloatPos(EDGE_SPACING, -EDGE_SPACING);
        self.input.scale = 2.0;
        self.input.orientation = gfx::BOTTOM_LEFT;

        self.input.text_processing = Some(Box::new(|text: char| text.is_ascii().then_some(text)));

        self.input.shadow_intensity = 0;
    }

    fn update(&mut self, _delta_time: f32, _graphics_context: &mut gfx::GraphicsContext) {
        self.input.width = self.container.rect.size.0 / 2.0 - EDGE_SPACING * 2.0;
        //TODO: add scroll
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        let max_y = -self.input.pos.1 - self.input.get_size().1 - gfx::SPACING / 2.0 + 1.0;
        let min_y = -self.container.rect.size.1 - max_y;
        for line in &mut self.text_lines {
            line.render(graphics_context, &self.container, max_y, min_y);
        }
        self.input.render(graphics_context, Some(&self.container));
    }

    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    ) {
        if let UiMessageType::ConsoleMessage(message) = message {
            self.add_line(message.clone(), graphics_context);
        }
    }

    fn get_container_mut(&mut self) -> &mut gfx::Container {
        &mut self.container
    }

    fn get_name(&self) -> &str {
        "Console"
    }

    fn set_sender(&mut self, sender: Sender<Vec<u8>>) {
        self.sender = Some(sender);
    }

    fn on_event(&mut self, event: &gfx::Event, graphics_context: &mut gfx::GraphicsContext) {
        self.input
            .on_event(event, graphics_context, Some(&self.container));
        match event {
            gfx::Event::MouseScroll(scroll) => {
                self.scroll += scroll * self.text_lines.first().map_or_else(|| 0.0, |line| line.sprite.texture.get_texture_size().1 + gfx::SPACING / 2.0);
                self.scroll = self.scroll.max(0.0);
                self.position_lines();
            }
            gfx::Event::KeyPress(key, _repeat) => {
                if matches!(key, gfx::Key::Enter) && self.input.selected {
                    send_to_srv(
                        &UiMessageType::ConsoleMessage(self.input.get_text().to_owned()),
                        &self.sender,
                    );
                    self.add_line(self.input.text.clone(), graphics_context);
                    self.input.set_text(String::new());
                }
            }
            _ => {}
        }
    }
}

//sends any data to the server
pub fn send_to_srv(data: &UiMessageType, ui_event_sender: &Option<Sender<Vec<u8>>>) {
    if let Some(sender) = ui_event_sender {
        let data = &bincode::serialize(&data).unwrap_or_default();
        let data = snap::raw::Encoder::new()
            .compress_vec(data)
            .unwrap_or_default();
        let result = sender.send(data);

        if let Err(_e) = result {
            println!("error sending data to srv");
        }
    }
}
