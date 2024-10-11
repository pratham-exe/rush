mod built_ins;
mod usr_bins;

use std::io::{self, Write};

fn repeat() {
    built_ins::cd::cd_command([].to_vec());
    loop {
        println!("");
        built_ins::pwd::display_colored_pwd();
        print!("\x1B[32m--> \x1B[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_dup = input.trim();
        let command = built_ins::others::tokens(input_dup);
        let param_command: Vec<&str> = command[1..].to_vec();
        match command[..] {
            ["exit"] => break,
            [] => continue,
            ["echo", ..] => built_ins::echo::echo_command(param_command),
            ["type", ..] => built_ins::typee::type_command(param_command),
            ["pwd"] => built_ins::pwd::pwd_command(),
            ["cd", ..] => built_ins::cd::cd_command(param_command),
            ["clear"] => usr_bins::clear::clear_command(),
            ["ls", ..] => usr_bins::ls::ls_command(param_command),
            ["cat", ..] => usr_bins::cat::cat_command(param_command),
            ["mkdir", ..] => usr_bins::dir::mkdir_command(param_command),
            ["rmdir", ..] => usr_bins::dir::rmdir_command(param_command),
            ["rmdirr", ..] => usr_bins::dir::rmdirr_command(param_command),
            ["rm", ..] => usr_bins::file::rm_command(param_command),
            ["mv", ..] => usr_bins::file::mv_command(param_command),
            ["touch", ..] => usr_bins::file::touch_command(param_command),
            ["cp", ..] => usr_bins::copy::cp_command(param_command),
            ["grep", ..] => usr_bins::grep::grep_command(param_command),
            _ => built_ins::others::not_found_command(command)
        }
    }
}

fn main() {
    repeat();
}
