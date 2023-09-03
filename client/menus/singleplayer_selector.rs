use super::background_rect::BackgroundRect;
use super::run_choice_menu;
use super::world_creation::run_world_creation;
use crate::client::game::private_world::run_private_world;

use crate::libraries::graphics as gfx;
use directories::BaseDirs;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

pub const MENU_WIDTH: f32 = 800.0;

/// This function returns formatted string "%d %B %Y %H:%M" of the time
/// that the file was last modified.
pub fn get_last_modified_time(file_path: &str) -> String {
    let metadata = fs::metadata(file_path);
    let modified_time;
    if let Ok(metadata_) = metadata {
        if let Ok(modified_time_) = metadata_.modified() {
            modified_time = modified_time_;
        } else {
            modified_time = SystemTime::now();
        }
    } else {
        modified_time = SystemTime::now();
    }
    let datetime = chrono::DateTime::<chrono::Local>::from(modified_time);
    datetime.format("%d %B %Y %H:%M").to_string()
}

/// struct to pass around the UI elements for rendering and updating.
struct SigleplayerSelectorElements {
    position: f32,
    top_rect: gfx::RenderRect,
    bottom_rect: gfx::RenderRect,
    title: gfx::Sprite,
    back_button: gfx::Button,
    new_world_button: gfx::Button,
    world_list: WorldList,
    top_height: f32,
    bottom_height: f32,
}

/// World is a struct that contains all information to
/// render the world in singleplayer selector.
pub struct World {
    pub name: String,
    rect: gfx::RenderRect,
    play_button: gfx::Button,
    delete_button: gfx::Button,
    last_modified: gfx::Sprite,
    title: gfx::Sprite,
    icon: gfx::Sprite,
    file_path: PathBuf,
}

impl World {
    pub fn new(graphics: &gfx::GraphicsContext, file_path: PathBuf) -> Self {
        let stem = file_path.file_stem();
        let name = stem.map_or("incorrect_file_path", |name_| {
            name_.to_str().unwrap_or("invalid_text_format")
        }).to_owned();

        let mut rect = gfx::RenderRect::new(
            gfx::FloatPos(0.0, 0.0),
            gfx::FloatSize(MENU_WIDTH - 2.0 * gfx::SPACING, 0.0),
        );
        rect.orientation = gfx::TOP;
        rect.fill_color.a = 100;
        rect.smooth_factor = 60.0;

        let mut icon = gfx::Sprite::new();
        icon.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/world_icon.opa"
            )).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))),
        );
        rect.size.1 = icon.get_size().1 + 2.0 * gfx::SPACING;
        icon.pos.0 = gfx::SPACING;
        icon.orientation = gfx::LEFT;

        let mut title = gfx::Sprite::new();
        title.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(&name));
        title.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        title.pos.1 = gfx::SPACING;
        title.scale = 3.0;

        let mut play_button = gfx::Button::new();
        play_button.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/play_button.opa"
            )).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))),
        );
        play_button.scale = 3.0;
        play_button.padding = 5.0;
        play_button.pos.0 = icon.pos.0 + icon.get_size().0 + gfx::SPACING;
        play_button.pos.1 = -gfx::SPACING;
        play_button.orientation = gfx::BOTTOM_LEFT;

        let mut delete_button = gfx::Button::new();
        delete_button.texture = gfx::Texture::load_from_surface(
            &gfx::Surface::deserialize_from_bytes(include_bytes!(
                "../../Build/Resources/delete_button.opa"
            )).unwrap_or_else(|_| gfx::Surface::new(gfx::IntSize(1, 1))),
        );
        delete_button.scale = 3.0;
        delete_button.padding = 5.0;
        delete_button.pos.0 = play_button.pos.0 + play_button.get_size().0 + gfx::SPACING;
        delete_button.pos.1 = -gfx::SPACING;
        delete_button.orientation = gfx::BOTTOM_LEFT;

        let mut last_modified = gfx::Sprite::new();
        last_modified.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface(
            get_last_modified_time(file_path.as_path().to_str().unwrap_or("")).as_str(),
        ));
        last_modified.color = gfx::GREY;
        last_modified.orientation = gfx::BOTTOM_RIGHT;
        last_modified.pos.0 = -gfx::SPACING;
        last_modified.pos.1 = -gfx::SPACING;
        last_modified.scale = 2.0;

        Self {
            name,
            rect,
            play_button,
            delete_button,
            last_modified,
            title,
            icon,
            file_path,
        }
    }

    /// This function renders the world card on the x and y position.
    pub fn render(
        &mut self,
        graphics: &mut gfx::GraphicsContext,
        pos: gfx::FloatPos,
        parent_container: Option<&gfx::Container>,
    ) {
        self.rect.pos = pos;
        self.rect.render(graphics, parent_container);

        let rect_container = self.rect.get_container(graphics, parent_container);
        self.icon.render(graphics, Some(&rect_container));
        self.title.render(graphics, Some(&rect_container));
        self.play_button.render(graphics, Some(&rect_container));
        self.delete_button.render(graphics, Some(&rect_container));
        self.last_modified.render(graphics, Some(&rect_container));
    }

    /// This function returns height of the world card.
    pub const fn get_height(&self) -> f32 {
        self.rect.size.1
    }

    /// This function disables/enables the world card buttons.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.play_button.disabled = !enabled;
        self.delete_button.disabled = !enabled;
    }

    /// This function returns the container of the world card.
    pub fn get_container(
        &self,
        graphics: &gfx::GraphicsContext,
        parent_container: Option<&gfx::Container>,
    ) -> gfx::Container {
        self.rect.get_container(graphics, parent_container)
    }

    const fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

