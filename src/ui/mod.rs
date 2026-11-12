use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, Paragraph},
};

pub mod help;
pub mod layout;
pub mod provider;
pub mod query;

#[derive(PartialEq, Eq)]
pub enum PopupResult {
    Consume,
    Close,
}

fn dialog_sized(frame: &Frame, width: Constraint, height: Constraint) -> Rect {
    let screen = frame.area();
    screen.centered(width, height)
}

fn dialog(frame: &Frame) -> Rect {
    let screen = frame.area();
    screen.centered(Constraint::Percentage(50), Constraint::Percentage(50))
}

fn panel(title: impl ToString) -> Block<'static> {
    Block::default()
        .title(title.to_string())
        .title_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
}

fn list<'items>(items: Vec<ListItem<'items>>, title: impl ToString) -> List<'items> {
    List::new(items)
        .block(panel(title))
        .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .highlight_spacing(HighlightSpacing::Always)
}

fn input<'text>(value: &'text str) -> Paragraph<'text> {
    Paragraph::new(value)
        .style(Style::default().fg(Color::White))
        .block(panel("Input"))
}
