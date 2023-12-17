use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;

/// The debug menu shows useful information about the game.
/// Like fps, time per frame, position, etc.
pub struct DebugMenu {
    open: bool,
    back_rect: gfx::RenderRect,
}

impl DebugMenu {
    pub fn new() -> Self {
        Self {
            open: false,
            back_rect: gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)),
        }
    }

    pub fn init(&mut self) {
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.border_color = gfx::BORDER_COLOR;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        self.back_rect.smooth_factor = 60.0;
        self.back_rect.orientation = gfx::BOTTOM_RIGHT;
        self.back_rect.pos.1 = -gfx::SPACING;
        self.back_rect.size.0 = 300.0;
        self.back_rect.size.1 = 200.0;
    }

    pub fn render(&mut self, graphics: &gfx::GraphicsContext, lines: &[String]) {
        self.back_rect.pos.0 = if self.open { -gfx::SPACING } else { self.back_rect.size.0 + 100.0 };
        self.back_rect.render(graphics, None);

        if self.is_visible(graphics) {
            let scale = 3.0;

            let mut width = 0.0;
            let mut height = 0.0;
            for line in lines {
                let size = graphics.font.get_text_size_scaled(line, scale, None);

                width = f32::max(width, size.0);
                height += size.1;
            }

            self.back_rect.size.0 = width + 2.0 * gfx::SPACING;
            self.back_rect.size.1 = height + 2.0 * gfx::SPACING;

            let mut y = gfx::SPACING;
            let debug_menu_rect_container = self.back_rect.get_container(graphics, None);
            let debug_menu_rect_container = debug_menu_rect_container.get_absolute_rect();
            for line in lines {
                graphics.font.render_text(
                    graphics,
                    line,
                    gfx::FloatPos(debug_menu_rect_container.pos.0 + gfx::SPACING, debug_menu_rect_container.pos.1 + y),
                    scale,
                );

                let size = graphics.font.get_text_size_scaled(line, scale, None);
                y += size.1;
            }
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        if matches!(event.downcast::<gfx::Event>(), Some(gfx::Event::KeyPress(gfx::Key::M, false))) {
            self.open = !self.open;
        }
    }

    fn is_visible(&self, graphics: &gfx::GraphicsContext) -> bool {
        (self.back_rect.get_container(graphics, None).rect.pos.0 as i32) < (graphics.renderer.get_window_size().0 as i32)
    }
}
