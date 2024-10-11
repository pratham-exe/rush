use std::fs;

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
