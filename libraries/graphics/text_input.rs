use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

use super::theme::{SHADOW_INTENSITY, TEXT_INPUT_BORDER_COLOR, TEXT_INPUT_COLOR, TEXT_INPUT_HOVER_BORDER_COLOR, TEXT_INPUT_HOVER_COLOR, TEXT_INPUT_PADDING, TEXT_INPUT_WIDTH};

const WORD_DELIMITERS: &str = " /\\()\"\'-.,:;<>~!@#$%^&*|+=[]{}~?\u{2502}";

/// A single line editable text field, with a selection, a clipboard and a hint.
pub struct TextInput {
    pub pos: gfx::FloatPos,
    pub orientation: gfx::Orientation,
    pub width: f32,
    hint_texture: gfx::Texture,
    pub padding: f32,
    pub scale: f32,
    pub color: gfx::Color,
    pub border_color: gfx::Color,
    pub hover_color: gfx::Color,
    pub hover_border_color: gfx::Color,
    hover_progress: f32,
    cursor_color_progress: f32,
    hint_color_progress: f32,
    animation_timer: gfx::AnimationTimer,
    text: String,
    text_texture: gfx::Texture,
    text_changed: bool,
    pub selected: bool,
    pub shadow_intensity: i32,
    /// Both ends of the selection, as **byte** offsets into `text`. They are in no particular
    /// order - `get_cursor` sorts them - and the second one is the end the user is dragging.
    cursor: (usize, usize),
    cursor_rect: gfx::RenderRect,
    /// Filters every character on its way in, whether typed or pasted. Returning `None` drops
    /// the character.
    pub text_processing: Option<Box<dyn Fn(char) -> Option<char>>>,
}

