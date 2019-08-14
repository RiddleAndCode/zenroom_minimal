use rlua::{prelude::*, Context, Error, Result, Table, Value};

fn json_encode<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
    let json_value: serde_json::Value = rlua_serde::from_value(value)?;
    let json =
        serde_json::to_string(&json_value).map_err(|e| Error::RuntimeError(e.to_string()))?;
    Ok(json.to_lua(ctx)?)
}

fn json_decode<'lua>(ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
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

pub fn build_json_module(ctx: Context) -> Result<Table> {
    let module = ctx.create_table()?;
    module.set("encode", ctx.create_function(json_encode)?)?;
    module.set("decode", ctx.create_function(json_decode)?)?;
    Ok(module)
}
