extern crate rlua;
extern crate rlua_examples;

use rlua::{Lua, Result};
use rlua_examples::json::build_json_module;
use rlua_examples::keyring::build_keyring_module;
use rlua_examples::octet::build_octet_module;
use rlua_examples::utils::read_file;

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        lua_ctx.globals().set("JSON", build_json_module(lua_ctx)?)?;
        lua_ctx
            .globals()
            .set("OCTET", build_octet_module(lua_ctx)?)?;
        lua_ctx
            .globals()
            .set("KEYRING", build_keyring_module(lua_ctx)?)?;
        lua_ctx.load(&read_file("lua/example.lua")?).exec()?;
        Ok(())
    })
}
