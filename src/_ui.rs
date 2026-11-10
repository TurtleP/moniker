use std::path::Path;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Paragraph,
        Row, Table,
    },
};

use crate::{
    keys::BINDINGS,
    matchers::Provider,
};

fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .title_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
}

fn input<'a>(value: &'a str) -> Paragraph<'a> {
    Paragraph::new(value)
        .style(Style::default().fg(Color::White))
        .block(panel("Input"))
}

fn list<'a>(items: Vec<ListItem<'a>>, title: &'a str) -> List<'a> {
    List::new(items)
        .block(panel(title))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .highlight_spacing(HighlightSpacing::Always)
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let [_, v, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .areas(area);
    let [_, h, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .areas(v);
    h
}
