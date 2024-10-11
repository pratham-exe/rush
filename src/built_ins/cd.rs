use std::{
    env,
    path::Path
};

pub fn cd_command(command: Vec<&str>) {
    let mut dir = String::new();
    if command.len() == 0 {
        dir = "~".to_string();
    } else if command.len() > 1 {
        println!("rush: cd: too many arguments");
    } else {
        dir = command[0].to_string();
    }
    if dir == "~" {
        let home = env::var("HOME").unwrap();
        let home_dir = Path::new(&home);
        if env::set_current_dir(&home_dir).is_err() {
            println!("rush: cd: {:?}: No such file or directory", dir);
        }
    } else {
        let new_dir = Path::new(&dir);
        if env::set_current_dir(&new_dir).is_err() {
            println!("rush: cd: {:?}: No such file or directory", dir);
        }
    }
}
