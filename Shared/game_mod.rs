use std::collections::HashMap;
use rlua::Lua;
use serde_derive::{Serialize, Deserialize};

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
     */
    pub fn init(&mut self) {
        self.lua.context(|lua| {
            lua.load(&self.data.lua_code).exec().unwrap();
        });
    }

    /**
    Loads mod from a byte array.
     */
    pub fn from_bytes(data: Vec<u8>) -> Self {
        let data = bincode::deserialize(&data).unwrap();
        Self::new(data)
    }

    /**
    Converts the game mod to a byte array.
     */
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self.data).unwrap()
    }
}