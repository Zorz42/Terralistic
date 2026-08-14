use anyhow::Result;

use crate::libraries::events::Event;
use crate::libraries::scripting::{ScriptHost, ScriptModule};
use crate::server::server_core::networking::SendTarget;
use crate::server::server_core::print_to_console;
use crate::shared::packet::ModsWelcomePacket;
use crate::shared::packet::Packet;
use crate::shared::MOD_FUNCTION_PREFIX;

use super::networking::{NewConnectionEvent, ServerNetworking};
use crate::libraries::serialization;

/// server mod manager that manages all the mods for the server.
/// It is used to initialize, update and stop all the mods.
/// It uses the shared mod manager to do this.
/// It gets all the mods from the world
/// and always loads the base game mod.
pub struct ServerModManager {
    pub mod_manager: ScriptHost,
}

impl ServerModManager {
    /// Creates a new server mod manager.
    pub const fn new(mods: Vec<ScriptModule>) -> Self {
        Self {
            mod_manager: ScriptHost::new(mods, MOD_FUNCTION_PREFIX),
        }
    }

    /// This function initializes the server mod manager.
    /// It adds the base game mod to the shared mod manager and initializes it.
    pub fn init(&mut self) -> Result<()> {
        self.mod_manager.add_global_function("print", |_, text: String| {
            print_to_console(&format!("[server mod] {text}"), 0);
            //println!("[server mod] {text}");
            Ok(())
        })?;

        self.mod_manager.init()?;
        for game_mod in self.mod_manager.modules_iter_mut() {
            game_mod.call_function::<(), ()>("init_server", ())?;
        }
        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let mut mods = Vec::new();
            for game_mod in self.mod_manager.modules_iter_mut() {
                mods.push(serialization::serialize(game_mod)?);
            }
            let welcome_packet = Packet::new(ModsWelcomePacket { mods })?;
            networking.send_packet(&welcome_packet, SendTarget::Connection(event.conn.clone()))?;
        }
        Ok(())
    }

    /// This function updates the client mod manager.
    /// It updates the shared mod manager.
    pub fn update(&mut self) -> Result<()> {
        self.mod_manager.update()
    }

    /// This function stops the client mod manager.
    /// It stops the shared mod manager.
    pub fn stop(&mut self) -> Result<()> {
        self.mod_manager.stop()
    }
}
