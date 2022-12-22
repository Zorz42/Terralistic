use std::fs;
use std::path::{Path, PathBuf};
use chrono;
use graphics as gfx;
use directories::{BaseDirs, UserDirs, ProjectDirs};
use graphics::GraphicsContext;
use crate::menus::background_rect::BackgroundRect;

const MENU_WIDTH: i32 = 800;

/**
This function returns formatted string "%d %B %Y %H:%M" of the time
that the file was last modified.
 */
pub fn get_last_modified_time(file_path: &str) -> String {
    let metadata = fs::metadata(file_path).unwrap();
    let modified_time = metadata.modified().unwrap();
    let datetime = chrono::DateTime::<chrono::Local>::from(modified_time);
    datetime.format("%d %B %Y %H:%M").to_string()
}

/**
World is a struct that contains all information to
render the world in singleplayer selector.
 */
pub struct World {
    pub name: String,
    rect: gfx::RenderRect,
    play_button: gfx::Button,
    delete_button: gfx::Button,
    last_modified: gfx::Sprite,
    title: gfx::Sprite,
    icon: gfx::Sprite,
}

impl World {
    pub fn new(graphics: &GraphicsContext, file_path: PathBuf) -> Self {
        let name = file_path.file_name().unwrap().to_str().unwrap().to_string();

        let mut rect = gfx::RenderRect::new(0.0, 0.0, (MENU_WIDTH - 2 * gfx::SPACING) as f32, 0.0);
        rect.orientation = gfx::TOP;
        rect.fill_color.a = 100;

        let mut icon = gfx::Sprite::new();
        icon.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(include_bytes!("../../Build/Resources/world_icon.opa").to_vec()));
        rect.h = icon.get_height() as f32 + 2.0 * gfx::SPACING as f32;
        icon.x = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(name.clone()));
        title.x = icon.x + icon.get_width() + gfx::SPACING;
        title.y = gfx::SPACING;
        title.scale = 3.0;

        let mut play_button = gfx::Button::new();
        play_button.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(include_bytes!("../../Build/Resources/play_button.opa").to_vec()));
        play_button.scale = 3.0;
        play_button.margin = 5;
        play_button.x = icon.x + icon.get_width() + gfx::SPACING;
        play_button.y = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new();
        delete_button.texture = gfx::Texture::load_from_surface(&gfx::Surface::deserialize(include_bytes!("../../Build/Resources/delete_button.opa").to_vec()));
        delete_button.scale = 3.0;
        delete_button.margin = 5;
        delete_button.x = play_button.x + play_button.get_width() + gfx::SPACING;
        delete_button.y = -gfx::SPACING;
        delete_button.orientation = gfx::BOTTOM_LEFT;

        let mut last_modified = gfx::Sprite::new();
        last_modified.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(get_last_modified_time(file_path.as_path().to_str().unwrap())));
        last_modified.color = gfx::GREY;
        last_modified.orientation = gfx::BOTTOM_RIGHT;
        last_modified.x = -gfx::SPACING;
        last_modified.y = -gfx::SPACING;
        last_modified.scale = 2.0;

        World {
            name,
            rect,
            play_button,
            delete_button,
            last_modified,
            title,
            icon,
        }
    }

    /**
    This function renders the world card on the x and y position.
     */
    pub fn render(&mut self, graphics: &mut gfx::GraphicsContext, x: i32, y: i32, parent_container: Option<&gfx::Container>) {
        self.rect.x = x as f32;
        self.rect.y = y as f32;
        self.rect.render(&graphics, parent_container);

        let rect_container = self.rect.get_container(graphics, parent_container);
        self.icon.render(graphics, Some(&rect_container));
        self.title.render(graphics, Some(&rect_container));
        self.play_button.render(graphics, Some(&rect_container));
        self.delete_button.render(graphics, Some(&rect_container));
        self.last_modified.render(graphics, Some(&rect_container));
    }

    /**
    This function returns width of the world card.
     */
    pub fn get_width(&self) -> i32 {
        self.rect.w as i32
    }

    /**
    This function returns height of the world card.
     */
    pub fn get_height(&self) -> i32 {
        self.rect.h as i32
    }

    /**
    This function disables/enables the world card buttons.
     */
    pub fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }
}

/**
WorldList is a struct that is used to list all worlds in the world folder
and render them in the singleplayer selector menu.
 */
pub struct WorldList {
    pub worlds: Vec<World>,
}

impl WorldList {
    pub fn new(graphics: &GraphicsContext) -> WorldList {
        let mut world_list = WorldList {
            worlds: Vec::new(),
        };
        world_list.refresh(graphics);
        world_list
    }

