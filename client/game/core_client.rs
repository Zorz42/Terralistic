use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::{Mutex, PoisonError};

use anyhow::Result;

use crate::client::game::chat::ClientChat;
use crate::client::game::debug_menu::DebugMenu;
use crate::client::game::entities::ClientEntities;
use crate::client::game::floating_text::FloatingTextManager;
use crate::client::game::framerate_measurer::FramerateMeasurer;
use crate::client::game::health::ClientHealth;
use crate::client::game::inventory::ClientInventory;
use crate::client::game::items::ClientItems;
use crate::client::game::lights::ClientLights;
use crate::client::game::liquids::ClientLiquids;
use crate::client::game::pause_menu::PauseMenu;
use crate::client::game::players::ClientPlayers;
use crate::client::game::respawn_screen::RespawnScreen;
use crate::client::global_settings::GlobalSettings;
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
    server_port: u16,
    server_address: String,
    player_name: &str,
    settings: &Rc<RefCell<Settings>>,
    global_settings: &Rc<RefCell<GlobalSettings>>,
) -> Result<()> {
    // load base game mod
    let mut pre_events = EventManager::new();
    let mut networking = ClientNetworking::new(server_port, server_address);
    networking.init(player_name.to_owned())?;
    while networking.is_welcoming() {
        // wait 1 ms
        std::thread::sleep(std::time::Duration::from_millis(1));
        networking.check_thread_for_errors()?;
    }

    networking.update(&mut pre_events)?;
    networking.start_receiving();

    let timer = std::time::Instant::now();

    let loading_text = Arc::new(Mutex::new("Loading".to_owned()));
    let loading_text2 = loading_text;

    let temp_fn = || -> Result<(ClientModManager, ClientBlocks, ClientWalls, ClientLiquids, ClientEntities, ClientItems, ClientNetworking)> {
        "Loading mods".clone_into(&mut loading_text2.lock().unwrap_or_else(PoisonError::into_inner));
        let mut mods = ClientModManager::new();
        let mut blocks = ClientBlocks::new();
        let walls = ClientWalls::new(&mut blocks.get_blocks());
        let mut liquids = ClientLiquids::new();
        let entities = ClientEntities::new();
        let mut items = ClientItems::new();

        while let Some(event) = pre_events.pop_event() {
            mods.on_event(&event)?;
            blocks.on_event(&event, &mut pre_events, &mut networking)?;
            walls.on_event(&event)?;
            liquids.on_event(&event, &mut pre_events)?;
            items.on_event(&event, &mut entities.get_entities(), &mut pre_events)?;
        }

        blocks.init(&mut mods.mod_manager)?;
        walls.init(&mut mods.mod_manager)?;
        liquids.init(&mut mods.mod_manager)?;
        items.init(&mut mods.mod_manager, &entities.get_entities_arc())?;

        "Initializing mods".clone_into(&mut loading_text2.lock().unwrap_or_else(PoisonError::into_inner));
        mods.init()?;

        anyhow::Ok((mods, blocks, walls, liquids, entities, items, networking))
    };
    // if the init fails, we clear the loading text so the error can be displayed
    let result = temp_fn()?;
    loading_text2.lock().unwrap_or_else(PoisonError::into_inner).clear();

    let mut mods = result.0;
    let mut blocks = result.1;
    let mut walls = result.2;
    let mut liquids = result.3;
    let entities = result.4;
    let mut items = result.5;
    let mut networking = result.6;

    let mut background = Background::new();
    let mut inventory = ClientInventory::new();
    let mut lights = ClientLights::new();
    let mut events = EventManager::new();
    let mut camera = Camera::new();
    let mut players = ClientPlayers::new(player_name);
    let mut block_selector = BlockSelector::new();
    let mut pause_menu = PauseMenu::new(graphics, settings.clone(), global_settings.clone());
    let mut debug_menu = DebugMenu::new();
    let mut framerate_measurer = FramerateMeasurer::new();
    let mut chat = ClientChat::new(graphics);
    let mut health = ClientHealth::new();
    let mut floating_text = FloatingTextManager::new();
    let mut respawn_screen = RespawnScreen::new();

    background.init()?;
    inventory.init(graphics);
    lights.init(&blocks.get_blocks(), settings)?;

    blocks.load_resources(&mods.mod_manager)?;
    walls.load_resources(&mods.mod_manager)?;
    liquids.load_resources(&mods.mod_manager)?;
    items.load_resources(&mods.mod_manager)?;
    camera.load_resources(graphics);
    players.load_resources(&mods.mod_manager)?;
    health.load_resources(&mods.mod_manager)?;

    pause_menu.init(graphics);
    debug_menu.init();
    chat.init();
    respawn_screen.init(graphics);

    // print the time it took to initialize
    println!("Game joined in {}ms", timer.elapsed().as_millis());

    'main_loop: while graphics.is_window_open() {
        framerate_measurer.update();

        let frame_timer = std::time::Instant::now();

        while let Some(event) = graphics.get_event() {
            events.push_event(events::Event::new(event));
        }

        graphics.block_key_states = chat.is_selected();

        networking.update(&mut events)?;
        mods.update()?;
        blocks.update(framerate_measurer.get_delta_time(), &mut events)?;
        walls.update(framerate_measurer.get_delta_time(), &mut events)?;

        if let Some(main_player) = players.get_main_player() {
            let player_pos = entities.get_entities().ecs.get::<&PositionComponent>(main_player)?.deref().clone();

            camera.set_position(player_pos.x(), player_pos.y());
        }

        while framerate_measurer.has_5ms_passed() {
            camera.update_ms(graphics);
            players.controls_enabled = !camera.is_detached();
            players.update(graphics, &mut entities.get_entities(), &mut networking, &blocks.get_blocks(), &liquids.get_liquids())?;
            entities.get_entities().update_entities_ms(&blocks.get_blocks(), &liquids.get_liquids(), &mut events)?;
        }

        respawn_screen.is_shown = players.get_main_player().is_none() && !players.is_waiting_for_player();

        items.update(&mut events);

        background.render(graphics, &camera);
        walls.render(graphics, &camera, &frame_timer)?;
        blocks.render(graphics, &camera /*&frame_timer*/)?;
        players.render(graphics, &mut entities.get_entities(), &camera);
        items.render(graphics, &camera, &mut entities.get_entities())?;
        // after everything that stands in it, so a player wading through water is behind
        // the surface rather than pasted on top of it. Before the floating damage text,
        // which has to stay readable.
        liquids.render(graphics, &camera)?;
        floating_text.render(graphics, &camera);
        lights.render(graphics, &camera, &blocks.get_blocks(), settings, &frame_timer)?;
        camera.render(graphics);
        block_selector.render(graphics, &mut networking, &camera)?;
        inventory.render(graphics, &items, &mut networking, &blocks.get_blocks())?;
        health.render(graphics);
        chat.render(graphics);
        respawn_screen.render(graphics);

        pause_menu.render(graphics);

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
            blocks.on_event(&event, &mut events, &mut networking)?;
            walls.on_event(&event)?;
            liquids.on_event(&event, &mut events)?;
            entities.on_event(&event, &mut events, &players, &mut networking)?;
            items.on_event(&event, &mut entities.get_entities(), &mut events)?;
            block_selector.on_event(graphics, &mut networking, &camera, &event, &mut events)?;
            players.on_event(&event, &mut entities.get_entities())?;
            lights.on_event(&event, &blocks.get_blocks())?;
            camera.on_event(&event);
            health.on_event(&event, graphics, &mut floating_text, &players, &entities.get_entities());
            if pause_menu.on_event(&event, graphics) {
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
