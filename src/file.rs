use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

pub struct File {
    pub path: PathBuf,
    pub extension: String,
    pub line_count: usize,
}

impl File {
    pub fn build(path: &PathBuf) -> Option<File> {
        let file_metadata = fs::metadata(&path).unwrap();

        if !file_metadata.is_file() {
            return None;
        }

        let path = Path::new(&path);
        let extension = path.extension()?.to_str()?.to_string();
        let file = read_to_string(&path);

        if let Ok(file) = file {
            return Some(File {
                path: path.to_path_buf(),
                extension,
                line_count: file.lines().count(),
            });
        }

        None
    }
}
