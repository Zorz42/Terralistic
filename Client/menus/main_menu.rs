use crate::menus::background_rect::BackgroundRect;
use graphics as gfx;
use rand;
use rand::Rng;

pub fn run_main_menu(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    menu_back.set_back_rect_width(200);

    let mut button = gfx::Button::new();
    let mut text_surface = graphics.font.create_text_surface(String::from("Click Me!"));
    text_surface.set_pixel(0, 0, gfx::Color::new(255, 0, 0, 0));
    button.texture = gfx::Texture::load_from_surface(&text_surface);
    button.scale = 3.0;
    button.x = 30;
    button.y = 30;

    while graphics.renderer.is_window_open() {
        let events = graphics.renderer.get_events();
        for event in events {
            // when the m key is pressed, set back width to something random between 100 and 500
            match event {
                gfx::Event::KeyPress(key) => {
                    if key == gfx::Key::K {
                        menu_back.set_back_rect_width(rand::thread_rng().gen_range(100..500));
                    }
                }
                _ => {}
            }
        }

        menu_back.render_back(graphics);

        button.render(graphics, None);

        graphics.renderer.post_render();
    }
}