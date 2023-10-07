use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;

use crate::libraries::graphics as gfx;
use crate::server::server_core::Server;
use crate::server::server_ui::ui_module_manager::{
    ModuleManager, ModuleTreeNodeType, ModuleTreeSplit, SplitType,
};
use crate::server::server_ui::{console, empty_module, player_list, server_info};
use crate::server::server_ui::{ServerState, UiMessageType};

pub const SCALE: f32 = 2.0;
pub const EDGE_SPACING: f32 = 4.0;

/// `UiManager` is the main struct of the UI. It manages the UI modules and the UI window and communicates with the server.
pub struct UiManager {
    server: Server,
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<UiMessageType>,
    server_message_sender: Sender<UiMessageType>,
    modules: Vec<Box<dyn ModuleTrait>>,
    save_path: PathBuf,
    module_edit_mode: bool,
    module_manager: ModuleManager,
    window_size: gfx::FloatSize,
}

impl UiManager {
    /// Creates a new `UiManager`.
    #[must_use]
    pub fn new(
        server: Server,
        graphics_context: gfx::GraphicsContext,
        event_receiver: Receiver<UiMessageType>,
        event_sender: Sender<UiMessageType>,
        path: PathBuf,
    ) -> Self {
        let module_manager = ModuleManager::from_save_file(&path);
        let mut temp = Self {
            server,
            graphics_context,
            server_message_receiver: event_receiver,
            server_message_sender: event_sender,
            modules: Vec::new(),
            save_path: path,
            module_edit_mode: false,
            module_manager,
            window_size: gfx::FloatSize(0.0, 0.0),
        };
        temp.modules = vec![
            Box::new(server_info::ServerInfo::new(&mut temp.graphics_context)),
            Box::new(player_list::PlayerList::new(&mut temp.graphics_context)),
            Box::new(console::Console::new(&mut temp.graphics_context)),
        ];
        temp
    }

