use ratatui::Frame;

use crate::keys::Action;

pub mod help;
pub mod manager;
pub mod provider;

pub enum PopupKind {
    Help,
    Provider,
}

pub enum PopupResult {
    Confirm,
    Consume,
    Close,
}

pub trait Popup {
    fn render(&mut self, frame: &mut Frame);
    fn keypressed(&self, action: &Action) -> PopupResult;
    fn text_input(&mut self, _c: char) -> PopupResult {
        PopupResult::Consume
    }
}
