use ratatui::{Frame, widgets::ListState};

use crate::{
    keys::Action,
    popup::{Popup, PopupResult},
};

#[derive(Default)]
pub struct ProviderPopup {
    list_state: ListState,
}

impl Popup for ProviderPopup {
    fn render(&mut self, frame: &mut Frame) {
        /* TODO: render */
    }

    fn keypressed(&self, action: &Action) -> PopupResult {
        PopupResult::Consume
    }
}

impl ProviderPopup {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
        }
    }
}
