use std::path::Path;

pub fn name_from_path(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap_or("noname")
        .to_owned()
}
