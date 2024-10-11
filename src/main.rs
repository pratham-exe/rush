mod built_in_functions;
mod usr_bin_functions;

use std::io::{self, Write};

fn repeat() {
    built_in_functions::cd_command("~");
    loop {
        println!("");
        built_in_functions::display_colored_pwd();
        print!("\x1B[32m--> \x1B[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_dup = input.trim();
        let command = built_in_functions::tokens(input_dup);
        match command[..] {
            ["exit"] => break,
            [] => continue,
            ["echo", ..] => built_in_functions::echo_command(command[1..].to_vec()),
            ["type", ..] => built_in_functions::type_command(command[1]),
            ["pwd"] => built_in_functions::pwd_command(),
            ["cd"] => built_in_functions::cd_command("~"),
            ["cd", ..] => built_in_functions::cd_command(command[1]),
            ["clear"] => usr_bin_functions::clear_command(),
            ["ls", "-l", ..] => usr_bin_functions::ls_command_perm(command[2..].to_vec()),
            ["ls", ..] => usr_bin_functions::ls_command(command[1..].to_vec()),
            ["cat", ..] => usr_bin_functions::cat_command(command[1..].to_vec()),
            ["mkdir", ..] => usr_bin_functions::mkdir_command(command[1..].to_vec()),
            ["rmdir", ..] => usr_bin_functions::rmdir_command(command[1..].to_vec()),
            ["rmdirr", ..] => usr_bin_functions::rmdirr_command(command[1..].to_vec()),
            ["rm", ..] => usr_bin_functions::rm_command(command[1..].to_vec()),
            ["mv", ..] => usr_bin_functions::mv_command(command[1..].to_vec()),
            ["touch", ..] => usr_bin_functions::touch_command(command[1..].to_vec()),
            ["cp", ..] => usr_bin_functions::cp_command(command[1..].to_vec()),
            ["grep", ..] => usr_bin_functions::grep_command(command[1..].to_vec()),
            _ => built_in_functions::not_found_command(command)
        }
    }
}

fn main() {
    repeat();
}
