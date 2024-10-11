use std::{
    process::Command,
    fs,
};
use std::os::unix::fs::PermissionsExt;

pub fn clear_command() {
    Command::new("clear").status().unwrap();
}

pub fn ls_command(mut command: Vec<&str>) {
    if command.len() == 0 {
        command = ["."].to_vec();
    }
    for each_command in command {
        println!("\n\x1B[33m{}:\x1B[0m", each_command);
        match fs::read_dir(each_command) {
            Ok(entries) => for entry in entries {
                let each = entry.unwrap().path();
                let metadata_each = fs::metadata(each.clone());
                let str_path = each.to_str().unwrap();
                let mut sliced_path = str_path;
                if each_command == "." {
                    sliced_path = &str_path[2..];
                }
                if metadata_each.unwrap().is_dir() {
                    let ch = sliced_path.to_string().as_bytes()[0] as char;
                    if ch == '.' {
                        continue;
                    }
                    println!("\x1B[34m{}\x1B[0m", sliced_path);
                } else {
                    let ch = sliced_path.to_string().as_bytes()[0] as char;
                    if ch == '.' {
                        continue;
                    }
                    println!("{}", sliced_path);
                }
            },
            Err(_err) => println!("rush: ls: cannot access {:?}: No such file or directory", each_command)
        }
    }
}

fn get_perm_str(mode: u32) -> String {
    let mut res = String::new();
    res.push(if mode & 0o40000 != 0 { 'd' } else { '-' });
    res.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    res.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    res.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    res.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    res.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    res.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    res.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    res.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    res.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    res
}

pub fn ls_command_perm(mut command: Vec<&str>) {
    if command.len() == 0 {
        command = ["."].to_vec();
    }
    for each_command in command {
        println!("\n\x1B[33m{}:\x1B[0m", each_command);
        match fs::read_dir(each_command) {
           Ok(entries) => for entry in entries {
               let each = entry.unwrap().path();
               let metadata_each = fs::metadata(each.clone());
               let metadata = each.metadata();
               let perm = metadata.unwrap().permissions();
               let perm_mode = perm.mode();
               let perm_str = get_perm_str(perm_mode);
               let str_path = each.to_str().unwrap();
               let mut sliced_path = str_path;
               if each_command == "." {
                   sliced_path = &str_path[2..];
               }
               if metadata_each.unwrap().is_dir() {
                   println!("\x1B[34m{} {}\x1B[0m", perm_str, sliced_path);
               } else {
                   println!("{} {}", perm_str, sliced_path);
               }
           },
           Err(_err) => println!("rush: ls: cannot access {:?}: No such file or directory", each_command)
        }
    }
}

pub fn cat_command(files: Vec<&str>) {
    for file in files {
        println!("\n\x1B[33m{}:\x1B[0m", file);
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

pub fn touch_command(files: Vec<&str>) {
    for file in files {
        fs::File::create_new(file).unwrap();
    }
}

pub fn cp_command(files: Vec<&str>) {
    fs::copy(files[0], files[1]).unwrap();
}

pub fn grep_command(command: Vec<&str>) {
    let pattern = command[0];
    let len_pattern = pattern.len();
    let files = &command[1..];
    for file in files {
        let content: String = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(_err) => {
                let metadata = fs::metadata(file);
                if metadata.unwrap().is_dir() {
                    print!("grep: {}: Is a directory", file);
                } else {
                    print!("grep: {}: No such file or directory", file);
                }
                String::new()
            }
        };
        for line in content.lines() {
            if !line.find(pattern).is_some() {
                continue;
            }
            let len_line = line.len();
            let stop_num = len_line - len_pattern + 1;
            print!("\n\x1B[33m{}: \x1B[0m", file);
            let mut i = 0;
            while i < stop_num {
                if line[i..i+len_pattern] == *pattern {
                    print!("\x1B[31m{}\x1B[0m", &line[i..i+len_pattern]);
                    i += len_pattern;
                } else {
                    let ch = line.to_string().as_bytes()[i] as char;
                    print!("{}", ch);
                    i += 1;
                }
            }
            print!("{}", &line[i..]);
        }
        println!("");
    }
}
