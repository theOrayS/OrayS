use crate::platform::input::{InputEvent, KEY_TAB, KeyState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shortcut {
    ToggleLauncher,
    CloseLauncher,
    ToggleTheme,
    AltTab { reverse: bool },
}

pub fn resolve(event: InputEvent) -> Option<Shortcut> {
    let InputEvent::Key {
        code,
        state: KeyState::Pressed | KeyState::Repeated,
        modifiers,
        ..
    } = event
    else {
        return None;
    };
    match code {
        KEY_TAB if modifiers.alt => Some(Shortcut::AltTab {
            reverse: modifiers.shift,
        }),
        57 if modifiers.super_key => Some(Shortcut::ToggleLauncher),
        20 if modifiers.super_key => Some(Shortcut::ToggleTheme),
        1 => Some(Shortcut::CloseLauncher),
        _ => None,
    }
}
