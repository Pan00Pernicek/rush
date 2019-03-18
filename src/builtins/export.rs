use std::env;
use shellstate::ShellState;

pub fn export(args: &Vec<String>, shell_state: &mut ShellState) -> i32 {
    if args.len() > 0 {
        for arg in args {
            let parts: Vec<&str> = arg.split("=").collect();
            if parts.len() != 2 {
                println!("Malformed arugment");
                return 1;
            }
            env::set_var(parts[0], parts[1]);
        }
    } else {
        for (key, value) in env::vars() {
            println!("{}={}", key, value);
        }
    }
    0
}