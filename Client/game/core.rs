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
use crate::game::walls::ClientWalls;
use crate::menus::{run_loading_screen, BackgroundRect};

pub struct Game {
    events: EventManager,
    networking: ClientNetworking,
    mods: ClientModManager,
    camera: Camera,
    background: Background,
    block_selector: BlockSelector,
    blocks: ClientBlocks,
    walls: ClientWalls,
}

impl Game {
    pub fn new(server_port: u16, server_address: String) -> Self {
        let mut blocks = ClientBlocks::new();
        let walls = ClientWalls::new(&mut blocks.blocks);

        Self {
            events: EventManager::new(),
            networking: ClientNetworking::new(server_port, server_address),
            mods: ClientModManager::new(),
            camera: Camera::new(),
            background: Background::new(),
            block_selector: BlockSelector::new(),
            blocks,
            walls,
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn run(&mut self, graphics: &mut GraphicsContext, menu_back: &mut dyn BackgroundRect) {
        // load base game mod
        let mut events = EventManager::new();
        self.networking.init(&mut events);

        let timer = std::time::Instant::now();

        let loading_text = Arc::new(Mutex::new("Loading".to_string()));
        let loading_text2 = loading_text.clone();

        let init_thread = std::thread::spawn(move || {
            *loading_text2.lock().unwrap() = "Loading mods".to_string();
            let mut mods = ClientModManager::new();
            let mut blocks = ClientBlocks::new();
            let mut walls = ClientWalls::new(&mut blocks.blocks);

            while let Some(event) = events.pop_event() {
                mods.on_event(&event);
                blocks.on_event(&event, &mut events);
                walls.on_event(&event);
            }

            blocks.init(&mut mods.mod_manager);
            walls.init(&mut mods.mod_manager);

            *loading_text2.lock().unwrap() = "Initializing mods".to_string();
            mods.init();

            loading_text2.lock().unwrap().clear();
            (mods, blocks, walls)
        });

        run_loading_screen(graphics, menu_back, &loading_text);

        let result = init_thread.join().unwrap();
        self.mods = result.0;
        self.blocks = result.1;
        self.walls = result.2;

        self.camera.set_position(
            self.blocks.blocks.get_width() as f32 / 2.0 * BLOCK_WIDTH as f32,
            self.blocks.blocks.get_height() as f32 / 3.0 * BLOCK_WIDTH as f32,
        );

        self.background.init();

        self.blocks.load_resources(&mut self.mods.mod_manager);
        self.walls.load_resources(&mut self.mods.mod_manager);

        // print the time it took to initialize
        println!("Game joined in {}ms", timer.elapsed().as_millis());

        let mut paused = false;

        let mut resume_button = gfx::Button::new();
        resume_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Resume"));
        resume_button.scale = 3.0;
        resume_button.y = gfx::SPACING;
        resume_button.x = -gfx::SPACING;
        resume_button.orientation = gfx::TOP_RIGHT;

        let mut quit_button = gfx::Button::new();
        quit_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Quit"));
        quit_button.scale = 3.0;
        quit_button.y = 2 * gfx::SPACING + resume_button.get_height();
        quit_button.x = -gfx::SPACING;
        quit_button.orientation = gfx::TOP_RIGHT;

        let pause_rect_width =
            i32::max(resume_button.get_width(), quit_button.get_width()) + 2 * gfx::SPACING;
        let mut pause_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, 0.0);
        pause_rect.fill_color = gfx::BLACK;
        pause_rect.fill_color.a = gfx::TRANSPARENCY;
        pause_rect.border_color = gfx::BORDER_COLOR;
        pause_rect.blur_radius = gfx::BLUR;
        pause_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        pause_rect.smooth_factor = 60.0;
        pause_rect.w = pause_rect_width as f32;

        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        let mut prev_time = std::time::Instant::now();

        let mut fps_counter = 0;
        let mut frame_time_counter = 0.0;
        let mut max_frame_time = 0.0;
        let mut stat_update_timer = std::time::Instant::now();

        let mut fps_stat = 0;
        let mut max_frame_time_stat = 0.0;
        let mut avg_frame_time_stat = 0.0;

        let mut debug_menu_open = false;

        let mut debug_menu_rect = gfx::RenderRect::new(0.0, 0.0, 0.0, 0.0);
        debug_menu_rect.fill_color = gfx::BLACK;
        debug_menu_rect.fill_color.a = gfx::TRANSPARENCY;
        debug_menu_rect.border_color = gfx::BORDER_COLOR;
        debug_menu_rect.blur_radius = gfx::BLUR;
        debug_menu_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        debug_menu_rect.smooth_factor = 60.0;
        debug_menu_rect.orientation = gfx::BOTTOM_RIGHT;
        debug_menu_rect.y = -gfx::SPACING as f32;
        debug_menu_rect.w = 300.0;
        debug_menu_rect.h = 200.0;

        'main_loop: while graphics.renderer.is_window_open() {
            let delta_time = prev_time.elapsed().as_secs_f32() * 1000.0;
            prev_time = std::time::Instant::now();

            while let Some(event) = graphics.renderer.get_event() {
                match event {
                    gfx::Event::KeyPress(key, false) => {
                        if key == gfx::Key::Escape {
                            paused = !paused;
                        } else if key == gfx::Key::M {
                            debug_menu_open = !debug_menu_open;
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

                self.events.push_event(events::Event::new(event));
            }

            if stat_update_timer.elapsed().as_secs() >= 1 {
                fps_stat = fps_counter;
                max_frame_time_stat = max_frame_time;
                avg_frame_time_stat = frame_time_counter / fps_counter as f32;

                fps_counter = 0;
                max_frame_time = 0.0;
                frame_time_counter = 0.0;
                stat_update_timer = std::time::Instant::now();
            }

            self.networking.update(&mut self.events);
            self.mods.update();
            self.blocks.update(delta_time, &mut self.events);
            self.walls.update(delta_time, &mut self.events);

            while ms_counter < ms_timer.elapsed().as_millis() as i32 {
                self.camera.update_ms(graphics);
                ms_counter += 1;
            }

            self.background.render(graphics, &self.camera);
            self.walls.render(graphics, &self.camera);
            self.blocks.render(graphics, &self.camera);
            self.block_selector
                .render(graphics, &mut self.networking, &self.camera);

            pause_rect.x = if paused { 0.0 } else { -pause_rect.w - 100.0 };
            pause_rect.render(graphics, None);

            if graphics.renderer.get_window_height() != pause_rect.h as u32 {
                pause_rect.h = graphics.renderer.get_window_height() as f32;
                pause_rect.jump_to_target();
            }

            resume_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));
            quit_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));

            while let Some(event) = self.events.pop_event() {
                self.mods.on_event(&event);
                self.blocks.on_event(&event, &mut self.events);
                self.walls.on_event(&event);
                self.block_selector
                    .on_event(graphics, &mut self.networking, &self.camera, &event);
            }

            debug_menu_rect.x = if debug_menu_open {
                -gfx::SPACING as f32
            } else {
                debug_menu_rect.w + 100.0
            };
            debug_menu_rect.render(graphics, None);

            if debug_menu_rect.get_container(graphics, None).rect.x
                < graphics.renderer.get_window_width() as i32
            {
                let mut texts = Vec::new();
                texts.push(format!("FPS: {fps_stat}"));
                texts.push(format!("{max_frame_time_stat:.2} ms max"));
                texts.push(format!("{avg_frame_time_stat:.2} ms avg"));

                let mut text_textures = Vec::new();
                for text in texts {
                    text_textures.push(gfx::Texture::load_from_surface(
                        &graphics.font.create_text_surface(&text),
                    ));
                }

                let scale = 3.0;

                let mut width = 0;
                let mut height = 0;
                for texture in &text_textures {
                    width = i32::max(width, (texture.get_texture_width() as f32 * scale) as i32);
                    height += (texture.get_texture_height() as f32 * scale) as i32;
                }

                debug_menu_rect.w = width as f32 + 2.0 * gfx::SPACING as f32;
                debug_menu_rect.h = height as f32 + 2.0 * gfx::SPACING as f32;

                let mut y = gfx::SPACING;
                let debug_menu_rect_container = debug_menu_rect.get_container(graphics, None);
                let debug_menu_rect_container = debug_menu_rect_container.get_absolute_rect();
                for texture in &text_textures {
                    texture.render(
                        &graphics.renderer,
                        scale,
                        (
                            debug_menu_rect_container.x + gfx::SPACING,
                            debug_menu_rect_container.y + y,
                        ),
                        None,
                        false,
                        None,
                    );
                    y += (texture.get_texture_height() as f32 * scale) as i32;
                }
            }

            fps_counter += 1;
            frame_time_counter += prev_time.elapsed().as_secs_f32() * 1000.0;
            max_frame_time = f32::max(max_frame_time, prev_time.elapsed().as_secs_f32() * 1000.0);

            graphics.renderer.update_window();
        }

        self.networking.stop();
        self.mods.stop();
    }
}
