use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::apps::{AppResponse, fs_error_label};
use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::filesystem;
use crate::platform::input::{InputEvent, KeyState};
use crate::widgets::text_input::TextInput;

const MAX_OUTPUT_LINES: usize = 128;
const MAX_HISTORY: usize = 32;

pub struct Terminal {
    current_directory: String,
    input: TextInput,
    output: Vec<String>,
    history: Vec<String>,
    history_index: Option<usize>,
    scroll: usize,
}

impl Terminal {
    pub fn new(current_directory: &str) -> Self {
        Self {
            current_directory: current_directory.to_string(),
            input: TextInput::new(256),
            output: alloc::vec![
                "ORAYS DESKTOP TERMINAL".to_string(),
                "TYPE HELP FOR COMMANDS".to_string()
            ],
            history: Vec::new(),
            history_index: None,
            scroll: 0,
        }
    }

    pub fn current_directory(&self) -> &str {
        &self.current_directory
    }

    pub fn output(&self) -> &[String] {
        &self.output
    }

    pub fn execute_command(&mut self, command: &str) {
        let command = command.trim();
        if command.is_empty() {
            return;
        }
        self.push_output(alloc::format!("> {command}"));
        self.history.push(command.to_string());
        if self.history.len() > MAX_HISTORY {
            self.history.remove(0);
        }
        self.history_index = None;
        let mut words = command.split_whitespace();
        match words.next().unwrap_or_default() {
            "help" => self.push_output("HELP PWD LS CD CAT MKDIR WRITE ECHO CLEAR".to_string()),
            "pwd" => self.push_output(self.current_directory.clone()),
            "clear" => self.output.clear(),
            "echo" => self.push_output(words.collect::<Vec<_>>().join(" ")),
            "ls" => {
                let path = words
                    .next()
                    .map(|path| self.resolve_path(path))
                    .unwrap_or_else(|| Ok(self.current_directory.clone()));
                match path.and_then(|path| filesystem::read_directory(&path)) {
                    Ok(entries) if entries.is_empty() => {
                        self.push_output("EMPTY DIRECTORY".to_string())
                    }
                    Ok(entries) => {
                        for entry in entries {
                            let marker = if entry.is_directory { "DIR " } else { "FILE" };
                            self.push_output(alloc::format!(
                                "{marker} {} {}",
                                entry.name,
                                entry.size
                            ));
                        }
                    }
                    Err(error) => self.push_output(fs_error_label(&error)),
                }
            }
            "cd" => {
                let Some(path) = words.next() else {
                    self.push_output("CD REQUIRES A PATH".to_string());
                    return;
                };
                let resolved = self.resolve_path(path);
                match resolved.and_then(|path| filesystem::read_directory(&path).map(|_| path)) {
                    Ok(path) => self.current_directory = path,
                    Err(error) => self.push_output(fs_error_label(&error)),
                }
            }
            "cat" => {
                let Some(path) = words.next() else {
                    self.push_output("CAT REQUIRES A PATH".to_string());
                    return;
                };
                match self
                    .resolve_path(path)
                    .and_then(|path| filesystem::read_text(&path))
                {
                    Ok(text) => {
                        for line in text.lines() {
                            self.push_output(line.to_string());
                        }
                        if text.is_empty() {
                            self.push_output("EMPTY FILE".to_string());
                        }
                    }
                    Err(error) => self.push_output(fs_error_label(&error)),
                }
            }
            "mkdir" => {
                let Some(name) = words.next() else {
                    self.push_output("MKDIR REQUIRES A NAME".to_string());
                    return;
                };
                match filesystem::join(&self.current_directory, name)
                    .and_then(|path| filesystem::create_directory(&path))
                {
                    Ok(()) => self.push_output("DIRECTORY CREATED".to_string()),
                    Err(error) => self.push_output(fs_error_label(&error)),
                }
            }
            "write" => {
                let Some(name) = words.next() else {
                    self.push_output("WRITE REQUIRES A NAME AND TEXT".to_string());
                    return;
                };
                let contents = words.collect::<Vec<_>>().join(" ");
                match filesystem::join(&self.current_directory, name)
                    .and_then(|path| filesystem::write_text(&path, &contents))
                {
                    Ok(()) => self.push_output("FILE WRITTEN".to_string()),
                    Err(error) => self.push_output(fs_error_label(&error)),
                }
            }
            unknown => self.push_output(alloc::format!("COMMAND NOT SUPPORTED: {unknown}")),
        }
        self.scroll = 0;
    }

