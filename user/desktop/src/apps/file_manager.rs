use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::apps::{AppAction, AppResponse, fs_error_label, uppercase_extension};
use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::filesystem::{self, DirectoryEntry, FsError};
use crate::platform::input::{InputEvent, KeyState, PointerButton};
use crate::widgets::{button::Button, dialog, list, scrollbar, text_input::TextInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilePromptKind {
    CreateDirectory,
    Rename,
    Delete,
}

struct FilePrompt {
    kind: FilePromptKind,
    input: TextInput,
}

pub struct FileManager {
    current_directory: String,
    entries: Vec<DirectoryEntry>,
    selected: Option<usize>,
    offset: usize,
    error: Option<String>,
    prompt: Option<FilePrompt>,
}

impl FileManager {
    pub fn new(current_directory: &str) -> Self {
        let mut manager = Self {
            current_directory: current_directory.to_string(),
            entries: Vec::new(),
            selected: None,
            offset: 0,
            error: None,
            prompt: None,
        };
        let _ = manager.refresh();
        manager
    }

    pub fn current_directory(&self) -> &str {
        &self.current_directory
    }

    pub fn entries(&self) -> &[DirectoryEntry] {
        &self.entries
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn select(&mut self, index: usize) -> bool {
        if index >= self.entries.len() {
            return false;
        }
        self.selected = Some(index);
        true
    }

    pub fn refresh(&mut self) -> Result<(), FsError> {
        match filesystem::read_directory(&self.current_directory) {
            Ok(entries) => {
                self.entries = entries;
                self.selected = None;
                self.offset = 0;
                self.error = None;
                Ok(())
            }
            Err(error) => {
                self.error = Some(fs_error_label(&error));
                Err(error)
            }
        }
    }

    pub fn create_directory(&mut self, name: &str) -> Result<(), FsError> {
        let path = filesystem::join(&self.current_directory, name)?;
        filesystem::create_directory(&path).inspect_err(|error| {
            self.error = Some(fs_error_label(error));
        })?;
        self.refresh()
    }

    pub fn rename_selected(&mut self, name: &str) -> Result<(), FsError> {
        let Some(index) = self.selected else {
            let error = FsError::invalid("rename", "");
            self.error = Some("NO FILE SELECTED".to_string());
            return Err(error);
        };
        let destination = filesystem::join(&self.current_directory, name)?;
        let source = self.entries[index].path.clone();
        filesystem::rename(&source, &destination).inspect_err(|error| {
            self.error = Some(fs_error_label(error));
        })?;
        self.refresh()
    }

    pub fn delete_selected(&mut self) -> Result<(), FsError> {
        let Some(index) = self.selected else {
            let error = FsError::invalid("remove", "");
            self.error = Some("NO FILE SELECTED".to_string());
            return Err(error);
        };
        let entry = self.entries[index].clone();
        filesystem::remove(&entry.path, entry.is_directory).inspect_err(|error| {
            self.error = Some(fs_error_label(error));
        })?;
        self.refresh()
    }

    pub fn open_selected(&mut self) -> AppAction {
        let Some(entry) = self
            .selected
            .and_then(|index| self.entries.get(index))
            .cloned()
        else {
            self.error = Some("NO FILE SELECTED".to_string());
            return AppAction::None;
        };
        if entry.is_directory {
            self.current_directory = entry.path;
            let _ = self.refresh();
            AppAction::None
        } else if matches!(uppercase_extension(&entry.path).as_str(), "PPM" | "PNM") {
            AppAction::OpenImage(entry.path)
        } else {
            AppAction::OpenEditor(entry.path)
        }
    }

    pub fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        if self.prompt.is_some() {
            return self.handle_prompt_input(event, bounds);
        }
        let rows = rows_rect(bounds);
        match event {
            InputEvent::PointerButton {
                button: PointerButton::Left,
                state: KeyState::Pressed,
                position,
            } => {
                if up_button(bounds).contains(position) {
                    self.current_directory = filesystem::parent(&self.current_directory);
                    let _ = self.refresh();
                    return AppResponse::consumed(AppAction::None);
                }
                if refresh_button(bounds).contains(position) {
                    let _ = self.refresh();
                    return AppResponse::consumed(AppAction::None);
                }
                if new_button(bounds).contains(position) {
                    self.prompt = Some(FilePrompt {
                        kind: FilePromptKind::CreateDirectory,
                        input: TextInput::new(96),
                    });
                    return AppResponse::consumed(AppAction::None);
                }
                if rename_button(bounds).contains(position) {
                    if let Some(entry) = self.selected.and_then(|index| self.entries.get(index)) {
                        self.prompt = Some(FilePrompt {
                            kind: FilePromptKind::Rename,
                            input: TextInput::with_value(&entry.name, 96),
                        });
                    } else {
                        self.error = Some("NO FILE SELECTED".to_string());
                    }
                    return AppResponse::consumed(AppAction::None);
                }
                if delete_button(bounds).contains(position) {
                    if self.selected.is_some() {
                        self.prompt = Some(FilePrompt {
                            kind: FilePromptKind::Delete,
                            input: TextInput::new(0),
                        });
                    } else {
                        self.error = Some("NO FILE SELECTED".to_string());
                    }
                    return AppResponse::consumed(AppAction::None);
                }
                if let Some(index) = list::hit_row(rows, position, self.offset, self.entries.len())
                {
                    self.selected = Some(index);
                    return AppResponse::consumed(AppAction::None);
                }
                AppResponse::ignored()
            }
            InputEvent::Key {
                code: 28,
                state: KeyState::Pressed,
                ..
            } => AppResponse::consumed(self.open_selected()),
            InputEvent::Key {
                code: 103,
                state: KeyState::Pressed,
                ..
            } => {
                self.selected = Some(self.selected.unwrap_or(1).saturating_sub(1));
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                code: 108,
                state: KeyState::Pressed,
                ..
            } => {
                if !self.entries.is_empty() {
                    self.selected =
                        Some((self.selected.unwrap_or(0) + 1).min(self.entries.len() - 1));
                }
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Scroll { lines, .. } => {
                let visible = (rows.height / list::ROW_HEIGHT) as usize;
                let maximum = self.entries.len().saturating_sub(visible);
                if lines < 0 {
                    self.offset = self
                        .offset
                        .saturating_add(lines.unsigned_abs() as usize)
                        .min(maximum);
                } else {
                    self.offset = self.offset.saturating_sub(lines as usize);
                }
                AppResponse::consumed(AppAction::None)
            }
            _ => AppResponse::ignored(),
        }
    }

    fn handle_prompt_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        let prompt_bounds = dialog::centered(bounds, 360, 144);
        let execute = matches!(
            event,
            InputEvent::Key {
                code: 28,
                state: KeyState::Pressed,
                ..
            }
        ) || matches!(
            event,
            InputEvent::PointerButton {
                button: PointerButton::Left,
                state: KeyState::Pressed,
                position,
            } if prompt_ok_button(prompt_bounds).contains(position)
        );
        let cancel = matches!(
            event,
            InputEvent::Key {
                code: 1,
                state: KeyState::Pressed,
                ..
            }
        ) || matches!(
            event,
            InputEvent::PointerButton {
                button: PointerButton::Left,
                state: KeyState::Pressed,
                position,
            } if prompt_cancel_button(prompt_bounds).contains(position)
        );
        if cancel {
            self.prompt = None;
            return AppResponse::consumed(AppAction::None);
        }
        if execute {
            let prompt = self
                .prompt
                .take()
                .expect("prompt exists while handling prompt input");
            let result = match prompt.kind {
                FilePromptKind::CreateDirectory => self.create_directory(prompt.input.value()),
                FilePromptKind::Rename => self.rename_selected(prompt.input.value()),
                FilePromptKind::Delete => self.delete_selected(),
            };
            if let Err(error) = result {
                self.error = Some(fs_error_label(&error));
            }
            return AppResponse::consumed(AppAction::None);
        }
        if let Some(prompt) = &mut self.prompt
            && prompt.input.handle_input(event)
        {
            return AppResponse::consumed(AppAction::None);
        }
        AppResponse::consumed(AppAction::None)
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        painter.fill_rect(bounds, theme.window);
        let toolbar = Rect::new(bounds.x, bounds.y, bounds.width, 42);
        painter.fill_rect(toolbar, Color::rgb(224, 230, 241));
        Button {
            bounds: up_button(bounds),
            label: "UP",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: refresh_button(bounds),
            label: "REF",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: new_button(bounds),
            label: "NEW",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: rename_button(bounds),
            label: "RENAME",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: delete_button(bounds),
            label: "DELETE",
            destructive: true,
        }
        .draw(painter, theme);
        draw_text(
            painter,
            bounds.x + 326,
            bounds.y + 17,
            &self.current_directory,
            Color::rgb(43, 54, 74),
            1,
        );
        let labels = list::labels(self.entries.iter().map(|entry| {
            alloc::format!(
                "{} {} {}",
                if entry.is_directory { "DIR " } else { "FILE" },
                entry.name,
                entry.size
            )
        }));
        let rows = rows_rect(bounds);
        list::draw_rows(painter, rows, &labels, self.selected, self.offset, theme);
        scrollbar::draw(
            painter,
            Rect::new(rows.right() - 8, rows.y, 6, rows.height),
            labels.len(),
            (rows.height / list::ROW_HEIGHT) as usize,
            self.offset,
            theme,
        );
        if let Some(error) = &self.error {
            painter.fill_rect(
                Rect::new(bounds.x, bounds.bottom() - 25, bounds.width, 25),
                Color::rgb(255, 225, 226),
            );
            draw_text(
                painter,
                bounds.x + 8,
                bounds.bottom() - 16,
                error,
                Color::rgb(142, 39, 48),
                1,
            );
        }
        if let Some(prompt) = &self.prompt {
            let prompt_bounds = dialog::centered(bounds, 360, 144);
            let title = match prompt.kind {
                FilePromptKind::CreateDirectory => "CREATE DIRECTORY",
                FilePromptKind::Rename => "RENAME SELECTED ENTRY",
                FilePromptKind::Delete => "DELETE SELECTED ENTRY",
            };
            dialog::draw(painter, bounds, prompt_bounds, title, theme);
            if !matches!(prompt.kind, FilePromptKind::Delete) {
                prompt.input.draw(
                    painter,
                    Rect::new(
                        prompt_bounds.x + 16,
                        prompt_bounds.y + 48,
                        prompt_bounds.width.saturating_sub(32),
                        32,
                    ),
                    theme,
                    true,
                );
            } else if let Some(entry) = self.selected.and_then(|index| self.entries.get(index)) {
                draw_text(
                    painter,
                    prompt_bounds.x + 16,
                    prompt_bounds.y + 58,
                    &entry.name,
                    Color::rgb(134, 43, 52),
                    1,
                );
            }
            Button {
                bounds: prompt_ok_button(prompt_bounds),
                label: "OK",
                destructive: matches!(prompt.kind, FilePromptKind::Delete),
            }
            .draw(painter, theme);
            Button {
                bounds: prompt_cancel_button(prompt_bounds),
                label: "CANCEL",
                destructive: false,
            }
            .draw(painter, theme);
        }
    }
}

fn up_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 8, bounds.y + 7, 40, 28)
}

fn refresh_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 52, bounds.y + 7, 48, 28)
}

fn new_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 104, bounds.y + 7, 50, 28)
}

fn rename_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 158, bounds.y + 7, 76, 28)
}

fn delete_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 238, bounds.y + 7, 80, 28)
}

fn rows_rect(bounds: Rect) -> Rect {
    Rect::new(
        bounds.x + 8,
        bounds.y + 50,
        bounds.width.saturating_sub(20),
        bounds.height.saturating_sub(58),
    )
}

fn prompt_ok_button(dialog: Rect) -> Rect {
    Rect::new(dialog.right() - 174, dialog.bottom() - 46, 72, 30)
}

fn prompt_cancel_button(dialog: Rect) -> Rect {
    Rect::new(dialog.right() - 94, dialog.bottom() - 46, 78, 30)
}
