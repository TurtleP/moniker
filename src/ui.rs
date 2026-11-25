use ratatui::{
    Frame,
    layout::HorizontalAlignment,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List},
};

use crate::{app::layout::AppLayout, files::Workspace};

pub fn render(frame: &mut Frame, layout: AppLayout, workspace: &mut Workspace) {
    let state = &mut workspace.state.clone();
    let local_list = List::new(workspace.files_to_list()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Files")
            .title_alignment(HorizontalAlignment::Center)
            .title_style(Style::default().fg(Color::LightCyan)),
    );
    frame.render_stateful_widget(local_list, layout.files_list, state);
    let remote_list = List::new(workspace.renamed_to_list()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Rename")
            .title_alignment(HorizontalAlignment::Center)
            .title_style(Style::default().fg(Color::LightCyan)),
    );
    frame.render_widget(remote_list, layout.rename_list);
}
