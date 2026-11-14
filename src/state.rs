use std::path::PathBuf;

use ratatui::widgets::ListState;

use crate::config::Config;
use crate::files::FileEntry;
use crate::matchers::{Provider, SeriesMatch};

#[derive(Default, Debug, PartialEq)]
pub enum Dialog {
    #[default]
    None,
    Help,
    Sources(ListState),
    Search(ListState),
    Shows(ListState),
}

impl Dialog {
    fn fresh_list() -> ListState {
        ListState::default().with_selected(Some(0))
    }

    pub fn sources() -> Self {
        Self::Sources(Self::fresh_list())
    }
    pub fn matches() -> Self {
        Self::Search(Self::fresh_list())
    }
    pub fn shows() -> Self {
        Self::Shows(Self::fresh_list())
    }
}

#[derive(Debug)]
pub struct WorkspaceState {
    pub dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub series: SeriesMatch,
    pub list_state: ListState,
}

impl Default for WorkspaceState {
    fn default() -> Self {
        let Ok(dir) = std::env::current_dir() else {
            panic!("Failed to get current directory");
        };
        Self {
            dir,
            files: Vec::new(),
            series: SeriesMatch::default(),
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct SearchState {
    pub query: String,
    pub results: Vec<SeriesMatch>,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub workspace: WorkspaceState,
    pub provider: Provider,
    pub search: SearchState,
    pub dialog: Dialog,
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: config,
            ..AppState::default()
        }
    }
}
