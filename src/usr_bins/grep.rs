use std::fs;

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
