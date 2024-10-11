use std::fs;

pub fn cat_command(files: Vec<&str>) {
    for file in files {
        println!("\n\x1B[33m{}:\x1B[0m", file);
        let content: String = fs::read_to_string(file).unwrap();
        println!("{}", content);
    }
}
