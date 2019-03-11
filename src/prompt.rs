extern crate dirs;

use std::env;
use std::env::var;
use std::env::current_dir;
use self::dirs::home_dir;
use std::io::{stdout, Write};
use std::process::Command;

///Prompt
///Struct containing prompt and cwd for use on every new line
///in Rush
#[derive(Default)]
pub struct Prompt {
    user_p: String,
    cwd: String,
}

impl Prompt {
    ///Instantiates a new Prompt with default values
    ///that will be overwritten when the configuration is updated
    ///in the main file for execution
    pub fn new() -> Prompt {
        let mut object = Prompt {
            user_p: "user@localhost %".to_owned(),
            cwd: "~/".to_owned(),
        };
        object.update_cwd();
        object.update_prompt();
        object
    }

    ///Update Prompt
    ///Calls method in rush::config to update the current prompt
    ///Only needs to be called if using cd or su at this point
    ///in time
    pub fn update_prompt(&mut self) {
        let left_value = match env::var("RUSH_PROMPT") {
            Ok(prompt) => prompt,
            Err(_) => "R$ ".to_owned(),
        };
        
        let mut left = left_value.split('%');
        let mut prompt = left.next().unwrap().to_string();
        for i in left {
            if !i.is_empty() {
                let escape: Result<String, ()> = self.interpret_escapes(i.chars().next().unwrap());
                match escape {
                    Ok(e) => prompt.push_str(&e),
                    Err(_) => prompt.push_str("Failed to parse escape"),
                }
                //Add non Prompt special chars to prompt
                prompt.push_str(&i[1..]);
            }
        }
        prompt.push(' ');
        
        self.user_p = prompt;
    }

    ///Get User P
    ///Returns prompt to be displayed on the command line
    pub fn get_user_p(&self) -> String {
        self.user_p.to_owned()
    }

    ///Get CWD
    ///Returns the CWD for use in prompts
    pub fn get_cwd(&self) -> String {
        self.cwd.to_owned()
    }

    pub fn get_cwn_abs(&self) -> String {
        let buff = current_dir().expect("No current directory");
        buff.to_str().expect("Failed to become a str").to_owned()
    }

    fn interpret_escapes(&self, escape: char) -> Result<String, ()> {
        match escape {
            'U' if cfg!(windows) => Ok(var("USERNAME").expect("$USERNAME not set")),
            'U' if cfg!(unix)    => Ok(var("USER").expect("$USER not set")),
            'H' if cfg!(windows) => Ok(var("USERDOMAIN").expect("$USERDOMAIN not set")),
            'H' if cfg!(unix)    =>
                Ok(String::from_utf8(Command::new("uname")
                .arg("-n").output()
                .expect("No uname command").stdout)
                .expect("Failed to convert to string")
                .trim().to_string()),

            'L' => Ok(self.get_cwd()),
            'R' => Ok("$".to_string()),
            _ => Err(()),
        }
    }
    
    ///Update CWD
    ///Used to update the CWD if using CD
    pub fn update_cwd(&mut self) {
        let buff = current_dir().expect("No current directory");

        //Makes cwd ~/ if in home directory of user otherwise
        //just the current directory
        if buff.starts_with(home_dir().expect("No Home directory").as_path()) {
            let mut home = "~/".to_owned();
            home.push_str(buff.as_path().strip_prefix(home_dir()
                .expect("No Home directory")
                .as_path()
            )
                .expect("Couldn't get relative path")
                .to_str().expect("Failed to become a str"));
            self.cwd = home;
        } else {
            self.cwd = buff.as_path()
                .to_str().expect("Failed to turn path into str").to_owned();
        }
    }

    pub fn print(&mut self) {
        self.update_cwd();
        self.update_prompt();
        print!("{}", self.get_user_p());
        stdout().flush().expect("Could not flush stdout");
    }
}
