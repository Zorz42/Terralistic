use crate::libraries::graphics as gfx;
use crate::server::server_core::{ServerState, UiMessageType};
use alloc::sync::Arc;
use bincode::deserialize;
use core::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;

pub struct UiManager {
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<Vec<u8>>,
    _modules: Vec<Box<dyn ModuleTrait>>,
}

impl UiManager {
    #[must_use]
    pub fn new(graphics_context: gfx::GraphicsContext, event_receiver: Receiver<Vec<u8>>) -> Self {
        Self {
            graphics_context,
            server_message_receiver: event_receiver,
            _modules: Vec::new(),
        }
    }

    #[allow(clippy::match_wildcard_for_single_variants)]
    pub fn run(&mut self, server_running: &Arc<AtomicBool>) {
        loop {
            let mut server_state = ServerState::Nothing;
            self.graphics_context.renderer.get_event(); //idk this somehow updates the window
            self.graphics_context.renderer.update_window();
            if !self.graphics_context.renderer.is_window_open() {
                server_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }

            //goes through the messages received from the server
            while let Ok(data) = self.server_message_receiver.try_recv() {
                let data = snap::raw::Decoder::new()
                    .decompress_vec(&data)
                    .unwrap_or_default();
                let message = deserialize::<UiMessageType>(&data);
                if let Ok(UiMessageType::ServerState(state)) = message {
                    server_state = state;
                }
            }

            if server_state == ServerState::Stopped {
                break;
            }
        }
    }
}

trait ModuleTrait {
    //updates the module
    fn update(&mut self, delta_time: f32);
    //renders the module
    fn render(&mut self, graphics_context: &mut gfx::GraphicsContext);
}
