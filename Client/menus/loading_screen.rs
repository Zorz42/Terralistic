use graphics::GraphicsContext;
use shared_mut::SharedMut;
use crate::menus::background_rect::BackgroundRect;

/**
Loading screen displays back_menu and text which describes the current loading state.
The text is shared through the SharedMut<String> which is updated by the loading thread.
When the string is empty, the loading screen is closed.
 */
pub fn run_loading_screen(graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect, loading_text: SharedMut<String>) {
    while graphics.renderer.is_window_open() && !loading_text.borrow().is_empty() {
        while let Some(_) = graphics.renderer.get_event() {}

        menu_back.set_back_rect_width(400);

        menu_back.render_back(graphics);



        graphics.renderer.update_window();
    }
}