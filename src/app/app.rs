use std::path::PathBuf;

use anyhow::Result;
use ratatui::{DefaultTerminal, Frame};

use crate::{
    app::layout::AppLayout,
    files::{self, Workspace},
    ui,
};

pub struct Application {
    workspace: Workspace,
    exit: bool,
}

impl Application {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            workspace: Workspace::new(directory),
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.workspace.scan()?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut layout = AppLayout::new(frame);
        ui::render(frame, layout, &mut self.workspace);
    }
}
