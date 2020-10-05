use std::path::{Path,PathBuf};

pub fn name_from_path(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap_or("noname")
        .to_owned()
}


pub fn is_hidden(path: &Path) -> bool {
    match path.file_name() {
        Some(name) => name.to_str().map(|s| s.starts_with(".")).unwrap_or(false),
        None => false
    }
}

pub fn hiddens(path:&Path) -> u16 {
   path.components().map(|comp| comp.as_os_str()).map(|comp| comp.to_str()).map(|name|name.unwrap()).filter(|name|  name.starts_with(".")&&name.len()>1).count() as u16
}

pub fn truncate_prefix(path:&Path,prefix:&Path) -> PathBuf {
    if let Ok(p)= path.strip_prefix(prefix) {
        p.to_path_buf()
    }else {
        path.to_path_buf()
    }
}