/// `WorldList` is a struct that is used to list all worlds in the world folder
/// and render them in the singleplayer selector menu.
pub struct WorldList {
    pub worlds: Vec<World>,
}

impl WorldList {
    pub fn new(graphics: &gfx::GraphicsContext) -> Self {
        let mut world_list = Self { worlds: Vec::new() };
        world_list.refresh(graphics);
        world_list
    }

    pub fn refresh(&mut self, graphics: &gfx::GraphicsContext) {
        let base_dirs;
        if let Some(base_dirs_) = BaseDirs::new() {
            base_dirs = base_dirs_;
        } else {
            return;
        }
        let world_dir = base_dirs.data_dir().join("Terralistic").join("Worlds");
        if !world_dir.exists() {
            let res = fs::create_dir_all(&world_dir);
            if res.is_err() {
                println!("could not create world dirs");
                return;
            }
        }
        self.worlds.clear();
        if let Ok(dir) = fs::read_dir(&world_dir) {
            for entry in dir.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if !path.is_dir() && ext == "world" {
                        self.worlds.push(World::new(graphics, path));
                    }
                }
            }
        }
    }
}

pub fn run_singleplayer_selector(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
) {
    let world_list = WorldList::new(graphics);

    let mut title = gfx::Sprite::new();
    title.scale = 3.0;
    title.texture = gfx::Texture::load_from_surface(
        &graphics.font.create_text_surface("Select a world to play!"),
    );
    title.pos.1 = gfx::SPACING;
    title.orientation = gfx::TOP;

    let mut back_button = gfx::Button::new();
    back_button.scale = 3.0;
    back_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Back"));
    back_button.pos.1 = -gfx::SPACING;
    back_button.orientation = gfx::BOTTOM;

    let mut new_world_button = gfx::Button::new();
    new_world_button.scale = 3.0;
    new_world_button.texture = gfx::Texture::load_from_surface(&graphics.font.create_text_surface("New"));
    new_world_button.pos.0 = -gfx::SPACING;
    new_world_button.pos.1 = -gfx::SPACING;
    new_world_button.orientation = gfx::BOTTOM_RIGHT;

    let top_height = title.get_size().1 + 2.0 * gfx::SPACING;
    let bottom_height = back_button.get_size().1 + 2.0 * gfx::SPACING;

    let mut top_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, top_height));
    top_rect.orientation = gfx::TOP;

    let mut bottom_rect = gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, bottom_height));
    bottom_rect.fill_color.a = gfx::TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
    bottom_rect.blur_radius = gfx::BLUR;
    bottom_rect.orientation = gfx::BOTTOM;

    let position: f32 = 0.0;

    let mut elements = SigleplayerSelectorElements {
        position,
        top_rect,
        bottom_rect,
        title,
        back_button,
        new_world_button,
        world_list,
        top_height,
        bottom_height,
    };

    let mut top_rect_visibility = 1.0;
    while graphics.renderer.is_window_open() {
        if update_elements(graphics, menu_back, &mut elements) {
            break;
        }
        render_elements(graphics, menu_back, &mut elements, &mut top_rect_visibility);
    }
}

