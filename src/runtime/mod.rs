mod default;

pub use default::DefaultRuntime;

use rlua::Result;

pub trait Runtime {
    fn run(&self, source: &str) -> Result<Option<String>>;
}
