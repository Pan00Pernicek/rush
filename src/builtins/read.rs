use std::io;
use std::env;
use shellstate::ShellState;

pub fn read(args: &Vec<String>, shell_state: &mut ShellState) -> i32 {
    let mut input = String::new();
    if args.len() > 0 {
        match io::stdin().read_line(&mut input) {
            Ok(_) => env::set_var(&args[0], input),
            Err(_) => return 1,
        }
    } else {
        match io::stdin().read_line(&mut input) {
            Ok(_) => return 0,
            Err(_) => return 1,
        }
    }
    0
}