use crate::menus::background_rect::BackgroundRect;
use graphics::GraphicsContext;
use graphics as gfx;
use crate::game::private_world::run_private_world;


/**this function runs the world creation menu.*/
pub fn run_world_creation(graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Create a world")));
    title.y = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Back")));
    back_button.y = -gfx::SPACING;
    back_button.x = -gfx::SPACING;
    back_button.orientation = gfx::BOTTOM_RIGHT;

    let mut create_button = gfx::Button::new();
    create_button.scale = 3.0;
    create_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Create world")));
    create_button.y = -gfx::SPACING;
    create_button.orientation = gfx::BOTTOM;

    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {//sorts out the events
            match event {
                gfx::Event::KeyRelease(key) => {
                    if key == gfx::Key::MouseLeft {
                        if back_button.is_hovered(graphics, Some(&menu_back.get_back_rect_container())) {
                            break 'render_loop;
                        }
                        if create_button.is_hovered(graphics, Some(&menu_back.get_back_rect_container())) {
                            run_private_world(graphics, menu_back);
                        }
                    }
                }
                _ => {}
            }
        }
        menu_back.set_back_rect_width(800);

        menu_back.render_back(graphics);

        //render input fields

        title.render(graphics, Some(&menu_back.get_back_rect_container()));
        back_button.render(graphics, Some(&menu_back.get_back_rect_container()));

        create_button.render(graphics, Some(&menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
}