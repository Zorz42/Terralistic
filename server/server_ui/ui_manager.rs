use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::thread::sleep;

use crate::libraries::graphics as gfx;
use crate::server::server_core::Server;
use crate::server::server_ui::player_list;
use crate::server::server_ui::server_info;
use crate::server::server_ui::{console, ServerState, UiMessageType};

pub const SCALE: f32 = 2.0;
pub const EDGE_SPACING: f32 = 4.0;

/// This enum indicates the type of the `ModuleTree` Node.
/// `Nothing` means that the node and its window area are empty.
/// `Split` means that the node's window area splits into 2 more nodes.
/// `Module` means that the node is a module which takes up the node's window area.
/// Ideally Nothing should never be used as that means the upper node splits into a module ans nothing, when it itself should just be a module. Nothing may be used for editing the tree in the future
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
enum ModuleTreeNodeType {
    Nothing,
    Split(Box<ModuleTreeSplit>),
    Module(String),
}

/// This enum indicates the type of split. `Vertical` splits the window area of that node into 2 areas, one on the left and one on the right. `Horizontal` splits the window area of that node into 2 areas, one on the top and one on the bottom.
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
enum SplitType {
    Vertical,
    Horizontal,
}

/// This struct is used to save the module's positions on the screen in a binary tree like structure.
/// The `orientation` indicates the type of split.
/// The `split_pos` indicates the position of the split, with the number from 0 to 1 indicating what part of the area is assigned to the left or top subpart, and the rest is allocated to the bottom part.
/// The `first` and `second` indicate the first and second nodes of the split. The first node is the left or top node, and the second node is the right or bottom node, depending on the orientation.
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct ModuleTreeSplit {
    orientation: SplitType,
    split_pos: f32,
    first: ModuleTreeNodeType,
    second: ModuleTreeNodeType,
}

