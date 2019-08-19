use rlua::{Context, Result, Table};

pub trait Module {
    fn build_module(ctx: Context) -> Result<Table>;
    fn module_identifier() -> &'static str;

    fn load_module(ctx: Context) -> Result<()> {
        ctx.globals().set(Self::module_identifier(), Self::build_module(ctx)?)
    }
}
