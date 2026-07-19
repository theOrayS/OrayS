use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

#[derive(Debug, Clone, Copy)]
pub struct Button<'a> {
    pub bounds: Rect,
    pub label: &'a str,
    pub destructive: bool,
}

impl Button<'_> {
    pub fn contains(&self, point: Point) -> bool {
        self.bounds.contains(point)
    }

    pub fn draw(&self, painter: &mut Painter<'_>, theme: Theme) {
        let color = if self.destructive {
            Color::rgba(198, 66, 76, 220)
        } else {
            Color::rgba(theme.accent.r, theme.accent.g, theme.accent.b, 220)
        };
        painter.fill_rounded_rect(self.bounds, 7, color);
        painter.stroke_rect(self.bounds, 1, Color::rgba(255, 255, 255, 72));
        draw_text(
            painter,
            self.bounds.x + 10,
            self.bounds.y + (self.bounds.height as i32 - 7) / 2,
            self.label,
            Color::rgb(250, 252, 255),
            1,
        );
    }
}