impl TextInput {
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext) -> Self {
        Self::with_text_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("", None)))
    }

    /// A text input with no GPU textures behind it, for tests. The text texture is given the
    /// height of one line of the real font and no width, which is what `new` ends up with for
    /// empty text. Only rendering and the *width* of the drawn text depend on the difference.
    #[cfg(test)]
    #[must_use]
    pub fn new_headless() -> Self {
        Self::with_text_texture(gfx::Texture::new_sized(gfx::FloatSize(0.0, 16.0)))
    }

    fn with_text_texture(text_texture: gfx::Texture) -> Self {
        let mut cursor_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(1.0, 1.0));
        cursor_rect.smooth_factor = 30.0;
        cursor_rect.fill_color = gfx::WHITE;

        Self {
            pos: gfx::FloatPos(0.0, 0.0),
            orientation: gfx::TOP_LEFT,
            width: TEXT_INPUT_WIDTH,
            hint_texture: gfx::Texture::new(),
            padding: TEXT_INPUT_PADDING,
            scale: 1.0,
            color: TEXT_INPUT_COLOR,
            border_color: TEXT_INPUT_BORDER_COLOR,
            hover_color: TEXT_INPUT_HOVER_COLOR,
            hover_border_color: TEXT_INPUT_HOVER_BORDER_COLOR,
            hover_progress: 0.0,
            cursor_color_progress: 0.0,
            hint_color_progress: 1.0,
            animation_timer: gfx::AnimationTimer::new(1),
            text: String::new(),
            text_texture,
            text_changed: true,
            selected: false,
            shadow_intensity: SHADOW_INTENSITY,
            cursor: (0, 0),
            cursor_rect,
            text_processing: None,
        }
    }

    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(self.width * self.scale, (self.text_texture.get_texture_size().1 + self.padding * 2.0) * self.scale)
    }

    /// The selection as an ordered `(start, end)` byte range. Exposed for tests, which
    /// otherwise could not see where the cursor ended up.
    #[cfg(test)]
    #[must_use]
    pub const fn get_cursor_range(&self) -> (usize, usize) {
        self.get_cursor()
    }

    #[must_use]
    pub fn is_hovered(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> bool {
        self.get_container(graphics, parent_container).get_absolute_rect().contains(graphics.get_mouse_pos())
    }

    #[must_use]
    pub const fn get_text(&self) -> &String {
        &self.text
    }

    /// Freezes the hover, cursor and hint fades for the golden-image tests.
    #[cfg(feature = "render-tests")]
    pub const fn settle_animation(&mut self) {
        self.animation_timer.freeze();
    }

    /// Replaces the text, clamping each half of the cursor into it separately - clamping the
    /// pair as a tuple compares lexicographically, which leaves the far half out of range.
    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.text_changed = true;
        self.cursor = (self.clamp_into_text(self.cursor.0), self.clamp_into_text(self.cursor.1));
    }

    pub fn set_hint(&mut self, graphics: &gfx::GraphicsContext, hint: &str) {
        self.hint_texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(hint, None));
    }

    /// The selection, lowest offset first.
    const fn get_cursor(&self) -> (usize, usize) {
        if self.cursor.0 > self.cursor.1 {
            (self.cursor.1, self.cursor.0)
        } else {
            (self.cursor.0, self.cursor.1)
        }
    }

    /// The byte index of the character boundary immediately before `pos`.
    ///
    /// **The cursor is a byte offset that moves by characters**, which is the one thing to
    /// keep straight in here. Byte offsets are what `replace_range` and `insert_str` want, but
    /// stepping one *byte* lands inside a multi-byte character and the next edit panics.
    fn prev_boundary(&self, pos: usize) -> usize {
        let mut pos = pos.saturating_sub(1);
        while pos > 0 && !self.text.is_char_boundary(pos) {
            pos -= 1;
        }
        pos
    }

    /// The byte index of the character boundary immediately after `pos`, or the end.
    fn next_boundary(&self, pos: usize) -> usize {
        let mut pos = pos.saturating_add(1).min(self.text.len());
        while pos < self.text.len() && !self.text.is_char_boundary(pos) {
            pos += 1;
        }
        pos
    }

    /// Brings a byte offset inside the text and onto a character boundary.
    fn clamp_into_text(&self, pos: usize) -> usize {
        let mut pos = pos.min(self.text.len());
        while pos > 0 && !self.text.is_char_boundary(pos) {
            pos -= 1;
        }
        pos
    }

    /// The character starting at `pos`, or `None` at the end of the text.
    fn char_at(&self, pos: usize) -> Option<char> {
        self.text.get(pos..).and_then(|rest| rest.chars().next())
    }

    /// One character to the right, or a whole word with the modifier held.
    fn step_right(&self, initial_pos: usize, whole_word: bool) -> usize {
        let mut pos = self.next_boundary(initial_pos);
        if whole_word {
            while pos < self.text.len() && !self.char_at(pos).is_some_and(|c| WORD_DELIMITERS.contains(c)) {
                pos = self.next_boundary(pos);
            }
        }
        pos
    }

    /// One character to the left, or a whole word with the modifier held.
    fn step_left(&self, initial_pos: usize, whole_word: bool) -> usize {
        let mut pos = self.prev_boundary(initial_pos);
        if whole_word {
            while pos > 0 && !self.char_at(self.prev_boundary(pos)).is_some_and(|c| WORD_DELIMITERS.contains(c)) {
                pos = self.prev_boundary(pos);
            }
        }
        pos
    }

    /// The selected text.
    fn selection(&self) -> &str {
        let (start, end) = self.get_cursor();
        self.text.get(start..end).unwrap_or("")
    }

    /// Removes the selection and collapses the cursor onto its start. `false` if there was no
    /// selection to remove.
    fn delete_selection(&mut self) -> bool {
        let (start, end) = self.get_cursor();
        if start == end {
            return false;
        }
        self.text.replace_range(start..end, "");
        // Onto the *start*, not onto `cursor.0`: after a right to left selection that is the
        // end, which no longer exists once the range is gone.
        self.cursor = (start, start);
        self.text_changed = true;
        true
    }

    /// Where the visible window into the text starts, in **texture** pixels.
    ///
    /// Text wider than the box is cropped, and by default it is cropped from the left so that
    /// the end being typed stays visible. That alone is not enough once the field is selected:
    /// the cursor has to stay inside the box too, or moving it left through a long value walks
    /// it out of the left edge and paints a white bar over whatever is beside the input. So the
    /// window is pulled back to whichever end of it the cursor has left.
    fn view_offset(&self, font: &gfx::Font) -> f32 {
        let visible_width = self.visible_width();
        let furthest = f32::max(self.width_up_to(font, self.text.len()) - visible_width, 0.0);

        let tail = furthest;
        if !self.selected {
            return tail;
        }
        // The second half of the cursor is the end the user is moving, so that is the one that
        // has to stay on screen.
        let cursor_x = self.width_up_to(font, self.cursor.1);
        tail.min(cursor_x).max(cursor_x - visible_width).clamp(0.0, furthest)
    }

    /// How much of the text fits between the two paddings, unscaled.
    fn visible_width(&self) -> f32 {
        f32::max(self.width - self.padding * 2.0, 0.0)
    }

    /// How wide the text up to `end` is, unscaled.
    ///
    /// Measured rather than read off `text_texture`, so it is right even on the frame the
    /// texture has not been rebuilt on - and so all of this works with no GPU. It is the same
    /// number: `get_text_size` is what `create_text_surface` sizes itself by. Measuring also
    /// beats rasterising a throwaway surface on every frame the field is selected.
    ///
    /// `get_text_size` never reports a zero width, so an empty prefix is answered directly.
    fn width_up_to(&self, font: &gfx::Font, end: usize) -> f32 {
        if end == 0 {
            return 0.0;
        }
        font.get_text_size(self.text.get(..end).unwrap_or(""), None).0 as f32
    }

    /// The region of the text texture that is drawn, in texture pixels.
    pub(super) fn visible_text_rect(&self, font: &gfx::Font) -> gfx::Rect {
        gfx::Rect::new(
            gfx::FloatPos(self.view_offset(font), 0.0),
            gfx::FloatSize(f32::min(self.width_up_to(font, self.text.len()), self.visible_width()), self.text_texture.get_texture_size().1),
        )
    }

    /// The cursor or selection rectangle, relative to the input's own top left.
    ///
    /// Split out of `render_inner` together with the two above because none of this needs a
    /// GPU - a `Font` to measure with is the whole of it, and `Font::new_headless` is one.
    pub(super) fn cursor_rect(&self, font: &gfx::Font, size: gfx::FloatSize) -> gfx::Rect {
        let text_begin_x = (self.padding - self.view_offset(font)) * self.scale;
        let (start, end) = self.get_cursor();
        // A pixel of lead-in and lead-out, so a collapsed cursor is a visible bar rather than
        // nothing at all.
        let x1 = text_begin_x + self.width_up_to(font, start) * self.scale - 3.0;
        let x2 = text_begin_x + self.width_up_to(font, end) * self.scale + 1.0;

        gfx::Rect::new(gfx::FloatPos(x1, self.padding * self.scale), gfx::FloatSize(x2 - x1, size.1 - self.padding * self.scale * 2.0))
    }

    /// Replaces the selection with `text` and leaves the cursor after it.
    ///
    /// Every route text takes into the field goes through here, so that pasting is filtered by
    /// `text_processing` exactly the way typing is.
    fn insert(&mut self, text: &str) {
        self.delete_selection();

        let filtered = self.text_processing.as_ref().map_or_else(|| text.to_owned(), |process| text.chars().filter_map(process).collect());
        self.text.insert_str(self.cursor.0, &filtered);
        self.cursor.0 += filtered.len();
        self.cursor.1 = self.cursor.0;
        self.text_changed = true;
    }
}

