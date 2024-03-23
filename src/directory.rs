use std::{
    collections::HashSet,
    fs::{read_dir, ReadDir},
    path::PathBuf,
};

use super::file::File;

pub struct Directory {
    pub path: PathBuf,
    pub files: Vec<File>,
    pub directories: Vec<Directory>,
}

impl Directory {
    pub fn build(path: &PathBuf) -> Result<Directory, &'static str> {
        let directory = read_dir(&path).expect("Directory not found");

        let (files, directories) = Self::get_files_and_directories(directory);

        Ok(Directory {
            path: path.clone(),
            files,
            directories,
        })
    }

    fn get_files_and_directories(entries: ReadDir) -> (Vec<File>, Vec<Directory>) {
        let mut directories: Vec<Directory> = vec![];
        let mut files: Vec<File> = vec![];

        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type();

                if let Ok(file_type) = file_type {
                    if file_type.is_dir() {
                        if let Ok(directory) = Self::build(&entry.path()) {
                            directories.push(directory);
                        }
                    } else if file_type.is_file() {
                        if let Some(file) = File::build(&entry.path()) {
                            files.push(file);
                        }
                    }
                } else {
                    continue;
                }
            }
        }

        (files, directories)
    }

    pub fn get_line_count(&self) -> usize {
        let mut count: usize = 0;

        for file in &self.files {
            count += file.line_count;
        }

        for directory in &self.directories {
            count += directory.get_line_count();
        }

        count
    }

    pub fn get_line_count_with_extensions(&self, extensions: &HashSet<&String>) -> usize {
        let mut count: usize = 0;

        for file in &self.files {
            if extensions.contains(&file.extension) {
                count += file.line_count;
            }
        }

        for directory in &self.directories {
            count += directory.get_line_count_with_extensions(&extensions);
        }

        count
    }

    pub fn get_file_extensions(&self) -> Vec<&String> {
        let mut extensions: HashSet<&String> = HashSet::new();

        for file in &self.files {
            if !extensions.contains(&file.extension) {
                extensions.insert(&file.extension);
            }
        }

        for directory in &self.directories {
            let child_extensions = directory.get_file_extensions();

            for extension in child_extensions {
                if !extensions.contains(&extension) {
                    extensions.insert(extension);
                }
            }
        }

        Vec::from_iter(extensions)
    }
}
