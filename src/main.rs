mod functions;

use std::io::{self, Write};

fn repeat() {
    functions::cd_command("~");
    loop {
        println!("");
        functions::display_colored_pwd();
        print!("\x1B[32m--> \x1B[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_dup = input.trim();
        let command = functions::tokens(input_dup);
        match command[..] {
            ["exit"] => break,
            [] => continue,
            ["clear"] => functions::clear_command(command[0]),
            ["echo", ..] => functions::echo_command(command[1..].to_vec()),
            ["type", ..] => functions::type_command(command[1]),
            ["ls"] => functions::ls_command("."),
            ["ls", ..] => functions::ls_command(command[1]),
            ["cat", ..] => functions::cat_command(command[1..].to_vec()),
            ["mkdir", ..] => functions::mkdir_command(command[1..].to_vec()),
            ["rmdir", ..] => functions::rmdir_command(command[1..].to_vec()),
            ["rmdirr", ..] => functions::rmdirr_command(command[1..].to_vec()),
            ["rm", ..] => functions::rm_command(command[1..].to_vec()),
            ["mv", ..] => functions::mv_command(command[1..].to_vec()),
            ["pwd"] => functions::pwd_command(),
            ["cd"] => functions::cd_command("~"),
            ["cd", ..] => functions::cd_command(command[1]),
            ["touch", ..] => functions::touch_command(command[1..].to_vec()),
            ["cp", ..] => functions::cp_command(command[1..].to_vec()),
            _ => functions::not_found_command(command)
        }
    }
}

fn main() {
    repeat();
}
