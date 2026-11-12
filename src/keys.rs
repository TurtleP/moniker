use std::fmt::Display;

use crossterm::event::KeyCode;
use strum::Display;

#[derive(Clone, Copy, PartialEq, Display)]
pub(crate) enum Action {
    Quit,
    Rescan,
    Up,
    Down,
    ToggleHelp,
    Match,
    Rename,
    ChangeSource,
    RenameAll,
    Enter,
}

pub struct KeyEvent {
    pub keys: &'static [KeyCode],
    pub action: Action,
}

impl Display for KeyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.action)
    }
}

pub struct Binding {
    pub event: KeyEvent,
    pub label: &'static str,
    pub desc: &'static str,
}

pub const BINDINGS: &[Binding] = &[
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Up, KeyCode::Char('k')],
            action: Action::Up,
        },
        label: "↑ / k",
        desc: "Select previous file",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Down, KeyCode::Char('j')],
            action: Action::Down,
        },
        label: "↓ / j",
        desc: "Select next file",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('r')],
            action: Action::Rescan,
        },
        label: "r",
        desc: "Rescan directory",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::F(1), KeyCode::Char('?')],
            action: Action::ToggleHelp,
        },
        label: "F1 / ?",
        desc: "Toggle help",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('q')],
            action: Action::Quit,
        },
        label: "q",
        desc: "Quit",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('m'), KeyCode::Char('/')],
            action: Action::Match,
        },
        label: "m / /",
        desc: "Match with source",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Enter],
            action: Action::Enter,
        },
        label: "enter",
        desc: "select item",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('n')],
            action: Action::Rename,
        },
        label: "n",
        desc: "Rename file",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('s')],
            action: Action::ChangeSource,
        },
        label: "s",
        desc: "Change source",
    },
    Binding {
        event: KeyEvent {
            keys: &[KeyCode::Char('a')],
            action: Action::RenameAll,
        },
        label: "a",
        desc: "Rename all files",
    },
];
