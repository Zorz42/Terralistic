use crate::libraries::graphics as gfx;

use super::background_rect::BackgroundRect;
use gfx::{BaseUiElement, UiElement};

/// `MenuBack` is a struct that contains the background rectangle for
/// the most main menus. It implements the `BackgroundRect` trait. It
/// draws the background.opa image scaled to the window's height and
/// scrolled to the left.
pub struct MenuBack {
    background: gfx::Texture,
    background_timer: std::time::Instant,
    back_rect: gfx::RenderRect,
    back_container: gfx::Container,
    pub main_back_menu: bool,
}

impl MenuBack {
    /// Creates a new `MenuBack`.
    #[must_use]
    pub fn new(graphics: &gfx::GraphicsContext) -> Self {
        let mut back_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0));
        back_rect.border_color = gfx::BORDER_COLOR;
        back_rect.fill_color.a = gfx::TRANSPARENCY;
        back_rect.orientation = gfx::CENTER;
        back_rect.blur_radius = gfx::BLUR;
        back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        back_rect.smooth_factor = 60.0;

        Self {
            background: gfx::Texture::load_from_surface(
                &gfx::Surface::deserialize_from_bytes(include_bytes!("../../Build/Resources/background.opa")).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))),
            ),
            background_timer: std::time::Instant::now(),
            back_rect,
            back_container: gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::TOP_LEFT, None),
            main_back_menu: true,
        }
    }

    #[must_use]
    pub fn new_synced(graphics_context: &gfx::GraphicsContext, timer: std::time::Instant) -> Self {
        let mut temp = Self::new(graphics_context);
        temp.background_timer = timer;
        temp
    }

    #[must_use]
    pub const fn get_timer(&self) -> std::time::Instant {
        self.background_timer
    }
}

impl BackgroundRect for MenuBack {
    /// Renders the background.
    fn render_back(&mut self, graphics: &mut gfx::GraphicsContext) {
        let parent_container = gfx::Container::default(graphics);
        self.back_container = self.back_rect.get_container(graphics, &parent_container);

        if self.main_back_menu {
            let scale = graphics.get_window_size().1 / self.background.get_texture_size().1;
            let texture_width_scaled = self.background.get_texture_size().0 * scale;
            let pos = ((self.background_timer.elapsed().as_millis() as f32 * scale / 150.0) as u64 % texture_width_scaled as u64) as f32;

            for i in -1..graphics.get_window_size().0 as i32 / (self.background.get_texture_size().0 * scale) as i32 + 2 {
                self.background.render(graphics, scale, gfx::FloatPos(pos + i as f32 * texture_width_scaled, 0.0), None, false, None);
            }
        }

        if (self.back_rect.size.1 - graphics.get_window_size().1).abs() > f32::EPSILON {
            self.back_rect.size.1 = graphics.get_window_size().1;
            self.back_rect.jump_to_target();
        }
        self.back_rect.render(graphics, &parent_container);
    }

    /// Sets the width of the background rectangle.
    fn set_back_rect_width(&mut self, width: f32) {
        self.back_rect.size.0 = width;
    }

    /// Gets the width of the background rectangle.
    fn get_back_rect_width(&self, graphics: &gfx::GraphicsContext, parent_container: Option<&gfx::Container>) -> f32 {
        self.back_rect.get_container(graphics, parent_container.unwrap_or(&gfx::Container::default(graphics))).rect.size.0
    }

    /// Gets the background rectangle's container.
    fn get_back_rect_container(&self) -> &gfx::Container {
        &self.back_container
    }

    ///sets the background's x position.
    fn set_x_position(&mut self, center_pos: f32) {
        self.back_rect.pos.0 = center_pos;
    }
}
