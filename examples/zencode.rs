extern crate rlua;
extern crate zenroom_minimal;

use rlua::{Lua, Result};
use zenroom_minimal::{prelude::*, util::read_file, Zencode};

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
