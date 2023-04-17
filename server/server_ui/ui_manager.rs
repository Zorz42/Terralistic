use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time;

use crate::libraries::graphics::Event;

use crate::libraries::graphics as gfx;
use crate::server::server_ui::player_list;
use crate::server::server_ui::server_info;
use crate::server::server_ui::{console, ServerState, UiMessageType};

pub const SCALE: f32 = 2.0;
pub const EDGE_SPACING: f32 = 4.0;

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
enum ModuleTreeNodeType {
    Nothing,
    Node(Box<ModuleTreeNode>),
    Module(String),
}
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
enum SplitType {
    Vertical,
    Horizontal,
}

//this saves the module's positions on the screen in a binary tree like structure
#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct ModuleTreeNode {
    orientation: SplitType,
    split_pos: f32,             //goes from 0.0 to 1.0
    first: ModuleTreeNodeType,  //Left or Top
    second: ModuleTreeNodeType, //Right or Bottom
}

pub struct UiManager {
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<UiMessageType>,
    server_message_sender: Sender<UiMessageType>,
    modules: Vec<Box<dyn ModuleTrait>>,
    save_path: PathBuf,
}

impl UiManager {
    #[must_use]
    pub fn new(
        graphics_context: gfx::GraphicsContext,
        event_receiver: Receiver<UiMessageType>,
        event_sender: Sender<UiMessageType>,
        path: PathBuf,
    ) -> Self {
        let mut temp = Self {
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

    pub fn run(&mut self, server_running: &Arc<AtomicBool>) {
        //give sender to the modules
        for module in &mut self.modules {
            module.set_sender(self.server_message_sender.clone());
        }

        //init the modules
        for module in &mut self.modules {
            module.init(&mut self.graphics_context);
        }

        //load the module tree
        let module_tree = self.read_module_tree();

        //this saves the window size
        let mut window_size = gfx::FloatSize(0.0, 0.0);

        gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0))
            .render(&self.graphics_context, None); //rect that makes rendering work

        let mut last_frame_time = time::Instant::now();

        loop {
            let mut server_state = ServerState::Nothing;

            while let Some(event) = self.graphics_context.renderer.get_event() {
                for module in &mut self.modules {
                    module.on_event(&event, &mut self.graphics_context);
                }
            }

            if !self.graphics_context.renderer.is_window_open() {
                server_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }
            self.graphics_context.renderer.handle_window_resize();

            //goes through the messages received from the server
            while let Ok(message) = self.server_message_receiver.try_recv() {
                if let UiMessageType::ServerState(state) = message {
                    server_state = state;
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
                    module
                        .get_container_mut()
                        .update(&self.graphics_context, None);
                }
            }

            let time = last_frame_time.elapsed().as_secs_f32();
            //updates the modules
            for module in &mut self.modules {
                module.update(time, &mut self.graphics_context);
            }
            last_frame_time = time::Instant::now();

            gfx::Rect::new(
                gfx::FloatPos(0.0, 0.0),
                self.graphics_context.renderer.get_window_size(),
            )
            .render(&self.graphics_context, gfx::DARK_GREY);

            //renders the modules
            for module in &mut self.modules {
                //background
                module
                    .get_container_mut()
                    .rect
                    .render(&self.graphics_context, gfx::GREY);

                module.render(&mut self.graphics_context);
            }

            self.graphics_context.renderer.update_window();

            if server_state == ServerState::Stopped {
                break;
            }
        }
    }

    fn tile_modules(&mut self, pos: gfx::FloatPos, size: gfx::FloatSize, node: &ModuleTreeNode) {
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
        match &node.first {
            ModuleTreeNodeType::Module(module) => {
                self.transform_module(module, first_pos, first_size);
            }
            ModuleTreeNodeType::Node(node) => {
                self.tile_modules(first_pos, first_size, node);
            }
            ModuleTreeNodeType::Nothing => {}
        }
        match &node.second {
            ModuleTreeNodeType::Module(module) => {
                self.transform_module(module, second_pos, second_size);
            }
            ModuleTreeNodeType::Node(node) => {
                self.tile_modules(second_pos, second_size, node);
            }
            ModuleTreeNodeType::Nothing => {}
        }
    }

    fn transform_module(&mut self, name: &str, pos: gfx::FloatPos, size: gfx::FloatSize) {
        let module = self.get_module(name);
        if let Some(module) = module {
            module.get_container_mut().rect.size =
                size - gfx::FloatSize(EDGE_SPACING * 2.0, EDGE_SPACING * 2.0);
            module.get_container_mut().rect.pos = pos + gfx::FloatPos(EDGE_SPACING, EDGE_SPACING);
        }
    }

    fn get_module(&mut self, name: &str) -> Option<&mut Box<dyn ModuleTrait>> {
        self.modules
            .iter_mut()
            .find(|module| module.get_name() == name)
    }

    fn read_module_tree(&self) -> ModuleTreeNode {
        //read the config file in server_data/ui_config.json
        //if the file doesn't exist or is not a valid format, use the default config
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

    fn write_default_module_tree(&self) -> ModuleTreeNode {
        let split = ModuleTreeNode {
            orientation: SplitType::Horizontal,
            split_pos: 0.1,
            first: ModuleTreeNodeType::Module("ServerInfo".to_owned()),
            second: ModuleTreeNodeType::Node(Box::from(ModuleTreeNode {
                orientation: SplitType::Vertical,
                split_pos: 0.5,
                first: ModuleTreeNodeType::Module("PlayerList".to_owned()),
                second: ModuleTreeNodeType::Module("Console".to_owned()),
            })),
        };
        let file = File::create(self.save_path.join("ui_config.json"));
        file.map_or_else(
            |_| {
                println!("Failed to create ui_config.json");
            },
            |mut file| {
                let json_str = serde_json::to_string_pretty(&split);
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
        split
    }
}

pub trait ModuleTrait {
    //initializes the module
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext);
    //updates the module
    fn update(&mut self, delta_time_seconds: f32, graphics_context: &mut gfx::GraphicsContext);
    //renders the module
    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext);
    //relay messages from the server to the module
    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    );
    //returns the mutable reference to the module's rect
    fn get_container_mut(&mut self) -> &mut gfx::Container;
    //returns the name of the module
    fn get_name(&self) -> &str;
    //gives the event sender to the module, so it can send data to the server
    fn set_sender(&mut self, _sender: Sender<UiMessageType>) {}
    //sends sdl2 events to the module
    fn on_event(&mut self, _event: &Event, _graphics_context: &mut gfx::GraphicsContext) {}
}
