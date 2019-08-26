extern crate rlua;
extern crate rlua_examples;

use rlua::{Lua, Result};
use rlua_examples::{prelude::*, util::read_file, Zencode};

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        Zencode::import_module(lua_ctx)?;
        lua_ctx
            .load(&read_file("lua/examples/zencode.lua")?)
            .exec()?;
        Ok(())
    })
}
