use crate::libraries::events::Event;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};

/// The pause menu actually does not pause the game (ironic, I know).
/// It just shows a menu with options to quit the world or go back to the game.
pub struct PauseMenu {
    open: bool,
    resume_button: gfx::Button,
    quit_button: gfx::Button,
    back_rect: gfx::RenderRect,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            open: false,
            resume_button: gfx::Button::new(),
            quit_button: gfx::Button::new(),
            back_rect: gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0)),
        }
    }

    pub fn init(&mut self, graphics: &mut GraphicsContext) {
        self.resume_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Resume"));
        self.resume_button.scale = 3.0;
        self.resume_button.pos.0 = -gfx::SPACING;
        self.resume_button.pos.1 = gfx::SPACING;
        self.resume_button.orientation = gfx::TOP_RIGHT;

        self.quit_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Quit"));
        self.quit_button.scale = 3.0;
        self.quit_button.pos.0 = -gfx::SPACING;
        self.quit_button.pos.1 = 2.0 * gfx::SPACING + self.resume_button.get_size().1;
        self.quit_button.orientation = gfx::TOP_RIGHT;

        let pause_rect_width = f32::max(
            self.resume_button.get_size().0,
            self.quit_button.get_size().0,
        ) + 2.0 * gfx::SPACING;
        self.back_rect.fill_color = gfx::BLACK;
        self.back_rect.fill_color.a = gfx::TRANSPARENCY;
        self.back_rect.border_color = gfx::BORDER_COLOR;
        self.back_rect.blur_radius = gfx::BLUR;
        self.back_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        self.back_rect.smooth_factor = 60.0;
        self.back_rect.size.0 = pause_rect_width;
    }

    pub fn render(&mut self, graphics: &mut GraphicsContext) {
        self.back_rect.pos.0 = if self.open {
            0.0
        } else {
            -self.back_rect.size.0 - 100.0
        };
        self.back_rect.render(graphics, None);

        if graphics.renderer.get_window_size().1 as u32 != self.back_rect.size.1 as u32 {
            self.back_rect.size.1 = graphics.renderer.get_window_size().1;
            self.back_rect.jump_to_target();
        }

        self.resume_button.render(
            graphics,
            Some(&self.back_rect.get_container(graphics, None)),
        );
        self.quit_button.render(
            graphics,
            Some(&self.back_rect.get_container(graphics, None)),
        );
    }

    /// returns true if the game should quit
    pub fn on_event(&mut self, event: &Event, graphics: &mut GraphicsContext) -> bool {
        if let Some(event) = event.downcast::<gfx::Event>() {
            match event {
                gfx::Event::KeyPress(key, false) => {
                    if *key == gfx::Key::Escape {
                        self.open = !self.open;
                    }
                }
                gfx::Event::KeyRelease(key, false) => {
                    if *key == gfx::Key::MouseLeft && self.open {
                        if self.resume_button.is_hovered(
                            graphics,
                            Some(&self.back_rect.get_container(graphics, None)),
                        ) {
                            self.open = false;
                        } else if self.quit_button.is_hovered(
                            graphics,
                            Some(&self.back_rect.get_container(graphics, None)),
                        ) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }
}
