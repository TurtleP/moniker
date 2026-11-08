use std::{path::PathBuf, time::Duration};

use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};
use reqwest::Client;
use tracing::info;
use walkdir::WalkDir;

use crate::{
    config::Config,
    keys::{Action, BINDINGS},
    matchers::{EpisodeResult, Provider, SearchResult, ShowResult},
    ui,
};

#[derive(Debug, Default)]
enum AppPopup {
    #[default]
    None,
    Help,
    Sources,
    Match,
    Shows,
}

#[derive(Debug, Default)]
struct ProviderState {
    state: ListState,
    selection: Option<usize>,
    results: SearchResult,
}

#[derive(Debug, Default)]
pub struct ShowState {
    pub state: ListState,
    pub shows: Vec<SearchResult>,
}

#[derive(Debug)]
struct AppState {
    dir: PathBuf,
    file_list: FileList,
    config: Config,
    popup: AppPopup,
    source_state: ProviderState,
    show_state: ShowState,
    match_input: String,
    client: Client,
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    state: AppState,
}

#[derive(Debug)]
pub struct FileEntry {
    pub path: PathBuf,
    pub display: String,
}

#[derive(Debug, Default)]
pub struct FileList {
    pub files: Vec<FileEntry>,
    pub state: ListState,
}

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl App {
    pub fn new(dir: Option<PathBuf>, config: Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()?;

        Ok(Self {
            exit: false,
            state: AppState {
                dir: dir.unwrap_or_else(|| PathBuf::from(".")),
                file_list: FileList::default(),
                config,
                popup: AppPopup::None,
                source_state: ProviderState::default(),
                show_state: ShowState::default(),
                match_input: String::new(),
                client,
            },
        })
    }

    fn keep_selection_valid(state: &mut ListState, len: usize) {
        match state.selected() {
            _ if len == 0 => state.select(None),
            Some(i) if i >= len => state.select(Some(len - 1)),
            None => state.select(Some(0)),
            _ => {}
        }
    }

    fn scan_for_files(&mut self) -> Result<()> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.state.dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.into_path();
                if path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| self.state.config.filter(extension))
                {
                    let display = path
                        .strip_prefix(&self.state.dir)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .into_owned();
                    files.push(FileEntry { path, display });
                }
            }
        }
        self.state.file_list.files = files;

        // keep selection valid after a rescan
        let len = self.state.file_list.files.len();
        Self::keep_selection_valid(&mut self.state.file_list.state, len);
        Ok(())
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.scan_for_files()?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn current_source(&self) -> Option<Provider> {
        let selection = self.state.source_state.selection?;
        Provider::ALL.get(selection).copied()
    }

    fn render(&mut self, frame: &mut Frame) {
        let source_name = self
            .current_source()
            .map_or("Not selected", |source| source.name());

        ui::draw_main(
            frame,
            &self.state.dir,
            &mut self.state.file_list,
            source_name,
            &self.state.source_state.results,
        );

        match self.state.popup {
            AppPopup::Help => ui::draw_help(frame),
            AppPopup::Sources => ui::draw_sources(frame, &mut self.state.source_state.state),
            AppPopup::Match => ui::draw_match(frame, &self.state.match_input),
            AppPopup::Shows => ui::draw_shows(frame, &mut self.state.show_state),
            _ => {}
        }
    }

    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                self.handle_key_event(key_event).await?;
            }
        }
        Ok(())
    }

    async fn handle_popup_key(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.state.popup {
            AppPopup::Help => self.handle_help_key(key_event),
            AppPopup::Sources => self.handle_sources_key(key_event),
            AppPopup::Match => self.handle_match_key(key_event).await?,
            AppPopup::Shows => self.handle_shows_key(key_event),
            AppPopup::None => {}
        }
        Ok(())
    }

    fn filter_new_episodes(
        results: &[EpisodeResult],
        file_list: &[FileEntry],
    ) -> Vec<EpisodeResult> {
        results
            .iter()
            .filter(|r| {
                let by_season = format!("S{:02}E{:02}", r.season, r.number);
                let by_name = &r.name;
                file_list
                    .iter()
                    .any(|f| f.display.contains(&by_season) || f.display.contains(by_name))
            })
            .cloned()
            .collect()
    }

    fn apply_format(result: &mut SearchResult, format: &str) {
        for episode in &mut result.episodes {
            let title = episode.name.clone();
            episode.name = format
                .replace(
                    "{s00e00}",
                    &format!("S{:02}E{:02}", episode.season, episode.number),
                )
                .replace("{n}", &result.show.name)
                .replace("{t}", &title)
                .replace("{s}", &episode.season.to_string())
                .replace("{e}", &episode.number.to_string());
        }
    }

    fn close_popup(&mut self) {
        self.state.popup = AppPopup::None;
    }

    fn handle_help_key(&mut self, key_event: KeyEvent) {
        if Self::is_close_key(key_event.code) {
            self.close_popup();
        }
    }

    fn handle_sources_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            code if Self::is_close_key(code) => self.close_popup(),
            KeyCode::Enter => {
                self.state.source_state.selection = self.state.source_state.state.selected();
                self.close_popup();
            }
            _ => self.move_selection(key_event),
        }
    }

    fn handle_shows_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            code if Self::is_close_key(code) => self.close_popup(),
            KeyCode::Enter => {
                let index = self.state.show_state.state.selected().unwrap_or(0);
                if let Some(mut result) = self.state.show_state.shows.get(index).cloned() {
                    let format = self.state.config.format();
                    Self::apply_format(&mut result, &format);
                    self.state.source_state.results = result;
                }
                self.close_popup();
            }
            _ => self.move_selection(key_event),
        }
    }

    fn handle_matches(&mut self, results: Vec<SearchResult>) {
        if results.len() == 1 {
            let mut result = match results.into_iter().next() {
                Some(result) => result,
                None => return,
            };
            let episodes = Self::filter_new_episodes(&result.episodes, &self.state.file_list.files);
            result.episodes = episodes;
            let format = self.state.config.format();
            Self::apply_format(&mut result, &format);
            self.state.source_state.results = result;
            return;
        }
        self.state.popup = AppPopup::Shows;
    }

    async fn handle_match_key(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Esc => self.close_popup(),
            KeyCode::Enter => {
                match self.current_source() {
                    Some(s) => {
                        let query = self.state.match_input.clone();
                        self.state.show_state.shows = s.search(&self.state.client, &query).await?;
                    }
                    None => {}
                }
                self.close_popup();
                self.state.match_input.clear();

                self.state.show_state.state.select_first();
                self.handle_matches(self.state.show_state.shows.clone());
            }
            KeyCode::Backspace => {
                self.state.match_input.pop();
            }
            KeyCode::Char(c) => self.state.match_input.push(c),
            _ => {}
        }
        Ok(())
    }

    fn is_close_key(code: KeyCode) -> bool {
        matches!(
            code,
            KeyCode::Esc | KeyCode::F(1) | KeyCode::Char('?') | KeyCode::Char('q')
        )
    }

    fn move_selection(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            _ => {}
        }
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }

        if !matches!(self.state.popup, AppPopup::None) {
            return self.handle_popup_key(key_event).await;
        }

        let Some(binding) = BINDINGS.iter().find(|b| b.keys.contains(&key_event.code)) else {
            return Ok(());
        };

        match binding.action {
            Action::Quit => self.quit(),
            Action::Rescan => self.scan_for_files()?,
            Action::Up | Action::Down => self.move_selection(key_event),
            Action::ToggleHelp => self.toggle_help(),
            Action::Match => self.toggle_match(),
            Action::Rename => self.handle_rename(false),
            Action::ChangeSource => self.toggle_sources(),
            Action::RenameAll => self.handle_rename(true),
        }
        Ok(())
    }

    fn handle_rename(&mut self, all: bool) {
        let episodes = &self.state.source_state.results.episodes;
        let selected_index = self.state.file_list.state.selected();

        let files = self
            .state
            .file_list
            .files
            .iter()
            .enumerate()
            .filter(|(index, _)| all || Some(*index) == selected_index)
            .map(|(_, file_entry)| file_entry);

        for file_entry in files {
            let display = file_entry.display.to_uppercase();
            let Some(episode) = episodes
                .iter()
                .find(|e| display.contains(&format!("S{:02}E{:02}", e.season, e.number)))
            else {
                info!("No matching episode for {}", file_entry.display);
                continue;
            };

            if file_entry.display != episode.name {
                info!("Renaming {} to {}", file_entry.display, episode.name);
                let validated = match file_entry.path.extension() {
                    Some(ext) => format!(
                        "{}.{}",
                        sanitize_filename::sanitize(&episode.name),
                        ext.to_string_lossy()
                    ),
                    None => sanitize_filename::sanitize(&episode.name),
                };
                let new_path = file_entry.path.with_file_name(&validated);
                if let Err(error) = std::fs::rename(&file_entry.path, new_path) {
                    info!("Failed to rename {}: {error}", file_entry.display);
                }
            }
        }
    }

    fn select_next(&mut self) {
        match self.state.popup {
            AppPopup::Sources => self.state.source_state.state.select_next(),
            AppPopup::Shows => self.state.show_state.state.select_next(),
            _ => self.state.file_list.state.select_next(),
        }
    }

    fn select_previous(&mut self) {
        match self.state.popup {
            AppPopup::Sources => self.state.source_state.state.select_previous(),
            AppPopup::Shows => self.state.show_state.state.select_previous(),
            _ => self.state.file_list.state.select_previous(),
        }
    }

    fn toggle_help(&mut self) {
        self.state.popup = if matches!(self.state.popup, AppPopup::Help) {
            AppPopup::None
        } else {
            AppPopup::Help
        };
    }

    fn toggle_sources(&mut self) {
        self.state.popup = if matches!(self.state.popup, AppPopup::Sources) {
            AppPopup::None
        } else {
            let len = Provider::ALL.len();
            Self::keep_selection_valid(&mut self.state.source_state.state, len);
            AppPopup::Sources
        };
    }

    fn toggle_match(&mut self) {
        self.state.popup = if matches!(self.state.popup, AppPopup::Match) {
            AppPopup::None
        } else {
            self.state.match_input.clear();
            AppPopup::Match
        };
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}
