use super::{DefaultModule, Json, Keyring, Module, Octet, Zencode};
use rlua::{Context, Error, Result, Value};

pub struct Importer;

fn import<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
    let name = match value {
        Value::String(s) => s,
        _ => {
            return Err(Error::RuntimeError(
                "module name must be a string".to_string(),
            ))
        }
    };
    match name.to_str()? {
        Json::IDENTIFIER => Json::build_module(ctx),
        Keyring::IDENTIFIER => Keyring::build_module(ctx),
        Octet::IDENTIFIER => Octet::build_module(ctx),
        Zencode::IDENTIFIER => Zencode::build_module(ctx),
        _ => Err(Error::RuntimeError(format!(
            "module '{}' could not be found",
            name.to_str()?
        ))),
    }
}

impl Module for Importer {
    const IDENTIFIER: &'static str = "import";

    fn build_module(ctx: Context) -> Result<Value> {
        Ok(Value::Function(ctx.create_function(import)?))
    }
}

impl DefaultModule for Importer {
    const GLOBAL_VAR: &'static str = "import";
}
