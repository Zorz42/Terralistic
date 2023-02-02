use std::sync::{Arc, Mutex};

use graphics as gfx;
use graphics::GraphicsContext;
use shared::blocks::BLOCK_WIDTH;

use events::EventManager;

use crate::game::background::Background;
use crate::game::block_selector::BlockSelector;
use crate::game::blocks::ClientBlocks;
use crate::game::camera::Camera;
use crate::game::mod_manager::ClientModManager;
use crate::game::networking::ClientNetworking;
use crate::menus::{run_loading_screen, BackgroundRect};

pub struct Game {
    events: EventManager,
    networking: ClientNetworking,
    mods: ClientModManager,
    camera: Camera,
    background: Background,
    block_selector: BlockSelector,
    blocks: ClientBlocks,
}

impl Game {
    pub fn new(server_port: u16, server_address: String) -> Self {
        Self {
            events: EventManager::new(),
            networking: ClientNetworking::new(server_port, server_address),
            mods: ClientModManager::new(),
            camera: Camera::new(),
            background: Background::new(),
            block_selector: BlockSelector::new(),
            blocks: ClientBlocks::new(),
        }
    }

    pub fn run(&mut self, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) {
        // load base game mod
        let timer = std::time::Instant::now();

        let mut events = EventManager::new();
        self.networking.init(&mut events);

        let loading_text = Arc::new(Mutex::new("Loading".to_string()));
        let loading_text2 = loading_text.clone();

        let init_thread = std::thread::spawn(move || {
            *loading_text2.lock().unwrap() = "Loading mods".to_string();
            let mut mods = ClientModManager::new();
            let mut blocks = ClientBlocks::new();

            while let Some(event) = events.pop_event() {
                mods.on_event(&event);
                blocks.on_event(&event);
            }

            blocks.init(&mut mods.mod_manager);

            *loading_text2.lock().unwrap() = "Initializing mods".to_string();
            mods.init();

            loading_text2.lock().unwrap().clear();
            (mods, blocks)
        });

        run_loading_screen(graphics, menu_back, &loading_text);

        let result = init_thread.join().unwrap();
        self.mods = result.0;
        self.blocks = result.1;

        self.camera.set_position(
            self.blocks.blocks.get_width() as f32 / 2.0 * BLOCK_WIDTH as f32,
            self.blocks.blocks.get_height() as f32 / 3.0 * BLOCK_WIDTH as f32,
        );

        self.background.init();

        self.blocks.load_resources(&mut self.mods.mod_manager);

        // print the time it took to initialize
        println!("Game joined in {}ms", timer.elapsed().as_millis());

        let mut paused = false;

        let mut resume_button = gfx::Button::new();
        resume_button.texture = gfx::Texture::load_from_surface(
            &graphics.font.create_text_surface("Resume".to_string()),
        );
        resume_button.scale = 3.0;
        resume_button.y = gfx::SPACING;
        resume_button.x = -gfx::SPACING;
        resume_button.orientation = gfx::TOP_RIGHT;

        let mut quit_button = gfx::Button::new();
        quit_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Quit".to_string()));
        quit_button.scale = 3.0;
        quit_button.y = 2 * gfx::SPACING + resume_button.get_height();
        quit_button.x = -gfx::SPACING;
        quit_button.orientation = gfx::TOP_RIGHT;

        let pause_rect_width =
            i32::max(resume_button.get_width(), quit_button.get_width()) + 2 * gfx::SPACING;
        let mut pause_rect =
            gfx::RenderRect::new(0.0, 0.0, 0.0, graphics.renderer.get_window_height() as f32);
        pause_rect.fill_color = gfx::BLACK;
        pause_rect.fill_color.a = gfx::TRANSPARENCY;
        pause_rect.border_color = gfx::BORDER_COLOR;
        pause_rect.blur_radius = gfx::BLUR;
        pause_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        pause_rect.smooth_factor = 60.0;

        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        'main_loop: while graphics.renderer.is_window_open() {
            while let Some(event) = graphics.renderer.get_event() {
                match event {
                    gfx::Event::KeyPress(key, false) => {
                        if key == gfx::Key::Escape {
                            paused = !paused;
                        }
                    }
                    gfx::Event::KeyRelease(key, false) => {
                        if key == gfx::Key::MouseLeft && paused {
                            if resume_button.is_hovered(
                                graphics,
                                Some(&pause_rect.get_container(graphics, None)),
                            ) {
                                paused = false;
                            } else if quit_button.is_hovered(
                                graphics,
                                Some(&pause_rect.get_container(graphics, None)),
                            ) {
                                break 'main_loop;
                            }
                        }
                    }
                    _ => {}
                }

                self.events.push_event(events::Event::new(Box::new(event)));
            }

            self.networking.update(&mut self.events);
            self.mods.update();

            while ms_counter < ms_timer.elapsed().as_millis() as i32 {
                self.camera.update_ms(graphics);
                ms_counter += 1;
            }

            self.background.render(graphics, &self.camera);
            self.blocks.render(graphics, &self.camera);
            self.block_selector
                .render(graphics, &mut self.networking, &self.camera);

            pause_rect.w = if paused { pause_rect_width as f32 } else { 0.0 };
            pause_rect.shadow_intensity = (pause_rect.get_container(graphics, None).rect.w as f32
                / pause_rect_width as f32
                * gfx::SHADOW_INTENSITY as f32) as i32;
            pause_rect.render(graphics, None);

            if graphics.renderer.get_window_height() != pause_rect.h as u32 {
                pause_rect.h = graphics.renderer.get_window_height() as f32;
                pause_rect.jump_to_target();
            }

            resume_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));
            quit_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));

            while let Some(event) = self.events.pop_event() {
                self.mods.on_event(&event);
                self.blocks.on_event(&event);
                self.block_selector.on_event(
                    graphics,
                    &mut self.networking,
                    &mut self.camera,
                    &event,
                );
            }

            graphics.renderer.update_window();
        }

        self.networking.stop();
        self.mods.stop();
    }
}
