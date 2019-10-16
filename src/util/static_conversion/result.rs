use super::StaticFromLua;
use rlua::{Context, Result, Value};

/// Helper to a Lua Result to a static outside Result
pub trait MapStaticFromLua<'lua> {
    /// Map from Lua Result to static outside Result
    fn map_static_from_lua<T>(self, ctx: Context<'lua>) -> Result<T>
    where
        T: StaticFromLua;
}

impl<'lua> MapStaticFromLua<'lua> for Result<Value<'lua>> {
    fn map_static_from_lua<T>(self, ctx: Context<'lua>) -> Result<T>
    where
        T: StaticFromLua,
    {
        self.and_then(|val| T::static_from_lua(val, ctx))
    }
}
