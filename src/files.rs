use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub display: String,
}

pub fn scan_directory(dir: &PathBuf) -> Result<Vec<FileEntry>> {
    let mut files = Vec::<FileEntry>::new();
    let filtered_dir = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| e.file_type().is_file());

    for entry in filtered_dir {
        let entry = entry?;
        let entry_path = entry.path();
        let extension = entry_path.extension();
        let display = entry_path.strip_prefix(dir)?.to_string_lossy();
        files.push(FileEntry {
            path: PathBuf::from(entry_path),
            display: String::from(display),
        });
    }
    Ok(files)
}

pub fn filter_episodes() {}

pub fn apply_config_format() {}

pub fn rename_files() {}
