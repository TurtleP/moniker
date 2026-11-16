use std::{path::PathBuf, time::Duration};

use anyhow::Result;
use crossterm::event::{
    self, Event,
    KeyEventKind::{self},
};
use ratatui::{DefaultTerminal, Frame};
use reqwest::Client;
use tracing::info;

use crate::{
    config::Config,
    files,
    keys::{Action, BINDINGS},
    state::{AppState, Dialog},
    ui::{self, PopupResult, help, layout, provider, query, shows},
};

pub struct App {
    state: AppState,
    client: Client,
    exit: bool,
}

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl App {
    pub fn new(dir: Option<PathBuf>, config: Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(5))
            .build()?;

        let mut state = AppState::new(config);
        state.workspace.dir = match dir {
            Some(v) => v,
            None => std::env::current_dir()?,
        };

        Ok(Self {
            state,
            client,
            exit: false,
        })
    }

    pub fn rescan_directory(&mut self) -> Result<()> {
        let included_files = self.state.config.general.files.clone();
        let directory = &self.state.workspace.dir;
        self.state.workspace.files = files::scan_directory(directory, &included_files)?;
        Ok(())
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.rescan_directory()?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let window = layout::create_layout(frame);
        layout::render_workspace(frame, window.files_list, &mut self.state.workspace);
        layout::render_rename(frame, window.rename_list, &mut self.state.workspace);

        let current_source = format!("Source: [{}]", self.state.provider.name());
        layout::render_status_bar(frame, window.status_bar, &current_source);

        match &mut self.state.dialog {
            Dialog::Help => help::render(frame),
            Dialog::Sources(list_state) => provider::render(frame, list_state),
            Dialog::Search(_) => query::render(frame, &mut self.state.search.query),
            Dialog::Shows(list_state) => {
                shows::render(frame, list_state, &self.state.search.results)
            }
            _ => {}
        };
    }

    fn display_popup(&mut self, dialog: Dialog) {
        self.state.dialog = dialog;
    }

    async fn search_for_series(&mut self) -> Result<()> {
        let query = self.state.search.query.clone();
        let mut result = self.state.provider.search(&self.client, &query).await?;
        match result.len() {
            0 => {}
            1 => {
                if let Some(series) = result.pop() {
                    self.state.search.results.push(series);
                }
            }
            _ => {
                info!("Showing popup..");
                self.state.search.results = result;
                self.display_popup(Dialog::shows());
            }
        }
        Ok(())
    }

    async fn popup_keypressed(&mut self, action: &Action) -> Result<()> {
        let result = match &mut self.state.dialog {
            Dialog::None => {
                self.keypressed(action)?;
                PopupResult::Consume
            }
            Dialog::Help => help::keypressed(action),
            Dialog::Sources(list_state) => {
                provider::keypressed(action, list_state, &mut self.state.provider)
            }
            Dialog::Search(_) => query::keypressed(action),
            Dialog::Shows(list_state) => shows::keypressed(action, list_state),
        };
        if matches!(result, PopupResult::Confirm) {
            match self.state.dialog {
                Dialog::Search(_) => self.search_for_series().await?,
                Dialog::Shows(state) => {
                    if let Some(selection) = state.selected() {
                        let series = self.state.search.results[selection].clone();
                        info!("Selected {}", series);
                        self.state.workspace.series = series;
                    }
                    self.display_popup(Dialog::None);
                }
                _ => self.display_popup(Dialog::None),
            }
        } else if matches!(result, PopupResult::Close) {
            self.display_popup(Dialog::None);
        }
        Ok(())
    }

    fn keypressed(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::Quit => self.quit(),
            Action::Rescan => self.rescan_directory()?,
            Action::ToggleHelp => self.display_popup(Dialog::Help),
            Action::ChangeSource => self.display_popup(Dialog::sources()),
            Action::Match => {
                self.state.search.query.clear();
                self.display_popup(Dialog::matches())
            }
            Action::Rename => {}
            Action::Down => self.state.workspace.list_state.select_next(),
            Action::Up => self.state.workspace.list_state.select_previous(),
            _ => {}
        }
        Ok(())
    }

    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key_event) = event::read()?
            && key_event.kind == KeyEventKind::Press
        {
            if matches!(self.state.dialog, Dialog::Search(_)) {
                ui::query::send_input(&key_event.code, &mut self.state.search.query);
            }
            if let Some(action) = BINDINGS
                .iter()
                .find(|b| b.event.keys.contains(&key_event.code))
                .map(|b| &b.event.action)
            {
                self.popup_keypressed(action).await?;
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}
