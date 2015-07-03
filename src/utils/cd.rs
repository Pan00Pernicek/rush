use std::path::Path;
use std::env;
//as_slice() is considered unstable right now
pub fn change_directory(directory: Vec<&str>){
    let dir_vec = directory.as_slice();
    let dir = Path::new("/");
    if dir.is_relative(){
        println!("dir is relative");
    } else {
        println!("dir is absolute");
    }
    env::set_current_dir(dir).unwrap();
}



#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::env;
    use super::*;
    
    #[test]
    #[should_panic]
    fn test_change_directory_ok(){
        let dir = Path::new("/tmp").to_str();
        change_directory("/");
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

    #[test]
    fn test_change_directory_fail(){
        let dir = Path::new("/").to_str();
        change_directory("/");
        let new_dir = env::current_dir().unwrap();
        let new_dir = new_dir.to_str();
        assert_eq!(dir, new_dir); 
    }

}

