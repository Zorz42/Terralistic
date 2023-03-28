use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;

use crate::client::menus::{run_choice_menu, MenuBack};
use bincode::deserialize;

use crate::libraries::graphics as gfx;
use crate::server::server_core::{ServerState, UiMessageType};
use crate::server::server_ui::server_info;

pub struct UiManager {
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<Vec<u8>>,
    modules: Vec<Box<dyn ModuleTrait>>,
}

impl UiManager {
    #[must_use]
    pub fn new(graphics_context: gfx::GraphicsContext, event_receiver: Receiver<Vec<u8>>) -> Self {
        let mut temp = Self {
            graphics_context,
            server_message_receiver: event_receiver,
            modules: Vec::new(),
        };
        temp.modules = vec![Box::new(server_info::ServerInfo::new(
            &mut temp.graphics_context,
        ))];
        temp
    }

    pub fn run(&mut self, server_running: &Arc<AtomicBool>) {
        //init the modules
        for module in &mut self.modules {
            module.init(&mut self.graphics_context);
        }

        let mut menu_back = MenuBack::new(&mut self.graphics_context);
        run_choice_menu(
            "deez nuts",
            &mut self.graphics_context,
            &mut menu_back,
            None,
            None,
        );

        loop {
            let mut server_state = ServerState::Nothing;

            while let Some(_event) = self.graphics_context.renderer.get_event() {}

            if !self.graphics_context.renderer.is_window_open() {
                server_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }

            //goes through the messages received from the server
            while let Ok(data) = self.server_message_receiver.try_recv() {
                let data = snap::raw::Decoder::new()
                    .decompress_vec(&data)
                    .unwrap_or_default();
                let message = deserialize::<UiMessageType>(&data);
                if let Ok(message) = message {
                    if let UiMessageType::ServerState(state) = message {
                        server_state = state;
                    }
                    for module in &mut self.modules {
                        module.on_server_message(&message, &mut self.graphics_context);
                    }
                }
            }

            //updates the modules
            for module in &mut self.modules {
                module.update(0.0, &mut self.graphics_context);
            }

            //renders the modules
            for module in &mut self.modules {
                module.render(&mut self.graphics_context);
            }

            self.graphics_context.renderer.update_window();

            if server_state == ServerState::Stopped {
                break;
            }
        }
    }
}

pub trait ModuleTrait {
    //initializes the module
    fn init(&mut self, graphics_context: &mut gfx::GraphicsContext);
    //updates the module
    fn update(&mut self, delta_time: f32, graphics_context: &mut gfx::GraphicsContext);
    //renders the module
    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext);
    //relay messages from the server to the module
    fn on_server_message(&mut self, message: &UiMessageType, graphics_context: &mut gfx::GraphicsContext);
}
