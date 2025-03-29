const HELP_MESSAGE: &str = r#"Show Node.js package's dependencies in a fancy way

Usage: node-deps

Simply run this command inside a Node package, and you will see its dependencies

You can also add path like this:

node-deps ./path/to/package

And see that directory's dependencies
It looks up the current directory by default
You can also include the package.json file in path if you want

Alternatively you can do:

node-deps -

For it to read the package.json contents from stdin"#;

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
