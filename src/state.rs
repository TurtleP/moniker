use std::collections::HashSet;
use std::path::PathBuf;

use ratatui::widgets::ListState;
use tracing::info;

use crate::config::Config;
use crate::files::FileEntry;
use crate::matchers::{Episode, Provider, SeriesMatch};

#[derive(PartialEq)]
pub enum DialogMessageType {
    Info,
    Warn,
}

#[derive(Default, PartialEq)]
pub enum Dialog {
    #[default]
    None,
    Help,
    Sources(ListState),
    Search(ListState),
    Shows(ListState),
    Message {
        title: String,
        body: String,
    },
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

    pub fn info(body: Vec<impl Into<String>>) -> Self {
        Self::message("[ INFO ]", body)
    }

    pub fn warn(body: Vec<impl Into<String>>) -> Self {
        Self::message("[ WARN ]", body)
    }

    pub fn message(title: impl Into<String>, body: Vec<impl Into<String>>) -> Self {
        let lines: Vec<String> = body.into_iter().map(Into::into).collect();
        let body = lines.join("\n");

        Self::Message {
            title: title.into(),
            body,
        }
    }
}

pub struct WorkspaceState {
    pub dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub series: SeriesMatch,
    pub list_state: ListState,
    matched_names: HashSet<String>,
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
            matched_names: HashSet::new(),
        }
    }
}

impl WorkspaceState {
    fn normalize(s: &str) -> String {
        s.chars()
            .flat_map(|c| c.to_lowercase())
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    pub fn rebuild_match_index(&mut self) {
        self.matched_names = self
            .files
            .iter()
            .map(|f| Self::normalize(&f.display))
            .collect();
        info!("{:?}", self.matched_names);
    }

    pub fn is_matched(&self, episode: &Episode) -> bool {
        self.matched_names
            .contains(&Self::normalize(&episode.display))
    }
}

#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub results: Vec<SeriesMatch>,
}

#[derive(Default)]
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
            config,
            ..AppState::default()
        }
    }
}
