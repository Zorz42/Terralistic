use std::sync::{Arc, Mutex, PoisonError};

use anyhow::Result;

use crate::libraries::scripting::ScriptHost;
use crate::shared::liquids::{LiquidId, LiquidType, Liquids};

pub fn init_liquids_mod_interface(mods: &mut ScriptHost, liquids: &Arc<Mutex<Liquids>>) -> Result<()> {
    let liquids2 = liquids.clone();
    mods.add_global_function("register_liquid_type", move |_lua, (name, flow_time, speed_multiplier): (String, i32, f32)| {
        let mut liquid_type = LiquidType::new();
        liquid_type.name = name;
        liquid_type.flow_time = flow_time;
        liquid_type.speed_multiplier = speed_multiplier;

        let result = liquids2.lock().unwrap_or_else(PoisonError::into_inner).register_new_liquid_type(liquid_type);
        Ok(result)
    })?;

    let liquids2 = liquids.clone();
    mods.add_global_function("get_liquid_id_by_name", move |_lua, name: String| {
        liquids2
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_liquid_id_by_name(&name)
            .map_err(|err| rlua::Error::RuntimeError(err.to_string()))
    })?;

    let liquids2 = liquids.clone();
    mods.add_global_function("get_liquid_level", move |_lua, (x, y): (i32, i32)| {
        liquids2
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_liquid_level(x, y)
            .map_err(|err| rlua::Error::RuntimeError(err.to_string()))
    })?;

    let liquids2 = liquids.clone();
    mods.add_global_function("get_liquid", move |_lua, (x, y): (i32, i32)| {
        liquids2
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get_liquid_id_at(x, y)
            .map_err(|err| rlua::Error::RuntimeError(err.to_string()))
    })?;

    Ok(())
}

// make LiquidId lua compatible
impl rlua::FromLua<'_> for LiquidId {
    fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
        match value {
            rlua::Value::UserData(ud) => Ok(*ud.borrow::<Self>()?),
            _ => unreachable!(),
        }
    }
}

impl rlua::UserData for LiquidId {
    // implement equals comparison for LiquidId
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: Self| Ok(this.id == other.id));
    }
}
