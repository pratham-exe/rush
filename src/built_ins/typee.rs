pub fn type_command(command: Vec<&str>) {
    for each_command in command {
        if (each_command == "exit") || (each_command == "echo") || (each_command == "type") || (each_command == "pwd") || (each_command == "cd") {
            println!("{} is a shell builtin", each_command);
        } else {
            println!("rush: type: {}: not found", each_command);
        }
    }
}
