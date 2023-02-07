use super::singleplayer_selector::World;
use crate::client::game::private_world::run_private_world;
use crate::client::menus::background_rect::BackgroundRect;
use crate::libraries::graphics as gfx;
use crate::libraries::graphics::GraphicsContext;
use directories::BaseDirs;

fn world_name_exists(worlds_list: &Vec<World>, name: &str) -> bool {
    for world in worlds_list {
        if world.name == name {
            return true;
        }
    }
    false
}

/**this function runs the world creation menu.*/
pub fn run_world_creation(
    graphics: &mut GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    worlds_list: &mut Vec<World>,
) {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create a new world:"));
    title.y = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(0, 0, 0, 0, gfx::BOTTOM);

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));

    let mut create_button = gfx::Button::new();
    create_button.scale = 3.0;
    create_button.darken_on_disabled = true;
    create_button.texture =
        gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create world"));
    create_button.x = back_button.get_width() + gfx::SPACING;

    buttons_container.rect.w = back_button.get_width() + create_button.get_width() + gfx::SPACING;
    buttons_container.rect.h = back_button.get_height();
    buttons_container.rect.y = -gfx::SPACING;

    let mut world_name_input = gfx::TextInput::new(graphics);
    world_name_input.scale = 3.0;
    world_name_input.set_hint(graphics, "World name");
    world_name_input.orientation = gfx::CENTER;
    world_name_input.selected = true;
    world_name_input.y = -(world_name_input.get_height() + gfx::SPACING) / 2;

    let mut world_seed_input = gfx::TextInput::new(graphics);
    world_seed_input.scale = 3.0;
    world_seed_input.set_hint(graphics, "World seed");
    world_seed_input.orientation = gfx::CENTER;
    world_seed_input.y = (world_seed_input.get_height() + gfx::SPACING) / 2;

    world_name_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts letters, numbers and _ symbol
        if text.is_alphanumeric() || text == '_' {
            return Some(text);
        }
        None
    }));

    world_seed_input.text_processing = Some(Box::new(|text: char| {
        // this closure only accepts numbers
        if text.is_numeric() {
            return Some(text);
        }
        None
    }));

    //this is where the menu is drawn
    'render_loop: while graphics.renderer.is_window_open() {
        let Some(base_dirs) = BaseDirs::new() else {
            println!("Failed to get base directories!");
            return;
        };

        let world_path = base_dirs
            .data_dir()
            .join("Terralistic")
            .join("Worlds")
            .join(world_name_input.text.clone() + ".world");

        create_button.disabled = world_name_exists(worlds_list, &world_name_input.text)
            || world_name_input.text.is_empty();

        while let Some(event) = graphics.renderer.get_event() {
            //sorts out the events
            world_name_input.on_event(&event, graphics, None);
            world_seed_input.on_event(&event, graphics, None);
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, Some(&buttons_container)) {
                            break 'render_loop;
                        }
                        if create_button.is_hovered(graphics, Some(&buttons_container)) {
                            run_private_world(graphics, menu_back, &world_path);
                            break 'render_loop;
                        }
                    }
                    gfx::Key::Escape => {
                        if world_name_input.selected || world_seed_input.selected {
                            world_name_input.selected = false;
                            world_seed_input.selected = false;
                        } else {
                            break 'render_loop;
                        }
                    }
                    gfx::Key::Enter => {
                        if !create_button.disabled {
                            run_private_world(graphics, menu_back, &world_path);
                            break 'render_loop;
                        }
                    }
                    _ => {}
                }
            }
        }
        menu_back.set_back_rect_width(700);

        menu_back.render_back(graphics);

        //render input fields
        buttons_container.update(graphics, Some(menu_back.get_back_rect_container()));

        title.render(graphics, Some(menu_back.get_back_rect_container()));
        back_button.render(graphics, Some(&buttons_container));

        create_button.render(graphics, Some(&buttons_container));

        world_name_input.render(graphics, Some(menu_back.get_back_rect_container()));
        world_seed_input.render(graphics, Some(menu_back.get_back_rect_container()));

        graphics.renderer.update_window();
    }
}
