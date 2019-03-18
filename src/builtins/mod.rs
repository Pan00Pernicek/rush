use std::collections::HashMap;
use shellstate::ShellState;

mod cd;
mod export;
mod source;
mod exit;
mod read;
mod alias;

pub type Builtin = fn(&Vec<String>, &mut ShellState) -> i32;

pub fn get_builtins() -> HashMap<String, Builtin> {
    let mut builtins = HashMap::new();
    builtins.insert("exit".to_string(), exit::exit as Builtin);
    builtins.insert("cd".to_string(), cd::change_directory as Builtin);
    builtins.insert("export".to_string(), export::export as Builtin);
    builtins.insert("source".to_string(), source::source as Builtin);
    builtins.insert("read".to_string(), read::read as Builtin);
    builtins.insert("alias".to_string(), alias::alias as Builtin);
    builtins
}