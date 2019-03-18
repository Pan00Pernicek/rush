use interpreter::*;
use builtins::get_builtins;
use shellstate::ShellState;
use script::run_script;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;

pub fn source(args: &Vec<String>, shell_state: &mut ShellState) -> i32 {
    for arg in args {
        run_script(Path::new(&arg), shell_state);
    }
    0
}