const HELP_MESSAGE: &str = r#"Show Node.js package's dependencies in a fancy way

Usage: node-deps [path to package (current dir by default)]"#;

pub fn get_path_input() -> String {
    if let Some(input_arg) = std::env::args().nth(1) {
        if input_arg == "--help" || input_arg == "-h" {
            println!("{}", HELP_MESSAGE);
            std::process::exit(0);
        }
        input_arg
    } else {
        String::from(".") // the default is this directory
    }
}
