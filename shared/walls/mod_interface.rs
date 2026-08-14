use crate::shared::mod_manager::ModManager;
use crate::shared::walls::{Wall, WallId, Walls};
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
        wall_types.get_id_by_name(&name).map_err(|e| rlua::Error::RuntimeError(e.to_string()))
    })?;
    Ok(())
}

// make WallId lua compatible
impl rlua::FromLua<'_> for WallId {
    fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
        match value {
            rlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}

impl rlua::UserData for WallId {
    // implement equals comparison for BlockId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: Self| Ok(this.id == other.id));
    }
}
