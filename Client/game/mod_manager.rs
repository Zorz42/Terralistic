use shared::mod_manager::{GameMod, ModManager};

/**
Client mod manager that manages all the mods for the client.
It is used to initialize, update and stop all the mods.
It uses the shared mod manager to do this.
It also gets mods from the server and adds them to the shared mod manager
and always loads the BaseGame mod.
 */
pub struct ClientModManager {
    mod_manager: ModManager,
}

impl ClientModManager {
    /**
    Creates a new client mod manager.
     */
    pub fn new() -> Self {
        Self {
            mod_manager: ModManager::new(),
        }
    }

    /**
    This function initializes the client mod manager.
    It adds the BaseGame mod to the shared mod manager and initializes it.
     */
    pub fn init(&mut self) {
        let base_mod = GameMod::from_bytes(include_bytes!("../../BaseGame/BaseGame.mod").to_vec());
        self.mod_manager.add_mod(base_mod);
        self.mod_manager.init();
    }

    /**
    This function updates the client mod manager.
    It updates the shared mod manager.
     */
    pub fn update(&mut self) {
        self.mod_manager.update();
    }

    /**
    This function stops the client mod manager.
    It stops the shared mod manager.
     */
    pub fn stop(&mut self) {
        self.mod_manager.stop();
    }
}