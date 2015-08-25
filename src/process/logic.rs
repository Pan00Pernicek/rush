#![allow(unused_imports)] //for some of the things in this file for now
#![allow(dead_code)]

use process::stdproc::{get_stdout_or_stderr, get_status};
use process::execute::run;

//false if it failed true if it didn't
fn and(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let ran1 = run(command1);
    let status = get_status(ran1.clone());
    let output1 = get_stdout_or_stderr(ran1);
    println!("{}", output1);
    if status {
        println!("{}",get_stdout_or_stderr(run(command2)));
        return true;
    }
    false
}

fn or(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let ran = run(command1);
    let status = get_status(ran.clone());
    println!("{}",get_stdout_or_stderr(ran));
    if status {
        true
    } else {
        println!("{}",get_stdout_or_stderr(run(command2)));
        false
    }
}

fn xor(command1: Vec<&str>, command2: Vec<&str>) -> bool {
    let ran1 = run(command1);
    let status1 = get_status(ran1.clone());
    let ran2 =run(command2);
    let status2 = get_status(ran2.clone());
    println!("{}",get_stdout_or_stderr(ran1));
    println!("{}",get_stdout_or_stderr(ran2));
    if !status1 && status2 || status1 && !status2 {
        return true;
    }
    false
}

// //Not Implemented
// fn nand(command1: Vec<&str>, command2: Vec<&str>) -> bool {
//     false
// }

#[cfg(test)]
mod tests{
    use super::{and,xor};

    #[test]
    fn and_test(){
        //Both pass
        let command1: Vec<&str> = "date".trim().split(' ').collect();
        let command2: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(true, and(command1,command2));

        //First Fails
        let command3: Vec<&str> = "date %d".trim().split(' ').collect();
        let command4: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(false, and(command3,command4));

        //Last Fails
        let command5: Vec<&str> = "date".trim().split(' ').collect();
        let command6: Vec<&str> = "ls /blah".trim().split(' ').collect();
        assert_eq!(true, and(command5,command6));

        //Both fail
        let command7: Vec<&str> = "ls /blah".trim().split(' ').collect();
        let command8: Vec<&str> = "date %d".trim().split(' ').collect();
        assert_eq!(false, and(command7,command8));
    }

    #[test]
    fn xor_test(){
        //Both pass
        let command1: Vec<&str> = "date".trim().split(' ').collect();
        let command2: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(false, xor(command1,command2));

        //First Fails
        let command3: Vec<&str> = "date %d".trim().split(' ').collect();
        let command4: Vec<&str> = "ls /".trim().split(' ').collect();
        assert_eq!(true, xor(command3,command4));

        //Last Fails
        let command5: Vec<&str> = "date".trim().split(' ').collect();
        let command6: Vec<&str> = "ls /blah".trim().split(' ').collect();
        assert_eq!(true, xor(command5,command6));

        //Both fail
        let command7: Vec<&str> = "ls /blah".trim().split(' ').collect();
        let command8: Vec<&str> = "date %d".trim().split(' ').collect();
        assert_eq!(false, xor(command7,command8));
    }
}
