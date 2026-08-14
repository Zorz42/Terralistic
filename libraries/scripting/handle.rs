/// Makes a handle type passable to and from scripts as opaque userdata.
///
/// Scripts hold these but cannot look inside them or make one up - which is the point. A
/// handle that came from the host is the only kind a script can hand back.
///
/// `script_handle!(Foo)` gives conversion only; `script_handle!(Foo, eq)` also makes `==`
/// work in lua, which needs `PartialEq`.
///
/// # Why this is a macro
///
/// `FromLua` and `UserData` are `rlua`'s traits and the handle types are the caller's, so
/// neither side can write a blanket impl without the orphan rule stopping it. Six hand
/// written copies is what that costs otherwise, and all six had the same latent panic: an
/// `unreachable!()` on anything that was not userdata, which a script reaches by passing a
/// string to a function that wanted a handle. That is now a lua error the script can catch.
#[macro_export]
macro_rules! script_handle {
    ($type:ty) => {
        impl rlua::FromLua<'_> for $type {
            fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
                $crate::libraries::scripting::handle_from_lua(value, stringify!($type))
            }
        }

        impl rlua::UserData for $type {}
    };
    ($type:ty, eq) => {
        impl rlua::FromLua<'_> for $type {
            fn from_lua(value: rlua::Value, _context: rlua::Context) -> rlua::Result<Self> {
                $crate::libraries::scripting::handle_from_lua(value, stringify!($type))
            }
        }

        impl rlua::UserData for $type {
            fn add_methods<'lua, Methods: rlua::UserDataMethods<'lua, Self>>(methods: &mut Methods) {
                methods.add_meta_method(rlua::MetaMethod::Eq, |_, this, other: Self| Ok(*this == other));
            }
        }
    };
}

/// The body of every `script_handle!` conversion, out of line so the macro stays small and
/// the error is worded in one place.
///
/// Anything that is not userdata of this type is a conversion error rather than a panic: a
/// script passing a string where a handle was wanted is a mistake in the script, and it
/// should say so rather than take the host down.
pub fn handle_from_lua<Handle: Clone + rlua::UserData + 'static>(value: rlua::Value, type_name: &'static str) -> rlua::Result<Handle> {
    match value {
        rlua::Value::UserData(data) => Ok(data.borrow::<Handle>()?.clone()),
        other => Err(rlua::Error::FromLuaConversionError {
            from: other.type_name(),
            to: type_name,
            message: Some(format!("expected a {type_name} handle, which only the host can produce")),
        }),
    }
}
