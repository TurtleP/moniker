use crossterm::event::KeyCode;
use ratatui::{Frame, layout::Constraint, widgets::Clear};

use crate::{
    keys::Action,
    ui::{PopupResult, dialog_sized, input, panel},
};

pub fn render(frame: &mut Frame, search_query: &mut String) {
    let area = dialog_sized(frame, Constraint::Percentage(50), Constraint::Length(3));
    let input = input(&search_query).block(panel("Match"));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

pub fn send_input(key: &KeyCode, search_query: &mut String) {
    match key {
        KeyCode::Char(ch) => search_query.push(*ch),
        KeyCode::Backspace => {
            search_query.pop();
        }
        _ => {}
    }
}

pub fn keypressed(action: &Action) -> PopupResult {
    if matches!(action, Action::Enter) {
        return PopupResult::Close;
    }
    PopupResult::Consume
}
