use crate::locale::Locale;

pub fn get_path_input(locale: &Locale) -> String {
    if let Some(input_arg) = std::env::args().nth(1) {
        if input_arg == "--help" || input_arg == "-h" {
            print!("{}", match locale {
                Locale::English => include_str!("texts/help-en.txt"),
                Locale::Esperanto => include_str!("texts/help-eo.txt"),
            });
            std::process::exit(0);
        }
        input_arg
    } else {
        String::from(".") // the default is this directory
    }
}
