extern crate rlua;
extern crate zenroom_minimal;

use rlua::{Lua, Result};
use zenroom_minimal::{prelude::*, util::read_file, Json, KeyringClass, OctetClass};

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        Json::import_module(lua_ctx)?;
        OctetClass::import_module(lua_ctx)?;
        KeyringClass::import_module(lua_ctx)?;
        lua_ctx
            .load(&read_file("lua/examples/signing_script.lua")?)
            .exec()?;
        Ok(())
    })
}
