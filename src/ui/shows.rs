use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Clear, ListItem, ListState},
};

use crate::{
    keys::Action,
    matchers::SeriesMatch,
    ui::{PopupResult, dialog_sized, list},
};

// TODO Add an input widget to search for things with > 5 results (and a scrollbar)
pub fn render(frame: &mut Frame, state: &mut ListState, search_results: &[SeriesMatch]) {
    let area = dialog_sized(frame, Constraint::Percentage(50), Constraint::Length(7));

    let items: Vec<_> = search_results
        .iter()
        .map(|value| ListItem::from(value.series.to_string()))
        .collect();

    let len = items.len();
    let index = state.selected().map_or(1, |v| v.min(len - 1) + 1);

    let shows_list = list(items, format!("Select a Series ({index}/{len})"));

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(shows_list, area, state);
}

pub fn keypressed(action: &Action, list_state: &mut ListState) -> PopupResult {
    match action {
        Action::Up => list_state.select_previous(),
        Action::Down => list_state.select_next(),
        Action::Enter => return PopupResult::Confirm,
        Action::Back => return PopupResult::Close,
        _ => {}
    }
    PopupResult::Consume
}
