use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

pub fn centered(container: Rect, width: u32, height: u32) -> Rect {
    let width = width.min(container.width.saturating_sub(24));
    let height = height.min(container.height.saturating_sub(24));
    Rect::new(
        container.x + (container.width.saturating_sub(width) / 2) as i32,
        container.y + (container.height.saturating_sub(height) / 2) as i32,
        width,
        height,
    )
}

pub fn draw(painter: &mut Painter<'_>, container: Rect, dialog: Rect, title: &str, theme: Theme) {
    painter.blend_rect(container, Color::rgba(8, 12, 20, 96));
    painter.fill_rounded_rect(dialog, 10, theme.window);
    painter.stroke_rect(dialog, 1, theme.accent);
    draw_text(
        painter,
        dialog.x + 16,
        dialog.y + 18,
        title,
        Color::rgb(38, 48, 66),
        1,
    );
}
