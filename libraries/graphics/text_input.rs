use super::theme::{
    GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR, GFX_DEFAULT_TEXT_INPUT_COLOR,
    GFX_DEFAULT_TEXT_INPUT_HOVER_BORDER_COLOR, GFX_DEFAULT_TEXT_INPUT_HOVER_COLOR,
    GFX_DEFAULT_TEXT_INPUT_PADDING, GFX_DEFAULT_TEXT_INPUT_SHADOW_INTENSITY,
    GFX_DEFAULT_TEXT_INPUT_WIDTH,
};
use super::{
    Color, Container, Event, GraphicsContext, Key, Orientation, Rect, RenderRect, Texture, GREY,
    TOP_LEFT, WHITE,
};
use crate::libraries::graphics::{FloatPos, FloatSize};
use copypasta::ClipboardProvider;

const SPACE_CHARACTERS: [char; 3] = [' ', '-', '_'];

pub struct TextInput {
    pub pos: FloatPos,
    pub orientation: Orientation,
    pub width: f32,
    hint_texture: Texture,
    pub padding: f32,
    pub scale: f32,
    pub color: Color,
    pub border_color: Color,
    pub hover_color: Color,
    pub hover_border_color: Color,
    hover_progress: f32,
    cursor_color_progress: f32,
    hint_color_progress: f32,
    timer: std::time::Instant,
    timer_counter: u32,
    pub text: String,
    text_texture: Texture,
    text_changed: bool,
    pub selected: bool,
    pub shadow_intensity: i32,
    cursor: (usize, usize),
    cursor_rect: RenderRect,
    // text_processing is a closure, that takes a char and returns a char
    pub text_processing: Option<Box<dyn Fn(char) -> Option<char>>>,
}

impl TextInput {
    pub fn new(graphics: &mut GraphicsContext) -> Self {
        let mut cursor_rect = RenderRect::new(FloatPos(0.0, 0.0), FloatSize(1.0, 1.0));
        cursor_rect.smooth_factor = 30.0;
        cursor_rect.fill_color = WHITE;

        Self {
            pos: FloatPos(0.0, 0.0),
            orientation: TOP_LEFT,
            width: GFX_DEFAULT_TEXT_INPUT_WIDTH,
            hint_texture: Texture::new(),
            padding: GFX_DEFAULT_TEXT_INPUT_PADDING,
            scale: 1.0,
            color: GFX_DEFAULT_TEXT_INPUT_COLOR,
            border_color: GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR,
            hover_color: GFX_DEFAULT_TEXT_INPUT_HOVER_COLOR,
            hover_border_color: GFX_DEFAULT_TEXT_INPUT_HOVER_BORDER_COLOR,
            hover_progress: 0.0,
            cursor_color_progress: 0.0,
            hint_color_progress: 1.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
            text: String::new(),
            text_texture: Texture::load_from_surface(&graphics.font.create_text_surface("")),
            text_changed: true,
            selected: false,
            shadow_intensity: GFX_DEFAULT_TEXT_INPUT_SHADOW_INTENSITY,
            cursor: (0, 0),
            cursor_rect,
            text_processing: None,
        }
    }

    #[must_use]
    pub fn get_size(&self) -> FloatSize {
        FloatSize(
            (self.width + self.padding * 2.0) * self.scale,
            (self.text_texture.get_texture_size().1 + self.padding * 2.0) * self.scale,
        )
    }