    pub fn handle_input(&mut self, event: InputEvent, _bounds: Rect) -> AppResponse {
        match event {
            InputEvent::Key {
                code: 28,
                state: KeyState::Pressed,
                ..
            } => {
                let command = self.input.value().to_string();
                self.input.clear();
                self.execute_command(&command);
                AppResponse::consumed(crate::apps::AppAction::None)
            }
            InputEvent::Key {
                code: 103,
                state: KeyState::Pressed,
                ..
            } => {
                if !self.history.is_empty() {
                    let index = self
                        .history_index
                        .map_or(self.history.len() - 1, |index| index.saturating_sub(1));
                    self.history_index = Some(index);
                    self.input = TextInput::with_value(&self.history[index], 256);
                }
                AppResponse::consumed(crate::apps::AppAction::None)
            }
            InputEvent::Scroll { lines, .. } => {
                if lines > 0 {
                    self.scroll = self
                        .scroll
                        .saturating_add(lines as usize)
                        .min(self.output.len());
                } else {
                    self.scroll = self.scroll.saturating_sub(lines.unsigned_abs() as usize);
                }
                AppResponse::consumed(crate::apps::AppAction::None)
            }
            _ if self.input.handle_input(event) => {
                AppResponse::consumed(crate::apps::AppAction::None)
            }
            _ => AppResponse::ignored(),
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, _theme: Theme) {
        painter.fill_rect(bounds, Color::rgb(20, 25, 34));
        let input_height = 32;
        let output_bounds = Rect::new(
            bounds.x + 10,
            bounds.y + 10,
            bounds.width.saturating_sub(20),
            bounds.height.saturating_sub(input_height + 20),
        );
        let visible = (output_bounds.height / 14) as usize;
        let end = self
            .output
            .len()
            .saturating_sub(self.scroll)
            .min(self.output.len());
        let start = end.saturating_sub(visible);
        for (line, text) in self.output[start..end].iter().enumerate() {
            draw_text(
                painter,
                output_bounds.x,
                output_bounds.y + (line as i32 * 14),
                text,
                Color::rgb(199, 224, 210),
                1,
            );
        }
        let input_bounds = Rect::new(
            bounds.x + 10,
            bounds.bottom() - 38,
            bounds.width.saturating_sub(20),
            28,
        );
        draw_text(
            painter,
            input_bounds.x,
            input_bounds.y + 10,
            ">",
            Color::rgb(92, 211, 160),
            1,
        );
        self.input.draw(
            painter,
            Rect::new(
                input_bounds.x + 14,
                input_bounds.y,
                input_bounds.width.saturating_sub(14),
                input_bounds.height,
            ),
            Theme::default(),
            true,
        );
    }

    fn push_output(&mut self, line: String) {
        self.output.push(line);
        if self.output.len() > MAX_OUTPUT_LINES {
            self.output.remove(0);
        }
    }

    fn resolve_path(&self, path: &str) -> Result<String, filesystem::FsError> {
        match path {
            "." => Ok(self.current_directory.clone()),
            ".." => Ok(filesystem::parent(&self.current_directory)),
            absolute if absolute.starts_with('/') => Ok(absolute.to_string()),
            name => filesystem::join(&self.current_directory, name),
        }
    }
}
