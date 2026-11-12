use std::{path::PathBuf, time::Duration};

use anyhow::{Ok, Result, bail};
use crossterm::event::{
    self, Event,
    KeyEventKind::{self},
};
use ratatui::{DefaultTerminal, Frame};
use reqwest::Client;
use tracing::error;

use crate::{
    config::Config,
    files,
    keys::{Action, BINDINGS},
    matchers::SeriesMatch,
    state::{AppPopup, AppState},
    ui::{self, PopupResult, help, layout, provider, query},
};

#[derive(Debug)]
pub struct App {
    exit: bool,
    state: AppState,
    client: Client,
}

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

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
            exit: false,
            state,
            client,
        })
    }

    pub fn rescan_directory(&mut self) -> Result<()> {
        let included_files = self.state.config.general.files.clone();
        files::scan_directory(&mut self.state.workspace, &included_files)
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
        let window = layout::create_layout(&frame);
        layout::render_workspace(frame, window.files_list, &mut self.state.workspace);
        layout::render_rename(frame, window.rename_list, &mut self.state.metadata);

        let current_source = format!("Source: [{}]", self.state.provider.provider.name());
        layout::render_status_bar(frame, window.status_bar, &current_source);

        match self.state.dialog.popup {
            AppPopup::Help => help::render(frame),
            AppPopup::Sources => provider::render(frame, &mut self.state.provider),
            AppPopup::Match => query::render(frame, &mut self.state.metadata),
            _ => {}
        };
    }

    fn display_popup(&mut self, dialog: AppPopup) {
        self.state.dialog.popup = dialog;
    }

    async fn search_for_series(&mut self) -> Result<()> {
        let query = self.state.metadata.search_query.clone();
        let mut result = self.state.provider.search(&self.client, &query).await?;
        let series = match result.len() {
            0 => None,
            1 => result.pop(),
            _ => {
                self.display_popup(AppPopup::Shows);
                None
            }
        };
        Ok(())
    }

    async fn popup_keypressed(&mut self, action: &Action) -> Result<()> {
        let result = match self.state.dialog.popup {
            AppPopup::None => {
                self.keypressed(&action)?;
                PopupResult::Consume
            }
            AppPopup::Help => help::keypressed(&action),
            AppPopup::Sources => provider::keypressed(&action, &mut self.state.provider),
            AppPopup::Match => query::keypressed(action, &mut self.state.metadata),
            _ => todo!("Binding '{}'", action),
        };
        if result == PopupResult::Close {
            if self.state.dialog.popup == AppPopup::Match {
                self.search_for_series().await?;
            }
            self.display_popup(AppPopup::None);
        }
        Ok(())
    }

    fn keypressed(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::Quit => self.quit(),
            Action::Rescan => self.rescan_directory()?,
            Action::ToggleHelp => self.display_popup(AppPopup::Help),
            Action::ChangeSource => self.display_popup(AppPopup::Sources),
            Action::Match => {
                self.state.metadata.search_query.clear();
                self.display_popup(AppPopup::Match)
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
            if self.state.dialog.popup == AppPopup::Match {
                ui::query::send_input(&key_event.code, &mut self.state.metadata);
            }
            if let Some(action) = BINDINGS
                .iter()
                .find(|b| b.event.keys.contains(&key_event.code))
                .map(|b| &b.event.action)
            {
                self.popup_keypressed(&action).await?;
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}
