use std::ffi::OsString;
use crate::{ulidgen, sfgen, uuidgen};

pub struct Command {
    pub generator: String,
    pub option: String,
    pub value: String,
}

pub fn parse_cmd(args: Vec<OsString>) -> Command {
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

pub fn handle_cmd(cmd: Command) -> String {
    if cmd.option.len() > 0 {
        if cmd.generator == "ulid" {
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
    }

    return match cmd.generator.as_str() {
        "ulid" => {
            ulidgen::gen_ulid()
        }
        "uuid" => {
            uuidgen::gen_uuid()
        }
        "sf" => {
            sfgen::gen_symfony_secret()
        }
        _ => {
            "Unknown generator".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use ulid::Ulid;
    use super::*;

    #[test]
    fn test_parse_cmd() {
        let args = vec![
            OsString::from("ulidgen"),
            OsString::from("-t"),
            OsString::from("ulid"),
        ];

        let command = parse_cmd(args);

        assert_eq!(command.generator, "ulid");
        assert_eq!(command.option, "");
        assert_eq!(command.value, "");
    }

    #[test]
    fn test_parse_cmd_with_option() {
        let args = vec![
            OsString::from("ulidgen"),
            OsString::from("-t"),
            OsString::from("ulid"),
            OsString::from("-c"),
            OsString::from("time"),
            OsString::from("01"),
        ];

        let command = parse_cmd(args);

        assert_eq!(command.generator, "ulid");
        assert_eq!(command.option, "time");
        assert_eq!(command.value, "01");
    }

    #[test]
    fn test_handle_cmd() {
        let command = Command {
            generator: "ulid".to_string(),
            option: "".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        assert!(Ulid::from_string(output.as_str()).is_ok());
    }

    #[test]
    fn test_handle_cmd_with_option() {
        let command = Command {
            generator: "ulid".to_string(),
            option: "uuid".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        let uuid_format_regex = regex::Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89ABab][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
        assert!(uuid_format_regex.is_match(output.as_str()));
    }

    #[test]
    fn test_handle_cmd_with_option_time() {
        let command = Command {
            generator: "ulid".to_string(),
            option: "time".to_string(),
            value: Ulid::new().to_string(),
        };

        let output = handle_cmd(command);

        let atom_format_regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap();
        assert!(atom_format_regex.is_match(output.as_str()));
    }

    #[test]
    fn test_handle_cmd_with_symfony() {
        let command = Command {
            generator: "sf".to_string(),
            option: "".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        assert_eq!(output.len(), 32);
    }

    #[test]
    fn test_handle_cmd_with_random_option() {
        let command = Command {
            generator: "sf".to_string(),
            option: "time".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        assert_eq!(output.len(), 32);
    }

    #[test]
    fn test_handle_cmd_with_unknown_generator() {
        let command = Command {
            generator: "unknown".to_string(),
            option: "".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        assert_eq!(output, "Unknown generator");
    }

    #[test]
    fn test_handle_cmd_with_uuid() {
        let command = Command {
            generator: "uuid".to_string(),
            option: "".to_string(),
            value: "".to_string(),
        };

        let output = handle_cmd(command);

        let uuid_format_regex = regex::Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89ABab][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
        assert!(uuid_format_regex.is_match(output.as_str()));
    }
}
