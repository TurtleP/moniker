use crate::{
    keys::Action,
    ui::{PopupResult, dialog_sized, panel},
};

use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment},
    widgets::{Clear, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, title: &str, text: &String) {
    let len = text.split("\n").count() as u16 + 3;
    let inner = dialog_sized(frame, Constraint::Percentage(35), Constraint::Length(len));
    let widget = Paragraph::new(text.to_string())
        .wrap(Wrap { trim: true })
        .centered()
        .block(panel(title).title_alignment(HorizontalAlignment::Center));
    frame.render_widget(Clear, inner);
    frame.render_widget(widget, inner);
}

pub fn keypressed(action: &Action) -> PopupResult {
    match action {
        Action::Enter => PopupResult::Close,
        _ => PopupResult::Consume,
    }
}
