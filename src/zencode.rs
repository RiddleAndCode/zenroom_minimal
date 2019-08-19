use crate::module::Module;
use crate::utils::read_file;
use rlua::{Context, Result, Table};
use std::env;
use std::path::Path;

pub struct Zencode;

impl Module for Zencode {
    fn module_identifier() -> &'static str {
        "ZEN"
    }

    fn build_module(ctx: Context) -> Result<Table> {
        lazy_static! {
            static ref ZENCODE_SRC: String = {
                let file = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("lua/zencode/src/zencode.lua");
                read_file(file).unwrap()
            };
        }
        let module = ctx
            .load(&ZENCODE_SRC.to_string())
            .set_name("Zencode::build_module")?
            .eval()?;
        Ok(module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlua::{Lua, Result};

    #[test]
    fn helloworld() -> Result<()> {
        let lua = Lua::new();

        lua.context(|lua_ctx| {
            Zencode::load_module(lua_ctx)?;
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
