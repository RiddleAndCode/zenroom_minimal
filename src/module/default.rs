use super::Module;
use rlua::{Context, Result};

pub trait DefaultModule: Module + Default {
    const GLOBAL_VAR: &'static str;

    fn import_module<'lua>(ctx: Context<'lua>) -> Result<()> {
        ctx.globals()
            .set(Self::GLOBAL_VAR, Self::default().build_module(ctx)?)
    }
}
