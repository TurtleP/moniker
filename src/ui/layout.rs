use crate::{state::WorkspaceState, ui::list};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{ListItem, Paragraph},
};

pub struct Window {
    pub files_list: Rect,
    pub rename_list: Rect,
    pub status_bar: Rect,
}

const VERTICAL_CONSTRAINTS: &[Constraint; 2] = &[Constraint::Min(0), Constraint::Length(1)];
const HORIZONTAL_CONSTRAINTS: &[Constraint; 2] =
    &[Constraint::Percentage(50), Constraint::Percentage(50)];

pub fn create_layout(frame: &Frame) -> Window {
    let [main_area, status_bar] = Layout::vertical(VERTICAL_CONSTRAINTS).areas(frame.area());
    let [files_list, rename_list] = Layout::horizontal(HORIZONTAL_CONSTRAINTS).areas(main_area);
    Window {
        files_list,
        rename_list,
        status_bar,
    }
}

pub fn render_workspace(frame: &mut Frame, area: Rect, workspace: &mut WorkspaceState) {
    let items: Vec<ListItem> = workspace
        .files
        .iter()
        .map(|f| ListItem::from(f.display.as_str()))
        .collect();

    let title = match workspace.dir.to_str() {
        Some(value) => value.to_string(),
        None => String::new(),
    };

    let widget = list(items, title);
    frame.render_stateful_widget(widget, area, &mut workspace.list_state);
}

pub fn render_rename(frame: &mut Frame, area: Rect, workspace: &mut WorkspaceState) {
    let items: Vec<ListItem> = workspace
        .series
        .episodes
        .iter()
        .map(|e| ListItem::from(e.name.as_str()))
        .collect();

    let widget = list(items, "Rename");
    frame.render_widget(widget, area);
}

pub fn render_status_bar(frame: &mut Frame, area: Rect, text: &str) {
    frame.render_widget(Paragraph::new(text), area);
}
