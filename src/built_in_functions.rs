use std::{
    env,
    path::Path
};

pub fn not_found_command(command: Vec<&str>) {
    println!("{}: command not found", command.join(" "));
}

pub fn tokens(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn echo_command(command: Vec<&str>) {
    println!("{}", command.join(" "));
}

pub fn type_command(command: &str) {
    if (command == "exit") || (command == "echo") || (command == "type") || (command == "pwd") || (command == "cd") {
        println!("{} is a shell builtin", command);
    } else {
        println!("rush: type: {}: not found", command);
    }
}

pub fn pwd_command() {
    println!("{}", env::current_dir().unwrap().display());
}

pub fn display_colored_pwd() {
    let home_dir = env::var("HOME").unwrap();
    let len_home = home_dir.len();
    let present_dir = env::current_dir().unwrap();
    let str_present_dir = present_dir.to_str().unwrap();
    if home_dir == present_dir.to_str().unwrap() {
        println!("\x1B[36m ~\x1B[0m");
    } else {
        if str_present_dir.starts_with(&home_dir) {
            let replaced_present_dir = &str_present_dir[len_home..];
            println!("\x1B[36m ~{}\x1B[0m", replaced_present_dir);
        } else {
            println!("\x1B[36m{}\x1B[0m", present_dir.display());
        }
    }
}

pub fn cd_command(dir: &str) {
    if dir == "~" {
        let home = env::var("HOME").unwrap();
        let home_dir = Path::new(&home);
        if env::set_current_dir(&home_dir).is_err() {
            println!("rush: cd: {:?}: No such file or directory", dir);
        }
    } else {
        let new_dir = Path::new(dir);
        if env::set_current_dir(&new_dir).is_err() {
            println!("rush: cd: {:?}: No such file or directory", dir);
        }
    }
}
