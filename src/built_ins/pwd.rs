use std::env;

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

