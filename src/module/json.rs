use super::{DefaultModule, Module};
use rlua::{prelude::*, Context, Error, Result, Value};

/// Json encoding and decoding for Lua values.
///
/// Exposes a `JSON` module in Lua as a default
/// * `JSON.encode(<lua value>)`: Encodes a Lua Value as a JSON Lua String
/// * `JSON.decode(<lua string>)`: Decodes a Lua String as a JSON Lua Value
#[derive(Default, Clone)]
pub struct Json;

impl Json {
    fn encode<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
        let json_value: serde_json::Value = rlua_serde::from_value(value)?;
        let json =
            serde_json::to_string(&json_value).map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(json.to_lua(ctx)?)
    }

    fn decode<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
        let json = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "expecting string to decode".to_string(),
                ))
            }
        };
        let json_value: serde_json::Value =
            serde_json::from_str(json.to_str()?).map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(rlua_serde::to_value(ctx, json_value)?)
    }
}

impl Module for Json {
    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let module = ctx.create_table()?;
        module.set("encode", ctx.create_function(Json::encode)?)?;
        module.set("decode", ctx.create_function(Json::decode)?)?;
        Ok(Value::Table(module))
    }
}

impl DefaultModule for Json {
    const GLOBAL_VAR: &'static str = "JSON";
}
