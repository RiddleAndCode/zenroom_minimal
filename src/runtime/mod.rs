mod default;
mod zencode;

pub use default::DefaultRuntime;
pub use zencode::ZencodeRuntime;

use rlua::Result;

pub trait Runtime {
    fn load(&mut self, source: &str) -> Result<&mut Self>;
    fn eval(&self) -> Result<Option<String>>;
}
