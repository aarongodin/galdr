use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Context, Diagnostics, Source, Sources, Vm};
use std::env;
use std::sync::Arc;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let script_filename = &args[1];

        let context = Context::with_default_modules().expect("creating rune context");
        let runtime = Arc::new(context.runtime().expect("creating runtime memory"));

        let mut sources = Sources::new();
        
        sources.insert(Source::from_path(script_filename).expect("TODO panic")).expect("TODO: panic message");

        let mut diagnostics = Diagnostics::new();

        let prepared = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources).expect("write to diagnostics");
        }

        let unit = prepared.expect("preparing runtime");
        let mut vm = Vm::new(runtime, Arc::new(unit));

        vm.call(["main"], ()).expect("calling main function");
    } else {
        println!("Error: Please provide a script filename.");
    }

    return Ok(())
}
