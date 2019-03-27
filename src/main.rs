#![allow(unused_must_use)]

extern crate rush;
extern crate rustyline;
extern crate libc;
extern crate nix;
extern crate dirs;
extern crate clap;

use rush::builtins;
use rush::prompt::Prompt;
use rush::interpreter::*;
use rush::script::*;
use rush::shellstate::ShellState;
use rush::completion::RushHelper;
use rustyline::error::ReadlineError;
use rustyline::{Config, CompletionType, Editor, Helper};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::hint::Hinter;
use rustyline::highlight::Highlighter;
use self::dirs::home_dir;
use std::process;
use std::path::Path;
use nix::sys::signal;
use nix::sys::signal::{SigAction, SigHandler, SaFlags, SigSet, sigaction};
use clap::{Arg, App};

fn main() {
    #[cfg(unix)]    {
        while nix::unistd::tcgetpgrp(0).unwrap() != nix::unistd::getpgrp() {
            nix::sys::signal::kill(nix::unistd::getpgrp(), nix::sys::signal::Signal::SIGTTIN);
        }
        let hdl = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
        unsafe {
            sigaction(signal::SIGINT, &hdl).unwrap();
            sigaction(signal::SIGQUIT, &hdl).unwrap();
            sigaction(signal::SIGTSTP, &hdl).unwrap();
            sigaction(signal::SIGTTIN, &hdl).unwrap();
            sigaction(signal::SIGTTOU, &hdl).unwrap();
            sigaction(signal::SIGTSTP, &hdl).unwrap();
        }
        let pid = nix::unistd::getpid();
        match nix::unistd::setpgid(pid, pid) {
            Ok(_) => {}
            Err(_) => println!("Couldn't set pgid"),
        };
        match nix::unistd::tcsetpgrp(0, pid) {
            Ok(_) => {}
            Err(_) => println!("Couldn't set process to foreground"),
        }
    }

    // Parse command line options
    let matches = App::new("rush")
        .version("0.0.2")
        .about("Rust Shell")
        .arg(Arg::with_name("command")
            .short("c")
            .value_name("command")
            .multiple(true)
            .help("Command(s) to parse"))
        .arg(Arg::with_name("file")
            .short("f")
            .value_name("file")
            .multiple(true)
            .help("Files to run"))
        .arg(Arg::with_name("command_file")
            .multiple(true)
            .help("Commands or files"))
        .get_matches();

    // Initialize Shell State
    let shell_state = &mut ShellState {
        prompt: Prompt::new(),
        input_buffer: "".to_owned(),
        builtins: builtins::get_builtins(),
    };

    // Run config file
    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rushrc");
    run_script(home_config.as_path(), shell_state);

    // Run script(s)
    for command_or_file in matches.value_of("command_file") {
        run_script(Path::new(&command_or_file), shell_state);
        return
    }

    // Run command(s)
    for command in matches.value_of("command") {
        interpret_line(command.to_string(), shell_state);
        return
    }

    let mut history_file = home_dir().expect("No Home directory");
    history_file.push(".rush_history");
    let history =
        history_file.as_path().to_str().expect("Should have a home directory to turn into a str");

    // Set up buffer to read inputs and History Buffer
    let input_config = Config::builder().completion_type(CompletionType::Circular).build();
    let mut input_buffer = Editor::with_config(input_config);
    input_buffer.set_helper(Some(RushHelper(FilenameCompleter::new())));
    if let Err(_) = input_buffer.load_history(history) {}
    let mut prompt = Prompt::new();

    // Loop to recieve and execute commands
    loop {
        &prompt.update_cwd();
        &prompt.update_prompt();
        let line = input_buffer.readline(&prompt.get_user_p());
        match line {
            Ok(line) => {
                input_buffer.add_history_entry(line.as_ref());
                interpret_line(line, shell_state);
            }
            Err(ReadlineError::Interrupted) => {
                print!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
    input_buffer.save_history(history).unwrap();
}
