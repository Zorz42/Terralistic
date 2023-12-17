use crate::server::server_core::world_generator::biome::{Biome, Ore};
use crate::server::server_core::world_generator::WorldGenerator;
use crate::shared::blocks::BlockId;
use crate::shared::mod_manager::{get_mod_id, ModManager};
use crate::shared::walls::WallId;
use anyhow::Result;
use rlua::prelude::LuaUserData;
use rlua::UserDataMethods;
use std::sync::PoisonError;

// make Biome compatible with Lua
impl LuaUserData for Biome {
    // implement index and new_index metamethods to allow reading and writing to fields
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to set fields
        methods.add_meta_method_mut(rlua::MetaMethod::NewIndex, |_lua_ctx, this, (key, value): (String, rlua::Value)| {
            match value {
                rlua::Value::Integer(b) => match key.as_str() {
                    "min_width" => this.min_width = b as u32,
                    "max_width" => this.max_width = b as u32,
                    "min_terrain_height" => this.min_terrain_height = b as u32,
                    "max_terrain_height" => this.max_terrain_height = b as u32,
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of Biome")));
                    }
                },
                rlua::Value::UserData(b) => match key.as_str() {
                    "base_block" => match b.borrow::<BlockId>() {
                        Ok(b) => this.base_block = *b,
                        Err(_) => {
                            return Err(rlua::Error::RuntimeError("value is not a valid value for base_block".to_owned()));
                        }
                    },
                    "base_wall" => match b.borrow::<WallId>() {
                        Ok(b) => this.base_wall = *b,
                        Err(_) => {
                            return Err(rlua::Error::RuntimeError("value is not a valid value for base_wall".to_owned()));
                        }
                    },
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of Biome")));
                    }
                },
                rlua::Value::String(b) => match key.as_str() {
                    "generator_function" => {
                        this.generator_function = Some(b.to_str().unwrap_or("__undefined").to_owned());
                    }
                    _ => {
                        return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of Biome")));
                    }
                },
                _ => {
                    return Err(rlua::Error::RuntimeError(format!("{key} is not a valid field of Biome")));
                }
            }
            Ok(())
        });

        // add method to add an ore
        methods.add_method_mut("add_ore", |_, this, (block, start_noise, end_noise): (BlockId, f32, f32)| {
            this.ores.push(Ore { block, start_noise, end_noise });
            Ok(())
        });
    }
}

impl WorldGenerator {
    pub fn init(&mut self, mods: &mut ModManager) -> Result<()> {
        mods.add_global_function("new_biome", move |lua_ctx, ()| {
            let mod_id = get_mod_id(lua_ctx)?;
            Ok(Biome::new(mod_id))
        })?;

        let mut biomes = self.biomes.clone();
        mods.add_global_function("register_biome", move |_, biome: Biome| {
            biomes.lock().unwrap_or_else(PoisonError::into_inner).push(biome);
            Ok(biomes.lock().unwrap_or_else(PoisonError::into_inner).len() - 1)
        })?;

        // lua function connect_biomes(biome1, biome2, weight) takes two biome ids and a weight and connects them
        // the weight is how likely it is to go from biome1 to biome2 (and vice versa)
        biomes = self.biomes.clone();
        mods.add_global_function("connect_biomes", move |_, (biome1, biome2, weight): (i32, i32, i32)| {
            biomes
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_mut(biome1 as usize)
                .ok_or_else(|| rlua::Error::RuntimeError(format!("Biome {biome1} does not exist!")))?
                .adjacent_biomes
                .push((weight, biome2));
            biomes
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_mut(biome2 as usize)
                .ok_or_else(|| rlua::Error::RuntimeError(format!("Biome {biome2} does not exist!")))?
                .adjacent_biomes
                .push((weight, biome1));
            Ok(())
        })?;
        Ok(())
    }
}
