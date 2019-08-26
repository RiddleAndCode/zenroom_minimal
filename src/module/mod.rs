mod default;
mod importer;
mod json;
mod keyring;
mod octet;
mod zencode;

pub use default::DefaultModule;
pub use importer::Importer;
pub use json::Json;
pub use keyring::Keyring;
pub use octet::Octet;
pub use zencode::Zencode;

use rlua::{Context, Result, Value};

pub trait Module {
    const IDENTIFIER: &'static str;
    fn build_module(ctx: Context) -> Result<Value>;
}