fn update_elements(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut SigleplayerSelectorElements,
) -> bool {
    while let Some(event) = graphics.renderer.get_event() {
        match event {
            gfx::Event::KeyRelease(key, ..) => match key {
                gfx::Key::MouseLeft => {
                    if elements.back_button.is_hovered(graphics, Some(menu_back.get_back_rect_container())) {
                        return true;
                    }
                    if elements.new_world_button.is_hovered(graphics, Some(menu_back.get_back_rect_container())) {
                        run_world_creation(graphics, menu_back, &mut elements.world_list.worlds);
                        elements.world_list.refresh(graphics);
                    }

                    let mut needs_refresh = false;
                    for world in &mut elements.world_list.worlds {
                        if world.play_button.is_hovered(
                            graphics,
                            Some(&world.get_container(
                                graphics,
                                Some(menu_back.get_back_rect_container()),
                            )),
                        ) {
                            let game_result = run_private_world(graphics, menu_back, world.get_file_path());
                            if let Err(error) = game_result {
                                println!("Game error: {error}");
                            }
                        } else if world.delete_button.is_hovered(
                            graphics,
                            Some(&world.get_container(
                                graphics,
                                Some(menu_back.get_back_rect_container()),
                            )),
                        ) && run_choice_menu(
                            format!(
                                "The world \"{}\" will be deleted.\nDo you want to proceed?",
                                world.name
                            ).as_str(),
                            graphics,
                            menu_back,
                            None,
                            None,
                        ) {
                            let res = fs::remove_file(world.get_file_path());
                            if res.is_err() {
                                println!("failed to delete the world");
                            }
                            needs_refresh = true;
                        }
                    }
                    if needs_refresh {
                        elements.world_list.refresh(graphics);
                    }
                }
                gfx::Key::Escape => {
                    return true;
                }
                _ => {}
            },
            gfx::Event::MouseScroll(delta) => {
                elements.position += delta * 2.0;
            }
            _ => {}
        }
    }
    false
}

fn render_elements(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    elements: &mut SigleplayerSelectorElements,
    top_rect_visibility: &mut f32,
) {
    menu_back.set_back_rect_width(MENU_WIDTH);

    menu_back.render_back(graphics);

    let hoverable = graphics.renderer.get_mouse_pos().1 > elements.top_height && graphics.renderer.get_mouse_pos().1 < graphics.renderer.get_window_size().1 - elements.bottom_height;

    for world in &mut elements.world_list.worlds {
        world.set_enabled(hoverable);
    }

    let mut current_y = gfx::SPACING;

    for world in &mut elements.world_list.worlds {
        world.render(
            graphics,
            gfx::FloatPos(0.0, current_y + elements.top_height + elements.position),
            Some(menu_back.get_back_rect_container()),
        );
        current_y += world.get_height() + gfx::SPACING;
    }

    elements.top_rect.size.0 = menu_back.get_back_rect_width(graphics, None);
    *top_rect_visibility += ((if elements.position < -5.0 { 1.0 } else { 0.0 }) - *top_rect_visibility) / 20.0;

    if *top_rect_visibility < 0.01 {
        *top_rect_visibility = 0.0;
    }

    if *top_rect_visibility > 0.99 {
        *top_rect_visibility = 1.0;
    }

    elements.top_rect.fill_color.a = (*top_rect_visibility * gfx::TRANSPARENCY as f32 / 2.0) as u8;
    elements.top_rect.blur_radius = (*top_rect_visibility * gfx::BLUR as f32) as i32;
    elements.top_rect.shadow_intensity = (*top_rect_visibility * gfx::SHADOW_INTENSITY as f32) as i32;
    if *top_rect_visibility > 0.0 {
        elements.top_rect.render(graphics, Some(menu_back.get_back_rect_container()));
    }

    elements.bottom_rect.size.0 = menu_back.get_back_rect_width(graphics, None);
    let mut scroll_limit = current_y - graphics.renderer.get_window_size().1 + elements.top_height + elements.bottom_height;
    if scroll_limit < 0.0 {
        scroll_limit = 0.0;
    }

    if scroll_limit > 0.0 {
        elements.bottom_rect.render(graphics, Some(menu_back.get_back_rect_container()));
    }

    if elements.position > 0.0 {
        elements.position -= elements.position / 20.0;
    }

    if elements.position < -scroll_limit {
        elements.position -= (elements.position + scroll_limit) / 20.0;
    }

    elements.title.render(graphics, Some(menu_back.get_back_rect_container()));
    elements.back_button.render(graphics, Some(menu_back.get_back_rect_container()));

    elements.new_world_button.render(graphics, Some(menu_back.get_back_rect_container()));

    graphics.renderer.update_window();
}
