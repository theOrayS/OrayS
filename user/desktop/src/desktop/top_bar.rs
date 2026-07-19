use alloc::string::String;

use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

pub const TOP_BAR_HEIGHT: u32 = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarTarget {
    Launcher,
    Power,
}

pub struct TopBar;

impl TopBar {
    pub const fn new() -> Self {
        Self
    }

    pub fn hit_test(&self, bounds: Rect, point: Point) -> Option<TopBarTarget> {
        if launcher_rect(bounds).contains(point) {
            Some(TopBarTarget::Launcher)
        } else if power_rect(bounds).contains(point) {
            Some(TopBarTarget::Power)
        } else {
            None
        }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme, time: &str) {
        let bar = Rect::new(bounds.x, bounds.y, bounds.width, TOP_BAR_HEIGHT);
        painter.blend_rect(bar, theme.panel);
        painter.fill_rect(
            Rect::new(bar.x, bar.bottom() - 1, bar.width, 1),
            theme.panel_border,
        );
        let launcher = launcher_rect(bounds);
        painter.blend_rounded_rect(launcher, 10, Color::rgba(255, 255, 255, 22));
        draw_text(
            painter,
            launcher.x + 11,
            launcher.y + 10,
            "ORAYS",
            theme.text,
            1,
        );

        let label_width = time.len().saturating_mul(6) as i32;
        draw_text(
            painter,
            bounds.x + (bounds.width as i32 - label_width) / 2,
            bounds.y + 16,
            time,
            theme.muted_text,
            1,
        );

        let power = power_rect(bounds);
        painter.blend_rounded_rect(power, 10, Color::rgba(255, 255, 255, 22));
        draw_text(painter, power.x + 10, power.y + 10, "POWER", theme.text, 1);
    }
}

impl Default for TopBar {
    fn default() -> Self {
        Self::new()
    }
}

pub fn uptime_label(elapsed_ms: u64) -> String {
    let total_minutes = elapsed_ms / 60_000;
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    alloc::format!("UP {hours:02}:{minutes:02}")
}

fn launcher_rect(bounds: Rect) -> Rect {
    Rect::new(bounds.x + 10, bounds.y + 6, 76, 28)
}

fn power_rect(bounds: Rect) -> Rect {
    Rect::new(bounds.right() - 82, bounds.y + 6, 72, 28)
}
