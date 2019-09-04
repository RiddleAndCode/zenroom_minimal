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
pub use scenario::{FileScenarioLinker, ScenarioLinker, ScenarioLoader};
pub use zencode::Zencode;

use rlua::{Context, Result, Value};

/// A Module which can be imported into a Lua VM by [`rlua`]
pub trait Module {
    /// The identifier with which the [`Importer`] Module imports the Module
    const IDENTIFIER: &'static str;

    /// The function called when loading the Module into the Lua VM.
    /// This function should return a [`Value`] which is the
    /// Module's interface in Lua
    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>>;
}
