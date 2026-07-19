use crate::apps::{AppAction, AppResponse};
use crate::desktop::theme::{Theme, ThemeKind};
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::input::{InputEvent, KeyState, PointerButton};
use crate::widgets::button::Button;

pub struct Settings {
    theme: ThemeKind,
    wallpaper: u8,
}

impl Settings {
    pub const fn new() -> Self {
        Self::from_appearance(ThemeKind::Dark, 0)
    }

    pub const fn from_appearance(theme: ThemeKind, wallpaper: u8) -> Self {
        Self {
            theme,
            wallpaper: wallpaper % 3,
        }
    }

    pub fn sync_appearance(&mut self, theme: ThemeKind, wallpaper: u8) {
        self.theme = theme;
        self.wallpaper = wallpaper % 3;
    }

    pub const fn theme(&self) -> ThemeKind {
        self.theme
    }

    pub const fn wallpaper(&self) -> u8 {
        self.wallpaper
    }

    pub fn toggle_theme(&mut self) -> AppAction {
        self.theme = self.theme.toggled();
        AppAction::SetTheme(self.theme)
    }

    pub fn cycle_wallpaper(&mut self) -> AppAction {
        self.wallpaper = (self.wallpaper + 1) % 3;
        AppAction::SetWallpaper(self.wallpaper)
    }

    pub fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        let InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            position,
        } = event
        else {
            return AppResponse::ignored();
        };
        if theme_button(bounds).contains(position) {
            AppResponse::consumed(self.toggle_theme())
        } else if wallpaper_button(bounds).contains(position) {
            AppResponse::consumed(self.cycle_wallpaper())
        } else {
            AppResponse::ignored()
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        painter.fill_rect(bounds, theme.window);
        draw_text(
            painter,
            bounds.x + 24,
            bounds.y + 30,
            "APPEARANCE",
            Color::rgb(38, 48, 66),
            2,
        );
        draw_text(
            painter,
            bounds.x + 24,
            bounds.y + 82,
            "COLOR THEME",
            Color::rgb(67, 79, 98),
            1,
        );
        Button {
            bounds: theme_button(bounds),
            label: match self.theme {
                ThemeKind::Dark => "DARK",
                ThemeKind::Light => "LIGHT",
            },
            destructive: false,
        }
        .draw(painter, theme);
        draw_text(
            painter,
            bounds.x + 24,
            bounds.y + 150,
            "WALLPAPER",
            Color::rgb(67, 79, 98),
            1,
        );
        Button {
            bounds: wallpaper_button(bounds),
            label: match self.wallpaper {
                0 => "AURORA",
                1 => "SLATE",
                _ => "SUNRISE",
            },
            destructive: false,
        }
        .draw(painter, theme);
        draw_text(
            painter,
            bounds.x + 24,
            bounds.bottom() - 28,
            "SETTINGS APPLY TO THE LIVE SHELL",
            Color::rgb(105, 117, 136),
            1,
        );
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

fn theme_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 210, bounds.y + 60, 128, 38)
}

fn wallpaper_button(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 210, bounds.y + 128, 128, 38)
}
