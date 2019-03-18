use interpreter::*;
use builtins::get_builtins;
use shellstate::ShellState;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn source(args: &Vec<String>, shell_state: &mut ShellState) -> i32 {
    let f = match File::open(&args[0]) {
        Ok(f) => f,
        Err(_) => {
            println!("Couldn't open file {}", args[0]);
            return 1;
        }
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        interpret_line(l, shell_state);
    };
    0
}