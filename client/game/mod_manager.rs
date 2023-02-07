use super::networking::WelcomePacketEvent;
use crate::libraries::events::Event;
use crate::shared::mod_manager::{GameMod, ModManager, ModsWelcomePacket};

/**
client mod manager that manages all the mods for the client.
It is used to initialize, update and stop all the mods.
It uses the shared mod manager to do this.
It also gets mods from the server and adds them to the shared mod manager
and always loads the `base_game` mod.
 */
pub struct ClientModManager {
    pub mod_manager: ModManager,
}

impl ClientModManager {
    /**
    Creates a new client mod manager.
     */
    pub fn new() -> Self {
        Self {
            mod_manager: ModManager::new(Vec::new()),
        }
    }

    /**
    This function initializes the client mod manager.
    It adds the base_game mod to the shared mod manager and initializes it.
     */
    pub fn init(&mut self) {
        self.mod_manager
            .add_global_function("print", |_, text: String| {
                println!("[client mod] {text}");
                Ok(())
            })
            .unwrap();

        self.mod_manager.init().unwrap();
    }

    /**
    This function updates the client mod manager.
    It updates the shared mod manager.
     */
    pub fn update(&mut self) {
        self.mod_manager.update().unwrap();
    }

    pub fn on_event(&mut self, event: &Event) {
        if let Some(event) = event.downcast::<WelcomePacketEvent>() {
            if let Some(packet) = event.packet.try_deserialize::<ModsWelcomePacket>() {
                let mut game_mods = Vec::new();
                for mod_data in packet.mods {
                    let game_mod = bincode::deserialize(&mod_data).unwrap();
                    game_mods.push(game_mod);
                }
                self.mod_manager = ModManager::new(game_mods);
            }
        }
    }

    /**
    This function stops the client mod manager.
    It stops the shared mod manager.
     */
    pub fn stop(&mut self) {
        self.mod_manager.stop().unwrap();
    }
}