    pub fn refresh(&mut self, graphics: &GraphicsContext) {
        let base_dirs = BaseDirs::new().unwrap();
        let world_dir = base_dirs.data_dir().join("Terralistic").join("Worlds");
        if !world_dir.exists() {
            fs::create_dir_all(&world_dir).unwrap();
        }
        self.worlds.clear();
        for entry in fs::read_dir(&world_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() && path.extension().unwrap() == "world" {
                self.worlds.push(World::new(graphics, path));
            }
        }
    }
}

pub fn run_singleplayer_selector(graphics: &mut gfx::GraphicsContext, menu_back: &mut dyn BackgroundRect) {
    let mut world_list = WorldList::new(graphics);

    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Select a world to play!")));
    title.y = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("Back")));
    back_button.y = -gfx::SPACING;
    back_button.orientation = gfx::BOTTOM;

    let mut new_world_button = gfx::Button::new();
    new_world_button.scale = 3.0;
    new_world_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(String::from("New")));
    new_world_button.y = -gfx::SPACING;
    new_world_button.x = -gfx::SPACING;
    new_world_button.orientation = gfx::BOTTOM_RIGHT;

    let top_height = title.get_height() + 2 * gfx::SPACING;
    let bottom_height = back_button.get_height() + 2 * gfx::SPACING;

    let mut top_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, top_height as f32);
    top_rect.orientation = gfx::TOP;

    let mut bottom_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, bottom_height as f32);
    bottom_rect.fill_color.a = gfx::TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    bottom_rect.blur_radius = gfx::BLUR;
    bottom_rect.orientation = gfx::BOTTOM;

    let mut top_rect_visibility = 1.0;
    let mut position: f32 = 0.0;

    'render_loop: while graphics.renderer.is_window_open() {
        while let Some(event) = graphics.renderer.get_event() {
            match event {
                gfx::Event::KeyRelease(key) => {
                    if key == gfx::Key::MouseLeft {
                        if back_button.is_hovered(graphics, Some(&menu_back.get_back_rect_container())) {
                            break 'render_loop;
                        }
                    }
                }
                gfx::Event::MouseScroll(delta) => {
                    position += delta as f32 * 2.0;
                }
                _ => {}
            }
        }
        menu_back.set_back_rect_width(MENU_WIDTH);

        menu_back.render_back(graphics);

        let hoverable = graphics.renderer.get_mouse_y() as i32 > top_height && (graphics.renderer.get_mouse_y() as i32) < graphics.renderer.get_window_height() as i32 - bottom_height as i32;

        for world in &mut world_list.worlds {
            world.set_enabled(hoverable);
        }

        let mut current_y = gfx::SPACING;
        for i in 0..world_list.worlds.len() {
            world_list.worlds[i].render(graphics, 0, current_y + top_height + position as i32, Some(&menu_back.get_back_rect_container()));
            current_y += world_list.worlds[i].get_height() + gfx::SPACING;
        }

        top_rect.w = menu_back.get_back_rect_width() as f32;
        top_rect_visibility += ((if position < -5.0 { 1.0 } else { 0.0 }) - top_rect_visibility) / 20.0;

        if top_rect_visibility < 0.01 {
            top_rect_visibility = 0.0;
        }
        if top_rect_visibility > 0.99 {
            top_rect_visibility = 1.0;
        }
        top_rect.fill_color.a = (top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
        top_rect.blur_radius = (top_rect_visibility * gfx::BLUR as f32) as i32;
        top_rect.shadow_intensity = (top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;
        if top_rect_visibility > 0.0 {
            top_rect.render(graphics, Some(&menu_back.get_back_rect_container()));
        }

        bottom_rect.w = menu_back.get_back_rect_width() as f32;
        let mut scroll_limit = current_y - graphics.renderer.get_window_height() as i32 + top_height as i32 + bottom_height as i32;
        if scroll_limit < 0 {
            scroll_limit = 0;
        }

        if scroll_limit > 0 {
            bottom_rect.render(graphics, Some(&menu_back.get_back_rect_container()));
        }

        if position > 0.0 {
            position -= position / 20.0;
        }

        if position < -scroll_limit as f32 {
            position -= (position + scroll_limit as f32) / 20.0;
        }

        title.render(graphics, Some(&menu_back.get_back_rect_container()));
        back_button.render(graphics, Some(&menu_back.get_back_rect_container()));

        new_world_button.render(graphics, Some(&menu_back.get_back_rect_container()));


        graphics.renderer.update_window();
    }
}