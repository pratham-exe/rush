use std::{
    process::Command,
    fs,
    path::Path
};


pub fn clear_command() {
    let _clear_output = Command::new("/usr/bin/clear").status();
}

pub fn ls_command(mut command: Vec<&str>) {
    if command.len() == 0 {
        command = ["."].to_vec();
    }
    for each_command in command.clone() {
        let each_dir = Path::new(each_command).canonicalize().expect("Failed to get path");
        let ls_output = Command::new("/usr/bin/ls").arg(&each_dir).output().expect("Failed to execute ls");
        println!("\n\x1B[33m{}:\x1B[0m\n", each_command);
        if ls_output.status.success() {
            let ls_output_stdout = String::from_utf8_lossy(&ls_output.stdout);
            for each in ls_output_stdout.lines() {
                if each.is_empty() {
                    continue;
                }
                let full_path = each_dir.join(each);
                let metadata_each = fs::metadata(full_path);
                if metadata_each.unwrap().is_dir() {
                    println!("\x1B[34m{}\x1B[0m", each);
                } else {
                    println!("{}", each);
                }
            }
        }
    }
}

pub fn ls_command_perm(mut command: Vec<&str>) {
    if command.len() == 0 {
        command = ["."].to_vec();
    }
    for each_command in command.clone() {
        let each_dir = Path::new(each_command).canonicalize().expect("Failed to get path");
        let ls_output = Command::new("/usr/bin/ls").arg("-l").arg(&each_dir).output().expect("Failed to execute ls -l");
        println!("\n\x1B[33m{}:\x1B[0m\n", each_command);
        if ls_output.status.success() {
            let ls_output_stdout = String::from_utf8_lossy(&ls_output.stdout);
            for each in ls_output_stdout.lines() {
                if each.is_empty() {
                    continue;
                }
                let diff_parts : Vec<&str> = each.split_whitespace().collect();
                if diff_parts.len() < 9 {
                    continue;
                }
                let full_path = each_dir.join(diff_parts[8]);
                let metadata_each = fs::metadata(full_path);
                if metadata_each.unwrap().is_dir() {
                    println!("\x1B[34m{}\x1B[0m", each);
                } else {
                    println!("{}", each);
                }
            }
        }
    }
}

pub fn cat_command(files: Vec<&str>) {
    for file in files {
        println!("\n\x1B[33m{}:\x1B[0m\n", file);
        let cat_output = Command::new("/usr/bin/cat").arg(file).output().expect("Failed to execute cat");
        if cat_output.status.success() {
            let cat_output_stdout = String::from_utf8_lossy(&cat_output.stdout);
            print!("{}", cat_output_stdout);
        }
    }
}

pub fn mkdir_command(new_dirs: Vec<&str>) {
    for new_dir in new_dirs {
        let _mkdir_output = Command::new("/usr/bin/mkdir").arg(new_dir).status();
    }
}

pub fn rmdir_command(dirs: Vec<&str>) {
    for dir in dirs {
        let _rmdir_output = Command::new("/usr/bin/rmdir").arg(dir).status();
    }
}

pub fn rm_command(files: Vec<&str>) {
    for file in files {
        let _rm_output = Command::new("/usr/bin/rm").arg(file).status();
    }
}

pub fn mv_command(files: Vec<&str>) {
    let _mv_output = Command::new("/usr/bin/mv").arg(files[0]).arg(files[1]).status();
}

pub fn touch_command(files: Vec<&str>) {
    for file in files {
        let _touch_output = Command::new("/usr/bin/touch").arg(file).status();
    }
}

pub fn cp_command(files: Vec<&str>) {
    let _cp_output = Command::new("/usr/bin/cp").arg(files[0]).arg(files[1]).status();
}

pub fn grep_command(command: Vec<&str>) {
    for file in &command[1..] {
        println!("\n\x1B[33m{}:\x1B[0m\n", file);
        let _grep_output = Command::new("/usr/bin/grep").arg(command[0]).arg(file).status();
    }
}
