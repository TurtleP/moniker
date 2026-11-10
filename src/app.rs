use std::{path::PathBuf, time::Duration};

use anyhow::{Ok, Result};
use crossterm::event::{
    self, Event, KeyCode,
    KeyEventKind::{self},
};
use ratatui::{DefaultTerminal, Frame};
use reqwest::Client;

use crate::{config::Config, state::AppState, ui::layout};

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
            None => PathBuf::from("."),
        };

        Ok(Self {
            exit: false,
            state,
            client,
        })
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.workspace.files = crate::files::scan_directory(&self.state.workspace.dir)?;
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
        // layout::render_status_bar(frame, window.status_bar);
    }

    async fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()?
                && key_event.kind != KeyEventKind::Press
            {
                match key_event.code {
                    KeyCode::Char('q') => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.exit = true;
    }
}
