/// Makes a handle type passable to and from scripts as opaque userdata.
///
/// Scripts hold these but cannot look inside one or make one up, so the only handle a script can
/// hand back is one the host gave it. `script_handle!(Foo, eq)` also makes `==` work in lua,
/// needing `PartialEq`.
///
/// A macro because `FromLua` and `UserData` are `rlua`'s and the handle types are the caller's,
/// so the orphan rule stops either side writing the blanket impl. The six hand written copies
/// it replaced shared one latent panic: an `unreachable!()` on anything that was not userdata,
/// which a script reaches by passing a string. That is now a lua error the script can catch.
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

/// The body of every `script_handle!` conversion, out of line so the macro stays small and the
/// error is worded once.
///
/// Anything that is not userdata of this type is a conversion error rather than a panic - a
/// script's mistake should not take the host down.
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
