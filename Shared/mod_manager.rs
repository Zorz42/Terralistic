use std::collections::HashMap;
use rlua::{Context, FromLuaMulti, Lua, ToLuaMulti};
use rlua::prelude::LuaError;
use serde_derive::{Serialize, Deserialize};

#[derive(PartialEq)]
enum ModManagerState {
    LoadingMods,
    LoadingFunctions,
    Ready,
}

/**
Game mod contains lua code that can be used to modify the game.
It also contains resources that can be used by the lua code.
Resources are a map of strings to byte arrays. The string key
is the path of the resource file, relative to the game_mod
but without the game_mod/ prefix and instead with a / separator
it has a : separator. The byte array is the contents of the file.
 */
#[derive(Serialize, Deserialize)]
pub struct GameModData {
    lua_code: String,
    resources: HashMap<String, Vec<u8>>,
}

impl GameModData {
    pub fn new(lua_code: String, resources: HashMap<String, Vec<u8>>) -> Self {
        Self {
            lua_code,
            resources,
        }
    }
}

pub struct GameMod {
    data: GameModData,
    lua: Lua,
}

impl GameMod {
    pub fn new(data: GameModData) -> Self {
        Self {
            data,
            lua: Lua::new(),
        }
    }

    /**
    This function runs the lua code in the game mod.
    It loads the code and resources into the lua state.
    It then runs the code and the init function.
     */ fn init(&mut self) {
        self.lua.context(|lua| {
            // load the game mod code
            lua.load(&self.data.lua_code).exec().unwrap();
        });

        // execute the init function
        self.call_function::<(), ()>("init", ()).unwrap();
    }

    /**
    This function adds a global function to the game mod.
    It takes the name of the function and the closure as input.
     */
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F)
                                        where F: 'static + Send + Fn(Context, A) -> Result<R, LuaError>,
                                              A: for<'a> FromLuaMulti<'a>,
                                              R: for<'a> ToLuaMulti<'a>, {
        self.lua.context(|lua| {
            let globals = lua.globals();
            globals.set(name, lua.create_function(func).unwrap()).unwrap();
        });
    }

    /**
    This function calls a function in the game mod with args and returns the result.
    It takes the name of the function and the arguments as input.
     */
    pub fn call_function<A, R>(&mut self, name: &str, args: A) -> Result<R, LuaError>
                               where A: for<'a> ToLuaMulti<'a>,
                                     R: for<'a> FromLuaMulti<'a> {
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
    Loads mod from a byte array. It is serialized using bincode and
    compressed with snap.
     */
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let data = snap::raw::Decoder::new().decompress_vec(&data).unwrap();
        GameMod::new(bincode::deserialize(&data).unwrap())
    }

    /**
    Converts the game mod to a byte array. It is serialized using bincode and
    compressed with snap.
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        let data = bincode::serialize(&self.data).unwrap();
        snap::raw::Encoder::new().compress_vec(&data).unwrap()
    }

    /**
    This function gets the resource with the given path.
    It returns a byte array with the contents of the resource.
     */
    fn get_resource(&self, path: String) -> Option<&Vec<u8>> {
        self.data.resources.get(&path)
    }
}

/**
Mod manager is responsible for loading mods and managing them.
 */
pub struct ModManager {
    mods: Vec<GameMod>,
    state: ModManagerState,
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
        if self.state == ModManagerState::LoadingFunctions || self.state == ModManagerState::LoadingMods {
            self.state = ModManagerState::LoadingFunctions;
            for mod_ in &mut self.mods {
                mod_.add_global_function(&*("terralistic_".to_string() + name), func.clone());
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
        if self.state == ModManagerState::LoadingMods || self.state == ModManagerState::LoadingFunctions {
            for mod_ in &mut self.mods {
                mod_.init();
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
    pub fn get_resource(&self, path: String) -> Option<&Vec<u8>> {
        for game_mod in self.mods.iter().rev() {
            if let Some(data) = game_mod.get_resource(path.clone()) {
                return Some(data);
            }
        }
        None
    }
}