extern crate alloc;

use alloc::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::{bail, Result};

use crate::client::game::entities::ClientEntities;
use crate::client::game::items::ClientItems;
use crate::client::game::players::ClientPlayers;
use crate::client::menus::{run_loading_screen, BackgroundRect};
use crate::libraries::events::EventManager;
use crate::libraries::graphics::{FloatPos, FloatSize, GraphicsContext};
use crate::libraries::{events, graphics as gfx};
use crate::shared::entities::PositionComponent;

use super::background::Background;
use super::block_selector::BlockSelector;
use super::blocks::ClientBlocks;
use super::camera::Camera;
use super::mod_manager::ClientModManager;
use super::networking::ClientNetworking;
use super::walls::ClientWalls;

pub struct Game {
    events: EventManager,
    networking: ClientNetworking,
    mods: ClientModManager,
    camera: Camera,
    background: Background,
    block_selector: BlockSelector,
    blocks: ClientBlocks,
    walls: ClientWalls,
    entities: ClientEntities,
    items: ClientItems,
    players: ClientPlayers,
    player_name: String,
}

impl Game {
    #[must_use]
    pub fn new(server_port: u16, server_address: String, player_name: &str) -> Self {
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
            entities: ClientEntities::new(),
            items: ClientItems::new(),
            players: ClientPlayers::new(player_name),
            player_name: player_name.to_owned(),
        }
    }

    #[allow(clippy::too_many_lines)]
    /// # Errors
    /// If the game basically crashes
    pub fn run(
        &mut self,
        graphics: &mut GraphicsContext,
        menu_back: &mut dyn BackgroundRect,
    ) -> Result<()> {
        // load base game mod
        let mut events = EventManager::new();
        self.networking.init(self.player_name.clone())?;
        while self.networking.is_welcoming() {
            // wait 1 ms
            std::thread::sleep(core::time::Duration::from_millis(1));
        }
        self.networking.update(&mut events)?;
        self.networking.start_receiving();

        let timer = std::time::Instant::now();

        let loading_text = Arc::new(Mutex::new("Loading".to_owned()));
        let loading_text2 = loading_text.clone();

        let init_thread = std::thread::spawn(move || {
            let mut temp_fn = || -> Result<(ClientModManager, ClientBlocks, ClientWalls, ClientEntities, ClientItems)> {
                *loading_text2.lock().unwrap_or_else(PoisonError::into_inner) = "Loading mods".to_owned();
                let mut mods = ClientModManager::new();
                let mut blocks = ClientBlocks::new();
                let mut walls = ClientWalls::new(&mut blocks.blocks);
                let mut entities = ClientEntities::new();
                let mut items = ClientItems::new();

                while let Some(event) = events.pop_event() {
                    mods.on_event(&event)?;
                    blocks.on_event(&event, &mut events)?;
                    walls.on_event(&event)?;
                    items.on_event(&event, &mut entities.entities, &mut events)?;
                }

                blocks.init(&mut mods.mod_manager)?;
                walls.init(&mut mods.mod_manager)?;
                items.init(&mut mods.mod_manager)?;

                *loading_text2.lock().unwrap_or_else(PoisonError::into_inner) = "Initializing mods".to_owned();
                mods.init()?;

                anyhow::Ok((mods, blocks, walls, entities, items))
            };
            // if the init fails, we clear the loading text so the error can be displayed
            let result = temp_fn();
            loading_text2
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clear();
            result
        });

        run_loading_screen(graphics, menu_back, &loading_text);

        let result = init_thread.join();
        let Ok(result) = result else { bail!("Failed to join init thread"); };
        let result = result?;
        self.mods = result.0;
        self.blocks = result.1;
        self.walls = result.2;
        self.entities = result.3;
        self.items = result.4;

        self.background.init()?;

        self.blocks.load_resources(&mut self.mods.mod_manager)?;
        self.walls.load_resources(&mut self.mods.mod_manager)?;
        self.items.load_resources(&mut self.mods.mod_manager)?;
        self.camera.load_resources(graphics);
        self.players.load_resources(&mut self.mods.mod_manager)?;

        // print the time it took to initialize
        println!("Game joined in {}ms", timer.elapsed().as_millis());

        let mut paused = false;

        let mut resume_button = gfx::Button::new();
        resume_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Resume"));
        resume_button.scale = 3.0;
        resume_button.pos.0 = -gfx::SPACING;
        resume_button.pos.1 = gfx::SPACING;
        resume_button.orientation = gfx::TOP_RIGHT;

        let mut quit_button = gfx::Button::new();
        quit_button.texture =
            gfx::Texture::load_from_surface(&graphics.font.create_text_surface("Quit"));
        quit_button.scale = 3.0;
        quit_button.pos.0 = -gfx::SPACING;
        quit_button.pos.1 = 2.0 * gfx::SPACING + resume_button.get_size().1;
        quit_button.orientation = gfx::TOP_RIGHT;

        let pause_rect_width =
            f32::max(resume_button.get_size().0, quit_button.get_size().0) + 2.0 * gfx::SPACING;
        let mut pause_rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0));
        pause_rect.fill_color = gfx::BLACK;
        pause_rect.fill_color.a = gfx::TRANSPARENCY;
        pause_rect.border_color = gfx::BORDER_COLOR;
        pause_rect.blur_radius = gfx::BLUR;
        pause_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        pause_rect.smooth_factor = 60.0;
        pause_rect.size.0 = pause_rect_width;

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

        let mut debug_menu_rect = gfx::RenderRect::new(FloatPos(0.0, 0.0), FloatSize(0.0, 0.0));
        debug_menu_rect.fill_color = gfx::BLACK;
        debug_menu_rect.fill_color.a = gfx::TRANSPARENCY;
        debug_menu_rect.border_color = gfx::BORDER_COLOR;
        debug_menu_rect.blur_radius = gfx::BLUR;
        debug_menu_rect.shadow_intensity = gfx::SHADOW_INTENSITY;
        debug_menu_rect.smooth_factor = 60.0;
        debug_menu_rect.orientation = gfx::BOTTOM_RIGHT;
        debug_menu_rect.pos.1 = -gfx::SPACING;
        debug_menu_rect.size.0 = 300.0;
        debug_menu_rect.size.1 = 200.0;

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

            self.networking.update(&mut self.events)?;
            self.mods.update()?;
            self.blocks.update(delta_time, &mut self.events)?;
            self.walls.update(delta_time, &mut self.events)?;

            if let Some(main_player) = self.players.get_main_player() {
                let player_pos = self
                    .entities
                    .entities
                    .ecs
                    .get::<&PositionComponent>(main_player)?;

                self.camera.set_position(player_pos.x(), player_pos.y());
            }

            while ms_counter < ms_timer.elapsed().as_millis() as i32 {
                self.camera.update_ms(graphics);
                self.players.update(
                    graphics,
                    &mut self.entities.entities,
                    &mut self.networking,
                    &self.blocks.blocks,
                )?;
                self.entities
                    .entities
                    .update_entities_ms(&self.blocks.blocks);
                ms_counter += 5;
            }

            self.background.render(graphics, &self.camera);
            self.walls.render(graphics, &self.camera)?;
            self.blocks.render(graphics, &self.camera)?;
            self.players
                .render(graphics, &mut self.entities.entities, &self.camera);
            self.items
                .render(graphics, &self.camera, &mut self.entities.entities)?;
            self.camera.render(graphics);
            self.block_selector
                .render(graphics, &mut self.networking, &self.camera)?;

            pause_rect.pos.0 = if paused {
                0.0
            } else {
                -pause_rect.size.0 - 100.0
            };
            pause_rect.render(graphics, None);

            if graphics.renderer.get_window_size().1 as u32 != pause_rect.size.1 as u32 {
                pause_rect.size.1 = graphics.renderer.get_window_size().1;
                pause_rect.jump_to_target();
            }

            resume_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));
            quit_button.render(graphics, Some(&pause_rect.get_container(graphics, None)));

            while let Some(event) = self.events.pop_event() {
                self.mods.on_event(&event)?;
                self.blocks.on_event(&event, &mut self.events)?;
                self.walls.on_event(&event)?;
                self.entities.on_event(&event)?;
                self.items
                    .on_event(&event, &mut self.entities.entities, &mut self.events)?;
                self.block_selector.on_event(
                    graphics,
                    &mut self.networking,
                    &self.camera,
                    &event,
                )?;
                self.players.on_event(&event, &mut self.entities.entities);
                self.camera.on_event(&event);
            }

            debug_menu_rect.pos.0 = if debug_menu_open {
                -gfx::SPACING
            } else {
                debug_menu_rect.size.0 + 100.0
            };
            debug_menu_rect.render(graphics, None);

            if (debug_menu_rect.get_container(graphics, None).rect.pos.0 as i32)
                < graphics.renderer.get_window_size().0 as i32
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

                let mut width = 0.0;
                let mut height = 0.0;
                for texture in &text_textures {
                    width = f32::max(width, texture.get_texture_size().0 * scale);
                    height += texture.get_texture_size().1 * scale;
                }

                debug_menu_rect.size.0 = width + 2.0 * gfx::SPACING;
                debug_menu_rect.size.1 = height + 2.0 * gfx::SPACING;

                let mut y = gfx::SPACING;
                let debug_menu_rect_container = debug_menu_rect.get_container(graphics, None);
                let debug_menu_rect_container = debug_menu_rect_container.get_absolute_rect();
                for texture in &text_textures {
                    texture.render(
                        &graphics.renderer,
                        scale,
                        FloatPos(
                            debug_menu_rect_container.pos.0 + gfx::SPACING,
                            debug_menu_rect_container.pos.1 + y,
                        ),
                        None,
                        false,
                        None,
                    );
                    y += texture.get_texture_size().1 * scale;
                }
            }

            fps_counter += 1;
            frame_time_counter += prev_time.elapsed().as_secs_f32() * 1000.0;
            max_frame_time = f32::max(max_frame_time, prev_time.elapsed().as_secs_f32() * 1000.0);

            graphics.renderer.update_window();
        }

        self.networking.stop()?;
        self.mods.stop()?;

        Ok(())
    }
}
