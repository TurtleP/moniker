use ratatui::Frame;

use crate::{
    keys::Action,
    popup::{Popup, PopupResult},
};

#[derive(Default)]
pub struct HelpPopup;

impl Popup for HelpPopup {
    fn render(&mut self, frame: &mut Frame) {
        /* TODO: render */
    }

    fn keypressed(&self, action: &Action) -> PopupResult {
        match action {
            Action::Quit | Action::ToggleHelp => PopupResult::Close,
            _ => PopupResult::Consume,
        }
    }
}
