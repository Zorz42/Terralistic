use crate::libraries::graphics as gfx;
use gfx::{BaseUiElement, UiContext, UiElement};

use super::theme::{BUTTON_BORDER_COLOR, BUTTON_COLOR, BUTTON_PADDING, HOVERED_BUTTON_BORDER_COLOR, HOVERED_BUTTON_COLOR};

/// A clickable rectangle with an image in it and a hover animation.
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
    /// One hover step per elapsed millisecond, bounded so that a button built long before it
    /// is first drawn - the pause menu's, say - does not owe a step for every one of them.
    animation_timer: gfx::AnimationTimer,
    /// Whether the most recent press of the left button landed on this one.
    ///
    /// A click is a press *and* a release on the same button. Without this only the release
    /// was checked, so a press that landed anywhere else - on the menu behind, or in the menu
    /// this one replaced - activated whatever the pointer happened to be over when it came
    /// back up.
    ///
    /// Deliberately not cleared by the release that consumes it. Several menus hold their
    /// buttons as sub-elements *and* dispatch to them again from `on_event_inner`, so a
    /// release reaches a button twice and the menu reads the second answer.
    pressed_inside: bool,
    on_click: Box<dyn Fn()>,
}

impl Button {
    #[must_use]
    pub fn new<F: 'static + Fn()>(closure: F) -> Self {
        Self {
            pos: gfx::FloatPos(0.0, 0.0),
            orientation: gfx::TOP_LEFT,
            texture: gfx::Texture::new(),
            padding: BUTTON_PADDING,
            scale: 1.0,
            color: BUTTON_COLOR,
            border_color: BUTTON_BORDER_COLOR,
            hover_color: HOVERED_BUTTON_COLOR,
            hover_border_color: HOVERED_BUTTON_BORDER_COLOR,
            disabled: false,
            darken_on_disabled: false,
            hover_progress: 0.0,
            animation_timer: gfx::AnimationTimer::new(1),
            pressed_inside: false,
            on_click: Box::new(closure),
        }
    }

    /// The image plus the padding, scaled.
    #[must_use]
    pub fn get_size(&self) -> gfx::FloatSize {
        gfx::FloatSize(
            (self.texture.get_texture_size().0 + self.padding * 2.0) * self.scale,
            (self.texture.get_texture_size().1 + self.padding * 2.0) * self.scale,
        )
    }

    /// Whether the mouse is over the button. A disabled button is never hovered, which is also
    /// what stops it reacting to clicks.
    #[must_use]
    pub fn is_hovered(&self, graphics: &dyn gfx::UiContext, parent_container: &gfx::Container) -> bool {
        !self.disabled && self.get_container(graphics, parent_container).get_absolute_rect().contains(graphics.get_mouse_pos())
    }

    pub fn press(&self) {
        (self.on_click)();
    }

    /// Pins the hover animation at `progress` for the golden-image tests.
    ///
    /// The animation chases a target that depends on the real mouse position, so freezing the
    /// timer is what makes the value set here exactly what gets drawn.
    #[cfg(feature = "render-tests")]
    pub const fn settle_hover(&mut self, progress: f32) {
        self.hover_progress = progress;
        self.animation_timer.freeze();
    }
}

impl UiElement for Button {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        Vec::new()
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        Vec::new()
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &gfx::Container) {
        let container = self.get_container(graphics, parent_container);
        let rect = container.get_absolute_rect();

        let hover_target = if self.is_hovered(graphics, parent_container) {
            if graphics.get_key_state(gfx::Key::MouseLeft) {
                0.8
            } else {
                1.0
            }
        } else {
            0.0
        };

        while self.animation_timer.frame_ready() {
            self.hover_progress = gfx::approach(self.hover_progress, hover_target, 40.0, 0.01);
        }

        let button_color = gfx::interpolate_colors(self.color, self.hover_color, self.hover_progress);
        let button_border_color = gfx::interpolate_colors(self.border_color, self.hover_border_color, self.hover_progress);

        // The hover fill grows out of the middle of the button as the hover comes in.
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
        let texture_pos = gfx::FloatPos(
            rect.pos.0 + rect.size.0 / 2.0 - self.texture.get_texture_size().0 * texture_scale / 2.0,
            rect.pos.1 + rect.size.1 / 2.0 - self.texture.get_texture_size().1 * texture_scale / 2.0,
        );
        self.texture.render(graphics, texture_scale, texture_pos, None, false, None);

        if self.disabled && self.darken_on_disabled {
            rect.render(graphics, gfx::Color::new(0, 0, 0, 100));
        }
    }

    /// Fires on the release of a press that landed on this same button, so neither half of a
    /// click that started or finished somewhere else counts.
    fn on_event_inner(&mut self, graphics: &mut dyn gfx::UiContext, event: &gfx::Event, parent_container: &gfx::Container) -> bool {
        match event {
            gfx::Event::KeyPress(gfx::Key::MouseLeft, ..) => {
                self.pressed_inside = self.is_hovered(graphics, parent_container);
            }
            gfx::Event::KeyRelease(gfx::Key::MouseLeft, ..) if self.pressed_inside && self.is_hovered(graphics, parent_container) => {
                (self.on_click)();
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
