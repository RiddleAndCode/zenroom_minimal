use super::StaticUserData;
use core::hash::{BuildHasher, Hash};
use rlua::{prelude::*, Context, Error, LightUserData, Result, Value};
use std::collections::HashMap;
use std::ffi::CString;

/// A slight reimplementation of [`ToLua`]. This was found to be necessary after seeing how
/// difficult it was to pass in external data into a runtime. Therefore this trait doesn't have a
/// reference to a `'lua` lifetime and can be used outside of a context until necessarily imported.
/// Most [`ToLua`] implementors have also been implemented here
pub trait StaticToLua {
    /// Converts the value to a lua value given a context
    fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>>;
}

impl<T> StaticToLua for T
where
    T: 'static + StaticUserData + Send,
{
    #[inline]
    fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        self.to_lua(ctx)
    }
}

impl<T> StaticToLua for Vec<T>
where
    T: StaticToLua + Send,
{
    fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let res: Vec<Value<'lua>> = self
            .into_iter()
            .map(|v| v.static_to_lua(ctx).into_iter())
            .flatten()
            .collect();
        Ok(Value::Table(ctx.create_sequence_from(res)?))
    }
}

impl<K, V, S> StaticToLua for HashMap<K, V, S>
where
    K: Eq + Hash + StaticToLua,
    V: StaticToLua,
    S: BuildHasher,
{
    fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let res: Vec<(Value<'lua>, Value<'lua>)> = self
            .into_iter()
            .map(|(k, v)| {
                k.static_to_lua(ctx)
                    .and_then(|k| v.static_to_lua(ctx).map(|v| (k, v)))
            })
            .flatten()
            .collect();
        Ok(Value::Table(ctx.create_table_from(res)?))
    }
}

impl<T> StaticToLua for Option<T>
where
    T: StaticToLua,
{
    #[inline]
    fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        match self {
            Some(val) => val.static_to_lua(ctx),
            None => Ok(Value::Nil),
        }
    }
}

macro_rules! convert_simple_to_lua {
    ($x:ty) => {
        impl StaticToLua for $x {
            #[inline]
            fn static_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
                self.to_lua(ctx)
            }
        }
    };
}

convert_simple_to_lua!(String);
convert_simple_to_lua!(bool);
convert_simple_to_lua!(CString);
convert_simple_to_lua!(i8);
convert_simple_to_lua!(u8);
convert_simple_to_lua!(i16);
convert_simple_to_lua!(u16);
convert_simple_to_lua!(i32);
convert_simple_to_lua!(u32);
convert_simple_to_lua!(i64);
convert_simple_to_lua!(u64);
convert_simple_to_lua!(i128);
convert_simple_to_lua!(u128);
convert_simple_to_lua!(isize);
convert_simple_to_lua!(usize);
convert_simple_to_lua!(f32);
convert_simple_to_lua!(f64);
convert_simple_to_lua!(Error);
convert_simple_to_lua!(LightUserData);
