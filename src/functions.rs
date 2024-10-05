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
            let each = entry.unwrap().path();
            let metadata_each = fs::metadata(each.clone());
            let str_path = each.to_str().unwrap();
            let sliced_path = &str_path[2..];
            if metadata_each.unwrap().is_dir() {
                println!("\x1B[34m{}\x1B[0m", sliced_path);
            } else {
                println!("{}", sliced_path);
            }
        },
        Err(_err) => print!("rush: ls: cannot access {:?}: No such file or directory", command)
    }
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

pub fn touch_command(files: Vec<&str>) {
    for file in files {
        fs::File::create_new(file).unwrap();
    }
}

pub fn cp_command(files: Vec<&str>) {
    fs::copy(files[0], files[1]).unwrap();
}
