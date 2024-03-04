mod ulidgen;
mod sfgen;
mod cmd_handler;

use std::env;

fn main() {
    let command = cmd_handler::parse_cmd(env::args_os().collect());

    if command.generator.len() == 0 {
        println!("No generator specified");
        return;
    }

    if command.option == "time" && command.value.len() == 0 {
        println!("No value specified for arg -c for {} option", command.option);
        return;
    }

    let output = cmd_handler::handle_cmd(command);
    println!("{}", output);
}
