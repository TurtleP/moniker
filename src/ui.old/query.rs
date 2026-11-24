use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Position},
    widgets::Clear,
};

use crate::{
    keys::Action,
    ui::{PopupResult, dialog_sized, input, panel},
};

pub fn render(frame: &mut Frame, search_query: &mut str, editing: bool) {
    let area = dialog_sized(frame, Constraint::Percentage(50), Constraint::Length(3));

    if editing {
        let x = (area.x + 1).saturating_add(search_query.len() as u16);
        let cursor_position = Position::new(x, area.y + 1);
        frame.set_cursor_position(cursor_position);
    }

    let input = input(search_query).block(panel("Match"));

    frame.render_widget(Clear, area);
    frame.render_widget(input, area);
}

pub fn send_input(key: &KeyCode, search_query: &mut String) {
    match key {
        KeyCode::Char(char) => search_query.push(*char),
        KeyCode::Backspace => {
            search_query.pop();
        }
        _ => {}
    }
}

pub fn keypressed(action: &Action) -> PopupResult {
    match action {
        Action::Enter => PopupResult::Confirm,
        Action::Back => PopupResult::Close,
        _ => PopupResult::Consume,
    }
}
