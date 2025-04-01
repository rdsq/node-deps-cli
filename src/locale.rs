use std::env;

pub enum Locale {
    English,
    Esperanto,
}

pub fn get_locale() -> Locale {
    let lang = env::var("LANG").unwrap_or_else(|_| "en_US.UTF-8".to_string());
    if lang.starts_with("eo") {
        Locale::Esperanto
    } else {
        // default
        Locale::English
    }
}
