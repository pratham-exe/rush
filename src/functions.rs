use std::{
    process,
    fs,
    env,
    path::Path
};

pub fn not_found_command(command: Vec<&str>) {
    println!("{}: command not found", command.join(" "));
}

pub fn clear_command(command: &str) {
    process::Command::new(command).status().unwrap();
}

pub fn tokens(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn echo_command(command: Vec<&str>) {
    println!("{}", command.join(" "));
}

pub fn type_command(command: &str) {
    if (command == "exit") || (command == "echo") || (command == "type") {
        println!("{} is a shell builtin", command);
    } else {
        println!("rush: type: {}: not found", command);
    }
}

pub fn ls_command(command: &str) {
    match fs::read_dir(command) {
        Ok(entries) => for entry in entries {
            print!("{:?}\t", entry.unwrap().path());
        },
        Err(_err) => print!("rush: ls: cannot access {:?}: No such file or directory", command)
    }
    println!("");
}

pub fn empty_ls_command() {
    match fs::read_dir(".") {
        Ok(entries) => for entry in entries {
            print!("{:?}\t", entry.unwrap().path());
        },
        Err(err) => println!("{}", err)
    }
    println!("");
}

pub fn cat_command(files: Vec<&str>) {
    for file in files {
        let content: String = fs::read_to_string(file).unwrap();
        println!("{}", content);
    }
}

pub fn mkdir_command(new_dirs: Vec<&str>) {
    for new_dir in new_dirs {
        fs::create_dir(new_dir).unwrap();
    }
}

pub fn rmdir_command(dirs: Vec<&str>) {
    for dir in dirs {
        fs::remove_dir(dir).unwrap();
    }
}

pub fn rmdirr_command(dirs: Vec<&str>) {
    for dir in dirs {
        fs::remove_dir_all(dir).unwrap();
    }
}

pub fn rm_command(files: Vec<&str>) {
    for file in files {
        fs::remove_file(file).unwrap();
    }
}

pub fn mv_command(files: Vec<&str>) {
    fs::rename(files[0], files[1]).unwrap();
}

pub fn pwd_command() {
    println!("{}", env::current_dir().unwrap().display());
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
