use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::{bail, Result};

use crate::client::game::chat::ClientChat;
use crate::client::game::debug_menu::DebugMenu;
use crate::client::game::entities::ClientEntities;
use crate::client::game::floating_text::FloatingTextManager;
use crate::client::game::framerate_measurer::FramerateMeasurer;
use crate::client::game::health::ClientHealth;
use crate::client::game::inventory::ClientInventory;
use crate::client::game::items::ClientItems;
use crate::client::game::lights::ClientLights;
use crate::client::game::pause_menu::PauseMenu;
use crate::client::game::players::ClientPlayers;
use crate::client::game::respawn_screen::RespawnScreen;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{run_loading_screen, BackgroundRect};
use crate::client::settings::Settings;
use crate::libraries::events;
use crate::libraries::events::EventManager;
use crate::libraries::graphics as gfx;
use crate::shared::entities::PositionComponent;

use super::background::Background;
use super::block_selector::BlockSelector;
use super::blocks::ClientBlocks;
use super::camera::Camera;
use super::mod_manager::ClientModManager;
use super::networking::ClientNetworking;
use super::walls::ClientWalls;

#[allow(clippy::too_many_lines)]
pub fn run_game(
    graphics: &mut gfx::GraphicsContext,
    menu_back: &mut dyn BackgroundRect,
    server_port: u16,
    server_address: String,
    player_name: &str,
    settings: &mut Settings,
    global_settings: &mut GlobalSettings,
) -> Result<()> {
    // load base game mod
    let mut pre_events = EventManager::new();
    let mut networking = ClientNetworking::new(server_port, server_address);
    networking.init(player_name.to_owned())?;
    while networking.is_welcoming() {
        // wait 1 ms
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    networking.update(&mut pre_events)?;
    networking.start_receiving();

    let timer = std::time::Instant::now();

    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));
    let loading_text2 = loading_text.clone();

    let init_thread = std::thread::spawn(move || {
        let temp_fn = || -> Result<(ClientModManager, ClientBlocks, ClientWalls, ClientEntities, ClientItems, ClientNetworking)> {
            *loading_text2.lock().unwrap_or_else(PoisonError::into_inner) = "Loading mods".to_owned();
            let mut mods = ClientModManager::new();
            let mut blocks = ClientBlocks::new();
            let mut walls = ClientWalls::new(&mut blocks.get_blocks());
            let mut entities = ClientEntities::new();
            let mut items = ClientItems::new();

            while let Some(event) = pre_events.pop_event() {
                mods.on_event(&event)?;
                blocks.on_event(&event, &mut pre_events, &mut mods.mod_manager, &mut networking)?;
                walls.on_event(&event)?;
                items.on_event(&event, &mut entities.entities, &mut pre_events)?;
            }

            blocks.init(&mut mods.mod_manager)?;
            walls.init(&mut mods.mod_manager)?;
            items.init(&mut mods.mod_manager)?;

            *loading_text2.lock().unwrap_or_else(PoisonError::into_inner) = "Initializing mods".to_owned();
            mods.init()?;

            anyhow::Ok((mods, blocks, walls, entities, items, networking))
        };
        // if the init fails, we clear the loading text so the error can be displayed
        let result = temp_fn();
        loading_text2.lock().unwrap_or_else(PoisonError::into_inner).clear();
        result
    });

    run_loading_screen(graphics, menu_back, &loading_text);

    let result = init_thread.join();
    let Ok(result) = result else {
        bail!("Failed to join init thread");
    };
    let result = result?;

    let mut mods = result.0;
    let mut blocks = result.1;
    let mut walls = result.2;
    let mut entities = result.3;
    let mut items = result.4;
    let mut networking = result.5;

    let mut background = Background::new();
    let mut inventory = ClientInventory::new();
    let mut lights = ClientLights::new();
    let mut events = EventManager::new();
    let mut camera = Camera::new();
    let mut players = ClientPlayers::new(player_name);
    let mut block_selector = BlockSelector::new();
    let mut pause_menu = PauseMenu::new();
    let mut debug_menu = DebugMenu::new();
    let mut framerate_measurer = FramerateMeasurer::new();
    let mut chat = ClientChat::new(graphics);
    let mut health = ClientHealth::new();
    let mut floating_text = FloatingTextManager::new();
    let mut respawn_screen = RespawnScreen::new();

    background.init()?;
    inventory.init();
    lights.init(&blocks.get_blocks(), settings)?;

    blocks.load_resources(&mods.mod_manager)?;
    walls.load_resources(&mods.mod_manager)?;
    items.load_resources(&mods.mod_manager)?;
    camera.load_resources(graphics);
    players.load_resources(&mods.mod_manager)?;
    health.load_resources(&mods.mod_manager)?;

    pause_menu.init(graphics, settings);
    debug_menu.init();
    chat.init();
    respawn_screen.init(graphics);

    // print the time it took to initialize
    println!("Game joined in {}ms", timer.elapsed().as_millis());

    'main_loop: while graphics.is_window_open() {
        framerate_measurer.update();

        while let Some(event) = graphics.get_event() {
            events.push_event(events::Event::new(event));
        }

        graphics.block_key_states = chat.is_selected();

        networking.update(&mut events)?;
        mods.update()?;
        blocks.update(framerate_measurer.get_delta_time(), &mut events)?;
        walls.update(framerate_measurer.get_delta_time(), &mut events)?;

        if let Some(main_player) = players.get_main_player() {
            let player_pos = entities.entities.ecs.get::<&PositionComponent>(main_player)?;

            camera.set_position(player_pos.x(), player_pos.y());
        }

        while framerate_measurer.has_5ms_passed() {
            camera.update_ms(graphics);
            players.controls_enabled = !camera.is_detached();
            players.update(graphics, &mut entities.entities, &mut networking, &blocks.get_blocks())?;
            entities.entities.update_entities_ms(&blocks.get_blocks(), &mut events)?;
        }

        respawn_screen.is_shown = players.get_main_player().is_none() && !players.is_waiting_for_player();

        background.render(graphics, &camera);
        walls.render(graphics, &camera)?;
        blocks.render(graphics, &camera)?;
        players.render(graphics, &mut entities.entities, &camera);
        items.render(graphics, &camera, &mut entities.entities)?;
        floating_text.render(graphics, &camera);
        lights.render(graphics, &camera, &blocks.get_blocks(), settings)?;
        camera.render(graphics);
        block_selector.render(graphics, &mut networking, &camera)?;
        inventory.render(graphics, &items, &mut networking, &blocks.get_blocks())?;
        health.render(graphics);
        chat.render(graphics);
        respawn_screen.render(graphics);

        pause_menu.render(graphics, settings, global_settings);

        debug_menu.render(
            graphics,
            &[
                format!("FPS: {}", framerate_measurer.get_fps()),
                format!("{:.2} ms max", framerate_measurer.get_max_frame_time()),
                format!("{:.2} ms avg", framerate_measurer.get_avg_frame_time()),
            ],
        );

        while let Some(event) = events.pop_event() {
            if chat.on_event(&event, graphics, &mut networking)? {
                continue;
            }
            inventory.on_event(&event, &mut networking, &items, &mut blocks.get_blocks(), &mut events)?;
            mods.on_event(&event)?;
            blocks.on_event(&event, &mut events, &mut mods.mod_manager, &mut networking)?;
            walls.on_event(&event)?;
            entities.on_event(&event, &mut events)?;
            items.on_event(&event, &mut entities.entities, &mut events)?;
            block_selector.on_event(graphics, &mut networking, &camera, &event, &mut events)?;
            players.on_event(&event, &mut entities.entities)?;
            lights.on_event(&event, &blocks.get_blocks())?;
            camera.on_event(&event);
            health.on_event(&event, graphics, &mut floating_text, &players, &entities.entities);
            if pause_menu.on_event(&event, graphics, settings) {
                break 'main_loop;
            }
            debug_menu.on_event(&event);
            respawn_screen.on_event(&event, graphics, &mut networking)?;
        }

        framerate_measurer.update_post_render();

        graphics.update_window();
    }

    lights.stop(settings)?;
    networking.stop()?;
    mods.stop()?;

    Ok(())
}
