use super::Runtime;
use crate::DefaultModule;
use crate::Importer;
use rlua::{Lua, Result, StdLib};

pub struct DefaultRuntime(Lua);

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

        let runtime = DefaultRuntime(lua);
        runtime.lua().context(Importer::import_module).unwrap();
        runtime
    }
}

impl DefaultRuntime {
    pub fn new() -> Self {
        DefaultRuntime::default()
    }

    #[inline]
    fn lua(&self) -> &Lua {
        &self.0
    }
}

impl Runtime for DefaultRuntime {
    fn run(&self, source: &str) -> Result<Option<String>> {
        self.lua()
            .context(|lua_ctx| lua_ctx.load(&source).eval::<Option<String>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let runtime = DefaultRuntime::new();
        let res = runtime.run("");
        match res {
            Ok(None) => (),
            _ => panic!("empty script should return none"),
        }
    }

    #[test]
    fn require_fails() {
        let runtime = DefaultRuntime::new();
        let res = runtime.run("return require");
        match res {
            Ok(None) => (),
            _ => panic!("require should be none"),
        }
    }

    #[test]
    fn import() {
        let runtime = DefaultRuntime::new();
        let res = runtime
            .run(
                r#"
JSON = import('json')
return JSON.encode({a = 1})
        "#,
            )
            .unwrap();
        assert_eq!(res, Some("{\"a\":1}".to_string()));
    }
}
