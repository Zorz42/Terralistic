use crate::shared::blocks::Block;
use rlua::UserDataMethods;

/**
make `BlockType` Lua compatible, implement getter and setter for every field except id
 */
impl rlua::UserData for Block {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // add meta method to set fields, id and image are not accessible
        methods.add_meta_method_mut(
            rlua::MetaMethod::NewIndex,
            |_lua_ctx, this, (key, value): (String, rlua::Value)| match value {
                rlua::Value::Integer(value) => {
                    match key.as_str() {
                        "required_tool_power" => this.required_tool_power = value as i32,
                        "break_time" => this.break_time = value as i32,
                        "light_emission_r" => this.light_emission_r = value as u8,
                        "light_emission_g" => this.light_emission_g = value as u8,
                        "light_emission_b" => this.light_emission_b = value as u8,
                        "width" => this.width = value as i32,
                        "height" => this.height = value as i32,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for integer value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::Boolean(value) => {
                    match key.as_str() {
                        "ghost" => this.ghost = value,
                        "transparent" => this.transparent = value,
                        "can_update_states" => this.can_update_states = value,
                        "feet_collidable" => this.feet_collidable = value,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for boolean value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::String(value) => {
                    match key.as_str() {
                        "name" => this.name = value.to_str().unwrap_or("undefined").to_owned(),
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of BlockType for string value"
                            )))
                        }
                    };
                    Ok(())
                }
                _ => Err(rlua::Error::RuntimeError(
                    "Not a valid value type of BlockType".to_owned(),
                )),
            },
        );
    }
}
