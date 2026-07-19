use alloc::string::{String, ToString};

use crate::apps::{AppAction, AppResponse, fs_error_label};
use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::filesystem::{self, FsError};
use crate::platform::input::{InputEvent, KeyState, PointerButton};
use crate::widgets::{button::Button, dialog};

const MAX_EDITOR_BYTES: usize = filesystem::MAX_TEXT_BYTES;

pub struct TextEditor {
    path: Option<String>,
    contents: String,
    dirty: bool,
    error: Option<String>,
    confirm_close: bool,
    scroll_line: usize,
}

impl TextEditor {
    pub const fn new() -> Self {
        Self {
            path: None,
            contents: String::new(),
            dirty: false,
            error: None,
            confirm_close: false,
            scroll_line: 0,
        }
    }

    pub fn open(&mut self, path: &str) -> Result<(), FsError> {
        match filesystem::read_text(path) {
            Ok(contents) => {
                self.path = Some(path.to_string());
                self.contents = contents;
                self.dirty = false;
                self.error = None;
                self.confirm_close = false;
                Ok(())
            }
            Err(error) => {
                self.error = Some(fs_error_label(&error));
                Err(error)
            }
        }
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = Some(path.to_string());
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub const fn dirty(&self) -> bool {
        self.dirty
    }

    pub const fn close_prompt_visible(&self) -> bool {
        self.confirm_close
    }

    pub fn insert(&mut self, character: char) -> bool {
        if self.contents.len().saturating_add(character.len_utf8()) > MAX_EDITOR_BYTES {
            self.error = Some("DOCUMENT TOO LARGE".to_string());
            return false;
        }
        self.contents.push(character);
        self.dirty = true;
        true
    }

    pub fn save(&mut self) -> Result<(), FsError> {
        let Some(path) = self.path.as_deref() else {
            let error = FsError::invalid("save", "");
            self.error = Some("SAVE REQUIRES A FILE PATH".to_string());
            return Err(error);
        };
        filesystem::write_text(path, &self.contents).inspect_err(|error| {
            self.error = Some(fs_error_label(error));
        })?;
        self.dirty = false;
        self.error = None;
        Ok(())
    }

    pub fn request_close(&mut self) -> bool {
        if self.dirty {
            self.confirm_close = true;
            true
        } else {
            false
        }
    }

    pub fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        if self.confirm_close {
            return self.handle_dialog_input(event, bounds);
        }
        match event {
            InputEvent::Key {
                code: 31,
                state: KeyState::Pressed,
                modifiers,
                ..
            } if modifiers.control => {
                let _ = self.save();
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                code: 14,
                state: KeyState::Pressed | KeyState::Repeated,
                ..
            } => {
                if self.contents.pop().is_some() {
                    self.dirty = true;
                }
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                code: 28,
                state: KeyState::Pressed | KeyState::Repeated,
                ..
            } => {
                self.insert('\n');
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                state: KeyState::Pressed | KeyState::Repeated,
                text: Some(character),
                ..
            } => {
                self.insert(character);
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Scroll { lines, .. } => {
                if lines < 0 {
                    self.scroll_line = self
                        .scroll_line
                        .saturating_add(lines.unsigned_abs() as usize);
                } else {
                    self.scroll_line = self.scroll_line.saturating_sub(lines as usize);
                }
                AppResponse::consumed(AppAction::None)
            }
            _ => AppResponse::ignored(),
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        painter.fill_rect(bounds, theme.window);
        let toolbar = Rect::new(bounds.x, bounds.y, bounds.width, 34);
        painter.fill_rect(toolbar, Color::rgb(224, 230, 241));
        draw_text(
            painter,
            bounds.x + 10,
            bounds.y + 13,
            self.path
                .as_deref()
                .unwrap_or("UNTITLED - SET PATH TO SAVE"),
            Color::rgb(44, 54, 73),
            1,
        );
        if self.dirty {
            draw_text(
                painter,
                bounds.right() - 66,
                bounds.y + 13,
                "UNSAVED",
                Color::rgb(184, 82, 35),
                1,
            );
        }
        let text_bounds = Rect::new(
            bounds.x + 12,
            bounds.y + 45,
            bounds.width.saturating_sub(24),
            bounds.height.saturating_sub(72),
        );
        let visible_lines = (text_bounds.height / 14) as usize;
        for (index, line) in self
            .contents
            .lines()
            .skip(self.scroll_line)
            .take(visible_lines)
            .enumerate()
        {
            draw_text(
                painter,
                text_bounds.x,
                text_bounds.y + index as i32 * 14,
                line,
                Color::rgb(37, 48, 66),
                1,
            );
        }
        if self.contents.is_empty() {
            draw_text(
                painter,
                text_bounds.x,
                text_bounds.y,
                "START TYPING",
                Color::rgb(130, 140, 156),
                1,
            );
        }
        if let Some(error) = &self.error {
            draw_text(
                painter,
                bounds.x + 10,
                bounds.bottom() - 16,
                error,
                Color::rgb(154, 44, 52),
                1,
            );
        }
        if self.confirm_close {
            let prompt = dialog::centered(bounds, 330, 132);
            dialog::draw(
                painter,
                bounds,
                prompt,
                "SAVE CHANGES BEFORE CLOSING",
                theme,
            );
            Button {
                bounds: save_button(prompt),
                label: "SAVE",
                destructive: false,
            }
            .draw(painter, theme);
            Button {
                bounds: discard_button(prompt),
                label: "DISCARD",
                destructive: true,
            }
            .draw(painter, theme);
            Button {
                bounds: cancel_button(prompt),
                label: "CANCEL",
                destructive: false,
            }
            .draw(painter, theme);
        }
    }

    fn handle_dialog_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        let prompt = dialog::centered(bounds, 330, 132);
        match event {
            InputEvent::Key {
                code: 1,
                state: KeyState::Pressed,
                ..
            } => {
                self.confirm_close = false;
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::PointerButton {
                button: PointerButton::Left,
                state: KeyState::Pressed,
                position,
            } => {
                if save_button(prompt).contains(position) {
                    if self.save().is_ok() {
                        self.confirm_close = false;
                        return AppResponse::consumed(AppAction::CloseWindow);
                    }
                } else if discard_button(prompt).contains(position) {
                    self.confirm_close = false;
                    self.dirty = false;
                    return AppResponse::consumed(AppAction::CloseWindow);
                } else if cancel_button(prompt).contains(position) {
                    self.confirm_close = false;
                }
                AppResponse::consumed(AppAction::None)
            }
            _ => AppResponse::consumed(AppAction::None),
        }
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

fn save_button(dialog: Rect) -> Rect {
    Rect::new(dialog.x + 14, dialog.bottom() - 48, 82, 32)
}

fn discard_button(dialog: Rect) -> Rect {
    Rect::new(dialog.x + 104, dialog.bottom() - 48, 104, 32)
}

fn cancel_button(dialog: Rect) -> Rect {
    Rect::new(dialog.x + 216, dialog.bottom() - 48, 98, 32)
}
