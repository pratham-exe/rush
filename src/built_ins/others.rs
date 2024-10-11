pub fn not_found_command(command: Vec<&str>) {
    println!("{}: command not found", command.join(" "));
}

pub fn tokens(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
