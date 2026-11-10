use ratatui::{widgets::{List, ListItem, HighlightSpacing, Block, BorderType, Borders}, style::{Style, Color}};

pub mod layout; 

fn panel(title: impl ToString) -> Block<'static> {
    Block::default()
        .title(title.to_string())
        .title_style(Style::default().fg(Color::LightCyan))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL)
}

fn create_list<'items>(items: Vec<ListItem<'items>>, title: impl ToString) -> List<'items> {
    List::new(items)
        .block(panel(title))
        .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
        .highlight_spacing(HighlightSpacing::Always)
}
