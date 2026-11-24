use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use anyhow::Result;
use infer::MatcherType;
use tracing::info;
use walkdir::WalkDir;

use crate::matchers::{Episode, SeriesMatch};

#[derive(Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub display: String,
}

impl FileEntry {
    pub fn new(path: &Path, display: String) -> Self {
        Self {
            path: PathBuf::from(path),
            display,
        }
    }
}

impl std::fmt::Display for FileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

pub fn filter_results(files: &[FileEntry], episodes: &[Episode]) -> Vec<Episode> {
    episodes
        .iter()
        .filter(|r| {
            let by_season = format!("S{:02}E{:02}", r.season, r.number);
            let by_name = &r.name;
            files
                .iter()
                .any(|f| f.display.contains(&by_season) || f.display.contains(by_name))
        })
        .cloned()
        .collect()
}

pub fn scan_directory(directory: &PathBuf) -> Result<Vec<FileEntry>> {
    let mut files = Vec::new();
    let root = directory.display().to_string();

    let filtered_dir = WalkDir::new(&root)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|e| e.file_type().is_file());

    for entry in filtered_dir {
        let entry_path = entry.path();
        let Some(v) = infer::get_from_path(entry_path)? else {
            continue;
        };
        if !matches!(v.matcher_type(), MatcherType::Video) {
            continue;
        }
        let display = match entry_path.strip_prefix(directory) {
            Ok(v) => v.with_extension("").display().to_string(),
            Err(_) => continue,
        };
        files.push(FileEntry::new(entry_path, display));
    }
    Ok(files)
}

pub fn apply_config_format(format: &str, series: &mut SeriesMatch) {
    for episode in &mut series.episodes {
        let title = episode.name.clone();
        episode.display = format
            .replace(
                "{s00e00}",
                &format!("S{:02}E{:02}", episode.season, episode.number),
            )
            .replace("{n}", &series.series.name)
            .replace("{t}", &title)
            .replace("{s}", &episode.season.to_string())
            .replace("{e}", &episode.number.to_string());
        info!("Renamed {episode}");
    }
}

pub fn rename_file(file: &mut FileEntry, _series: &str, episode: &Episode) -> Result<bool> {
    let Some(extension) = file.path.extension() else {
        return Ok(false);
    };

    let stem = sanitize_filename::sanitize(&episode.display);
    if stem.is_empty() {
        return Ok(false);
    }

    let mut filename = OsString::from(stem);
    filename.push(".");
    filename.push(extension);

    let target = file.path.with_file_name(filename);
    if std::fs::exists(&target)? {
        return Ok(false);
    }
    std::fs::rename(&file.path, &target)?;
    Ok(true)
}
