use super::{DefaultModule, Module};
use crate::util::read_file;
use rlua::{Context, Error, Result, Value};
use std::path::{Path, PathBuf};

/// A trait to define how to load Lua code from an identifier
/// to be used by the [`ScenarioLoader`]
pub trait ScenarioLinker {
    /// Define a method for converting an identifier to Lua source
    fn read_scenario(&self, scenario: &str) -> Result<String>;
}

/// [`ScenarioLinker`] which loads Lua source from a File relative to
/// `cwd`. For example when given `"scenario"` the [`FileScenarioLinker`]
/// tries to load a file with the name `./zencode_senario.lua`
#[derive(Clone)]
pub struct FileScenarioLinker(PathBuf);

impl FileScenarioLinker {
    /// Create a new [`FileScenarioLinker`] relative to `path`
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        // TODO vector of paths?
        FileScenarioLinker(path.as_ref().to_path_buf())
    }
}

impl ScenarioLinker for FileScenarioLinker {
    fn read_scenario(&self, scenario: &str) -> Result<String> {
        // TODO prefix as option
        read_file(self.0.join(format!("zencode_{}.lua", scenario)))
    }
}

impl Default for FileScenarioLinker {
    fn default() -> Self {
        // TODO cwd?
        FileScenarioLinker::new(".")
    }
}

/// A module which given a [`ScenarioLinker`] exposes a function
/// `load_scenario` into the Lua VM which then after linking the
/// Lua source returned, runs the source in a global scope.
/// This allows the user to define custom environments in a sandboxed
/// way. Similar to a `.bashrc` for a Lua VM.
#[derive(Clone)]
pub struct ScenarioLoader<L: ScenarioLinker>(L);

impl<L> ScenarioLoader<L>
where
    L: ScenarioLinker,
{
    /// Create a new [`ScenarioLoader`] using the [`ScenarioLinker`] to load
    /// requested Lua source
    pub fn new(ld: L) -> Self {
        ScenarioLoader(ld)
    }

    fn load_scenario<'lua>(&self, ctx: Context<'lua>, value: Value<'lua>) -> Result<()> {
        let name = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "module name must be a string".to_string(),
                ))
            }
        };
        let scenario = self.0.read_scenario(name.to_str()?)?;
        ctx.load(&scenario).exec()
    }
}

impl Default for ScenarioLoader<FileScenarioLinker> {
    fn default() -> Self {
        ScenarioLoader::new(FileScenarioLinker::default())
    }
}

impl<L> Module for ScenarioLoader<L>
where
    L: 'static + ScenarioLinker + Sync + Send,
{
    const IDENTIFIER: &'static str = "load_scenario";

    fn build_module<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let func = ctx.create_function(move |ctx, val| Ok(self.load_scenario(ctx, val)?))?;
        Ok(Value::Function(func))
    }
}

impl DefaultModule for ScenarioLoader<FileScenarioLinker> {
    const GLOBAL_VAR: &'static str = "load_scenario";
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{prelude::*, thread_rng};
    use rlua::{Lua, Result};
    use std::fs::{remove_file, File};
    use std::io::prelude::*;

    #[derive(Clone, Debug)]
    struct DummyScenarioLinker;

    impl ScenarioLinker for DummyScenarioLinker {
        fn read_scenario(&self, scenario: &str) -> Result<String> {
            Ok(format!("_G['loaded_scenario'] = '{}'", scenario))
        }
    }

    fn random_scenario(len: usize) -> String {
        thread_rng()
            .sample_iter(rand::distributions::Alphanumeric)
            .take(len)
            .collect()
    }

    #[test]
    fn dummy_load() -> Result<()> {
        let lua = Lua::new();
        let loader = ScenarioLoader::new(DummyScenarioLinker);
        let scenario = "hello";
        lua.context(|ctx| {
            ctx.globals().set("scenario", loader.build_module(ctx)?)?;
            ctx.load(&format!("scenario('{}')", scenario)).exec()?;
            ctx.load("return _G['loaded_scenario']").eval()
        })
        .and_then(|res: std::string::String| {
            assert_eq!(res, scenario);
            Ok(())
        })
    }

    #[test]
    fn file_scenario_load() -> Result<()> {
        // TODO make this windows compatible
        let lua = Lua::new();
        let linker = FileScenarioLinker::new("/tmp");
        let loader = ScenarioLoader::new(linker);

        let scenario = random_scenario(10);
        let filename = format!("/tmp/zencode_{}.lua", scenario);
        File::create(&filename)
            .and_then(|mut file| {
                file.write_all(format!("_G['loaded_scenario'] = '{}'", scenario).as_ref())
            })
            .unwrap();

        lua.context(|ctx| {
            ctx.globals().set("scenario", loader.build_module(ctx)?)?;
            ctx.load(&format!("scenario('{}')", scenario)).exec()?;
            ctx.load("return _G['loaded_scenario']").eval()
        })
        .and_then(|res: std::string::String| {
            assert_eq!(res, scenario);
            remove_file(filename).unwrap();
            Ok(())
        })
    }
}