    /// runs the UI. This function should only be called once as it runs until the window is closed or the server is stopped (one of those 2 events also triggers the other one).
    pub fn run(
        &mut self,
        is_running: &Arc<AtomicBool>,
        status_text: &Mutex<String>,
        mods_serialized: Vec<Vec<u8>>,
        world_path: &Path,
    ) {
        let ms_timer = std::time::Instant::now();
        let mut ms_counter = 0;
        let mut s_counter = 0;
        let mut num_updates = 0;
        let mut ui_last_time = std::time::Instant::now();
        let mut server_last_time = std::time::Instant::now();

        self.init_modules();

        //init the server
        if let Err(e) = self.server.start(status_text, mods_serialized, world_path) {
            println!("Error starting server: {e}");
            return;
        }

        gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0))
            .render(&self.graphics_context, None); //rect that makes rendering work. It is useless but do not remove it or the rendering will not work. Blame the graphics library by Zorz42

        let mut ui_delta_time;

        'main: loop {
            ui_delta_time = ui_last_time.elapsed().as_secs_f32() * 1000.0;
            ui_last_time = std::time::Instant::now();
            let mut server_mspt = None;

            //update the server
            self.update_server(
                &mut num_updates,
                ms_timer,
                &mut server_last_time,
                &mut server_mspt,
                &mut ms_counter,
                &mut s_counter,
            );

            self.relay_graphics_events();

            //goes through the messages received from the server and sends them to the modules
            //also closes the window if the server is stopped
            while let Ok(message) = self.server_message_receiver.try_recv() {
                if message == UiMessageType::ServerState(ServerState::Stopped) {
                    break 'main;
                }
                for module in &mut self.modules {
                    module.on_server_message(&message, &mut self.graphics_context);
                }
            }

            if self.module_manager.changed {
                self.reset_modules();
            }

            //resize the modules if the window size has changed or the module tree has changed
            if self.window_size != self.graphics_context.renderer.get_window_size()
                || self.module_manager.changed
            {
                self.reposition_modules();
            }
            //updates the modules
            for module in &mut self.modules {
                module.update(ui_delta_time, &mut self.graphics_context);
            }

            //renders the background
            gfx::Rect::new(
                gfx::FloatPos(0.0, 0.0),
                self.graphics_context.renderer.get_window_size(),
            )
            .render(&self.graphics_context, gfx::DARK_GREY);

            if self.module_edit_mode {
                self.module_manager
                    .render_selection(&mut self.graphics_context);
            }

            self.render_modules();

            if self.module_edit_mode {
                self.module_manager
                    .render_overlay(&mut self.graphics_context);
            }

            //display the frame
            self.graphics_context.renderer.update_window();
            self.graphics_context.renderer.handle_window_resize(); //idk what this does

            //update the mspt
            let ui_mspt = ui_delta_time as f64 / 1000.0;

            //relays mspt changes to the module
            for module in &mut self.modules {
                module.on_server_message(
                    &UiMessageType::MsptUpdate((server_mspt, ui_mspt)),
                    &mut self.graphics_context,
                );
            }

            //close the window if the server is stopped
            if !is_running.load(std::sync::atomic::Ordering::Relaxed)
                || self.server.state == ServerState::Stopping
            {
                //state is there so outside events can stop it
                break;
            }
            //stops the server if the window is closed
            if !self.graphics_context.renderer.is_window_open() {
                is_running.store(false, std::sync::atomic::Ordering::Relaxed);
            }

            //sleep
            let sleep_time =
                1000.0 / 120.0 /*fps limit*/ - ui_last_time.elapsed().as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                sleep(std::time::Duration::from_secs_f32(sleep_time / 1000.0));
            }
        }

        if let Err(e) = self.server.stop(status_text, world_path) {
            println!("Error stopping server: {e}");
        }
    }

    /// Initializes the modules
    fn init_modules(&mut self) {
        for module in &mut self.modules {
            module.set_sender(self.server_message_sender.clone());
        }

        for module in &mut self.modules {
            module.init(&mut self.graphics_context);
        }
    }

    /// Updates the server when needed
    fn update_server(
        //TODO: rethink timings, this is too many variables
        &mut self,
        num_updates: &mut u64,
        ms_timer: std::time::Instant,
        server_last_time: &mut std::time::Instant,
        server_mspt: &mut Option<f64>,
        ms_counter: &mut i32,
        s_counter: &mut i32,
    ) {
        while *num_updates
            < ms_timer.elapsed().as_millis() as u64 * self.server.tps_limit as u64 / 1000
        {
            let server_delta_time = server_last_time.elapsed().as_secs_f32() * 1000.0;
            *server_last_time = std::time::Instant::now();

            //update the server by 1 tick
            let server_tick_start = ms_timer.elapsed().as_micros();
            if let Err(e) = self
                .server
                .update(server_delta_time, ms_timer, ms_counter, s_counter)
            {
                println!("Error running server: {e}");
                break;
            }
            *server_mspt =
                Some((ms_timer.elapsed().as_micros() - server_tick_start) as f64 / 1000.0);
            *num_updates += 1;
        }
    }

    fn relay_graphics_events(&mut self) {
        while let Some(event) = self.graphics_context.renderer.get_event() {
            if let gfx::Event::KeyPress(key, repeat) = event {
                if key == gfx::Key::F1 && !repeat {
                    self.module_edit_mode = !self.module_edit_mode;
                    self.module_manager
                        .save_to_file(&self.save_path, &self.server_message_sender);
                }
            }

            if self.module_edit_mode {
                self.module_manager.on_event(&event);
            } else {
                for module in &mut self.modules {
                    module.on_event(&event, &mut self.graphics_context);
                }
            }
        }
    }

    fn reposition_modules(&mut self) {
        self.module_manager.changed = false;
        self.window_size = self.graphics_context.renderer.get_window_size();

        //swap the module tree with nothing to avoid double borrow
        let temp_1 = self.module_manager.get_root_mut();
        let temp_2 = &mut ModuleTreeNodeType::Nothing;
        core::mem::swap(temp_1, temp_2);
        self.tile_modules(gfx::FloatPos(0.0, 0.0), self.window_size, temp_2);
        let temp_1 = self.module_manager.get_root_mut();
        core::mem::swap(temp_1, temp_2);

        //loop through the modules and update their containers
        for module in &mut self.modules {
            module
                .get_container_mut()
                .update(&self.graphics_context, None);
        }
    }

    fn render_modules(&mut self) {
        for module in &mut self.modules {
            if !*module.get_enabled_mut() {
                continue;
            }
            //background
            module
                .get_container_mut()
                .rect
                .render(&self.graphics_context, gfx::GREY);
            module.render(&mut self.graphics_context);
        }
    }

    fn reset_modules(&mut self) {
        //delete all empty modules and disable all other modules
        self.modules
            .retain(|module| !module.get_name().starts_with("Empty"));
        self.modules.iter_mut().for_each(|module| {
            *module.get_enabled_mut() = false;
        });
    }

    /// Moves and resizes the modules according to the config
    fn tile_modules(
        &mut self,
        pos: gfx::FloatPos,
        size: gfx::FloatSize,
        node: &ModuleTreeNodeType,
    ) {
        //recursively tile the node
        match &node {
            //node is a module. Transform it to its dedicated position and size
            ModuleTreeNodeType::Module(module_name) => {
                self.transform_module(module_name, pos, size);
            }
            //node is a split. Tile it
            ModuleTreeNodeType::Split(node) => {
                self.tile_module_split(pos, size, node);
            }
            //node is nothing. Do nothing
            ModuleTreeNodeType::Nothing => {}
        }
    }

    /// Tiles a split node
    fn tile_module_split(
        &mut self,
        pos: gfx::FloatPos,
        size: gfx::FloatSize,
        node: &ModuleTreeSplit,
    ) {
        //calculate pos and size for both sides of the split
        let (first_size, second_size, first_pos, second_pos) =
            if matches!(node.orientation, SplitType::Vertical) {
                (
                    gfx::FloatSize(size.0 * node.split_pos, size.1),
                    gfx::FloatSize(size.0 - size.0 * node.split_pos, size.1),
                    gfx::FloatPos(pos.0, pos.1),
                    gfx::FloatPos(pos.0 + size.0 * node.split_pos, pos.1),
                )
            } else {
                (
                    gfx::FloatSize(size.0, size.1 * node.split_pos),
                    gfx::FloatSize(size.0, size.1 - size.1 * node.split_pos),
                    gfx::FloatPos(pos.0, pos.1),
                    gfx::FloatPos(pos.0, pos.1 + size.1 * node.split_pos),
                )
            };

        //tile the modules
        self.tile_modules(first_pos, first_size, &node.first);
        self.tile_modules(second_pos, second_size, &node.second);
    }

    /// Resizes and moves the module with the given name to the given position and size
    fn transform_module(&mut self, module_name: &str, pos: gfx::FloatPos, size: gfx::FloatSize) {
        let module = if let Some(module) = self.get_module(module_name) {
            *module.get_enabled_mut() = true;
            module
        } else {
            let module = Box::new(empty_module::EmptyModule::new(
                &mut self.graphics_context,
                module_name.to_owned(),
            ));
            self.modules.push(module);
            #[allow(clippy::unwrap_used)]
            //unwrap is safe because we just pushed it to self.modules
            let module = self.modules.last_mut().unwrap();
            *module.get_enabled_mut() = true;
            module
        };

        module.get_container_mut().rect.size =
            size - gfx::FloatSize(EDGE_SPACING * 2.0, EDGE_SPACING * 2.0);
        module.get_container_mut().rect.pos = pos + gfx::FloatPos(EDGE_SPACING, EDGE_SPACING);
    }

    /// Returns a mutable reference to the module with the given name. Functions from the `ModuleTrait` can be called on the returned reference
    /// Returns None if no module with that name exists
    fn get_module(&mut self, name: &str) -> Option<&mut Box<dyn ModuleTrait>> {
        self.modules
            .iter_mut()
            .find(|module| module.get_name() == name)
    }

    ///returns the save path for config
    pub const fn get_save_path(&self) -> &PathBuf {
        &self.save_path
    }
}

pub trait ModuleTrait {
    /// initializes the module
    fn init(&mut self, _graphics_context: &mut gfx::GraphicsContext) {}
    /// updates the module
    fn update(&mut self, delta_time_seconds: f32, graphics_context: &mut gfx::GraphicsContext);
    /// renders the module
    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext);
    /// relay messages from the server to the module
    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    );
    /// returns the mutable reference to the module's rect
    fn get_container_mut(&mut self) -> &mut gfx::Container;
    /// returns the name of the module
    fn get_name(&self) -> &str;
    /// gives the event sender to the module, so it can send data to the server
    fn set_sender(&mut self, _sender: Sender<UiMessageType>) {}
    /// sends sdl2 events to the module
    fn on_event(&mut self, _event: &gfx::Event, _graphics_context: &mut gfx::GraphicsContext) {}
    /// returns a mut reference to whether the module is enabled. Modules will still be updated for consistency, but not rendered
    fn get_enabled_mut(&mut self) -> &mut bool;
}
