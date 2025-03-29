use std::path::PathBuf;
use std::ffi::OsStr;

const FILE_NAME: &str = "package.json";

pub fn create_path(input: &str) -> String {
    let mut path = PathBuf::from(input);
    if path.file_name() != Some(OsStr::new(FILE_NAME)) {
        // if not includes then add this part
        path.push(FILE_NAME);
    }
    path.to_str()
        .expect("invalid UTF-8 in path")
        .to_string()
}
