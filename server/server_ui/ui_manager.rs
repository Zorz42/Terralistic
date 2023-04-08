use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;

use bincode::deserialize;

use crate::libraries::graphics as gfx;
use crate::server::server_ui::{ServerState, UiMessageType};
#[allow(unused_imports)]//only for testing
use crate::server::server_ui::server_info;
use crate::server::server_ui::player_list;

pub const SCALE: f32 = 2.0;
pub const EDGE_SPACING: f32 = 4.0;

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
        temp.modules = vec![
            //Box::new(server_info::ServerInfo::new(&mut temp.graphics_context)),
            Box::new(player_list::PlayerList::new(&mut temp.graphics_context)),
        ];
        temp
    }

    pub fn run(&mut self, server_running: &Arc<AtomicBool>) {
        //init the modules
        for module in &mut self.modules {
            module.init(&mut self.graphics_context);
        }

        //this saves the window size
        let mut window_size = gfx::FloatSize(0.0, 0.0);

        gfx::RenderRect::new(gfx::FloatPos(0.0, 0.0), gfx::FloatSize(0.0, 0.0))
            .render(&self.graphics_context, None);//rect that makes rendering work

        loop {
            let mut server_state = ServerState::Nothing;

            while let Some(_event) = self.graphics_context.renderer.get_event() {}

            if !self.graphics_context.renderer.is_window_open() {
                server_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }
            self.graphics_context.renderer.handle_window_resize();

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

            //resize the modules if the window size has changed
            if window_size != self.graphics_context.renderer.get_window_size() {
                window_size = self.graphics_context.renderer.get_window_size();
                self.resize_modules();
            }

            //updates the modules
            for module in &mut self.modules {
                module.update(0.0, &mut self.graphics_context);
            }

            gfx::Rect::new(
                gfx::FloatPos(0.0, 0.0),
                self.graphics_context.renderer.get_window_size()
            ).render(&self.graphics_context, gfx::DARK_GREY);

            //renders the modules
            for module in &mut self.modules {
                //background
                module.get_container_mut().rect.render(&self.graphics_context, gfx::GREY);

                module.render(&mut self.graphics_context);
            }

            self.graphics_context.renderer.update_window();

            if server_state == ServerState::Stopped {
                break;
            }
        }
    }

    fn resize_modules(&mut self) {//will work like a tiling window manager (kinda) when finished
        let window_size = self.graphics_context.renderer.get_window_size();
        for module in &mut self.modules {
            module.get_container_mut().rect.size = window_size - gfx::FloatSize(EDGE_SPACING * 2.0, EDGE_SPACING * 2.0);
            module.get_container_mut().rect.pos = gfx::FloatPos(EDGE_SPACING, EDGE_SPACING);
            module.get_container_mut().update(&self.graphics_context, None);
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
    fn on_server_message(
        &mut self,
        message: &UiMessageType,
        graphics_context: &mut gfx::GraphicsContext,
    );
    //returns the mutable reference to the module's rect
    fn get_container_mut(&mut self) -> &mut gfx::Container;
}
