use ratatui::{
    Frame,
    layout::Constraint,
    widgets::ListState,
    widgets::{Clear, ListItem},
};

use crate::{
    keys::Action,
    matchers::Provider,
    ui::{PopupResult, dialog_sized, list},
};

pub fn render(frame: &mut Frame, list_state: &mut ListState) {
    let items: Vec<ListItem> = Provider::ALL
        .into_iter()
        .map(|source| ListItem::from(source.name()))
        .collect();

    let height = (items.len() + 2) as u16;
    let area = dialog_sized(
        frame,
        Constraint::Percentage(50),
        Constraint::Length(height),
    );
    let list = list(items, "Sources");

    frame.render_widget(Clear, area);
    frame.render_stateful_widget(list, area, list_state);
}

pub fn keypressed(
    action: &Action,
    list_state: &mut ListState,
    provider: &mut Provider,
) -> PopupResult {
    match action {
        Action::Up => list_state.select_previous(),
        Action::Down => list_state.select_next(),
        Action::Enter => {
            if let Some(selected) = list_state.selected() {
                *provider = Provider::ALL[selected];
            }
            return PopupResult::Close;
        }
        _ => {}
    }
    PopupResult::Consume
}
