use builtins::Builtin;
use builtins::get_builtins;
use interpreter::*;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

pub fn run_script(file_name: &Path, builtins: &HashMap<String, Builtin>) {
    match File::open(&file_name) {
        Ok(f) => {
            let content = BufReader::new(&f);
            for line in content.lines() {
                interpret_line(line.unwrap(), &builtins);
            }
        },
        Err(_) => {
            return;
        }
    };
}
