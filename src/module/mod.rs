mod default;
mod importer;
mod json;
mod keyring;
mod octet;
mod scenario;
mod zencode;

pub use default::DefaultModule;
pub use importer::Importer;
pub use json::Json;
pub use keyring::{Keyring, KeyringClass};
pub use octet::{Octet, OctetClass};
pub use zencode::Zencode;

use rlua::{Context, Result, Value};

pub trait Module {
    const IDENTIFIER: &'static str;
    fn build_module<'lua>(&self, ctx: Context<'lua>) -> Result<Value<'lua>>;
}
