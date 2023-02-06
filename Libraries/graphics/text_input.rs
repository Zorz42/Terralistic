use crate::theme::{
    GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR, GFX_DEFAULT_TEXT_INPUT_COLOR,
    GFX_DEFAULT_TEXT_INPUT_HOVER_BORDER_COLOR, GFX_DEFAULT_TEXT_INPUT_HOVER_COLOR,
    GFX_DEFAULT_TEXT_INPUT_PADDING, GFX_DEFAULT_TEXT_INPUT_SHADOW_INTENSITY,
    GFX_DEFAULT_TEXT_INPUT_WIDTH,
};
use crate::{
    Color, Container, Event, GraphicsContext, Key, Orientation, Rect, RenderRect, Texture, GREY,
    TOP_LEFT, WHITE,
};
use copypasta::ClipboardProvider;

const SPACE_CHARACTERS: [char; 3] = [' ', '-', '_'];

pub struct TextInput {
    pub x: i32,
    pub y: i32,
    pub orientation: Orientation,
    pub width: i32,
    hint_texture: Texture,
    pub padding: i32,
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
    pub fn new(graphics: &mut GraphicsContext) -> TextInput {
        let mut cursor_rect = RenderRect::new(0.0, 0.0, 1.0, 1.0);
        cursor_rect.smooth_factor = 30.0;
        cursor_rect.fill_color = WHITE;

        TextInput {
            x: 0,
            y: 0,
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

    /**
    Calculates the width.
     */
    pub fn get_width(&self) -> i32 {
        ((self.width + self.padding * 2) as f32 * self.scale) as i32
    }

    /**
    Calculates the height.
     */
    pub fn get_height(&self) -> i32 {
        ((self.text_texture.get_texture_height() + self.padding * 2) as f32 * self.scale) as i32
    }

    /**
    Generates the container for the text input. It it private, since a text input should never contain other elements.
     */
    fn get_container(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> Container {
        let mut container = Container::new(
            self.x,
            self.y,
            self.get_width(),
            self.get_height(),
            self.orientation,
        );
        container.update(graphics, parent_container);
        container
    }

    /**
    Checks if the button is hovered with a mouse
     */
    pub fn is_hovered(
        &self,
        graphics: &GraphicsContext,
        parent_container: Option<&Container>,
    ) -> bool {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_x = graphics.renderer.get_mouse_x() as i32;
        let mouse_y = graphics.renderer.get_mouse_y() as i32;
        mouse_x >= rect.x
            && mouse_x <= rect.x + rect.w
            && mouse_y >= rect.y
            && mouse_y <= rect.y + rect.h
    }

    /**
    returns the text in the input box
     */
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /**
    sets the text in the input box
     */
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed = true;
    }

    /**
    sets the hint text in the input box
     */
    pub fn set_hint(&mut self, graphics: &mut GraphicsContext, hint: &str) {
        self.hint_texture = Texture::load_from_surface(&graphics.font.create_text_surface(hint));
    }

    /**
    returns the cursor in order
     */
    fn get_cursor(&self) -> (usize, usize) {
        if self.cursor.0 > self.cursor.1 {
            (self.cursor.1, self.cursor.0)
        } else {
            (self.cursor.0, self.cursor.1)
        }
    }

    /**finds a space character on the right*/
    fn find_space_right(&self, mut initial_pos: usize, is_ctrl_pressed: bool) -> usize {
        if initial_pos < self.text.len() {
            initial_pos += 1;
        }
        if is_ctrl_pressed {
            while initial_pos < self.text.len()
                && !SPACE_CHARACTERS.contains(&self.text.chars().nth(initial_pos).unwrap())
            {
                initial_pos += 1;
            }
        }
        initial_pos
    }

    /**finds a space character on the left*/
    fn find_space_left(&self, mut initial_pos: usize, is_ctrl_pressed: bool) -> usize {
        // subtract 1 if initial_pos is bigger than 0
        initial_pos = initial_pos.saturating_sub(1);

        if is_ctrl_pressed {
            while initial_pos > 0
                && !SPACE_CHARACTERS.contains(&self.text.chars().nth(initial_pos - 1).unwrap())
            {
                initial_pos -= 1;
            }
        }
        initial_pos
    }

    /**
    renders the text input
     */
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

        let mut src_rect = Rect::new(
            0,
            0,
            self.text_texture.get_texture_width(),
            self.text_texture.get_texture_height(),
        );
        src_rect.w = i32::min(src_rect.w, self.width);
        if self.text.is_empty() {
            src_rect.w = 0;
        }
        src_rect.x = self.text_texture.get_texture_width() - src_rect.w;

        self.hint_texture.render(
            &graphics.renderer,
            self.scale,
            (
                (rect.x as f32 + rect.w as f32 / 2.0
                    - self.hint_texture.get_texture_width() as f32 / 2.0 * self.scale)
                    as i32,
                rect.y + (self.padding as f32 * self.scale) as i32,
            ),
            None,
            false,
            Some(GREY.set_a((255.0 * self.hint_color_progress) as u8)),
        );

        if !self.text.is_empty() {
            self.text_texture.render(
                &graphics.renderer,
                self.scale,
                (
                    rect.x + (self.padding as f32 * self.scale) as i32,
                    rect.y + rect.h / 2
                        - (self.text_texture.get_texture_height() as f32 * self.scale / 2.0) as i32,
                ),
                Some(src_rect),
                false,
                None,
            );
        }

        if self.text_changed || self.selected {
            let texture_width = if self.text.is_empty() {
                0
            } else {
                (self.text_texture.get_texture_width() as f32 * self.scale) as i32
            };

            let text_begin_x = i32::min(
                rect.x + (self.padding as f32 * self.scale) as i32,
                rect.x + (self.padding as f32 * self.scale) as i32 + rect.w - texture_width,
            );

            // w1 is the width of the text before the cursor.0
            let mut w1 = graphics
                .font
                .create_text_surface(&self.text[..self.get_cursor().0])
                .get_width() as f32
                * self.scale;
            if self.get_cursor().0 == 0 {
                w1 = 0.0;
            }
            // w2 is the width of the text before the cursor.1
            let mut w2 = graphics
                .font
                .create_text_surface(&self.text[..self.get_cursor().1])
                .get_width() as f32
                * self.scale;
            if self.get_cursor().1 == 0 {
                w2 = 0.0;
            }

            let x1 = text_begin_x as f32 + w1 - 3.0;
            let x2 = text_begin_x as f32 + w2 + 1.0;

            self.cursor_rect.x = x1;
            self.cursor_rect.y = rect.y as f32 + self.padding as f32 * self.scale;
            self.cursor_rect.h = rect.h as f32 - self.padding as f32 * self.scale * 2.0;
            self.cursor_rect.w = x2 - x1;

            if self.cursor_rect.get_container(graphics, None).rect.x == 0
                && self.cursor_rect.get_container(graphics, None).rect.y == 0
            {
                self.cursor_rect.jump_to_target();
            }
        }

        self.cursor_rect.fill_color.a = (255.0 * self.cursor_color_progress) as u8;

        self.cursor_rect.render(graphics, None);

        self.text_changed = false;
    }

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
                                    self.text[self.get_cursor().0..self.get_cursor().1].to_string(),
                                )
                                .unwrap();
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
                                        self.text[self.get_cursor().0..self.get_cursor().1]
                                            .to_string(),
                                    )
                                    .unwrap();
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
