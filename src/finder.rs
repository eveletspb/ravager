
use std::fs::ReadDir;
use std::path::PathBuf;

pub fn find(dir: ReadDir, filter: &str, mut vec: Vec<PathBuf>) -> Vec<PathBuf> {
    for entry in dir {
         let path = entry.unwrap().path();
        if path.is_dir() {
            return find(path.read_dir().unwrap(), filter, vec);
        } else {
            let file_name = path.file_name().unwrap();
            if file_name == filter {
                vec.push(path);
            }
        }
    }
    vec
}