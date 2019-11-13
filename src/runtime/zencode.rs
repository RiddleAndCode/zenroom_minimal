use super::Runtime;
use crate::{prelude::*, Importer, Json, ScenarioLoader, Zencode};
use rlua::{Lua, Result};
use serde::Serialize;

/// Execution environment to parse Zencode source and run
/// the Zencode against scenarios, data and keys
pub struct ZencodeRuntime {
    lua: Lua,
}

impl Default for ZencodeRuntime {
    fn default() -> Self {
        ZencodeRuntime::new(ScenarioLoader::default(), Importer::with_default_modules())
    }
}

impl ZencodeRuntime {
    /// Create a new [`ZencodeRuntime`] with a [`ScenarioLoader`] to load
    /// scenarios requested from Zencode
    pub fn new<L>(loader: ScenarioLoader<L>, importer: Importer) -> Self
    where
        L: 'static + ScenarioLinker + Sync + Send,
    {
        let lua = Lua::default();
        lua.context(|ctx| {
            Importer::import_module(ctx).unwrap();
            ctx.globals()
                .set(
                    ScenarioLoader::GLOBAL_VAR,
                    loader.build_module(ctx).unwrap(),
                )
                .unwrap();
            ctx.globals()
                .set(Importer::GLOBAL_VAR, importer.build_module(ctx).unwrap())
                .unwrap();
            Zencode::import_module(ctx).unwrap();
            Json::import_module(ctx).unwrap();
            // TODO verbosity
            ctx.load("ZEN:begin(1)").exec().unwrap();
        });
        ZencodeRuntime { lua }
    }

    /// Load data to be passed into `ZEN:run`
    pub fn load_data<T>(&mut self, data: T) -> Result<&mut Self>
    where
        T: for<'lua> ToLua<'lua>,
    {
        self.lua
            .context(|ctx| ctx.globals().set("_DATA", data.to_lua(ctx)?))?;
        Ok(self)
    }

    /// Serialize and load data to be passed into `ZEN:run`
    pub fn load_serializable_data<T>(&mut self, data: T) -> Result<&mut Self>
    where
        T: Serialize,
    {
        self.lua
            .context(|ctx| ctx.globals().set("_DATA", rlua_serde::to_value(ctx, data)?))?;
        Ok(self)
    }

    /// Load keys to be passed into `ZEN:run`
    pub fn load_keys<T>(&mut self, keys: T) -> Result<&mut Self>
    where
        T: for<'lua> ToLua<'lua>,
    {
        self.lua
            .context(|ctx| ctx.globals().set("_KEYS", keys.to_lua(ctx)?))?;
        Ok(self)
    }

    /// Serialize and load keys to be passed into `ZEN:run`
    pub fn load_serializable_keys<T>(&mut self, keys: T) -> Result<&mut Self>
    where
        T: Serialize,
    {
        self.lua
            .context(|ctx| ctx.globals().set("_KEYS", rlua_serde::to_value(ctx, keys)?))?;
        Ok(self)
    }
}

impl Runtime for ZencodeRuntime {
    fn load(&mut self, source: &str) -> Result<&mut Self> {
        self.lua.context(|ctx| {
            ctx.load(&format!(
                r#"
ZEN:reset()
script = [[{}]]
ZEN:parse(script)
            "#,
                source
            ))
            .exec()
        })?;
        Ok(self)
    }

    fn eval<T>(&self) -> Result<T>
    where
        T: for<'lua> FromLua<'lua>,
    {
        self.lua.context(|ctx| {
            ctx.load("ZEN:run(_DATA, _KEYS)")
                .eval()
                .and_then(|value| T::from_lua(value, ctx))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FileScenarioLinker, ScenarioLoader};
    use rand::{prelude::*, thread_rng};
    use std::collections::HashMap;
    use std::fs::{remove_file, File};
    use std::io::prelude::*;

    fn random_scenario(len: usize) -> String {
        thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(len)
            .collect()
    }

    #[test]
    fn empty() {
        let mut runtime = ZencodeRuntime::default();
        let res: HashMap<String, String> = runtime.load("").unwrap().eval().unwrap();
        assert_eq!(HashMap::new(), res);
    }

    #[test]
    fn helloworld() {
        // TODO make this windows compatible
        let loader = ScenarioLoader::new(FileScenarioLinker::new("/tmp"));
        let mut runtime = ZencodeRuntime::new(loader, Importer::with_default_modules());
        let scenario = random_scenario(10);
        let filename = format!("/tmp/zencode_{}.lua", scenario);
        File::create(&filename)
            .and_then(|mut file| {
                file.write_all(
                    r#"
Given("that my name is ''", function(name)
    ACK.name = name
end)

Then("say hello", function()
    OUT = "Hello, " .. ACK.name .. "!"
end)

Then("print all data", function()
    -- print(OUT)
end)
"#
                    .as_ref(),
                )
            })
            .unwrap();
        let res = runtime
            .load(&format!(
                r#"
Scenario '{}'
Given that my name is 'Julian'
Then say hello
And print all data
        "#,
                scenario
            ))
            .unwrap()
            .eval()
            .unwrap();
        assert_eq!(Some("Hello, Julian!".to_string()), res);
        remove_file(filename).unwrap();
    }

    #[test]
    fn addition() {
        // TODO make this windows compatible
        let loader = ScenarioLoader::new(FileScenarioLinker::new("/tmp"));
        let mut runtime = ZencodeRuntime::new(loader, Importer::with_default_modules());
        let scenario = random_scenario(10);
        let filename = format!("/tmp/zencode_{}.lua", scenario);
        File::create(&filename)
            .and_then(|mut file| {
                file.write_all(
                    r#"
Given("that I want to add '' with ''", function(a, b)
    ACK.left = IN[a]
    ACK.right = IN[b]
end)

Then("do addition", function()
    OUT = ACK.left + ACK.right
end)

Then("print all data", function()
    -- print(OUT)
end)
"#
                    .as_ref(),
                )
            })
            .unwrap();
        let mut data = HashMap::new();
        data.insert("a".to_string(), 1);
        data.insert("b".to_string(), 2);
        let res = runtime
            .load_data(data)
            .unwrap()
            .load(&format!(
                r#"
Scenario '{}'
Given that I want to add 'a' with 'b'
Then do addition
And print all data
        "#,
                scenario
            ))
            .unwrap()
            .eval()
            .unwrap();
        assert_eq!(Some("3".to_string()), res);
        remove_file(filename).unwrap();
    }
}
