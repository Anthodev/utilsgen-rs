mod ulidgen;
mod sfgen;
mod cmd_handler;

use std::env;
use arboard::Clipboard;

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

    add_to_clipboard(output);
}

fn add_to_clipboard(output: String) {
    let mut clipboard_ctx = Clipboard::new().unwrap();
    clipboard_ctx.set_text(output.to_owned()).unwrap();

    println!("Copied to clipboard");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_clipboard() {
        let output = "test".to_string();
        add_to_clipboard(output);

        let mut clipboard_ctx = Clipboard::new().unwrap();
        let clipboard_content = clipboard_ctx.get_text().unwrap();

        assert_eq!(clipboard_content, "test");
    }
}
