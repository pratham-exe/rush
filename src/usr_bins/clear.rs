use std::process::Command;

pub fn clear_command() {
    Command::new("clear").status().unwrap();
}
