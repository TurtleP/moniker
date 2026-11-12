use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Constraint, widgets::Clear};

use crate::{
    keys::Action,
    state::MetadataState,
    ui::{PopupResult, dialog_sized, input, panel},
};

pub fn render(frame: &mut Frame, state: &mut MetadataState) {
    let area = dialog_sized(frame, Constraint::Percentage(50), Constraint::Length(3));
    let input = input(&state.search_query).block(panel("Match"));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

pub fn send_input(key: &KeyCode, state: &mut MetadataState) {
    match key {
        KeyCode::Char(ch) => state.search_query.push(*ch),
        KeyCode::Backspace => {
            state.search_query.pop();
        }
        _ => {}
    }
}

pub fn keypressed(action: &Action, state: &mut MetadataState) -> PopupResult {
    if matches!(action, Action::Enter) {
        return PopupResult::Close;
    }
    PopupResult::Consume
}
