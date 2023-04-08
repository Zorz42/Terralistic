use crate::libraries::graphics as gfx;
use crate::server::server_ui::UiMessageType;
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
    ) {
        self.sprite.render(graphics_context, Some(container));
    }
}

pub struct Console {
    text_lines: Vec<ConsoleLine>,
    container: gfx::Container,
    sender: Option<Sender<Vec<u8>>>,
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
        }
    }
}

impl ui_manager::ModuleTrait for Console {
    fn init(&mut self, _graphics_context: &mut gfx::GraphicsContext) {
        //empty, nothing to do
    }

    fn update(&mut self, _delta_time: f32, _graphics_context: &mut gfx::GraphicsContext) {
        //TODO: add scroll
    }

    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext) {
        for line in &mut self.text_lines {
            line.render(graphics_context, &self.container);
        }
    }

    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    ) {
        if let UiMessageType::ConsoleMessage(message) = message {
            //add a line
            let line = ConsoleLine::new(graphics_context, message.clone());
            //move all lines up
            for line in &mut self.text_lines {
                line.sprite.pos.1 -= line.sprite.texture.get_texture_size().1 + gfx::SPACING / 2.0;
            }
            self.text_lines.push(line);
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
}
