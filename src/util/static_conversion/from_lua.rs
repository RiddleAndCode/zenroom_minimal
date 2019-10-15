use super::StaticUserData;
use core::hash::{BuildHasher, Hash};
use rlua::{prelude::*, Context, Error, LightUserData, Result, Value};
use std::collections::HashMap;
use std::ffi::CString;

// TODO consider an IntoStatic trait or something or nother.... (see Vec<T> implementation for why)

/// A slight reimplementation of [`FromLua`]. This was found to be necessary after seeing how
/// difficult it was to pass in external data into a runtime. Therefore this trait doesn't have a
/// reference to a `'lua` lifetime and can be used outside of a context until necessarily imported.
/// Most [`FromLua`] implementors have also been implemented here
pub trait StaticFromLua: Sized {
    /// Converts the value to a lua value given a context
    fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self>;
}

impl<T> StaticFromLua for T
where
    T: 'static + StaticUserData + Clone,
{
    fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self> {
        T::from_lua(value, ctx)
    }
}

impl<T> StaticFromLua for Vec<T>
where
    T: StaticFromLua + Send,
{
    fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self> {
        if let Value::Table(table) = value {
            table
                .sequence_values()
                .map(|elem: Result<Value<'lua>>| {
                    elem.and_then(|elem| T::static_from_lua(elem, ctx))
                })
                .collect()
        } else {
            Err(Error::FromLuaConversionError {
                from: "type name private", // TODO: Type name private
                to: "Vec",
                message: Some("expected table".to_string()),
            })
        }
    }
}

impl<K, V, S> StaticFromLua for HashMap<K, V, S>
where
    K: Eq + Hash + StaticFromLua,
    V: StaticFromLua,
    S: BuildHasher + Default,
{
    fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self> {
        if let Value::Table(table) = value {
            table
                .pairs()
                .map(|pair: Result<(Value<'lua>, Value<'lua>)>| {
                    pair.and_then(|(k, v)| {
                        K::static_from_lua(k, ctx)
                            .and_then(|k| V::static_from_lua(v, ctx).map(|v| (k, v)))
                    })
                })
                .collect()
        } else {
            Err(Error::FromLuaConversionError {
                from: "type name private", // TODO: Type name private
                to: "HashMap",
                message: Some("expected table".to_string()),
            })
        }
    }
}

impl<T> StaticFromLua for Option<T>
where
    T: StaticFromLua,
{
    fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self> {
        match value {
            Value::Nil => Ok(None),
            value => Ok(Some(T::static_from_lua(value, ctx)?)),
        }
    }
}

macro_rules! convert_simple_from_lua {
    ($x:ty) => {
        impl StaticFromLua for $x {
            fn static_from_lua<'lua>(value: Value<'lua>, ctx: Context<'lua>) -> Result<Self> {
                Self::from_lua(value, ctx)
            }
        }
    };
}

convert_simple_from_lua!(String);
convert_simple_from_lua!(bool);
convert_simple_from_lua!(CString);
convert_simple_from_lua!(i8);
convert_simple_from_lua!(u8);
convert_simple_from_lua!(i16);
convert_simple_from_lua!(u16);
convert_simple_from_lua!(i32);
convert_simple_from_lua!(u32);
convert_simple_from_lua!(i64);
convert_simple_from_lua!(u64);
convert_simple_from_lua!(i128);
convert_simple_from_lua!(u128);
convert_simple_from_lua!(isize);
convert_simple_from_lua!(usize);
convert_simple_from_lua!(f32);
convert_simple_from_lua!(f64);
convert_simple_from_lua!(Error);
convert_simple_from_lua!(LightUserData);
