#[macro_use]
extern crate lazy_static;

extern crate base64;
extern crate ring;
extern crate rlua;
extern crate rlua_serde;
extern crate untrusted;

mod module;
mod runtime;
pub mod util;

pub use module::{
    DefaultModule, FileScenarioLinker, Importer, Json, Keyring, KeyringClass, Module, Octet,
    OctetClass, ScenarioLinker, ScenarioLoader, Zencode,
};
pub use runtime::{DefaultRuntime, ZencodeRuntime};

// TODO add own error types
pub use rlua::Error;
pub use rlua::Result;

pub mod prelude {
    pub use crate::module::ScenarioLinker;
    pub use crate::module::{DefaultModule, Module};
    pub use crate::runtime::Runtime;

    // TODO abstract away rlua public traits?
    pub use rlua::prelude::*;
}
