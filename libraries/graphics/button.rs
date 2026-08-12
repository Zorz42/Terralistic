use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiElement};

use super::theme::{GFX_DEFAULT_BUTTON_BORDER_COLOR, GFX_DEFAULT_BUTTON_COLOR, GFX_DEFAULT_BUTTON_PADDING, GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR, GFX_DEFAULT_HOVERED_BUTTON_COLOR};

/// A Button is a rectangle with an image in it.
/// It can be clicked and has a hover animation.
use crate::libraries::graphics::UiContext;
pub struct Button {
    pub pos: gfx::FloatPos,
    pub orientation: gfx::Orientation,
    pub texture: gfx::Texture,
    pub padding: f32,
    pub scale: f32,
    pub color: gfx::Color,
    pub border_color: gfx::Color,
    pub hover_color: gfx::Color,
    pub hover_border_color: gfx::Color,
    pub disabled: bool,
    pub darken_on_disabled: bool,
    pub hover_progress: f32,
    timer: std::time::Instant,
    timer_counter: u64,
    on_click: Box<dyn Fn()>,
}

impl Button {
    /// Creates a new button.
    #[must_use]
    pub fn new<F: 'static + Fn()>(closure: F) -> Self {
        Self {
            pos: gfx::FloatPos(0.0, 0.0),
            orientation: gfx::TOP_LEFT,
            texture: gfx::Texture::new(),
            padding: GFX_DEFAULT_BUTTON_PADDING,
            scale: 1.0,
            color: GFX_DEFAULT_BUTTON_COLOR,
            border_color: GFX_DEFAULT_BUTTON_BORDER_COLOR,
            hover_color: GFX_DEFAULT_HOVERED_BUTTON_COLOR,
            hover_border_color: GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR,
            disabled: false,
            darken_on_disabled: false,
            hover_progress: 0.0,
            timer: std::time::Instant::now(),
            timer_counter: 0,
            on_click: Box::new(closure),
        }
    }

    /// Calculates the size based on the image height and the margin.
    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(
            (self.texture.get_texture_size().0 + self.padding * 2.0) * self.scale,
            (self.texture.get_texture_size().1 + self.padding * 2.0) * self.scale,
        )
    }

    /// Checks if the button is hovered with a mouse.
    #[must_use]
    pub fn is_hovered(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> bool {
        if self.disabled {
            return false;
        }

        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();
        let mouse_pos = graphics.get_mouse_pos();
        rect.contains(mouse_pos)
    }

    pub fn press(&self) {
        (self.on_click)();
    }

    /// Pins the hover animation at `progress` for the golden-image tests.
    ///
    /// Two things would otherwise make a capture non-reproducible: the animation advances
    /// once per elapsed millisecond since the button was constructed, and its target comes
    /// from `is_hovered`, which reads the real mouse position. Pushing `timer_counter` past
    /// any reachable elapsed time stops `render_inner` advancing it, so the value set here
    /// is exactly what gets drawn.
    #[cfg(feature = "render-tests")]
    pub const fn settle_hover(&mut self, progress: f32) {
        self.hover_progress = progress;
        self.timer_counter = u64::MAX;
    }
}

impl UiElement for Button {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    /// Renders the button.
    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        let hover_progress_target = if self.is_hovered(graphics, parent_container) {
            if graphics.get_key_state(gfx::Key::MouseLeft) {
                0.8
            } else {
                1.0
            }
        } else {
            0.0
        };

        while self.timer_counter < self.timer.elapsed().as_millis() as u64 {
            self.hover_progress += (hover_progress_target - self.hover_progress) / 40.0;
            if (hover_progress_target - self.hover_progress).abs() <= 0.01 {
                self.hover_progress = hover_progress_target;
            }
            self.timer_counter += 1;
        }

        let button_color = gfx::interpolate_colors(self.color, self.hover_color, self.hover_progress);
        let button_border_color = gfx::interpolate_colors(self.border_color, self.hover_border_color, self.hover_progress);

        let padding = (1.0 - self.hover_progress) * 30.0;
        let hover_rect = gfx::Rect::new(
            rect.pos + gfx::FloatPos(padding, padding),
            gfx::FloatSize(f32::max(0.0, rect.size.0 - 2.0 * padding), f32::max(0.0, rect.size.1 - 2.0 * padding)),
        );
        rect.render(graphics, self.color);
        rect.render_outline(graphics, self.border_color);
        hover_rect.render(graphics, button_color);
        hover_rect.render_outline(graphics, button_border_color);

        let texture_scale = self.scale + self.hover_progress * 0.3;
        let x = rect.pos.0 + rect.size.0 / 2.0 - self.texture.get_texture_size().0 * texture_scale / 2.0;
        let y = rect.pos.1 + rect.size.1 / 2.0 - self.texture.get_texture_size().1 * texture_scale / 2.0;
        self.texture.render(graphics, texture_scale, gfx::FloatPos(x, y), None, false, None);
        if self.disabled && self.darken_on_disabled {
            rect.render(graphics, gfx::Color::new(0, 0, 0, 100));
        }
    }

    ///calls `on_click` when clicked
    fn on_event_inner(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        if let gfx::Event::KeyRelease(key, ..) = event {
            if *key == gfx::Key::MouseLeft && self.is_hovered(graphics, parent_container) && !self.disabled {
                (self.on_click)();
                return true;
            }
        }
        false
    }

    /// Generates the container for the button.
    fn get_container(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> gfx::Container {
        gfx::Container::new(graphics, self.pos, self.get_size(), self.orientation, Some(parent_container))
    }
}
