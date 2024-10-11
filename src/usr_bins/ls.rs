use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn ls_command(mut command: Vec<&str>) {
    if command.len() == 0 {
        command = ["."].to_vec();
        ls_command_normal(command, '-', String::new())
    } else {
        let flags = command[0];
        let check_flags = flags.to_string().as_bytes()[0] as char;
        if check_flags == '-' && flags.find('l').is_some() {
            ls_command_perm(command[1..].to_vec(), check_flags, flags.to_string());
        } else if check_flags == '-' && flags.find('a').is_some() {
            let mut new_command = ["."].to_vec();
            if command.len() > 2 {
                new_command = command[1..].to_vec();
            }
            ls_command_normal(new_command, '-', flags.to_string())
        } else {
            ls_command_normal(command[0..].to_vec(), '-', String::new())
        }
    }
}

fn ls_command_normal(command: Vec<&str>, check_flags: char, flags: String) {
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
                    if check_flags == '-' && !flags.find('a').is_some() && ch == '.' {
                        continue;
                    }
                    println!("\x1B[34m{}\x1B[0m", sliced_path);
                } else {
                    let ch = sliced_path.to_string().as_bytes()[0] as char;
                    if check_flags == '-' && !flags.find('a').is_some() && ch == '.' {
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

fn ls_command_perm(mut command: Vec<&str>, check_flags: char, flags: String) {
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
                   let ch = sliced_path.to_string().as_bytes()[0] as char;
                   if check_flags == '-' && !flags.find('a').is_some() && ch == '.' {
                       continue;
                   }
                   println!("\x1B[34m{} {}\x1B[0m", perm_str, sliced_path);
               } else {
                   let ch = sliced_path.to_string().as_bytes()[0] as char;
                   if check_flags == '-' && !flags.find('a').is_some() && ch == '.' {
                       continue;
                   }
                   println!("{} {}", perm_str, sliced_path);
               }
           },
           Err(_err) => println!("rush: ls: cannot access {:?}: No such file or directory", each_command)
        }
    }
}
