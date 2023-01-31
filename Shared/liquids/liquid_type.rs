use rlua::{ToLua, UserDataMethods};


/**struct with information about a liquid type*/
pub struct LiquidType {
    //name of the liquid type
    pub name: String,
    //how fast the liquid flows
    pub flow_time: i32,
    //how fast the player in the liquid moves
    pub speed_multiplier: f64,
    //id of the liquid type
    pub(super) id: i32,
}

impl rlua::UserData for LiquidType {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(rlua::MetaMethod::Index, |lua_ctx, this, key: String| {
            match key.as_str() {
                "name" => Ok(this.name.clone().to_lua(lua_ctx).unwrap()),
                "flow_time" => Ok(this.flow_time.to_lua(lua_ctx).unwrap()),
                "speed_multiplier" => Ok(this.speed_multiplier.to_lua(lua_ctx).unwrap()),
                _ => Err(rlua::Error::RuntimeError(format!("{} is not a valid field of LiquidType", key))),
            }
        });
        methods.add_meta_method_mut(rlua::MetaMethod::NewIndex, |_lua_ctx, this, (key, value): (String, rlua::Value)| {
            match key.as_str() {
                "name" => {
                    match value {
                        rlua::Value::String(s) => this.name = s.to_str().unwrap().to_string(),
                        _ => return Err(rlua::Error::RuntimeError("value is not a valid value for name".to_string()))
                    }
                    Ok(())
                },
                "flow_time" => {
                    match value {
                        rlua::Value::Integer(i) => this.flow_time = i as i32,
                        _ => return Err(rlua::Error::RuntimeError("value is not a valid value for flow_time".to_string()))
                    }
                    Ok(())
                },
                "speed_multiplier" => {
                    match value {
                        rlua::Value::Number(n) => this.speed_multiplier = n,
                        _ => return Err(rlua::Error::RuntimeError("value is not a valid value for speed_multiplier".to_string()))
                    }
                    Ok(())
                },
                _ => Err(rlua::Error::RuntimeError(format!("{} is not a valid field of LiquidType", key))),
            }
        });
    }
}

impl LiquidType {
    pub fn new(name: String) -> LiquidType {
        LiquidType {
            name,
            flow_time: 1,
            speed_multiplier: 1.0,
            id: 0,
        }
    }
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

impl PartialEq for LiquidType {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}