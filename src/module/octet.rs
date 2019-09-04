use super::{DefaultModule, Module};
use rlua::{prelude::*, Context, Error, Result, UserData, UserDataMethods, Value};
use std::ops::{Deref, DerefMut};

/// A Wrapper around a ByteString for use inside and outside of Lua
///
/// An Octet instance exposes some useful encoding / decoding methods
/// * `octet:base64()`: encode the byte string as a url safe base64 string
/// * `octet:string()`: encode the byte string as a utf-8 string
#[derive(Clone, Debug, Default)]
pub struct Octet(Vec<u8>);

/// A [`Octet`] factory.
///
/// Exposes a default `OCTET` module which can generate octets in three ways
/// * `OCTET.new()`: new empty octet
/// * `OCTET.base64(<lua string>)`: new octet from url safe base64 string
/// * `OCTET.string(<lua string>)`: new octet from utf-8 string
#[derive(Default)]
pub struct OctetClass;

impl Octet {
    /// Create new Octet from a byte vector
    pub fn new(bytes: Vec<u8>) -> Self {
        Octet(bytes)
    }
}

impl From<Vec<u8>> for Octet {
    fn from(value: Vec<u8>) -> Self {
        Octet::new(value)
    }
}

impl From<Octet> for Vec<u8> {
    fn from(value: Octet) -> Self {
        value.0
    }
}

impl Deref for Octet {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Octet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Octet {
    fn from_base64(value: Value) -> Result<Self> {
        let input = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "expecting string to decode".to_string(),
                ))
            }
        };
        let bytes = base64::decode_config(input.to_str()?, base64::URL_SAFE_NO_PAD)
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(Octet::new(bytes))
    }

    fn to_base64<'lua>(&self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        Ok(base64::encode_config(&self.0, base64::URL_SAFE_NO_PAD).to_lua(ctx)?)
    }
}

impl Octet {
    fn from_string(value: Value) -> Result<Self> {
        let input = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "expecting string to decode".to_string(),
                ))
            }
        };
        Ok(Octet::new(input.to_str()?.to_string().into()))
    }

    fn to_string<'lua>(&self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let string = std::string::String::from_utf8(self.0.clone())
            .map_err(|e| Error::RuntimeError(e.to_string()))?;
        Ok(string.to_lua(ctx)?)
    }
}

impl UserData for Octet {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("base64", |ctx, this, ()| Ok(this.to_base64(ctx)?));
        methods.add_method("string", |ctx, this, ()| Ok(this.to_string(ctx)?));
    }
}

impl Module for OctetClass {
    const IDENTIFIER: &'static str = "octet";

    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let module = ctx.create_table()?;
        module.set("new", ctx.create_function(|_, ()| Ok(Octet::default()))?)?;
        module.set(
            "base64",
            ctx.create_function(|_, value: Value| Ok(Octet::from_base64(value)?))?,
        )?;
        module.set(
            "string",
            ctx.create_function(|_, value: Value| Ok(Octet::from_string(value)?))?,
        )?;
        Ok(Value::Table(module))
    }
}

impl DefaultModule for OctetClass {
    const GLOBAL_VAR: &'static str = "OCTET";
}
