use std::collections::HashMap;
use std::slice::{Iter, IterMut};

use anyhow::Result;
use rlua::prelude::LuaError;
use rlua::{Context, FromLuaMulti, IntoLuaMulti, Lua};
use serde::{Deserialize, Serialize};

use crate::libraries::scripting::ScriptModuleData;

/// Where a module's own id is kept inside its interpreter state, so a host function called
/// from lua can tell which module called it.
static MODULE_ID_IDENT: &str = "__SCRIPT_MODULE_ID";

/// Returns the id of the module whose lua context this is.
pub fn get_module_id(context: Context) -> Result<i32, LuaError> {
    let globals = context.globals();
    globals.get::<_, i32>(MODULE_ID_IDENT)
}

/// One script module: source, resources, and an interpreter state of its own.
///
/// **Each module gets its own `Lua`**, so two cannot clobber each other's globals, and
/// everything a module reaches outside itself is a host function the owner registered.
/// Resources are named bytes travelling with the source, keyed by a `:` separated path.
pub struct ScriptModule {
    name: String,
    source: String,
    resources: HashMap<String, Vec<u8>>,
    lua: Lua,
    id: i32,
}

impl ScriptModule {
    #[must_use]
    pub fn new(name: String, source: String, resources: HashMap<String, Vec<u8>>) -> Self {
        Self {
            name,
            source,
            resources,
            lua: Lua::new(),
            id: -1,
        }
    }

    /// Runs the module's source and then its `init`, if it has one.
    fn init(&mut self, id: i32) -> Result<()> {
        self.id = id;

        {
            self.lua.load(&self.source).exec()?;
            let globals = self.lua.globals();
            globals.set(MODULE_ID_IDENT, self.id)?;
        }

        self.call_function::<(), ()>("init", ())?;
        Ok(())
    }

    /// Makes `func` callable from this module's scripts under `name`.
    pub fn add_global_function<F, A, R>(&self, name: &str, func: F) -> Result<()>
    where
        F: 'static + Send + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> IntoLuaMulti<'lua>,
    {
        let globals = self.lua.globals();
        globals.set(name, self.lua.create_function(func)?)?;
        Ok(())
    }

    /// Calls a function the module defined, with arguments, and returns its result.
    pub fn call_function<A, R>(&self, name: &str, args: A) -> Result<R, LuaError>
    where
        A: for<'lua> IntoLuaMulti<'lua>,
        R: for<'lua> FromLuaMulti<'lua>,
    {
        let globals = self.lua.globals();
        let func = globals.get::<_, rlua::Function>(name)?;
        func.call(args)
    }

    /// Checks if a symbol is defined in the module.
    pub fn is_symbol_defined(&self, name: &str) -> Result<bool> {
        let globals = self.lua.globals();
        Ok(globals.contains_key(name)?)
    }

    fn update(&self) -> Result<()> {
        Ok(self.call_function::<(), ()>("update", ())?)
    }

    fn stop(&self) -> Result<()> {
        Ok(self.call_function::<(), ()>("stop", ())?)
    }

    fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        self.resources.get(path)
    }

    /// Every global the module has defined.
    #[must_use]
    pub fn get_all_symbols(&self) -> Vec<String> {
        let mut result = Vec::new();
        let globals = self.lua.globals();
        for (key, _value) in globals.pairs::<String, rlua::Value>().flatten() {
            result.push(key);
        }
        result
    }

    /// The globals whose name starts with `prefix`, with the prefix removed - how an owner
    /// discovers what a module offers by convention rather than declaration. A module defining
    /// `command_teleport` has a `teleport` command; what that *means* is the owner's.
    #[must_use]
    pub fn symbols_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.get_all_symbols().iter().filter_map(|symbol| symbol.strip_prefix(prefix).map(str::to_owned)).collect()
    }

    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Serialize for ScriptModule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let data = ScriptModuleData {
            name: self.name.clone(),
            source: self.source.clone(),
            resources: self.resources.clone().into_iter().collect(),
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ScriptModule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = ScriptModuleData::deserialize(deserializer)?;
        Ok(Self {
            name: data.name,
            source: data.source,
            resources: data.resources.into_iter().collect(),
            lua: Lua::new(),
            id: -1,
        })
    }
}

/// A set of script modules, driven together.
///
/// Host functions registered here reach every module under `function_prefix`, which keeps the
/// host's names out of a module's way and is the host's to choose rather than this library's.
///
/// **Not in scope**: what the lifecycle hooks mean, which functions are registered, and what a
/// symbol named by convention implies. This calls `init`, `update` and `stop` when told to.
pub struct ScriptHost {
    modules: Vec<ScriptModule>,
    function_prefix: &'static str,
}

impl ScriptHost {
    #[must_use]
    pub const fn new(modules: Vec<ScriptModule>, function_prefix: &'static str) -> Self {
        Self { modules, function_prefix }
    }

    /// Makes `func` callable from every module, under the host's prefix.
    pub fn add_global_function<F, A, R>(&mut self, name: &str, func: F) -> Result<()>
    where
        F: 'static + Send + Clone + Fn(Context, A) -> Result<R, LuaError>,
        A: for<'lua> FromLuaMulti<'lua>,
        R: for<'lua> IntoLuaMulti<'lua>,
    {
        for module in &mut self.modules {
            module.add_global_function(&(self.function_prefix.to_owned() + name), func.clone())?;
        }
        Ok(())
    }

    /// Loads and initializes every module, in order.
    pub fn init(&mut self) -> Result<()> {
        for (id, module) in self.modules.iter_mut().enumerate() {
            module.init(id as i32)?;
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        for module in &mut self.modules {
            module.update()?;
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        for module in &mut self.modules {
            module.stop()?;
        }
        Ok(())
    }

    /// Looks a resource up across every module, **last first**, so a module loaded later can
    /// replace a resource an earlier one provided.
    #[must_use]
    pub fn get_resource(&self, path: &str) -> Option<&Vec<u8>> {
        for module in self.modules.iter().rev() {
            if let Some(data) = module.get_resource(path) {
                return Some(data);
            }
        }
        None
    }

    pub fn get_module(&mut self, id: i32) -> Option<&mut ScriptModule> {
        self.modules.get_mut(id as usize)
    }

    pub fn modules_iter(&self) -> Iter<'_, ScriptModule> {
        self.modules.iter()
    }

    pub fn modules_iter_mut(&mut self) -> IterMut<'_, ScriptModule> {
        self.modules.iter_mut()
    }
}
