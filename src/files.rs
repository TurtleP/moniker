use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub display: String,
}

fn get_file_info(root: &String, path: &Path) -> Result<(String, String)> {
    let display = path.strip_prefix(root)?.to_string_lossy().into_owned();
    let extension = match path.extension() {
        Some(extension) => extension.to_string_lossy().into_owned(),
        None => return Err(anyhow::anyhow!("File has no extension")),
    };
    Ok((display, extension))
}

pub fn scan_directory(directory: &PathBuf, included: &[String]) -> Result<Vec<FileEntry>> {
    let mut files = Vec::new();
    let root = directory.to_string_lossy().into_owned();

    let filtered_dir = WalkDir::new(&root)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|entry| entry.file_type().is_file());

    for entry in filtered_dir {
        let entry_path = entry.path();
        let Ok((display, extension)) = get_file_info(&root, entry_path) else {
            continue;
        };
        if included
            .iter()
            .any(|f| extension.trim_start_matches('.') == *f)
        {
            files.push(FileEntry {
                path: PathBuf::from(entry_path),
                display: String::from(display),
            });
        }
    }
    Ok(files)
}

pub fn apply_config_format() {}

pub fn rename_files() {}
