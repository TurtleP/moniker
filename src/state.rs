use std::path::PathBuf;

use anyhow::Result;
use ratatui::widgets::ListState;
use reqwest::Client;

use crate::config::Config;
use crate::files::FileEntry;
use crate::matchers::{Provider, SeriesMatch};

#[derive(Default, Debug, PartialEq)]
pub enum AppPopup {
    #[default]
    None,
    Help,
    Sources,
    Match,
    Shows,
}

fn list_state() -> ListState {
    ListState::default().with_selected(Some(0))
}

#[derive(Debug)]
pub struct WorkspaceState {
    pub dir: PathBuf,
    pub files: Vec<FileEntry>,
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
            list_state: list_state(),
        }
    }
}

#[derive(Debug)]
pub struct MetadataState {
    pub series: SeriesMatch,
    pub list_state: ListState,
    pub search_query: String,
}

impl Default for MetadataState {
    fn default() -> Self {
        Self {
            series: SeriesMatch::default(),
            list_state: list_state(),
            search_query: String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ProviderState {
    pub provider: Provider,
    pub list_state: ListState,
}

impl Default for ProviderState {
    fn default() -> Self {
        Self {
            provider: Provider::default(),
            list_state: list_state(),
        }
    }
}

impl ProviderState {
    pub async fn search(&self, client: &Client, query: &String) -> Result<Vec<SeriesMatch>> {
        self.provider.search(&client, query).await
    }
}

#[derive(Default, Debug)]
pub struct DialogState {
    pub popup: AppPopup,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub workspace: WorkspaceState,
    pub metadata: MetadataState,
    pub provider: ProviderState,
    pub dialog: DialogState,
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
