use super::Module;
use rlua::{Context, Result};

/// Allows the [`Module`] to be imported globally on setup automatically.
/// Really just a convenience module for Setting up Runtime environments
pub trait DefaultModule: Module + Default {
    /// The global variable name to which the [`Module`] should be imported to
    const GLOBAL_VAR: &'static str;

    /// Imports the [`Module`] into the Lua VM's global scope
    fn import_module(ctx: Context) -> Result<()> {
        ctx.globals()
            .set(Self::GLOBAL_VAR, Self::default().build_module(ctx)?)
    }
}
