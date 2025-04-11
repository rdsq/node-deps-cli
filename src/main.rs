mod locale;
mod cli;
mod path;
mod from_file;
mod from_stdin;
mod color;
use std::process::exit;
use serde::Deserialize;
use serde_json;

type DepsItem = Option<std::collections::HashMap<String, String>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageStructure {
    dependencies: DepsItem,
    dev_dependencies: DepsItem,
    optional_dependencies: DepsItem,
    peer_dependencies: DepsItem,
}

fn process_deps(deps_option: DepsItem, title: &str, found_any: &mut bool) {
    if let Some(deps) = deps_option {
        if !deps.is_empty() {
            if *found_any {
                // add a little space between them
                println!();
            }
            println!("{}", title);
            for (package, version) in deps {
                println!("{} {}", color::color("34", &package), color::color("2", &version));
            }
            *found_any = true;
        }
    }
}

fn input_method(input: &str, locale_val: &locale::Locale) -> String {
    if input == "-" {
        from_stdin::from_stdin()
    } else {
        let full_path = path::create_path(&input);
        from_file::read_file(&full_path, locale_val)
    }
}

fn main() {
    let locale_val = locale::get_locale();
    let input = cli::get_path_input(&locale_val);
    let contents = input_method(&input, &locale_val);
    let parsed: PackageStructure = serde_json::from_str(&contents)
        .unwrap_or_else(|err| {
            eprintln!("JSON parsing error: {}", err);
            exit(1);
        });
    let mut found_any = false;
    process_deps(parsed.dependencies, "Dependencies", &mut found_any);
    process_deps(parsed.dev_dependencies, "Dev Dependencies", &mut found_any);
    process_deps(parsed.optional_dependencies, "Optional Dependencies", &mut found_any);
    process_deps(parsed.peer_dependencies, "Peer Dependencies", &mut found_any);
    if !found_any {
        println!("{}", color::color("36", &match locale_val {
            locale::Locale::English => include_str!("texts/none-en.txt"),
            locale::Locale::Esperanto => include_str!("texts/none-eo.txt"),
        }.trim()));
    }
}
