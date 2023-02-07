use anyhow::Result;
use rlua::prelude::LuaError;
use rlua::{Context, FromLuaMulti, Lua, ToLuaMulti};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

static MOD_ID_IDENT: &str = "__TERRALISTIC_MOD_ID";

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ModsWelcomePacket {
    pub mods: Vec<Vec<u8>>,
}

/// Returns the mod id of the current mod from its lua context.
/// This is used to identify which mod is calling a function.
/// # Errors
/// Returns an error if the mod id is not set.
pub fn get_mod_id(context: Context) -> Result<i32, LuaError> {
    let globals = context.globals();
    globals.get::<_, i32>(MOD_ID_IDENT)
}

/// Game mod contains lua code that can be used to modify the game.
/// It also contains resources that can be used by the lua code.
/// resources are a map of strings to byte arrays. The string key
/// is the path of the resource file, relative to the `game_mod`
/// but without the `game_mod`/ prefix and instead with a / separator
/// it has a : separator. The byte array is the contents of the file.
pub struct GameMod {
    lua_code: String,
    resources: HashMap<String, Vec<u8>>,
    lua: Lua,
    id: i32,
}

impl GameMod {
    #[must_use]
    pub fn new(lua_code: String, resources: HashMap<String, Vec<u8>>) -> Self {
        Self {
            lua_code,
            resources,
            lua: Lua::new(),
            id: -1,
        }
    }

    /// This function runs the lua code in the game mod.
    /// It loads the code and resources into the lua state.
    /// It then runs the code and the init function.
    /// # Errors
    /// Returns an error if the lua code fails to load or the init function fails.
    fn init(&mut self, id: i32) -> Result<()> {
        self.id = id;
        self.lua.context(|lua| {
            // load the game mod code
            lua.load(&self.lua_code).exec()?;
            let globals = lua.globals();
            // set the mod id
            globals.set(MOD_ID_IDENT, self.id)?;
            anyhow::Ok(())
        })?;

        // execute the init function
        self.call_function::<(), ()>("init", ())?;
        Ok(())
    }

    /// This function adds a global function to the game mod.
    /// It takes the name of the function and the closure as input.
    /// # Errors
    /// Returns an error if the function cannot be added.
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F) -> Result<()>
    where
        F: 'static + Send + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> ToLuaMulti<'lua>,
    {
        self.lua.context(|lua| {
            let globals = lua.globals();
            globals.set(name, lua.create_function(func)?)?;
            Ok(())
        })
    }

    /// This function calls a function in the game mod with args and returns the result.
    /// It takes the name of the function and the arguments as input.
    /// # Errors
    /// Returns an error if the function throws an error.
    pub fn call_function<A, R>(&mut self, name: &str, args: A) -> Result<R, LuaError>
    where
        A: for<'lua> ToLuaMulti<'lua>,
        R: for<'lua> FromLuaMulti<'lua>,
    {
        self.lua.context(|lua| {
            let globals = lua.globals();
            let func = globals.get::<_, rlua::Function>(name)?;
            func.call(args)
        })
    }

    /// This function updates the game mod.
    /// It runs the update function in the lua code.
    /// # Errors
    /// Returns an error if the update function throws an error.
    fn update(&mut self) -> Result<()> {
        Ok(self.call_function::<(), ()>("update", ())?)
    }

    /// This function stops the game mod.
    /// It runs the stop function in the lua code.
    /// # Errors
    /// Returns an error if the stop function throws an error.
    fn stop(&mut self) -> Result<()> {
        Ok(self.call_function::<(), ()>("stop", ())?)
    }

    /// This function gets the resource with the given path.
    /// It returns a byte array with the contents of the resource.
    fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        self.resources.get(path)
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct GameModData {
    lua_code: String,
    resources: HashMap<String, Vec<u8>>,
}

impl Serialize for GameMod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = GameModData {
            lua_code: self.lua_code.clone(),
            resources: self.resources.clone(),
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GameMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = GameModData::deserialize(deserializer)?;
        Ok(Self {
            lua_code: data.lua_code,
            resources: data.resources,
            lua: Lua::new(),
            id: -1,
        })
    }
}

/**
Mod manager is responsible for loading mods and managing them.
 */
pub struct ModManager {
    mods: Vec<GameMod>,
}

impl ModManager {
    /**
    Creates a new mod manager.
    */
    #[must_use]
    pub const fn new(mods: Vec<GameMod>) -> Self {
        Self { mods }
    }

    /// This function adds a lua function to the mod manager, which will be added to all the mods.
    /// # Errors
    /// Returns an error if the function cannot be added.
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F) -> Result<()>
    where
        F: 'static + Send + Clone + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> ToLuaMulti<'lua>,
    {
        for mod_ in &mut self.mods {
            mod_.add_global_function(&("terralistic_".to_owned() + name), func.clone())?;
        }
        Ok(())
    }

    /// This function initializes all the mods.
    /// # Errors
    /// Returns an error if the mods cannot be initialized.
    pub fn init(&mut self) -> Result<()> {
        for (id, mod_) in self.mods.iter_mut().enumerate() {
            mod_.init(id as i32)?;
        }
        Ok(())
    }

    /// This function updates all the mods.
    /// # Errors
    /// Returns an error if the mods cannot be updated.
    pub fn update(&mut self) -> Result<()> {
        for mod_ in &mut self.mods {
            mod_.update()?;
        }
        Ok(())
    }

    /// This function stops all the mods.
    /// # Errors
    /// Returns an error if the mods cannot be stopped.
    pub fn stop(&mut self) -> Result<()> {
        for mod_ in &mut self.mods {
            mod_.stop()?;
        }
        Ok(())
    }

    /// Get mutable mods iterator.
    pub fn mods_mut(&mut self) -> core::slice::IterMut<GameMod> {
        self.mods.iter_mut()
    }

    /// This function gets the resource with the given path.
    #[must_use]
    pub fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        for game_mod in self.mods.iter().rev() {
            if let Some(data) = game_mod.get_resource(path) {
                return Some(data);
            }
        }
        None
    }

    /// This function gets the mod with the given id.
    pub fn get_mod(&mut self, id: i32) -> Option<&mut GameMod> {
        self.mods.get_mut(id as usize)
    }
}
