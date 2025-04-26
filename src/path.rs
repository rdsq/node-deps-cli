use std::path::PathBuf;

const FILE_NAME: &str = "package.json";

pub fn create_path(input: &str) -> String {
    let mut path = PathBuf::from(input);
    if path.is_dir() {
        // add file if this is a directory
        path.push(FILE_NAME);
    }
    path.to_str()
        .expect("invalid UTF-8 in path")
        .to_string()
}
