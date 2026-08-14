use crate::libraries::graphics as gfx;
use crate::libraries::ui;

use super::background_rect::BackgroundRect;
use crate::libraries::ui::{BaseUiElement, UiElement};

/// `MenuBack` contains the background rectangle for most main menus.
///
/// It implements the `BackgroundRect` trait. It draws the background.opa image
/// scaled to the window's height and scrolled to the left.
use crate::libraries::ui::UiContext;
pub struct MenuBack {
    background: gfx::Texture,
    back_rect: ui::RenderRect,
    back_container: ui::Container,
}

impl MenuBack {
    /// Creates a new `MenuBack`.
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext) -> Self {
        let mut back_rect = ui::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
        back_rect.border_color = ui::BORDER_COLOR;
        back_rect.fill_color.a = ui::TRANSPARENCY;
        back_rect.orientation = ui::CENTER;
        back_rect.blur_radius = ui::BLUR;
        back_rect.shadow_intensity = ui::SHADOW_INTENSITY;
        back_rect.smooth_factor = 60.0;

        Self {
            background: gfx::Texture::load_from_bytes(include_bytes!("../../Build/Resources/background.opa")),
            back_rect,
            back_container: ui::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), ui::CENTER, None),
        }
    }
}

impl UiElement for MenuBack {
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        vec![&mut self.back_container]
    }

    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        vec![&self.back_container]
    }

    fn render_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        let scale = graphics.get_window_size().1 / self.background.get_texture_size().1;
        let texture_width_scaled = self.background.get_texture_size().0 * scale;
        let pos = ((std::time::UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as f64 * scale as f64 / 150.0) % texture_width_scaled as f64) as f32;

        for i in -1..graphics.get_window_size().0 as i32 / (self.background.get_texture_size().0 * scale) as i32 + 2 {
            self.background.render(graphics, scale, gfx::FloatPos(pos + i as f32 * texture_width_scaled, 0.0), None, false, None);
        }

        self.back_rect.render(graphics, parent_container);
    }

    fn update_inner(&mut self, graphics: &mut gfx::GraphicsContext, parent_container: &ui::Container) {
        if (self.back_rect.size.1 - graphics.get_window_size().1).abs() > f32::EPSILON {
            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.jump_to_target();
        }

        self.back_rect.update(graphics, parent_container);
        let new_container = self.back_rect.get_container(graphics, parent_container);
        self.back_container.rect = new_container.rect;
    }

    fn get_container(&self, graphics: &dyn ui::UiContext, parent_container: &ui::Container) -> ui::Container {
        ui::Container::new(
            graphics,
            self.back_container.rect.pos,
            self.back_container.rect.size,
            self.back_container.orientation,
            Some(parent_container),
        )
    }
}

impl BackgroundRect for MenuBack {
    /// Renders the background.
    fn render_back(&mut self, graphics: &mut gfx::GraphicsContext) {
        let parent_container = ui::Container::default(graphics);
        self.render(graphics, &parent_container);
    }

    /// Sets the width of the background rectangle.
    fn set_back_rect_width(&mut self, width: f32, instant: bool) {
        self.back_rect.size.0 = width;
        if instant {
            self.back_rect.jump_to_target();
        }
    }

    /// Gets the width of the background rectangle.
    fn get_back_rect_width(&self) -> f32 {
        self.back_rect.render_size.0
    }

    /// Gets the background rectangle's container.
    ///WARNING: do not use, use `get_container` instead
    fn get_back_rect_container(&self, graphics: &gfx::GraphicsContext) -> ui::Container {
        ui::Container::default(graphics)
    }

    ///sets the background's x position.
    fn set_x_position(&mut self, center_pos: f32) {
        self.back_rect.pos.0 = center_pos;
    }

    ///gets a vector of sub elements.
    fn get_sub_elements(&self) -> Vec<&dyn BaseUiElement> {
        self.back_container.get_sub_elements()
    }

    ///gets a mutable vector of mutable sub elements.
    fn get_sub_elements_mut(&mut self) -> Vec<&mut dyn BaseUiElement> {
        self.back_container.get_sub_elements_mut()
    }
}
