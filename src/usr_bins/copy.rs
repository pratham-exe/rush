use std::fs;

pub fn cp_command(files: Vec<&str>) {
    fs::copy(files[0], files[1]).unwrap();
}
