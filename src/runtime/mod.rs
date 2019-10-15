use crate::prelude::*;

mod default;
mod zencode;

pub use default::DefaultRuntime;
pub use zencode::ZencodeRuntime;

use rlua::Result;

/// A runtime to execute interpreted source
pub trait Runtime {
    /// Load source code into the runtime
    fn load(&mut self, source: &str) -> Result<&mut Self>;
    /// Evaluate the loaded source code and return some output as a String
    fn eval<T>(&self) -> Result<T>
    where
        T: StaticFromLua;
}
