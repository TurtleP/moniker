use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::config::Config;
use crate::files::FileEntry;
use crate::matchers::SeriesMatch;

#[derive(Default, Debug)]
enum AppPopup {
    #[default]
    None,
    Help,
    Sources,
    Match,
    Shows,
}

#[derive(Default, Debug)]
pub struct WorkspaceState {
    pub dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub state: ListState
}

#[derive(Default, Debug)]
pub struct MetadataState {
    pub series: SeriesMatch,
    pub state: ListState,
}

#[derive(Default, Debug)]
pub struct DialogState {
    pub popup: AppPopup,
    pub search_query: String,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub workspace: WorkspaceState,
    pub metadata: MetadataState,
    pub dialog: DialogState,
    pub config: Config
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            ..AppState::default()
        }
    }
}