/// `UiManager` is the main struct of the UI. It manages the UI modules and the UI window and communicates with the server.
pub struct UiManager {
    server: Server,
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<UiMessageType>,
    server_message_sender: Sender<UiMessageType>,
    modules: Vec<Box<dyn ModuleTrait>>,
    save_path: PathBuf,
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
        let mut temp = Self {
            server,
            graphics_context,
            server_message_receiver: event_receiver,
            server_message_sender: event_sender,
            modules: Vec::new(),
            save_path: path,
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
        let mut num_updates = 0;
        let mut last_time = std::time::Instant::now();

        //give sender to the modules
        for module in &mut self.modules {
            module.set_sender(self.server_message_sender.clone());
        }

        //init the modules
        for module in &mut self.modules {
            module.init(&mut self.graphics_context);
        }

        //init the server
        let res = self.server.start(status_text, mods_serialized, world_path);
        if let Err(e) = res {
            println!("Error starting server: {e}");
            return;
        }

        //load the module tree
        let module_tree = self.read_module_tree();

        //this saves the window size. It is initialized to 0,0 so that all modules are resized on the first frame
        let mut window_size = gfx::FloatSize(0.0, 0.0);

        gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0)).render(&self.graphics_context, None); //rect that makes rendering work. It is useless but do not remove it or the rendering will not work. Blame the graphics library by Zorz42

        'main: loop {
            let delta_time = last_time.elapsed().as_secs_f32() * 1000.0;
            last_time = std::time::Instant::now();
            let mut server_mspt = None;

            if num_updates < ms_timer.elapsed().as_millis() as u64 * self.server.tps_limit as u64 / 1000 {
                //update the server by 1 tick
                let server_tick_start = ms_timer.elapsed().as_micros();
                if let Err(e) = self.server.update(delta_time, ms_timer, &mut ms_counter) {
                    println!("Error running server: {e}");
                    break;
                }
                server_mspt = Some((ms_timer.elapsed().as_micros() - server_tick_start) as f64 / 1000.0);
                num_updates += 1;
            }

            //relays graphics events to the modules
            while let Some(event) = self.graphics_context.renderer.get_event() {
                for module in &mut self.modules {
                    module.on_event(&event, &mut self.graphics_context);
                }
            }

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

            //resize the modules if the window size has changed
            if window_size != self.graphics_context.renderer.get_window_size() {
                window_size = self.graphics_context.renderer.get_window_size();
                self.tile_modules(gfx::FloatPos(0.0, 0.0), window_size, &module_tree);
                //loop through the modules and update their containers
                for module in &mut self.modules {
                    module.get_container_mut().update(&self.graphics_context, None);
                }
            }
            //updates the modules
            for module in &mut self.modules {
                module.update(delta_time, &mut self.graphics_context);
            }

            //renders the background
            gfx::Rect::new(
                gfx::FloatPos(0.0, 0.0),
                self.graphics_context.renderer.get_window_size(),
            ).render(&self.graphics_context, gfx::DARK_GREY);

            //renders the modules
            for module in &mut self.modules {
                //background
                module.get_container_mut().rect.render(&self.graphics_context, gfx::GREY);

                module.render(&mut self.graphics_context);
            }

            //display the frame
            self.graphics_context.renderer.update_window();
            self.graphics_context.renderer.handle_window_resize(); //idk what this does

            //update the mspt
            let ui_mspt = last_time.elapsed().as_micros() as f64 / 1000.0;

            for module in &mut self.modules {
                if module.get_name() == "ServerInfo" {
                    module.on_server_message(
                        &UiMessageType::MsptUpdate((server_mspt, ui_mspt)),
                        &mut self.graphics_context,
                    );
                }
            }

            //close the window if the server is stopped
            if !is_running.load(core::sync::atomic::Ordering::Relaxed) || self.server.state == ServerState::Stopping {
                //state is there so outside events can stop it
                break;
            }
            //stops the server if the window is closed
            if !self.graphics_context.renderer.is_window_open() {
                is_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }

            //sleep
            let sleep_time = 1000.0 / 120.0 /*fps limit*/ - last_time.elapsed().as_secs_f32() * 1000.0;
            if sleep_time > 0.0 {
                sleep(core::time::Duration::from_secs_f32(sleep_time / 1000.0));
            }
        }

        if let Err(e) = self.server.stop(status_text, world_path) {
            println!("Error stopping server: {e}");
        }
    }

    /// Reads the module tree from the save file.
    fn tile_modules(&mut self, pos: gfx::FloatPos, size: gfx::FloatSize, node: &ModuleTreeSplit) {
        //calculate pos and size for both sides of the split
        let (first_size, second_size, first_pos, second_pos) = if matches!(node.orientation, SplitType::Vertical) {
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

        //recursively tile the nodes
        match &node.first {
            //first node is a module. Transform it to it's dedicated position and size
            ModuleTreeNodeType::Module(module) => {
                self.transform_module(module, first_pos, first_size);
            }
            //first node is a split. Tile it
            ModuleTreeNodeType::Split(node) => {
                self.tile_modules(first_pos, first_size, node);
            }
            //first node is nothing. Do nothing
            ModuleTreeNodeType::Nothing => {}
        }
        match &node.second {
            //second node is a module. Transform it to it's dedicated position and size
            ModuleTreeNodeType::Module(module) => {
                self.transform_module(module, second_pos, second_size);
            }
            //second node is a split. Tile it
            ModuleTreeNodeType::Split(node) => {
                self.tile_modules(second_pos, second_size, node);
            }
            //second node is nothing. Do nothing
            ModuleTreeNodeType::Nothing => {}
        }
    }

    /// Resizes and moves the module with the given name to the given position and size
    fn transform_module(&mut self, name: &str, pos: gfx::FloatPos, size: gfx::FloatSize) {
        if let Some(module) = self.get_module(name) {
            module.get_container_mut().rect.size = size - gfx::FloatSize(EDGE_SPACING * 2.0, EDGE_SPACING * 2.0);
            module.get_container_mut().rect.pos = pos + gfx::FloatPos(EDGE_SPACING, EDGE_SPACING);
        }
    }

    /// Returns a mutable reference to the module with the given name. Functions from the `ModuleTrait` can be called on the returned reference
    /// Returns None if no module with that name exists
    fn get_module(&mut self, name: &str) -> Option<&mut Box<dyn ModuleTrait>> {
        self.modules.iter_mut().find(|module| module.get_name() == name)
    }

    /// Reads the module tree from the save file in `server_data/ui_config.json`. if the file doesn't exist or is not a valid format, use the default config
    /// NOTE: this is currently disabled because the ui config is not final. The default config is used instead
    fn read_module_tree(&self) -> ModuleTreeSplit {
        self.write_default_module_tree()

        /*let config_file = self.save_path.join("ui_config.json");//TODO: uncomment after the final ui config is decided
        if config_file.exists() {
            let file = File::open(config_file);
            file.map_or_else(
                |_| self.write_default_module_tree(),
                |file| {
                    let reader = BufReader::new(file);
                    let res = serde_json::from_reader(reader);
                    res.map_or_else(|_| {
                        println!("Failed to parse ui_config.json");
                        self.write_default_module_tree()
                    }, |res| res)
                })
        } else {
            self.write_default_module_tree()
        }*/
    }

    /// Creates the default module tree and saves it to the save file in `server_data/ui_config.json`, then returns it
    fn write_default_module_tree(&self) -> ModuleTreeSplit {
        let split = ModuleTreeSplit {
            orientation: SplitType::Horizontal,
            split_pos: 0.1,
            first: ModuleTreeNodeType::Module("ServerInfo".to_owned()),
            second: ModuleTreeNodeType::Split(Box::from(ModuleTreeSplit {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: ModuleTreeNodeType::Module("PlayerList".to_owned()),
                second: ModuleTreeNodeType::Module("Console".to_owned()),
            })),
        };
        self.save_file_tree(&split);
        split
    }

    /// Saves the module tree to the save file in `server_data/ui_config.json`
    fn save_file_tree(&self, tree: &ModuleTreeSplit) {
        let file = File::create(self.save_path.join("ui_config.json"));
        file.map_or_else(
            |_| {
                println!("Failed to create ui_config.json");
            },
            |mut file| {
                let json_str = serde_json::to_string_pretty(&tree);
                json_str.map_or_else(
                    |_| {
                        println!("Failed to serialize ui_config.json");
                    },
                    |json_str| {
                        let res = file.write(json_str.as_bytes());
                        if let Err(err) = res {
                            println!("Failed to write ui_config.json: {err}");
                        }
                    },
                );
            },
        );
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
}
