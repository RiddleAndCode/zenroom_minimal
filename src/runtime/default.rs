use super::Runtime;
use crate::{prelude::*, Importer};
use rlua::{Lua, Result, StdLib};

/// The default runtime is a basic Lua environment, sandboxed without
/// file system or some standard OS function access. The environment
/// is preluded with the [`Importer`] module
pub struct DefaultRuntime {
    lua: Lua,
    source: String,
}

impl Default for DefaultRuntime {
    fn default() -> Self {
        // TODO make static
        let mut libs = StdLib::empty();
        libs.insert(StdLib::BASE);
        libs.insert(StdLib::COROUTINE);
        libs.insert(StdLib::TABLE);
        libs.insert(StdLib::STRING);
        libs.insert(StdLib::UTF8);
        libs.insert(StdLib::MATH);
        let lua = Lua::new_with(libs);
        DefaultRuntime::new(lua)
    }
}

impl DefaultRuntime {
    /// Create a new [`DefaultRuntime`]
    pub fn new(lua: Lua) -> Self {
        let runtime = DefaultRuntime {
            lua,
            source: "".to_string(),
        };
        runtime.lua.context(Importer::import_module).unwrap();
        runtime
    }
}

impl Runtime for DefaultRuntime {
    fn load(&mut self, source: &str) -> Result<&mut Self> {
        self.source = source.to_owned();
        Ok(self)
    }

    fn eval(&self) -> Result<Option<String>> {
        self.lua
            .context(|lua_ctx| lua_ctx.load(&self.source).eval::<Option<String>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut runtime = DefaultRuntime::default();
        let res = runtime.load("").unwrap().eval();
        match res {
            Ok(None) => (),
            _ => panic!("empty script should return none"),
        }
    }

    #[test]
    fn require_fails() {
        let mut runtime = DefaultRuntime::default();
        let res = runtime.load("return require").unwrap().eval();
        match res {
            Ok(None) => (),
            _ => panic!("require should be none"),
        }
    }

    #[test]
    fn import() {
        let mut runtime = DefaultRuntime::default();
        let res = runtime
            .load(
                r#"
JSON = import('json')
return JSON.encode({a = 1})
        "#,
            )
            .unwrap()
            .eval()
            .unwrap();
        assert_eq!(res, Some("{\"a\":1}".to_string()));
    }
}
