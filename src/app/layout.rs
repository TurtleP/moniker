use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    macros::constraints,
    widgets::ListState,
};

pub struct AppLayout {
    pub files_list: Rect,
    pub rename_list: Rect,
    pub status_bar: Rect,
    pub state: ListState,
}

const V_SPLIT: [Constraint; 2] = constraints![>=0, ==1];
const H_SPLIT: [Constraint; 2] = constraints![==50%, ==50%];

impl AppLayout {
    pub fn new(frame: &mut Frame) -> Self {
        let [main_area, status_bar] = Layout::vertical(V_SPLIT).areas(frame.area());
        let [files_list, rename_list] = Layout::horizontal(H_SPLIT).areas(main_area);

        Self {
            files_list,
            rename_list,
            status_bar,
            state: ListState::default().with_selected(Some(0)),
        }
    }
}
