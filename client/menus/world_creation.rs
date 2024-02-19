use directories::BaseDirs;
use std::cell::RefCell;
use std::rc::Rc;

use crate::client::game::private_world::run_private_world;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::background_rect::BackgroundRect;
use crate::client::settings::Settings;
use crate::libraries::graphics as gfx;
use gfx::BaseUiElement;

use super::singleplayer_selector::World;

fn world_name_exists(worlds_list: &Vec<World>, name: &str) -> bool {
    for world in worlds_list {
        if world.name.to_uppercase() == name.to_uppercase() {
            return true;
        }
    }
    false
}

/// this function runs the world creation menu.
#[allow(clippy::too_many_lines)] // TODO: reduce the number of lines in this function
pub fn run_world_creation(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    worlds_list: &Vec<World>,
    settings: &Rc<RefCell<Settings>>,
    global_settings: &Rc<RefCell<GlobalSettings>>,
) {
    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.set_texture(gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create a new world:", None)));
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut buttons_container = gfx::Container::new(graphics, gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0), gfx::BOTTOM, None);

    let mut back_button = gfx::Button::new(|| {});
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back", None));

    let mut create_button = gfx::Button::new(|| {});
    create_button.scale = 3.0;
    create_button.darken_on_disabled = true;
    create_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Create world", None));
    create_button.pos.0 = back_button.get_size().0 + gfx::SPACING;

    buttons_container.rect.size.0 = back_button.get_size().0 + create_button.get_size().0 + gfx::SPACING;
    buttons_container.rect.size.1 = back_button.get_size().1;
    buttons_container.rect.pos.1 = -gfx::SPACING;

    let mut world_name_input = gfx::TextInput::new(graphics);
    world_name_input.scale = 3.0;
    world_name_input.set_hint(graphics, "World name");
    world_name_input.orientation = gfx::CENTER;
    world_name_input.selected = true;
    world_name_input.pos.1 = -(world_name_input.get_size().1 + gfx::SPACING) / 2.0;

    let mut world_seed_input = gfx::TextInput::new(graphics);
    world_seed_input.scale = 3.0;
    world_seed_input.set_hint(graphics, "World seed");
    world_seed_input.orientation = gfx::CENTER;
    world_seed_input.pos.1 = (world_seed_input.get_size().1 + gfx::SPACING) / 2.0;

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
    'render_loop: while graphics.is_window_open() {
        let Some(base_dirs) = BaseDirs::new() else {
            println!("Failed to get base directories!");
            return;
        };

        let world_path = base_dirs.data_dir().join("Terralistic").join("Worlds").join(world_name_input.get_text().clone() + ".world");

        create_button.disabled = world_name_exists(worlds_list, world_name_input.get_text()) || world_name_input.get_text().is_empty();

        while let Some(event) = graphics.get_event() {
            //sorts out the events
            world_name_input.on_event(graphics, &event, menu_back.get_back_rect_container());
            world_seed_input.on_event(graphics, &event, menu_back.get_back_rect_container());
            if let gfx::Event::KeyRelease(key, ..) = event {
                match key {
                    gfx::Key::MouseLeft => {
                        if back_button.is_hovered(graphics, &buttons_container) {
                            break 'render_loop;
                        }
                        if create_button.is_hovered(graphics, &buttons_container) {
                            let game_result = run_private_world(graphics, menu_back, &world_path, settings, global_settings);
                            if let Err(error) = game_result {
                                println!("Game error: {error}");
                            }
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
                            let game_result = run_private_world(graphics, menu_back, &world_path, settings, global_settings);
                            if let Err(error) = game_result {
                                println!("Game error: {error}");
                            }
                            break 'render_loop;
                        }
                    }
                    _ => {}
                }
            }
        }
        menu_back.set_back_rect_width(700.0);

        menu_back.render_back(graphics);

        //render input fields
        buttons_container.update(graphics, Some(menu_back.get_back_rect_container()));

        title.render(graphics, menu_back.get_back_rect_container());
        back_button.render(graphics, &buttons_container);

        create_button.render(graphics, &buttons_container);

        world_name_input.render(graphics, menu_back.get_back_rect_container());
        world_seed_input.render(graphics, menu_back.get_back_rect_container());

        graphics.update_window();
    }
}
