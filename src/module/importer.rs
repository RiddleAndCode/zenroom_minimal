use super::{DefaultModule, Json, KeyringClass, Module, OctetClass, Zencode};
use rlua::{Context, Error, Result, Value};

/// A module which imports another module by its [`Module::IDENTIFIER`].
///
/// Available under `import("module_name")` in Lua as default
#[derive(Default)]
pub struct Importer;

impl<'lua> Importer {
    fn import(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
        let name = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "module name must be a string".to_string(),
                ))
            }
        };
        match name.to_str()? {
            Json::IDENTIFIER => Json::default().build_module(ctx),
            KeyringClass::IDENTIFIER => KeyringClass::default().build_module(ctx),
            OctetClass::IDENTIFIER => OctetClass::default().build_module(ctx),
            Zencode::IDENTIFIER => Zencode::default().build_module(ctx),
            _ => Err(Error::RuntimeError(format!(
                "module '{}' could not be found",
                name.to_str()?
            ))),
        }
    }
}

impl Module for Importer {
    const IDENTIFIER: &'static str = "import";

    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        Ok(Value::Function(ctx.create_function(Importer::import)?))
    }
}

impl DefaultModule for Importer {
    const GLOBAL_VAR: &'static str = "import";
}
