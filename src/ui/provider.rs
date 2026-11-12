use ratatui::{
    Frame,
    layout::Constraint,
    widgets::{Clear, ListItem},
};

use crate::{
    keys::Action,
    matchers::Provider,
    state::ProviderState,
    ui::{PopupResult, dialog, dialog_sized, list},
};

pub fn render(frame: &mut Frame, state: &mut ProviderState) {
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
    frame.render_stateful_widget(list, area, &mut state.list_state);
}

pub fn keypressed(action: &Action, state: &mut ProviderState) -> PopupResult {
    match action {
        Action::Up => state.list_state.select_previous(),
        Action::Down => state.list_state.select_next(),
        Action::Enter => {
            if let Some(selected) = state.list_state.selected() {
                state.provider = Provider::ALL[selected];
            }
            return PopupResult::Close;
        }
        _ => {}
    }
    PopupResult::Consume
}
