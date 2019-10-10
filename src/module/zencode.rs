use super::{DefaultModule, Module};
use rlua::{Context, Result, Value};

/// Exposes a `ZEN` module for parsing and running Zencode.
/// Take a look at [zencode-core](https://github.com/riddleandcode/zencode-core)
/// for more information on how to parse and run Zencode
#[derive(Default, Clone)]
pub struct Zencode;

static ZENCODE_SRC: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/lua/zencode/src/zencode.lua"
));

impl Module for Zencode {
    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let module = ctx
            .load(ZENCODE_SRC)
            .set_name("Zencode::build_module")?
            .eval()?;
        Ok(Value::Table(module))
    }
}

impl DefaultModule for Zencode {
    const GLOBAL_VAR: &'static str = "ZEN";
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlua::{Lua, Result};

    #[test]
    fn helloworld() -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            Zencode::import_module(lua_ctx)?;
            lua_ctx
                .load(
                    r#"
Given("that my name is ''", function(name)
    ACK.name = name
end)

Then("say hello", function()
    OUT = "Hello, " .. ACK.name .. "!"
end)

Then("print all data", function()
    -- print(OUT)
end)
"#,
                )
                .exec()
        })
        .and_then(|_| {
            lua.context(|lua_ctx| {
                lua_ctx
                    .load(
                        r#"
ZEN:begin(1)

local script = [[
Given that my name is 'Julian'
Then say hello
And print all data
]]

ZEN:parse(script)
return ZEN:run({}, {})
                "#,
                    )
                    .eval()
            })
        })
        .and_then(|out: std::string::String| {
            assert_eq!(out, "Hello, Julian!".to_string());
            Ok(())
        })
    }
}
