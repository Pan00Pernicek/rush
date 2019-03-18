use builtins::Builtin;
use interpreter::*;
use shellstate::ShellState;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

pub fn run_script(file_name: &Path, shell_state: &mut ShellState) {
    match File::open(&file_name) {
        Ok(f) => {
            let content = BufReader::new(&f);
            for line in content.lines() {
                interpret_line(line.unwrap(), shell_state);
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
}
