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
    app::{FileList, ShowState},
    keys::BINDINGS,
    matchers::{EpisodeResult, Provider, SearchResult},
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

pub fn draw_main(
    frame: &mut Frame,
    dir: &Path,
    file_list: &mut FileList,
    source_name: &str,
    results: &SearchResult,
) {
    let [main_area, status_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).areas(frame.area());
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    draw_file_list(frame, left, dir, file_list);
    draw_new_names(frame, right, &results.episodes);
    draw_status_bar(frame, status_area, source_name);
}

fn draw_file_list(frame: &mut Frame, area: Rect, dir: &Path, file_list: &mut FileList) {
    let items: Vec<ListItem> = file_list
        .files
        .iter()
        .map(|file| ListItem::from(file.display.as_str()))
        .collect();

    let dir = dir.to_string_lossy();
    let list = list(items, &dir);

    frame.render_stateful_widget(list, area, &mut file_list.state);
}

fn draw_new_names(frame: &mut Frame, area: Rect, results: &[EpisodeResult]) {
    let items: Vec<ListItem> = results
        .iter()
        .map(|result| ListItem::from(result.name.as_str()))
        .collect();

    let list = list(items, "New Names");
    frame.render_widget(list, area);
}

const HINTS: [(&str, &str); 3] = [("↑/↓", "Select"), ("r", "Rescan"), ("?", "Help")];

fn draw_status_bar(frame: &mut Frame, area: Rect, source_name: &str) {
    let hints = HINTS
        .iter()
        .map(|(key, action)| format!("{key} {action}"))
        .collect::<Vec<_>>()
        .join(" • ");
    let source = format!("Source: {source_name}");
    let [hints_area, source_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(source.len() as u16)])
            .areas(area);

    let hints = Paragraph::new(hints).style(Style::default().fg(Color::LightCyan));
    let source = Paragraph::new(source).style(Style::default().fg(Color::LightCyan));
    frame.render_widget(hints, hints_area);
    frame.render_widget(source, source_area);
}

pub fn draw_help(frame: &mut Frame) {
    let screen = frame.area();
    let height = (BINDINGS.len() as u16 + 2).min(screen.height);
    let area = centered_rect(44.min(screen.width), height.min(screen.height), screen);

    let rows = BINDINGS
        .iter()
        .map(|b| Row::new([b.label, b.desc]).fg(Color::White));
    let table = Table::new(rows, [Constraint::Length(9), Constraint::Min(0)])
        .column_spacing(2)
        .block(panel("Help"));

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

pub fn draw_sources(frame: &mut Frame, state: &mut ListState) {
    let screen = frame.area();
    let height = (Provider::ALL.len() as u16 + 2).min(screen.height);
    let area = centered_rect(44.min(screen.width), height, screen);

    let items: Vec<ListItem> = Provider::ALL
        .into_iter()
        .map(|source| ListItem::from(source.name()))
        .collect();
    let list = list(items, "Sources");

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, state);
}

pub fn draw_shows(frame: &mut Frame, state: &mut ShowState) {
    let screen = frame.area();
    let height = (state.shows.len() as u16 + 2).min(screen.height);
    let area = centered_rect(44.min(screen.width), height, screen);

    let items: Vec<ListItem> = state
        .shows
        .iter()
        .map(|show| ListItem::from(show.show.name.as_str()))
        .collect();
    let list = list(items, "Show Results");

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, &mut state.state);
}

pub fn draw_match(frame: &mut Frame, match_input: &str) {
    let screen = frame.area();
    let area = centered_rect(44.min(screen.width), 3.min(screen.height), screen);

    let input = input(match_input).block(panel("Match"));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}
