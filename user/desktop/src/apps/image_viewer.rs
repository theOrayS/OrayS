use alloc::string::{String, ToString};

use crate::apps::{AppAction, AppResponse, fs_error_label};
use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::image::{Bitmap, ImageError};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::filesystem::{self, FsError};
use crate::platform::input::{InputEvent, KeyState, PointerButton};
use crate::widgets::button::Button;

pub const MAX_IMAGE_FILE_BYTES: usize = 64 * 1024 * 1024;

pub struct ImageViewer {
    path: Option<String>,
    bitmap: Option<Bitmap>,
    zoom_percent: u32,
    error: Option<String>,
}

impl ImageViewer {
    pub const fn new() -> Self {
        Self {
            path: None,
            bitmap: None,
            zoom_percent: 100,
            error: None,
        }
    }

    pub fn open(&mut self, path: &str) -> Result<(), ImageOpenError> {
        let bytes =
            filesystem::read_bytes_bounded(path, MAX_IMAGE_FILE_BYTES).map_err(|error| {
                self.error = Some(fs_error_label(&error));
                ImageOpenError::Filesystem(error)
            })?;
        let bitmap = Bitmap::parse_ppm(&bytes).map_err(|error| {
            self.error = Some(alloc::format!("IMAGE ERROR: {error:?}"));
            ImageOpenError::Image(error)
        })?;
        self.path = Some(path.to_string());
        self.bitmap = Some(bitmap);
        self.zoom_percent = 100;
        self.error = None;
        Ok(())
    }

    pub const fn zoom_percent(&self) -> u32 {
        self.zoom_percent
    }

    pub fn bitmap(&self) -> Option<&Bitmap> {
        self.bitmap.as_ref()
    }

    pub fn zoom_in(&mut self) {
        self.zoom_percent = self.zoom_percent.saturating_add(25).min(400);
    }

    pub fn zoom_out(&mut self) {
        self.zoom_percent = self.zoom_percent.saturating_sub(25).max(25);
    }

    pub fn fit(&mut self) {
        self.zoom_percent = 100;
    }

    pub fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        match event {
            InputEvent::PointerButton {
                button: PointerButton::Left,
                state: KeyState::Pressed,
                position,
            } => {
                if minus_button(bounds).contains(position) {
                    self.zoom_out();
                } else if plus_button(bounds).contains(position) {
                    self.zoom_in();
                } else if fit_button(bounds).contains(position) {
                    self.fit();
                } else {
                    return AppResponse::ignored();
                }
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                code: 13,
                state: KeyState::Pressed,
                ..
            } => {
                self.zoom_in();
                AppResponse::consumed(AppAction::None)
            }
            InputEvent::Key {
                code: 12,
                state: KeyState::Pressed,
                ..
            } => {
                self.zoom_out();
                AppResponse::consumed(AppAction::None)
            }
            _ => AppResponse::ignored(),
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        painter.fill_rect(bounds, Color::rgb(36, 41, 50));
        painter.fill_rect(
            Rect::new(bounds.x, bounds.y, bounds.width, 38),
            Color::rgb(222, 228, 238),
        );
        Button {
            bounds: minus_button(bounds),
            label: "-",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: plus_button(bounds),
            label: "+",
            destructive: false,
        }
        .draw(painter, theme);
        Button {
            bounds: fit_button(bounds),
            label: "FIT",
            destructive: false,
        }
        .draw(painter, theme);
        draw_text(
            painter,
            bounds.x + 174,
            bounds.y + 15,
            &alloc::format!("{} PERCENT", self.zoom_percent),
            Color::rgb(42, 52, 68),
            1,
        );
        let viewport = Rect::new(
            bounds.x + 12,
            bounds.y + 50,
            bounds.width.saturating_sub(24),
            bounds.height.saturating_sub(62),
        );
        if let Some(bitmap) = &self.bitmap {
            bitmap.draw_fit(painter, viewport, self.zoom_percent);
        } else {
            draw_text(
                painter,
                viewport.x + 20,
                viewport.y + 24,
                "NO PPM IMAGE OPEN",
                Color::rgb(205, 212, 224),
                1,
            );
        }
        if let Some(error) = &self.error {
            draw_text(
                painter,
                viewport.x + 20,
                viewport.y + 48,
                error,
                Color::rgb(244, 132, 136),
                1,
            );
        }
    }
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageOpenError {
    Filesystem(FsError),
    Image(ImageError),
}

fn minus_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 8, bounds.y + 5, 42, 28)
}

fn plus_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 56, bounds.y + 5, 42, 28)
}

fn fit_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 104, bounds.y + 5, 58, 28)
}
