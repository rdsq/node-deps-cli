use std::io::{self, Read};

pub fn from_stdin() -> String {
    let mut input = String::new();

    // Read from stdin until EOF
    return match io::stdin().read_to_string(&mut input) {
        Ok(_) => input,
        Err(e) => {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        }
    }
}
