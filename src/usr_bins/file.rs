use std::fs;

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
