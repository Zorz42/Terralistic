use std::sync::mpsc::Sender;

use crate::libraries::graphics as gfx;
use crate::server::server_ui::{ConsoleMessageType, UiMessageType, EDGE_SPACING};

use super::ui_manager;

//this function formats the string to add the timestamp
fn format_timestamp(message: &String) -> String {
    let timestamp = chrono::Local::now().naive_local().timestamp();
    let timestamp = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
    format!(
        "[{}] {}",
        timestamp.map_or_else(
            || "???".to_owned(),
            |time| time.format("%m-%d %H:%M:%S").to_string(),
        ),
        message
    )
}

pub struct ConsoleLine {
    //currently kinda useless but can be used for sprite loading/unloading to prevent gpu memory usage when not on screen. Will only be useful when a lot of messages get sent to the ui console
    _text: String,
    sprite: gfx::Sprite,
}

impl ConsoleLine {
    pub fn new(graphics_context: &gfx::GraphicsContext, text: String) -> Self {
        let mut sprite = gfx::Sprite::new();
        let font = graphics_context
            .font_mono
            .as_ref()
            .map_or(&graphics_context.font, |mono_font| mono_font);
        sprite.texture = gfx::Texture::load_from_surface(&font.create_text_surface(&text, None));
        sprite.orientation = gfx::BOTTOM_LEFT;
        sprite.color = gfx::WHITE;
        sprite.pos = gfx::FloatPos(gfx::SPACING / 3.0, 0.0);

        Self {
            _text: text,
            sprite,
        }
    }

    pub fn render(
        &mut self,
        graphics_context: &gfx::GraphicsContext,
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
    sender: Option<Sender<UiMessageType>>,
    input: gfx::TextInput,
    scroll: f32,
    enabled: bool,
}

impl Console {
    pub fn new(graphics_context: &gfx::GraphicsContext) -> Self {
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
            enabled: false,
        }
    }

    fn add_line(
        &mut self,
        message: String,
        graphics_context: &gfx::GraphicsContext,
        color: gfx::Color,
    ) {
        //add a line
        let mut line = ConsoleLine::new(graphics_context, message);
        line.sprite.color = color;
        self.text_lines.push(line);
        self.position_lines();
    }

    //repositions the lines when new lines are added or the view is scrolled. TODO: Is inefficient. Fix
    fn position_lines(&mut self) {
        let mut offset;
        let mut y = -self.input.pos.1 - self.input.get_size().1 - gfx::SPACING / 2.0;
        for line in self.text_lines.iter_mut().rev() {
            offset = line.sprite.texture.get_texture_size().1 + EDGE_SPACING;
            line.sprite.pos.1 = y + self.scroll;
            y -= offset;
        }
    }
}

impl ui_manager::ModuleTrait for Console {
    fn init(&mut self, _graphics_context: &mut gfx::GraphicsContext) {
        //initializes the input box
        self.input.pos = gfx::FloatPos(EDGE_SPACING, -EDGE_SPACING);
        self.input.scale = 2.0;
        self.input.orientation = gfx::BOTTOM_LEFT;

        self.input.text_processing = Some(Box::new(|text: char| text.is_ascii().then_some(text)));

        self.input.shadow_intensity = 0;
    }

    fn update(&mut self, _delta_time: f32, _graphics_context: &mut gfx::GraphicsContext) {
        self.input.width = self.container.rect.size.0 / 2.0 - EDGE_SPACING * 2.0;
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
        if let UiMessageType::SrvToUiConsoleMessage(message) = message {
            //extract the string from the message enum and color the sprite
            let mut color = gfx::Color::new(200, 200, 200, 255);
            let message = match message {
                ConsoleMessageType::Info(message) => message,
                ConsoleMessageType::Warning(message) => {
                    color = gfx::Color::new(200, 200, 0, 255);
                    message
                }
                ConsoleMessageType::Error(message) => {
                    color = gfx::Color::new(200, 0, 0, 255);
                    message
                }
            };

            self.add_line(message.to_string(), graphics_context, color);
        }
    }

    fn get_container_mut(&mut self) -> &mut gfx::Container {
        &mut self.container
    }

    fn get_name(&self) -> &str {
        "console"
    }

    fn set_sender(&mut self, sender: Sender<UiMessageType>) {
        self.sender = Some(sender);
    }

    fn on_event(&mut self, event: &gfx::Event, graphics_context: &mut gfx::GraphicsContext) {
        self.input
            .on_event(event, graphics_context, Some(&self.container));
        match event {
            //move the view on mouse scroll
            gfx::Event::MouseScroll(scroll) => {
                let offset = self.text_lines.first().map_or_else(
                    || 0.0,
                    |line| line.sprite.texture.get_texture_size().1 + EDGE_SPACING,
                );
                self.scroll += scroll * offset;
                self.scroll = self.scroll.min(offset * (self.text_lines.len() - 1) as f32);
                self.scroll = self.scroll.max(0.0);
                self.position_lines();
            }
            gfx::Event::KeyPress(key, _repeat) => {
                //if enter is pressed, send the message to the server
                if matches!(key, gfx::Key::Enter)
                    && self.input.selected
                    && !self.input.get_text().is_empty()
                {
                    send_to_srv(
                        UiMessageType::UiToSrvConsoleMessage(self.input.get_text().clone()),
                        self.sender.as_ref(),
                    );
                    let message = format_timestamp(&format!(
                        "[console_input] {}",
                        self.input.get_text().clone()
                    ));
                    self.add_line(
                        message.clone(),
                        graphics_context,
                        gfx::Color::new(200, 200, 200, 255),
                    );
                    println!("{message}");
                    self.input.set_text(String::new());
                }
            }
            _ => {}
        }
    }
    fn get_enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
}

//sends any data to the server
pub fn send_to_srv(data: UiMessageType, ui_event_sender: Option<&Sender<UiMessageType>>) {
    if let Some(sender) = ui_event_sender {
        let result = sender.send(data);

        if let Err(_e) = result {
            println!("error sending data to srv");
        }
    }
}
