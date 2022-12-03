use crate::menus::background_rect::BackgroundRect;
use graphics as gfx;

pub fn run_main_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    while graphics.renderer.is_window_open() {
        for event in graphics.renderer.get_events() {

        }

        graphics.renderer.pre_render();

        menu_back.render_back(graphics);

        graphics.renderer.post_render();
    }
}