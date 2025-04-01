use std::fs;
use std::io::ErrorKind::NotFound;
use crate::locale::Locale;

pub fn read_file(path: &str, locale: &Locale) -> String {
    return fs::read_to_string(path)
        .unwrap_or_else(|err| {
            if err.kind() == NotFound {
                // special output for this case
                eprintln!("\x1b[33m{}\x1b[0m", match locale {
                    Locale::English => include_str!("texts/not-found-en.txt"),
                    Locale::Esperanto => include_str!("texts/not-found-eo.txt"),
                }.trim());
            } else {
                // just print the error
                eprintln!("{}", err);
            }
            std::process::exit(1);
        });
}
