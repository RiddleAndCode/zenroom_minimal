use super::Runtime;
use crate::{prelude::*, Importer, Json, ScenarioLoader, Zencode};
use rlua::{Lua, Result};

pub struct ZencodeRuntime {
    lua: Lua,
    data: String,
    keys: String,
}

impl Default for ZencodeRuntime {
    fn default() -> Self {
        ZencodeRuntime::new(ScenarioLoader::default())
    }
}

impl ZencodeRuntime {
    pub fn new<L>(loader: ScenarioLoader<L>) -> Self
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
            Zencode::import_module(ctx).unwrap();
            Json::import_module(ctx).unwrap();
            // TODO verbosity
            ctx.load("ZEN:begin(1)").exec().unwrap();
        });
        ZencodeRuntime {
            lua,
            data: "{}".to_string(),
            keys: "{}".to_string(),
        }
    }

    pub fn load_data(&mut self, data: &str) -> Result<&mut Self> {
        // TODO validation
        self.data = data.to_owned();
        Ok(self)
    }

    pub fn load_keys(&mut self, keys: &str) -> Result<&mut Self> {
        // TODO validation
        self.keys = keys.to_owned();
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

    fn eval(&self) -> Result<Option<String>> {
        // TODO encoding of data and keys
        self.lua.context(|ctx| {
            ctx.load(&format!(
                "return JSON.encode(ZEN:run({}, {}))",
                self.data, self.keys
            ))
            .eval()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FileScenarioLinker, ScenarioLoader};
    use rand::{prelude::*, thread_rng};
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
        let res = runtime.load("").unwrap().eval().unwrap();
        assert_eq!(Some("{}".to_string()), res);
    }

    #[test]
    fn helloworld() {
        // TODO make this windows compatible
        let loader = ScenarioLoader::new(FileScenarioLinker::new("/tmp"));
        let mut runtime = ZencodeRuntime::new(loader);
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
        assert_eq!(Some("\"Hello, Julian!\"".to_string()), res);
        remove_file(filename).unwrap();
    }

    #[test]
    fn addition() {
        // TODO make this windows compatible
        let loader = ScenarioLoader::new(FileScenarioLinker::new("/tmp"));
        let mut runtime = ZencodeRuntime::new(loader);
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
        let data = "{a = 1, b = 2}";
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
