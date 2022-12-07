use crate::menus::background_rect::BackgroundRect;
use graphics as gfx;
use rand;
use rand::Rng;

pub fn run_main_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    while graphics.renderer.is_window_open() {
        for event in graphics.renderer.get_events() {
            // when the m key is pressed, set back width to something random between 100 and 500
            match event {
                gfx::Event::KeyPress(key) => {
                    if key == gfx::Key::M {
                        menu_back.set_back_rect_width(rand::thread_rng().gen_range(100..500));
                    }
                }
                _ => {}
            }
        }

        graphics.renderer.pre_render();

        menu_back.set_back_rect_width(200);
        menu_back.render_back(graphics);

        graphics.renderer.post_render();
    }
}