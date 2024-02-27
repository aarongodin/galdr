use anyhow::Result;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, ContextError, Diagnostics, Module, Source, Sources, Vm};
use std::env as stdenv;
use std::sync::Arc;

mod env;
mod fetch;

pub fn create_module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate("galdr")?;
    module.function_meta(fetch::fetch)?;
    Ok(module)
}

fn main() -> Result<()> {
    let args: Vec<String> = stdenv::args().collect();

    if args.len() > 1 {
        let script_filename = &args[1];

        let mut context = Context::with_default_modules()?;
        context.install(create_module()?)?;
        context.install(env::create_module()?)?;
        let runtime = Arc::new(context.runtime()?);

        let mut sources = Sources::new();

        sources.insert(Source::from_path(script_filename)?)?;

        let mut diagnostics = Diagnostics::new();

        let prepared = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources)?;
        }

        let mut vm = Vm::new(runtime, Arc::new(prepared?));
        vm.call(["main"], ())?;
    } else {
        println!("Error: Please provide a script filename.");
    }

    return Ok(());
}