    /// Generates the container for the text input. It it private, since a text input should never contain other elements.
    fn get_container(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> Container {
        let mut container = Container::new(
            graphics,
            self.pos,
            self.get_size(),
            self.orientation,
            parent_container,
        );
        container
    }

    /// Checks if the button is hovered with a mouse
    #[must_use]
    pub fn is_hovered(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> bool {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_pos = graphics.renderer.get_mouse_pos();
        mouse_pos.0 >= rect.pos.0
            && mouse_pos.0 <= rect.pos.0 + rect.size.0
            && mouse_pos.1 >= rect.pos.1
            && mouse_pos.1 <= rect.pos.1 + rect.size.1
    }

    /// returns the text in the input box
    #[must_use]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// sets the text in the input box
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed = true;
    }

    /// sets the hint text in the input box
    pub fn set_hint(&mut self, graphics: &mut GraphicsContext, hint: &str) {
        self.hint_texture = Texture::load_from_surface(&graphics.font.create_text_surface(hint));
    }

    /// returns the cursor in order
    const fn get_cursor(&self) -> (usize, usize) {
        if self.cursor.0 > self.cursor.1 {
            (self.cursor.1, self.cursor.0)
        } else {
            (self.cursor.0, self.cursor.1)
        }
    }

    /// finds a space character on the right
    fn find_space_right(&self, mut initial_pos: usize, is_ctrl_pressed: bool) -> usize {
        if initial_pos < self.text.len() {
            initial_pos += 1;
        }

        if is_ctrl_pressed {
            while initial_pos < self.text.len()
                && !SPACE_CHARACTERS.contains(&self.text.chars().nth(initial_pos).unwrap_or('\0'))
            {
                initial_pos += 1;
            }
        }
        initial_pos
    }

    /// finds a space character on the left
    fn find_space_left(&self, mut initial_pos: usize, is_ctrl_pressed: bool) -> usize {
        // subtract 1 if initial_pos is bigger than 0
        initial_pos = initial_pos.saturating_sub(1);

        if is_ctrl_pressed {
            while initial_pos > 0
                && !SPACE_CHARACTERS
                    .contains(&self.text.chars().nth(initial_pos - 1).unwrap_or('\0'))
            {
                initial_pos -= 1;
            }
        }
        initial_pos
    }

    /// renders the text input
    #[allow(clippy::too_many_lines)] // TODO: split this function up
    pub fn render(&mut self, graphics: &GraphicsContext, parent_container: Option<&Container>) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        if self.text_changed && !self.text.is_empty() {
            self.text_texture =
                Texture::load_from_surface(&graphics.font.create_text_surface(&self.text));
        }

        let hover_progress_target = if self.is_hovered(graphics, parent_container) {
            1.0
        } else {
            0.0
        };
        let cursor_color_progress_target = if self.selected { 0.5 } else { 0.0 };
        let hint_color_progress_target = if self.text.is_empty() { 1.0 } else { 0.0 };

        while self.timer_counter < self.timer.elapsed().as_millis() as u32 {
            let mut smooth_factor = 40.0;
            if hover_progress_target < self.hover_progress {
                smooth_factor *= 10.0;
            }
            self.hover_progress += (hover_progress_target - self.hover_progress) / smooth_factor;

            self.cursor_color_progress +=
                (cursor_color_progress_target - self.cursor_color_progress) / 40.0;

            self.hint_color_progress +=
                (hint_color_progress_target - self.hint_color_progress) / 40.0;

            if (hover_progress_target - self.hover_progress).abs() <= 0.01 {
                self.hover_progress = hover_progress_target;
            }
            self.timer_counter += 1;
        }

        let color = Color::new(
            (self.hover_color.r as f32 * self.hover_progress
                + self.color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.g as f32 * self.hover_progress
                + self.color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.b as f32 * self.hover_progress
                + self.color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_color.a as f32 * self.hover_progress
                + self.color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        let border_color = Color::new(
            (self.hover_border_color.r as f32 * self.hover_progress
                + self.border_color.r as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.g as f32 * self.hover_progress
                + self.border_color.g as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.b as f32 * self.hover_progress
                + self.border_color.b as f32 * (1.0 - self.hover_progress)) as u8,
            (self.hover_border_color.a as f32 * self.hover_progress
                + self.border_color.a as f32 * (1.0 - self.hover_progress)) as u8,
        );

        rect.render(graphics, color);
        rect.render_outline(graphics, border_color);

        graphics.renderer.shadow_context.render(
            graphics,
            rect,
            self.shadow_intensity as f32 / 255.0,
        );

        let mut src_rect = Rect::new(FloatPos(0.0, 0.0), self.text_texture.get_texture_size());
        src_rect.size.0 = f32::min(src_rect.size.0, self.width);
        if self.text.is_empty() {
            src_rect.size.0 = 0.0;
        }
        src_rect.pos.0 = self.text_texture.get_texture_size().0 - src_rect.size.0;

        self.hint_texture.render(
            &graphics.renderer,
            self.scale,
            FloatPos(
                rect.pos.0 + rect.size.0 / 2.0
                    - self.hint_texture.get_texture_size().0 / 2.0 * self.scale,
                rect.pos.1 + self.padding as f32 * self.scale,
            ),
            None,
            false,
            Some(GREY.set_a((255.0 * self.hint_color_progress) as u8)),
        );

        if !self.text.is_empty() {
            self.text_texture.render(
                &graphics.renderer,
                self.scale,
                FloatPos(
                    rect.pos.0 + self.padding * self.scale,
                    rect.pos.1 + rect.size.1 / 2.0
                        - self.text_texture.get_texture_size().1 * self.scale / 2.0,
                ),
                Some(src_rect),
                false,
                None,
            );
        }

        if self.text_changed || self.selected {
            let texture_width = if self.text.is_empty() {
                0.0
            } else {
                self.text_texture.get_texture_size().0 * self.scale
            };

            let text_begin_x = f32::min(
                rect.pos.0 + self.padding * self.scale,
                rect.pos.0 + self.padding * self.scale + rect.size.0 - texture_width,
            );

            // w1 is the width of the text before the cursor.0
            let w1 = if self.get_cursor().0 == 0 {
                0.0
            } else {
                graphics
                    .font
                    .create_text_surface(self.text.get(..self.get_cursor().0).unwrap_or(""))
                    .get_size()
                    .0 as f32
                    * self.scale
            };

            // w2 is the width of the text before the cursor.1
            let w2 = if self.get_cursor().1 == 0 {
                0.0
            } else {
                graphics
                    .font
                    .create_text_surface(self.text.get(..self.get_cursor().1).unwrap_or(""))
                    .get_size()
                    .0 as f32
                    * self.scale
            };

            let x1 = text_begin_x + w1 - 3.0;
            let x2 = text_begin_x + w2 + 1.0;

            self.cursor_rect.pos.0 = x1;
            self.cursor_rect.pos.1 = rect.pos.1 + self.padding * self.scale;
            self.cursor_rect.size.0 = x2 - x1;
            self.cursor_rect.size.1 = rect.size.1 - self.padding * self.scale * 2.0;

            if self.cursor_rect.get_container(graphics, None).rect.pos.0 == 0.0
                && self.cursor_rect.get_container(graphics, None).rect.pos.1 == 0.0
            {
                self.cursor_rect.jump_to_target();
            }
        }

        self.cursor_rect.fill_color.a = (255.0 * self.cursor_color_progress) as u8;

        self.cursor_rect.render(graphics, None);

        self.text_changed = false;
    }

    #[allow(clippy::too_many_lines)] // TODO: split this up
    #[allow(clippy::cognitive_complexity)]
    pub fn on_event(
        &mut self,
        event: &Event,
        graphics: &mut GraphicsContext,
        parent_container: Option<&Container>,
    ) {
        match event {
            Event::TextInput(text) => {
                if self.selected {
                    if self.cursor.0 != self.cursor.1 {
                        self.text
                            .replace_range(self.get_cursor().0..self.get_cursor().1, "");
                        self.cursor.1 = self.cursor.0;
                    }
                    // run every character of text through text_processing closure if it exists and create new string
                    let mut new_text = String::new();
                    for c in text.chars() {
                        if let Some(text_processing) = &self.text_processing {
                            if let Some(c) = text_processing(c) {
                                new_text.push(c);
                            }
                        } else {
                            new_text.push(c);
                        }
                    }

                    self.text.insert_str(self.cursor.0, &new_text);
                    self.cursor.0 += new_text.len();
                    self.text_changed = true;
                    self.cursor.1 = self.cursor.0;
                }
            }

            Event::KeyPress(key, ..) => {
                if key == &Key::MouseLeft {
                    self.selected = self.is_hovered(graphics, parent_container);
                }

                if !self.selected {
                    return;
                }

                match key {
                    Key::Backspace => {
                        if self.cursor.0 != self.cursor.1 {
                            self.text
                                .replace_range(self.get_cursor().0..self.get_cursor().1, "");
                            self.cursor.0 = self.get_cursor().0;
                        } else if self.cursor.0 > 0 {
                            self.cursor.0 = self.find_space_left(
                                self.cursor.0,
                                graphics.renderer.get_key_state(Key::LeftControl),
                            );
                            self.text.replace_range(self.cursor.0..self.cursor.1, "");
                        }
                        self.cursor.1 = self.cursor.0;
                        self.text_changed = true;
                    }
                    Key::Delete => {
                        if self.cursor.0 != self.cursor.1 {
                            self.text
                                .replace_range(self.get_cursor().0..self.get_cursor().1, "");
                            self.cursor.0 = self.get_cursor().0;
                            self.cursor.1 = self.cursor.0;
                        } else if self.text.len() > self.cursor.0 {
                            self.cursor.1 = self.find_space_right(
                                self.cursor.0,
                                graphics.renderer.get_key_state(Key::LeftControl),
                            );
                            self.text.replace_range(self.cursor.0..self.cursor.1, "");
                            self.cursor.1 = self.cursor.0;
                        }
                        self.text_changed = true;
                    }
                    Key::Left => {
                        if graphics.renderer.get_key_state(Key::LeftShift) {
                            self.cursor.1 = self.find_space_left(
                                self.cursor.1,
                                graphics.renderer.get_key_state(Key::LeftControl),
                            );
                        } else {
                            if self.cursor.0 != self.cursor.1 {
                                self.cursor.0 = self.get_cursor().0;
                            } else if self.cursor.0 > 0 {
                                self.cursor.0 = self.find_space_left(
                                    self.cursor.0,
                                    graphics.renderer.get_key_state(Key::LeftControl),
                                );
                            }

                            self.cursor.1 = self.cursor.0;
                        }
                    }
                    Key::Right => {
                        if graphics.renderer.get_key_state(Key::LeftShift) {
                            self.cursor.1 = self.find_space_right(
                                self.cursor.1,
                                graphics.renderer.get_key_state(Key::LeftControl),
                            );
                        } else {
                            if self.cursor.0 != self.cursor.1 {
                                self.cursor.0 = self.get_cursor().1;
                            } else if self.cursor.0 < self.text.len() {
                                self.cursor.0 = self.find_space_right(
                                    self.cursor.0,
                                    graphics.renderer.get_key_state(Key::LeftControl),
                                );
                            }
                            self.cursor.1 = self.cursor.0;
                        }
                    }
                    Key::C => {
                        if graphics.renderer.get_key_state(Key::LeftControl) {
                            graphics
                                .renderer
                                .clipboard_context
                                .set_contents(
                                    self.text
                                        .get(self.get_cursor().0..self.get_cursor().1)
                                        .unwrap_or("")
                                        .to_owned(),
                                )
                                .unwrap_or_else(|e| {
                                    println!("Error setting clipboard contents: {e}");
                                });
                        }
                    }
                    Key::V => {
                        if graphics.renderer.get_key_state(Key::LeftControl) {
                            if let Ok(text) = graphics.renderer.clipboard_context.get_contents() {
                                if self.cursor.0 != self.cursor.1 {
                                    self.text.replace_range(
                                        self.get_cursor().0..self.get_cursor().1,
                                        "",
                                    ); //add text filtering lol
                                    self.cursor.0 = self.get_cursor().0;
                                }
                                self.text.insert_str(self.cursor.0, &text);
                                self.cursor.0 += text.len();
                                self.cursor.1 = self.cursor.0;
                                self.text_changed = true;
                            }
                        }
                    }
                    Key::X => {
                        if graphics.renderer.get_key_state(Key::LeftControl)
                            && self.cursor.0 != self.cursor.1
                        {
                            if graphics.renderer.get_key_state(Key::LeftControl) {
                                graphics
                                    .renderer
                                    .clipboard_context
                                    .set_contents(
                                        self.text
                                            .get(self.get_cursor().0..self.get_cursor().1)
                                            .unwrap_or("")
                                            .to_owned(),
                                    )
                                    .unwrap_or_else(|e| {
                                        println!("Failed to copy to clipboard {e}");
                                    });
                            }
                            self.text
                                .replace_range(self.get_cursor().0..self.get_cursor().1, ""); //add text filtering lol
                            self.cursor.0 = self.get_cursor().0;
                            self.cursor.1 = self.cursor.0;
                            self.text_changed = true;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
