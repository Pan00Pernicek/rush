#![allow(unused_imports)] //Here until interpret is complete
extern crate libc;
extern crate nix;

use std::process::*;
use process::logic::*;
use process::stdproc::*;
use process::unix::pipe::*;
use process::ops::*;
use process::pq::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use std::os::unix::process::CommandExt;

///Interpret
///Given an input command, interpret parses and determines what and how
///to execute it and returns output or error output

///Run
///Runs commands passed to it and returns the output
pub fn run(command: Vec<&str>) -> bool {
    let args = command.as_slice();
    if args.len() > 1 {
        match Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                let child_pgid = child.id() as i32;
                nix::unistd::tcsetpgrp(0, child_pgid);
                match child.wait() {
                    Ok(status) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        status.success()
                    },
                    Err(_) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    } else if args.len() == 1 {
        match Command::new(&args[0])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                let child_pgid = child.id() as i32;
                nix::unistd::tcsetpgrp(0, child_pgid);
                match child.wait() {
                    Ok(status) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        status.success()
                    },
                    Err(_) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    } else {
        match Command::new("")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .spawn() {
            Ok(mut child) => {
                let child_pgid = child.id() as i32;
                nix::unistd::tcsetpgrp(0, child_pgid);
                match child.wait() {
                    Ok(status) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        status.success()
                    },
                    Err(_) => {
                        nix::unistd::tcsetpgrp(0, nix::unistd::getpid());
                        println!("failed to wait for child");
                        false
                    },
                }
            },
            Err(_) => {
                println!("Failed to execute");
                false
            },
        }
    }
}

pub fn redirect(command: Vec<&str>) -> bool {
    let mut args = command;
    let mut file_path = "".to_owned();
    for i in 0..args.len() {
        if args[i].contains('>') {
            file_path.push_str(&args[i + 1..args.len()].to_vec().join(""));
            args.truncate(i);
            break;
        }
    }
    let args = args.as_slice();
    let output = if args.len() > 1 {
        Command::new(&args[0])
            .args(&args[1..])
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .output()
            .ok()
    } else if args.len() == 1 {
        Command::new(&args[0])
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .output()
            .ok()
    } else {
        Command::new("")
            .before_exec(move || {
                let pid = nix::unistd::getpid();
                nix::unistd::setpgid(pid, pid);
                unsafe {
                    libc::signal(libc::SIGINT, libc::SIG_DFL);
                    libc::signal(libc::SIGQUIT, libc::SIG_DFL);
                    libc::signal(libc::SIGTSTP, libc::SIG_DFL);
                    libc::signal(libc::SIGTTIN, libc::SIG_DFL);
                    libc::signal(libc::SIGTTOU, libc::SIG_DFL);
                }
                Result::Ok(())
            })
            .output()
            .ok()
    };
    let str_out = if output.is_some() {
        let temp = output.expect("Output has been checked");
        if temp.stdout.is_empty() {
            String::from_utf8(temp.stderr)
                .expect("Should have translated to string easily")
        } else {
            String::from_utf8(temp.stdout)
                .expect("Should have translated to string easily")
        }
    } else {
        "".to_owned()
    };
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(str_out.as_bytes()) {
        panic!("couldn't write to {}: {}", display, why.description());
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
}