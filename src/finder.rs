use std::path::PathBuf;

pub fn find(directory: &str, filter: &str) -> Vec<PathBuf> {
    let mut vec: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.file_name().unwrap() == filter {
                    vec.push(path.to_path_buf());
                } else if path.is_dir() {
                    vec.extend(find(&path.to_str().unwrap(), filter));
                }
            }
        }
    }
    vec
}
