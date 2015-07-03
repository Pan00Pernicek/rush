#![allow(unused_imports)]
extern crate rusty;

use rusty::utils::*;
use rusty::core::*;
use std::path::Path;
use std::io::stdin;
use std::env;

fn main() {
    //Initialization occurs here 
   
    //Loop to recieve and execute commands
    loop{
        
        let mut command = String::new();
        stdin().read_line(&mut command)
            .ok()
            .expect("Failure to read input");
        
        //Determines what to do with user input
        //problem _ might need to be used to execute command
        let mut command_split: Vec<&str> = command.trim().split(' ').collect(); 
        match command_split.get(0) {
            Some(&"cd")=> {
                command_split.remove(0); 
                cd::change_directory(command_split);
            }
            Some(&"")  => continue,
            Some(&"exit") => break,
            _ => {
                let output = execute::get_stdout(command_split);
                println!("{}",output);
            }
        }
    }

}
