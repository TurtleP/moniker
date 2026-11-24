use ratatui::{
    Frame,
    layout::HorizontalAlignment,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::layout::AppLayout;

pub fn render(frame: &mut Frame, layout: &mut AppLayout) {
    let list = List::new(vec![ListItem::from("Test!")]).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Files")
            .title_alignment(HorizontalAlignment::Center),
    );
    frame.render_stateful_widget(list, layout.files_list, &mut layout.state);
}
