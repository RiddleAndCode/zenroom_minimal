extern crate rlua;
extern crate zenroom_minimal;

use rlua::Result;
use zenroom_minimal::{
    prelude::*, util::read_file, FileScenarioLinker, Importer, ScenarioLoader, ZencodeRuntime,
};

fn main() -> Result<()> {
    let loader = ScenarioLoader::new(FileScenarioLinker::new("lua/examples/scenarios"));
    let mut runtime = ZencodeRuntime::new(loader, Importer::with_default_modules());
    let zencode = read_file("lua/examples/helloworld.zencode")?;
    let result = runtime.load(&zencode)?.eval()?;
    println!("{:?}", result);
    Ok(())
}
