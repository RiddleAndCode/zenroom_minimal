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
        L: 'static + ScenarioLinker + Sync + Send + Clone,
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
}

impl Runtime for ZencodeRuntime {
    fn load(&mut self, source: &str) -> Result<&Self> {
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
    fn file_scenario_load() {
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
}
