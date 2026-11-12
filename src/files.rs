use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

use crate::state::WorkspaceState;

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub display: String,
}

pub fn scan_directory(workspace: &mut WorkspaceState, included: &[String]) -> Result<()> {
    let directory = workspace.dir.clone();

    let filtered_dir = WalkDir::new(&directory)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().is_some());

    for entry in filtered_dir {
        let entry_path = entry.path();
        let display = entry_path.strip_prefix(&directory)?.to_string_lossy();
        if included.iter().any(|f| display.ends_with(f)) {
            workspace.files.push(FileEntry {
                path: PathBuf::from(entry_path),
                display: String::from(display),
            });
        }
    }
    Ok(())
}

pub fn apply_config_format() {}

pub fn rename_files() {}
