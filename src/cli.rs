const HELP_MESSAGE: &str = r#"Help wip"#;

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
