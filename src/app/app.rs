use std::path::PathBuf;

use anyhow::Result;
use ratatui::{DefaultTerminal, Frame};

use crate::{app::layout::AppLayout, files, ui};

pub struct Application {
    directory: PathBuf,
    exit: bool,
}

impl Application {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        files::scan_directory(&self.directory)?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut layout = AppLayout::new(frame);
        ui::render(frame, &mut layout);
    }
}
