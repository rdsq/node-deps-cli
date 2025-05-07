use std::env;
use std::io::{self, IsTerminal};

fn should_use_color() -> bool {
    env::var_os("NO_COLOR").is_none()
        && io::stdout().is_terminal()
}

pub fn color(code: &str, text: &str) -> String {
    if should_use_color() {
        format!("\x1b[{}m{}\x1b[0m", code, text)
    } else {
        text.to_string()
    }
}
