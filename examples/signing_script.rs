extern crate rlua;
extern crate rlua_examples;

use rlua::{Lua, Result};
use rlua_examples::{prelude::*, utils::read_file, Json, Keyring, Octet};

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        Json::load_module(lua_ctx)?;
        Octet::load_module(lua_ctx)?;
        Keyring::load_module(lua_ctx)?;
        lua_ctx
            .load(&read_file("lua/examples/signing_script.lua")?)
            .exec()?;
        Ok(())
    })
}
