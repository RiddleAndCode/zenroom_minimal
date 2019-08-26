use super::Module;
use rlua::{Context, Result};

pub trait DefaultModule: Module {
    const GLOBAL_VAR: &'static str;

    fn import_module(ctx: Context) -> Result<()> {
        ctx.globals()
            .set(Self::GLOBAL_VAR, Self::build_module(ctx)?)
    }
}
