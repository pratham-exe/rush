mod functions;

use std::io::{self, Write};

fn repeat() {
    loop {
        print!(" ");
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
            ["ls"] => functions::empty_ls_command(),
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
            _ => functions::not_found_command(command)
        }
    }
}

fn main() {
    repeat();
}
