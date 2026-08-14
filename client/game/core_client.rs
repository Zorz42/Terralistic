use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
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
use crate::client::game::liquids::ClientLiquids;
use crate::client::game::pause_menu::PauseMenu;
use crate::client::game::players::ClientPlayers;
use crate::client::game::respawn_screen::RespawnScreen;
use crate::client::global_settings::GlobalSettings;
use crate::client::menus::{BackgroundRect, LoadingScreen, MenuBack, MENU_WIDTH};
use crate::client::settings::Settings;
use crate::libraries::events;
use crate::libraries::events::EventManager;
use crate::libraries::graphics as gfx;
use crate::shared::entities::PositionComponent;
use gfx::BaseUiElement;

use super::background::Background;
use super::block_selector::BlockSelector;
use super::blocks::ClientBlocks;
use super::camera::Camera;
use super::mod_manager::ClientModManager;
use super::networking::ClientNetworking;
use super::walls::ClientWalls;

/// The loading screen the client draws for itself while it joins a world.
///
/// Everything that happens before the main loop below - connecting, waiting for the world to
/// arrive, and building the client's copy of it - runs inside a single call from the menu loop,
/// so that loop is not drawing while any of it happens. Left to itself the window stopped
/// repainting the moment the server's own loading screen closed and stayed frozen for the whole
/// join, which on a freshly generated world is seconds of a window the system reports as not
/// responding - indistinguishable from a game that has hung. So every phase names itself and
/// draws a frame, which also keeps the window answering the system and lets the close button
/// work while a world is still loading.
struct JoinScreen {
    back: MenuBack,
    screen: LoadingScreen,
    /// What the screen says. `LoadingScreen` reads its text through a shared handle because
    /// the server writes its own from another thread; here it is only ever this thread.
    text: Arc<Mutex<String>>,
}

impl JoinScreen {
    fn new(graphics: &gfx::GraphicsContext) -> Self {
        let text = Arc::new(Mutex::new(String::new()));
        let mut back = MenuBack::new(graphics);
        // the same panel the menu the player just came from uses, so the join looks like a
        // continuation of it rather than a different screen
        back.set_back_rect_width(MENU_WIDTH, true);

        Self {
            back,
            screen: LoadingScreen::new(text.clone()),
            text,
        }
    }

    /// Names the phase that is about to run and draws one frame of it.
    fn frame(&mut self, graphics: &mut gfx::GraphicsContext, status: &str) {
        status.clone_into(&mut self.text.lock().unwrap_or_else(PoisonError::into_inner));

        // The events are drained rather than handled: nothing here is interactive. But a
        // window whose queue is never read is a window the system thinks has stopped
        // responding, and the pump inside `update_window` is what notices a close request.
        while graphics.get_event().is_some() {}

        let container = gfx::Container::default(graphics);
        self.back.update(graphics, &container);
        self.screen.update(graphics, &container);
        self.back.render(graphics, &container);
        self.screen.render(graphics, &container);
        graphics.update_window();
    }
}

/// Joins a server and plays the game, until the player leaves or the window closes.
///
/// `server_alive` is how a singleplayer client knows its own server is still there - the flag
/// `PrivateWorld` runs the private server with, cleared however that server's thread ends.
/// Without it the client can only wait, and there is no welcome coming from a port whose server
/// has died or that something else is holding. Multiplayer passes `None`: a remote server's
/// health is not this process's to know, and the connection itself reports what it can.
#[allow(clippy::too_many_lines)]
pub fn run_game(
    graphics: &mut gfx::GraphicsContext,
    server_port: u16,
    server_address: String,
    player_name: &str,
    settings: &Rc<RefCell<Settings>>,
    global_settings: &Rc<RefCell<GlobalSettings>>,
    server_alive: Option<&AtomicBool>,
) -> Result<()> {
    // load base game mod
    let mut pre_events = EventManager::new();
    let mut join_screen = JoinScreen::new(graphics);
    let mut networking = ClientNetworking::new(server_port, server_address);
    networking.init(player_name.to_owned())?;
    while networking.is_welcoming() {
        // a frame is also the wait: `update_window` sleeps out the rest of the frame's share
        // of the clock, which is what the 1ms sleep this replaced was for
        join_screen.frame(graphics, "Joining the world");
        networking.check_thread_for_errors()?;

        if server_alive.is_some_and(|alive| !alive.load(Ordering::Relaxed)) {
            networking.stop()?;
            bail!("the world's server stopped before it could let this client in");
        }

        // Closing the window during a join used to be ignored until the whole world had
        // finished loading, because nothing here looked at the window at all.
        if !graphics.is_window_open() {
            networking.stop()?;
            return Ok(());
        }
    }

    networking.update(&mut pre_events)?;
    networking.start_receiving();

    let timer = std::time::Instant::now();

    join_screen.frame(graphics, "Loading mods");
    let mut mods = ClientModManager::new();
    let mut blocks = ClientBlocks::new();
    let mut walls = ClientWalls::new(&mut blocks.get_blocks());
    let mut liquids = ClientLiquids::new();
    let entities = ClientEntities::new();
    let mut items = ClientItems::new();

    // the welcome packets, which carry the mods and the whole world
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

    join_screen.frame(graphics, "Initializing mods");
    mods.init()?;

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

    // the light grid is as big as the world, so this is the last of the phases worth naming
    join_screen.frame(graphics, "Lighting the world");
    background.init()?;
    inventory.init(graphics);
    lights.init(&blocks.get_blocks(), settings)?;

    join_screen.frame(graphics, "Loading resources");
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
