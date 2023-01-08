use std::borrow::Borrow;
use graphics as gfx;
use shared_mut::SharedMut;
use crate::menus::background_rect::BackgroundRect;

/**
Loading screen displays back_menu and text which describes the current loading state.
The text is shared through the SharedMut<String> which is updated by the loading thread.
When the string is empty, the loading screen is closed.
 */
pub fn run_loading_screen(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect, loading_text: SharedMut<String>) {
    let mut loading_text_sprite = gfx::Sprite::new();
    loading_text_sprite.orientation = gfx::CENTER;
    loading_text_sprite.scale = 3.0;

    let mut curr_text = String::new();

    while graphics.renderer.is_window_open() && !loading_text.borrow().is_empty() {
        while let Some(_) = graphics.renderer.get_event() {}

        menu_back.set_back_rect_width(400);

        menu_back.render_back(graphics);

        if curr_text != *loading_text.borrow() {
            curr_text = loading_text.borrow().clone();
            if !curr_text.is_empty() {
                loading_text_sprite.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(curr_text.clone()));
            }
        }

        loading_text_sprite.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
}