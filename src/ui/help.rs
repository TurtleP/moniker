use crate::{
    keys::{Action, BINDINGS},
    ui::{PopupResult, dialog, panel},
};

use ratatui::{
    Frame,
    layout::Constraint,
    style::{Color, Stylize},
    widgets::{Clear, Row, Table},
};

const CONSTRAINTS: &[Constraint; 2] = &[Constraint::Length(9), Constraint::Min(0)];

pub fn render(frame: &mut Frame) {
    let area = dialog(frame);

    let rows = BINDINGS
        .iter()
        .map(|b| Row::new([b.label, b.desc]).fg(Color::White));

    let table = Table::new(rows, CONSTRAINTS)
        .column_spacing(2)
        .block(panel("Help"));

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

pub fn keypressed(action: &Action) -> PopupResult {
    match action {
        Action::Quit | Action::ToggleHelp => PopupResult::Close,
        _ => PopupResult::Consume,
    }
}
