use anyhow::Result;
use rune::{Any, ContextError, Module};
use std::env;

#[derive(Debug, Any)]
enum Err {
    VarError(env::VarError),
}

#[rune::function]
fn var(key: String) -> Result<String, Err> {
    env::var(key).map_err(|err| Err::VarError(err))
}

#[rune::function]
fn set_var(key: String, value: String) {
    env::set_var(key, value);
}

#[rune::function]
fn current_dir() -> Option<String> {
    match env::current_dir() {
        Ok(cd) => Some(cd.to_str()?.to_owned()),
        Err(_) => None,
    }
}

pub fn create_module() -> Result<Module, ContextError> {
    let mut module = Module::with_crate_item("galdr", ["env"])?;
    module.function_meta(var)?;
    module.function_meta(set_var)?;
    module.function_meta(current_dir)?;
    Ok(module)
}
