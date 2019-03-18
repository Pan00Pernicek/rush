extern crate dirs;

use shellstate::ShellState;
use std::path::{Path, PathBuf};
use std::env;
use self::dirs::home_dir;
//use std::fs::PathExt; //Use of exists() is considered unstable. Might break in the future


///Change Directory
///Function used to internally change the directory of the shell
pub fn change_directory(input: &Vec<String>, shell_state: &mut ShellState) -> i32 {
    if input.is_empty() {
        env::set_current_dir(Path::new(&home_dir().expect("No home directory"))).expect("Failed to set current directory");
    } else {
        let mut buffer = PathBuf::new();
        for i in input {
            if i.contains('~') {
                let i_split = i.split('~').next();
                buffer.push(Path::new(&home_dir().expect("No home directory")));
                if i_split.is_some() {
                    buffer.push(Path::new(i_split.expect("Failed to split")));
                }
            } else {
                buffer.push(Path::new(i));
            }
        }
        let dir = buffer.as_path();
        if dir.is_relative() {
            let mut temp = PathBuf::new();
            temp.push(env::current_dir().expect("Couldn't get current directory"));
            temp.push(dir);
            let path = temp.as_path();
            if path.exists() {
                env::set_current_dir(temp.as_path())
                    .expect("Failed to set current directory");
            } else {
                println!("Invalid path or input");
            }
        } else if dir.exists() {
            env::set_current_dir(dir)
                .expect("Failed to set current directory");
        } else {
            println!("Invalid path or input");
            return 1;
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use super::*;

    #[test]
    #[should_panic]
    fn test_change_directory_ok() {
        let vec = vec!["/".to_owned()];
        let dir = Path::new("/tmp").to_str().to_owned();
        change_directory(&vec);
        let new_dir = env::current_dir()
            .expect("Failed to get current directory");
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir);
    }

    #[test]
    fn test_change_directory_fail() {
        let vec = vec!["/".to_owned()];
        #[cfg(unix)]
        let dir = Path::new("/").to_str().to_owned();
        #[cfg(windows)]
        let dir = Path::new("C:\\").to_str().to_owned();
        change_directory(&vec);
        let new_dir = env::current_dir()
            .expect("Failed to get current directory");
        let new_dir = new_dir.to_str().to_owned();
        assert_eq!(dir, new_dir);
    }
}

