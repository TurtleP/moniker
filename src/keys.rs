use crossterm::event::KeyCode;

#[derive(Clone, Copy)]
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
}

pub(crate) struct Binding {
    pub(crate) keys: &'static [KeyCode],
    pub(crate) label: &'static str,
    pub(crate) desc: &'static str,
    pub(crate) action: Action,
}

pub(crate) const BINDINGS: &[Binding] = &[
    Binding {
        keys: &[KeyCode::Up, KeyCode::Char('k')],
        label: "↑ / k",
        desc: "Select previous file",
        action: Action::Up,
    },
    Binding {
        keys: &[KeyCode::Down, KeyCode::Char('j')],
        label: "↓ / j",
        desc: "Select next file",
        action: Action::Down,
    },
    Binding {
        keys: &[KeyCode::Char('r')],
        label: "r",
        desc: "Rescan directory",
        action: Action::Rescan,
    },
    Binding {
        keys: &[KeyCode::F(1), KeyCode::Char('?')],
        label: "F1 / ?",
        desc: "Toggle this help",
        action: Action::ToggleHelp,
    },
    Binding {
        keys: &[KeyCode::Char('q')],
        label: "q",
        desc: "Quit",
        action: Action::Quit,
    },
    Binding {
        keys: &[KeyCode::Char('m'), KeyCode::Char('/')],
        label: "m / /",
        desc: "Match with source",
        action: Action::Match,
    },
    Binding {
        keys: &[KeyCode::Char('n')],
        label: "n",
        desc: "Rename file",
        action: Action::Rename,
    },
    Binding {
        keys: &[KeyCode::Char('s')],
        label: "s",
        desc: "Change source",
        action: Action::ChangeSource,
    },
    Binding {
        keys: &[KeyCode::Char('a')],
        label: "a",
        desc: "Rename all files",
        action: Action::RenameAll,
    },
];
