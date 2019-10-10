use super::{DefaultModule, Json, KeyringClass, Module, OctetClass, Zencode};
use hashbrown::HashMap;
use rlua::{Context, Error, Result, Value};

/// A module which can import itself by reference
pub trait ImportableModule {
    /// import self by reference
    fn import_self<'lua>(&self, ctx: Context<'lua>) -> Result<Value<'lua>>;
}

impl<T> ImportableModule for T
where
    T: Module + Clone,
{
    fn import_self<'lua>(&self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        self.clone().build_module(ctx)
    }
}

/// A module which imports other modules which have been registered.
///
/// Available under `import("module_name")` in Lua as default
#[derive(Default)]
pub struct Importer {
    modules: HashMap<String, Box<dyn ImportableModule + Send + Sync>>,
}

impl<'lua> Importer {
    fn import(&mut self, ctx: Context<'lua>, value: Value<'lua>) -> Result<Value<'lua>> {
        let name = match value {
            Value::String(s) => s,
            _ => {
                return Err(Error::RuntimeError(
                    "module name must be a string".to_string(),
                ))
            }
        };
        self.modules
            .remove(name.to_str()?)
            .ok_or(Error::RuntimeError(format!(
                "module '{}' could not be found",
                name.to_str()?
            )))
            .and_then(|module| module.import_self(ctx))
    }

    /// Registers a new module key string to a importable module class
    pub fn register<M: ImportableModule + Send + Sync + 'static>(
        &mut self,
        name: &str,
        module: M,
    ) -> Result<()> {
        // TODO check if already set
        self.modules.insert(name.to_string(), Box::new(module));
        Ok(())
    }

    /// Create an Importer with the default library modules
    ///
    /// Modules:
    /// * `keyring`: [`KeyringClass`]
    /// * `json`: [`Json`]
    /// * `octet`: [`OctetClass`]
    /// * `zenroom`: [`Zencode`]
    pub fn with_default_modules() -> Importer {
        let mut importer = Importer::default();
        importer.register("json", Json::default()).unwrap();
        importer
            .register("keyring", KeyringClass::default())
            .unwrap();
        importer.register("octet", OctetClass::default()).unwrap();
        importer.register("zenroom", Zencode::default()).unwrap();
        importer
    }
}

impl Module for Importer {
    fn build_module<'lua>(mut self, ctx: Context<'lua>) -> Result<Value<'lua>> {
        let func = ctx.create_function_mut(move |ctx, val| Ok(self.import(ctx, val)?))?;
        Ok(Value::Function(func))
    }
}

impl DefaultModule for Importer {
    const GLOBAL_VAR: &'static str = "import";

    fn import_module(ctx: Context) -> Result<()> {
        ctx.globals().set(
            Self::GLOBAL_VAR,
            Self::with_default_modules().build_module(ctx)?,
        )
    }
}
