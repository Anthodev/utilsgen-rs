mod ulidgen;

use std::env;
use std::ffi::OsString;

struct Command {
    generator: String,
    option: String,
    value: String,
}

fn main() {
    let command = parse_cmd(env::args_os().collect());

    if command.generator.len() == 0 {
        println!("No generator specified");
        return;
    }

    if command.option == "time" && command.value.len() == 0 {
        println!("No value specified for arg -c for {} option", command.option);
        return;
    }

    let output = handle_cmd(command);
    println!("{}", output);
}

fn parse_cmd(args: Vec<OsString>) -> Command {
    let mut generator = String::new();
    let mut option = String::new();
    let mut value = String::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.to_str() {
            Some("-t") => {
                generator = iter.next().unwrap().to_str().unwrap().to_string();
            }
            Some("-c") => {
                option = iter.next().unwrap().to_str().unwrap().to_string();

                if option == "time" {
                    let iter_value = iter.next();

                    if iter_value.is_none() {
                        return Command { generator, option, value };
                    }

                    value = iter_value.unwrap().to_str().unwrap().to_string();
                }
            }
            _ => {}
        }
    }

    Command { generator, option, value }
}

fn handle_cmd(cmd: Command) -> String {
    if cmd.option.len() > 0 {
        return match cmd.option.as_str() {
            "uuid" => {
                ulidgen::convert_ulid_to_uuid()
            }
            "time" => {
                ulidgen::convert_ulid_to_datetime_atom(cmd.value)
            }
            _ => {
                "Unknown option".to_string()
            }
        };
    }

    return match cmd.generator.as_str() {
        "ulid" => {
            ulidgen::gen_ulid()
        }
        _ => {
            "Unknown generator".to_string()
        }
    }
}
