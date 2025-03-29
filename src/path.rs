use std::path::Path;

const FILE_NAME: &str = "package.json";

pub fn create_path(input: &str) -> Path {
    let path = Path::new(input);
    if path.file_name() == Some(str::ffi::OsStr::new(FILE_NAME)) {
        // already includes the file
        path
    } else {
        // if not then add this part
        path.join(FILE_NAME)
    }
}
