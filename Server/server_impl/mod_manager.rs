use shared::mod_manager::{GameMod, ModManager};

/**
Server mod manager that manages all the mods for the server.
It is used to initialize, update and stop all the mods.
It uses the shared mod manager to do this.
It gets all the mods from the world
and always loads the BaseGame mod.
 */
pub struct ServerModManager {
    pub mod_manager: ModManager,
}

impl ServerModManager {
    /**
    Creates a new server mod manager.
     */
    pub fn new() -> Self {
        Self {
            mod_manager: ModManager::new(),
        }
    }

    /**
    This function initializes the server mod manager.
    It adds the BaseGame mod to the shared mod manager and initializes it.
     */
    pub fn init(&mut self) {
        self.mod_manager.add_global_function("print", |_, text: String| {
            println!("[server mod] {}", text);
            Ok(())
        });

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