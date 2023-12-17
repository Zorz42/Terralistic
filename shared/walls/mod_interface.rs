use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{Wall, Walls};
use anyhow::Result;
use std::sync::{Arc, Mutex};

pub fn init_walls_mod_interface(mods: &mut ModManager, walls: &Arc<Mutex<Walls>>) -> Result<()> {
    let walls2 = walls.clone();
    mods.add_global_function("register_wall_type", move |_lua, (name, break_time): (String, Option<i32>)| {
        let mut wall_type = Wall::new();
        wall_type.name = name;
        wall_type.break_time = break_time;

        let result = Walls::register_new_wall_type(&mut walls2.lock().unwrap_or_else(std::sync::PoisonError::into_inner).wall_types, wall_type);
        Ok(result)
    })?;

    let walls2 = walls.clone();
    mods.add_global_function("get_wall_id_by_name", move |_lua, name: String| {
        let wall_types = &walls2.lock().unwrap_or_else(std::sync::PoisonError::into_inner).wall_types;
        let iter = wall_types.iter();
        for wall_type in iter {
            if wall_type.name == name {
                return Ok(wall_type.get_id());
            }
        }
        Err(rlua::Error::RuntimeError("Wall type not found".to_owned()))
    })?;
    Ok(())
}