/// Whether the shortcut modifier is held. Control on every platform, and the command key too,
/// which is what a mac keyboard reaches for.
fn shortcut_held(graphics: &dyn gfx::UiContext) -> bool {
    [gfx::Key::LeftControl, gfx::Key::RightControl, gfx::Key::LeftSuper, gfx::Key::RightSuper]
        .into_iter()
        .any(|key| graphics.get_key_state(key))
}

impl UiElement for TextInput {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.cursor_rect]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.cursor_rect]
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        if self.text_changed && !self.text.is_empty() {
            self.text_texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&self.text, None));
        }

        let hover_target = if self.is_hovered(graphics, parent_container) { 1.0 } else { 0.0 };
        let cursor_target = if self.selected { 0.5 } else { 0.0 };
        let hint_target = if self.text.is_empty() { 1.0 } else { 0.0 };

        while self.animation_timer.frame_ready() {
            // Fading out is ten times slower than fading in, so the box lingers under the
            // pointer instead of snapping back.
            let smooth_factor = if hover_target < self.hover_progress { 400.0 } else { 40.0 };
            self.hover_progress = gfx::approach(self.hover_progress, hover_target, smooth_factor, 0.01);
            self.cursor_color_progress = gfx::approach(self.cursor_color_progress, cursor_target, 40.0, 0.0);
            self.hint_color_progress = gfx::approach(self.hint_color_progress, hint_target, 40.0, 0.0);
        }

        rect.render(graphics, gfx::interpolate_colors(self.color, self.hover_color, self.hover_progress));
        rect.render_outline(graphics, gfx::interpolate_colors(self.border_color, self.hover_border_color, self.hover_progress));
        graphics.shadow_context.render(graphics, rect, self.shadow_intensity as f32 / 255.0);

        self.hint_texture.render(
            graphics,
            self.scale,
            gfx::FloatPos(
                rect.pos.0 + rect.size.0 / 2.0 - self.hint_texture.get_texture_size().0 / 2.0 * self.scale,
                rect.pos.1 + self.padding * self.scale,
            ),
            None,
            false,
            Some(gfx::GREY.set_a((255.0 * self.hint_color_progress) as u8)),
        );

        if !self.text.is_empty() {
            self.text_texture.render(
                graphics,
                self.scale,
                gfx::FloatPos(
                    rect.pos.0 + self.padding * self.scale,
                    rect.pos.1 + rect.size.1 / 2.0 - self.text_texture.get_texture_size().1 * self.scale / 2.0,
                ),
                Some(self.visible_text_rect(&graphics.font)),
                false,
                None,
            );
        }

        if self.text_changed || self.selected {
            let cursor = self.cursor_rect(&graphics.font, rect.size);
            self.cursor_rect.pos = cursor.pos;
            self.cursor_rect.size = cursor.size;

            // A cursor that has never been positioned starts at the origin; let it appear
            // where it belongs rather than sliding in from the corner.
            if self.cursor_rect.render_pos == gfx::FloatPos(0.0, 0.0) {
                self.cursor_rect.jump_to_target();
            }
        }

        // The cursor rectangle itself is drawn by the sub-element recursion.
        self.cursor_rect.fill_color.a = (255.0 * self.cursor_color_progress) as u8;
        self.text_changed = false;
    }

    fn on_event_inner(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        match event {
            gfx::Event::TextInput(text) => {
                if self.selected {
                    self.insert(text);
                    return true;
                }
            }
            gfx::Event::KeyPress(key, ..) => {
                if key == &gfx::Key::MouseLeft {
                    self.selected = self.is_hovered(graphics, parent_container);
                }
                if !self.selected {
                    return false;
                }

                // The same modifier that reaches the clipboard also turns a cursor step into a
                // word step.
                let shortcut = shortcut_held(graphics);
                let shift = graphics.get_key_state(gfx::Key::LeftShift) || graphics.get_key_state(gfx::Key::RightShift);

                match key {
                    gfx::Key::Backspace => {
                        if !self.delete_selection() && self.cursor.0 > 0 {
                            self.cursor.0 = self.step_left(self.cursor.0, shortcut);
                            self.text.replace_range(self.cursor.0..self.cursor.1, "");
                            self.cursor.1 = self.cursor.0;
                        }
                        self.text_changed = true;
                    }
                    gfx::Key::Delete => {
                        if !self.delete_selection() && self.cursor.0 < self.text.len() {
                            let end = self.step_right(self.cursor.0, shortcut);
                            self.text.replace_range(self.cursor.0..end, "");
                        }
                        self.text_changed = true;
                    }
                    gfx::Key::Left => {
                        if shift {
                            self.cursor.1 = self.step_left(self.cursor.1, shortcut);
                        } else {
                            // An existing selection collapses onto its near end rather than
                            // moving, which is what every other text field does.
                            self.cursor.0 = if self.cursor.0 == self.cursor.1 {
                                self.step_left(self.cursor.0, shortcut)
                            } else {
                                self.get_cursor().0
                            };
                            self.cursor.1 = self.cursor.0;
                        }
                    }
                    gfx::Key::Right => {
                        if shift {
                            self.cursor.1 = self.step_right(self.cursor.1, shortcut);
                        } else {
                            self.cursor.0 = if self.cursor.0 == self.cursor.1 {
                                self.step_right(self.cursor.0, shortcut)
                            } else {
                                self.get_cursor().1
                            };
                            self.cursor.1 = self.cursor.0;
                        }
                    }
                    gfx::Key::C if shortcut => graphics.set_clipboard_text(self.selection()),
                    gfx::Key::V if shortcut => {
                        if let Some(text) = graphics.get_clipboard_text() {
                            self.insert(&text);
                        }
                    }
                    gfx::Key::X if shortcut && self.cursor.0 != self.cursor.1 => {
                        graphics.set_clipboard_text(self.selection());
                        self.delete_selection();
                    }
                    _ => {}
                }
                return true;
            }
            _ => {}
        }
        false
    }

    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent_container))
    }
}
