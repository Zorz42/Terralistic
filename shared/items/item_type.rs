use crate::shared::blocks::{BlockId, ToolId};
use crate::shared::items::ItemId;
use crate::shared::walls::WallId;

#[derive(Clone)]
pub struct Item {
    pub name: String,
    pub display_name: String,
    pub max_stack: i32,
    pub places_block: Option<BlockId>,
    pub places_wall: Option<WallId>,
    pub tool: Option<ToolId>,
    pub tool_power: i32,
    pub(super) id: ItemId,
    pub width: f32,
    pub height: f32,
}

impl Item {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            max_stack: 0,
            places_block: None,
            places_wall: None,
            tool: None,
            tool_power: 0,
            id: ItemId::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    #[must_use]
    pub const fn get_id(&self) -> ItemId {
        self.id
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

// make item lua compatible
impl rlua::UserData for Item {
    fn add_methods<'lua, M: rlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method_mut(
            rlua::MetaMethod::NewIndex,
            |_, this, (key, value): (String, rlua::Value)| match value {
                rlua::Value::Integer(value) => {
                    match key.as_str() {
                        "max_stack" => this.max_stack = value as i32,
                        "width" => this.width = value as f32,
                        "height" => this.height = value as f32,
                        "tool_power" => this.tool_power = value as i32,
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for integer value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::String(value) => {
                    match key.as_str() {
                        "name" => this.name = value.to_str().unwrap_or("undefined").to_owned(),
                        "display_name" => {
                            this.display_name = value.to_str().unwrap_or("undefined").to_owned();
                        }
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for string value"
                            )))
                        }
                    };
                    Ok(())
                }
                rlua::Value::UserData(value) => {
                    match key.as_str() {
                        "places_block" => {
                            this.places_block = Some(*value.borrow::<BlockId>()?);
                        }
                        "places_wall" => {
                            this.places_wall = Some(*value.borrow::<WallId>()?);
                        }
                        "tool" => {
                            this.tool = Some(*value.borrow::<ToolId>()?);
                        }
                        _ => {
                            return Err(rlua::Error::RuntimeError(format!(
                                "{key} is not a valid field of Item for userdata value"
                            )))
                        }
                    };
                    Ok(())
                }
                _ => Err(rlua::Error::RuntimeError(
                    "Not a valid value type of Item".to_owned(),
                )),
            },
        );
    }
}
