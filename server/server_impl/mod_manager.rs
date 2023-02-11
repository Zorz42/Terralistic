use super::networking::{NewConnectionEvent, ServerNetworking};
use crate::libraries::events::Event;
use crate::shared::mod_manager::{GameMod, ModManager, ModsWelcomePacket};
use crate::shared::packet::Packet;
use anyhow::Result;

/// server mod manager that manages all the mods for the server.
/// It is used to initialize, update and stop all the mods.
/// It uses the shared mod manager to do this.
/// It gets all the mods from the world
/// and always loads the base game mod.
pub struct ServerModManager {
    pub mod_manager: ModManager,
}

impl ServerModManager {
    /// Creates a new server mod manager.
    pub fn new(mods: Vec<GameMod>) -> Self {
        Self {
            mod_manager: ModManager::new(mods),
        }
    }

    /// This function initializes the server mod manager.
    /// It adds the base game mod to the shared mod manager and initializes it.
    pub fn init(&mut self) -> Result<()> {
        self.mod_manager
            .add_global_function("print", |_, text: String| {
                println!("[server mod] {text}");
                Ok(())
            })?;

        self.mod_manager.init()?;
        for game_mod in self.mod_manager.mods_mut() {
            game_mod.call_function::<(), ()>("init_server", ())?;
        }
        Ok(())
    }

    pub fn on_event(&mut self, event: &Event, networking: &mut ServerNetworking) -> Result<()> {
        if let Some(event) = event.downcast::<NewConnectionEvent>() {
            let mut mods = Vec::new();
            for game_mod in self.mod_manager.mods_mut() {
                mods.push(bincode::serialize(game_mod)?);
            }
            let welcome_packet = Packet::new(ModsWelcomePacket { mods })?;
            networking.send_packet(&welcome_packet, &event.conn);
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
