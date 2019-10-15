//! A minimal Lua VM binding with default crypto / encoding modules
//! and a builtin human readable DSL for building scenario specific
//! execution environments.
//!
//! # Motivation
//!
//! `zenroom_minimal` is an offshoot from the DECODEproject's
//! [Zenroom](https://github.com/DECODEproject/zenroom/tree/master/src).
//! When evaluating Zenroom, we found that although the security and
//! cryptographic capabilities were very flexible, the performance of the VM
//! did not meet our standards for high-performance / high-throughput.
//! In addition we needed flexible support for scenario logic to run in various
//! secure environments, thus `zenroom_minimal` was born, using Rust as a
//! module building tool instead of Zenroom's C libraries.
//!
//! # Usage
//!
//! Although Zenroom's libraries can be used directly with
//! an [`rlua`] Lua environment. It is
//! recommended that you use one of `zenroom_minimal`'s Runtime Environments
//! for Code execution to harness the full power and security of the VM
//!
//! ## Default Runtime
//!
//! The defualt runtime provides a Sandboxed Lua environment. This Lua
//! environment prevents the use of OS commands (like Time / File System / etc)
//! and provides instead an `import` function for whitelisted modules.
//!
//! ```
//! # use zenroom_minimal::{prelude::*, DefaultRuntime, Result, Error};
//! # fn main() -> Result<()> {
//! let res = DefaultRuntime::default()
//!     .load("return 'Hello, world!'")?
//!     .eval()?;
//! # if Some("Hello, world!".to_string()) == res {
//! #   Ok(())
//! # } else {
//! #   Err(Error::RuntimeError(format!("Unexpected output: {:?}", res)))
//! # }
//! # }
//! ```
//!
//! ## Zenroom Runtime
//!
//! The Zenroom Runtime leverages Zencode to execute Human Readable protected
//! code through loaded scenarios. Take a look at the `examples` for more
//! information on how to use

#![warn(missing_docs)]

extern crate base64;
extern crate hashbrown;
extern crate ring;
extern crate rlua;
extern crate rlua_serde;
extern crate untrusted;

mod module;
mod runtime;

/// Utility functions
pub mod util;

pub use crate::util::{StaticFromLua, StaticToLua, StaticUserData};
pub use module::{
    DefaultModule, FileScenarioLinker, ImportableModule, Importer, Json, Keyring, KeyringClass,
    Module, Octet, OctetClass, ScenarioLinker, ScenarioLoader, Zencode,
};
pub use runtime::{DefaultRuntime, ZencodeRuntime};

// TODO add own error types
pub use rlua::Error;
pub use rlua::Result;

/// Useful traits for implementing the `zenroom_minimal` library
pub mod prelude {
    pub use crate::module::ScenarioLinker;
    pub use crate::module::{DefaultModule, ImportableModule, Module};
    pub use crate::runtime::Runtime;
    pub use crate::util::{StaticToLua, StaticUserData};

    // TODO abstract away rlua public traits?
    pub use rlua::prelude::*;
}
