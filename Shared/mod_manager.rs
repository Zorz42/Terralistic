use rlua::prelude::LuaError;
use rlua::{Context, FromLuaMulti, Lua, ToLuaMulti};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;

static MOD_ID_IDENT: &str = "__TERRALISTIC_MOD_ID";

#[derive(PartialEq)]
enum ModManagerState {
    LoadingMods,
    LoadingFunctions,
    Ready,
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ModsWelcomePacket {
    pub mods: Vec<Vec<u8>>,
}

pub fn get_mod_id(context: Context) -> Result<i32, LuaError> {
    let globals = context.globals();
    globals.get::<_, i32>(MOD_ID_IDENT)
}

/**
Game mod contains lua code that can be used to modify the game.
It also contains resources that can be used by the lua code.
Resources are a map of strings to byte arrays. The string key
is the path of the resource file, relative to the game_mod
but without the game_mod/ prefix and instead with a / separator
it has a : separator. The byte array is the contents of the file.
 */
pub struct GameMod {
    lua_code: String,
    resources: HashMap<String, Vec<u8>>,
    lua: Lua,
    id: i32,
}

impl GameMod {
    pub fn new(lua_code: String, resources: HashMap<String, Vec<u8>>) -> Self {
        Self {
            lua_code,
            resources,
            lua: Lua::new(),
            id: -1,
        }
    }

    /**
    This function runs the lua code in the game mod.
    It loads the code and resources into the lua state.
    It then runs the code and the init function.
     */
    fn init(&mut self, id: i32) {
        self.id = id;
        self.lua.context(|lua| {
            // load the game mod code
            lua.load(&self.lua_code).exec().unwrap();
            let globals = lua.globals();
            // set the mod id
            globals.set(MOD_ID_IDENT, self.id).unwrap();
        });

        // execute the init function
        self.call_function::<(), ()>("init", ()).unwrap();
    }

    /**
    This function adds a global function to the game mod.
    It takes the name of the function and the closure as input.
     */
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F)
    where
        F: 'static + Send + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'a> FromLuaMulti<'a>,
        R: for<'a> ToLuaMulti<'a>,
    {
        self.lua.context(|lua| {
            let globals = lua.globals();
            globals
                .set(name, lua.create_function(func).unwrap())
                .unwrap();
        });
    }

    /**
    This function calls a function in the game mod with args and returns the result.
    It takes the name of the function and the arguments as input.
     */
    pub fn call_function<A, R>(&mut self, name: &str, args: A) -> Result<R, LuaError>
    where
        A: for<'a> ToLuaMulti<'a>,
        R: for<'a> FromLuaMulti<'a>,
    {
        self.lua.context(|lua| {
            let globals = lua.globals();
            let func = globals.get::<_, rlua::Function>(name)?;
            func.call(args)
        })
    }

    /**
    This function updates the game mod.
    It runs the update function in the lua code.
     */
    fn update(&mut self) {
        self.call_function::<(), ()>("update", ()).unwrap();
    }

    /**
    This function stops the game mod.
    It runs the stop function in the lua code.
     */
    fn stop(&mut self) {
        self.call_function::<(), ()>("stop", ()).unwrap();
    }

    /**
    This function gets the resource with the given path.
    It returns a byte array with the contents of the resource.
     */
    fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        self.resources.get(path)
    }
}

#[derive(serde_derive::Serialize, serde_derive::Deserialize)]
struct GameModData {
    lua_code: String,
    resources: HashMap<String, Vec<u8>>,
}

// implement serialize and deserialize for game mod
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
    state: ModManagerState,
}

impl Default for ModManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ModManager {
    /**
    Creates a new mod manager.
    */
    pub fn new() -> Self {
        Self {
            mods: Vec::new(),
            state: ModManagerState::LoadingMods,
        }
    }

    /**
    This function adds a mod to the mod manager if it is loading mods.
    Else it throws an error.
     */
    pub fn add_mod(&mut self, mod_data: GameMod) {
        if self.state == ModManagerState::LoadingMods {
            self.mods.push(mod_data);
        } else {
            panic!("Cannot add mod after adding functions");
        }
    }

    /**
    This function adds a lua function to the mod manager if it is loading functions.
    Else it throws an error. It takes a closure as a parameter and then converts it
    to a lua function.
     */
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F)
    where
        F: 'static + Send + Clone + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'a> FromLuaMulti<'a>,
        R: for<'a> ToLuaMulti<'a>,
    {
        if self.state == ModManagerState::LoadingFunctions
            || self.state == ModManagerState::LoadingMods
        {
            self.state = ModManagerState::LoadingFunctions;
            for mod_ in &mut self.mods {
                mod_.add_global_function(&("terralistic_".to_string() + name), func.clone());
            }
        } else {
            panic!("Cannot add function after initializing");
        }
    }

    /**
    This function initializes all the mods if it is loading functions or loading mods.
    Else it throws an error.
     */
    pub fn init(&mut self) {
        if self.state == ModManagerState::LoadingMods
            || self.state == ModManagerState::LoadingFunctions
        {
            // iterate over all the mods and initialize them, id passed to the mod is the index of the mod in the vector
            for (id, mod_) in self.mods.iter_mut().enumerate() {
                mod_.init(id as i32);
            }
            self.state = ModManagerState::Ready;
        } else {
            panic!("Cannot init after init");
        }
    }

    /**
    This function updates all the mods if the mod manager is ready.
    Else it returns an error.
     */
    pub fn update(&mut self) {
        if self.state == ModManagerState::Ready {
            for mod_ in &mut self.mods {
                mod_.update();
            }
        } else {
            panic!("Cannot update before init");
        }
    }

    /**
    This function stops all the mods if the mod manager is ready.
    Else it returns an error.
     */
    pub fn stop(&mut self) {
        if self.state == ModManagerState::Ready {
            for mod_ in &mut self.mods {
                mod_.stop();
            }
        } else {
            panic!("Cannot stop before init");
        }
    }

    /**
    Get mutable mods iterator.
     */
    pub fn mods_mut(&mut self) -> std::slice::IterMut<GameMod> {
        self.mods.iter_mut()
    }

    /**
    This function gets the resource with the given path.
     */
    pub fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        for game_mod in self.mods.iter().rev() {
            if let Some(data) = game_mod.get_resource(path) {
                return Some(data);
            }
        }
        None
    }

    /**
    This function gets the mod with the given id.
     */
    pub fn get_mod(&mut self, id: i32) -> Option<&mut GameMod> {
        self.mods.get_mut(id as usize)
    }
}
