use std::sync::{mpsc::{Receiver, Sender}};
use alloc::sync::Arc;
use core::fmt::Debug;
use core::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use crate::libraries::graphics as gfx;
use bincode::deserialize;
use crate::server::server_core::ServerState;

pub struct UiManager {
    graphics_context: gfx::GraphicsContext,
    server_message_receiver: Receiver<Vec<u8>>,

}

impl UiManager {
    pub fn new(graphics_context: gfx::GraphicsContext, event_receiver: Receiver<Vec<u8>>) -> Self{
        Self {
            graphics_context,
            server_message_receiver: event_receiver,
        }
    }

    pub fn run(&mut self, server_thread: JoinHandle<()>, server_running: &Arc<AtomicBool>) {
        loop {
            let mut server_state = ServerState::Nothing;
            self.graphics_context.renderer.get_event();//idk this somehow updates the window
            self.graphics_context.renderer.update_window();
            if !self.graphics_context.renderer.is_window_open() {
                server_running.store(false, core::sync::atomic::Ordering::Relaxed);
            }

            while let Ok(data) = self.server_message_receiver.try_recv() {
                let data = snap::raw::Decoder::new().decompress_vec(&data).unwrap_or_default();
                let state = deserialize::<ServerState>(&*data);
                if let Ok(state) = state {
                    println!("state: {state}");
                    server_state = state;
                } else {
                    println!("got a UI message that isn't a server state");
                }
            }

            if server_state == ServerState::Stopped {
                break;
            }
        }
        let _thread_result = server_thread.join();
    }
}