use std::fs;
use std::io::ErrorKind::NotFound;

pub fn read_file(path: &str) -> String {
    return fs::read_to_string(path)
        .unwrap_or_else(|err| {
            if err.kind() == NotFound {
                // special output for this case
                eprintln!("\x1b[33m[no package.json found]\x1b[0m");
            } else {
                // just print the error
                eprintln!("{}", err);
            }
            std::process::exit(1);
        });
}
