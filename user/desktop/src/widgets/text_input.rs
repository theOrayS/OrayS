use alloc::string::String;

use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::input::{InputEvent, KeyState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInput {
    value: String,
    maximum_bytes: usize,
}

impl TextInput {
    pub fn new(maximum_bytes: usize) -> Self {
        Self {
            value: String::new(),
            maximum_bytes,
        }
    }

    pub fn with_value(value: &str, maximum_bytes: usize) -> Self {
        let mut input = Self::new(maximum_bytes);
        for character in value.chars() {
            input.push(character);
        }
        input
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn push(&mut self, character: char) -> bool {
        if character.is_control()
            || self.value.len().saturating_add(character.len_utf8()) > self.maximum_bytes
        {
            return false;
        }
        self.value.push(character);
        true
    }

    pub fn backspace(&mut self) -> bool {
        self.value.pop().is_some()
    }

    pub fn handle_input(&mut self, event: InputEvent) -> bool {
        let InputEvent::Key {
            code,
            state: KeyState::Pressed | KeyState::Repeated,
            text,
            ..
        } = event
        else {
            return false;
        };
        if code == 14 {
            self.backspace()
        } else {
            text.is_some_and(|character| self.push(character))
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme, focused: bool) {
        painter.fill_rounded_rect(bounds, 6, Color::rgba(255, 255, 255, 220));
        painter.stroke_rect(
            bounds,
            1,
            if focused {
                theme.accent
            } else {
                theme.panel_border
            },
        );
        let available = bounds.width.saturating_sub(22) as usize / 6;
        let start = self.value.len().saturating_sub(available);
        let visible = self.value.get(start..).unwrap_or(&self.value);
        draw_text(
            painter,
            bounds.x + 8,
            bounds.y + 9,
            visible,
            Color::rgb(35, 45, 62),
            1,
        );
        if focused {
            let width = (visible.len().min(available) * 6) as i32;
            painter.fill_rect(
                Rect::new(
                    bounds.x + 8 + width,
                    bounds.y + 7,
                    1,
                    bounds.height.saturating_sub(14),
                ),
                theme.accent,
            );
        }
    }
}
