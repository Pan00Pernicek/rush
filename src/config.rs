extern crate toml;
extern crate dirs;

use config::toml::Table;
use std::io::{Read, BufReader};
use std::fs::File;
use std::env::{set_var, var};
use self::dirs::home_dir;
use std::process::Command;
use prompt::Prompt;


///Read in Config
///Inner function used to pull in a default configuration file for parsing
///or the customized one if it exists
fn read_in_config() -> Option<String> {
    //Find a way to read from default if this doesn't work. let a = if else?
    let mut home_config = home_dir().expect("No Home directory");
    home_config.push(".rush.toml");
    let default = File::open(home_config.as_path().to_str()
        .expect("Should have a home directory to
                                     turn into a str"));
    let config = if default.is_err() {
        //Should be changed to location of git repo if compiling on your own machine
        return None;
    } else {
        default.expect("No files to open for config")
    };
    let mut reader = BufReader::new(&config);
    let mut buffer_string = String::new();
    reader.read_to_string(&mut buffer_string)
        .expect("Failed to read in config");
    Some(buffer_string)
}

///Read Config Prompt
///Used to read the options of the config file and parse
///the defined options to create a customized prompt
pub fn read_config_prompt(input: &Prompt) -> String {
    let buffer_string = read_in_config();

    let config: Option<toml::Value> = buffer_string
        .map(|b| b.parse().expect("Should have a config file"));
    let left_value = config.as_ref().and_then(|config| config.lookup("prompt.left"));
    let default_prompt = format!("rush-{}$", env!("CARGO_PKG_VERSION"));
    let mut left = left_value
        .map_or(&default_prompt as &str, |left_value| left_value.as_str().unwrap()).split('%');
    let mut prompt = left.next().unwrap().to_string();
    for i in left {
        if !i.is_empty() {
            let escape: Result<String, ()> = interpret_escapes(i.chars().next().unwrap(), input);
            match escape {
                Ok(e) => prompt.push_str(&e),
                Err(_) => prompt.push_str("Failed to parse escape"),
            }
            //Add non Prompt special chars to prompt
            prompt.push_str(&i[1..]);
        }
    }
    prompt.push(' ');
    prompt
}

fn interpret_escapes(escape: char, input: &Prompt) -> Result<String, ()> {
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

        'L' => Ok(input.get_cwd()),
        'R' => Ok("$".to_string()),
        _ => Err(()),
    }
}

///Check Alias
///Checks if there is an alias available before passing
///on commands for execution
pub fn check_alias(input: String) -> Option<String> {
    //Checks if alias is in config file and returns the altered
    //version as an Option of the input. If succesfully found
    //it can be unwraped for execution
    let input = input.split_whitespace().collect::<Vec<&str>>();
    //Makes sure there is something to execute
    if input.is_empty() {
        return None;
    }

    //Sets the alias to check for
    let alias_key = input.get(0).expect("Unwrapped an empty vector");

    //Check the config file for the key
    let config = read_in_config();
    let parsed: Option<Table> = config.map(|config| toml::Parser::new(&config)
        .parse().expect("Failed to parse config"));
    let alias_table = parsed.as_ref().and_then(|parsed| parsed.get("alias"));
    let alias = alias_table.and_then(|alias_table|alias_table.lookup(alias_key));

    //Checks if alias is in config file
    if let Some(alias) = alias {
        toml::decode(alias
            .to_owned()).expect("Failed to decode value")
    } else {
        None
    }
}

