use ratatui::Frame;

use crate::{
    keys::Action,
    popup::{Popup, PopupKind, PopupResult, help::HelpPopup, provider::ProviderPopup},
};

#[derive(Default)]
pub struct PopupManager {
    active: Option<Box<dyn Popup>>,
}

impl PopupManager {
    pub fn open(&mut self, kind: PopupKind) {
        let active: Box<dyn Popup> = match kind {
            PopupKind::Help => Box::new(HelpPopup::default()),
            PopupKind::Provider => Box::new(ProviderPopup::default()),
            _ => todo!(),
        };
        self.active = Some(active);
    }

    pub fn is_open(&self) -> bool {
        self.active.is_some()
    }

    pub fn close(&mut self) {
        self.active = None;
    }

    pub fn render(&mut self, frame: &mut Frame) {
        if let Some(dialog) = &mut self.active {
            dialog.render(frame);
        }
    }

    pub fn keypressed(&self, action: &Action) -> PopupResult {
        todo!();
        PopupResult::Close
    }
}
