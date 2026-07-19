use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};

pub fn draw(
    painter: &mut Painter<'_>,
    bounds: Rect,
    total_rows: usize,
    visible_rows: usize,
    offset: usize,
    theme: Theme,
) {
    if total_rows <= visible_rows || bounds.is_empty() {
        return;
    }
    painter.fill_rounded_rect(bounds, bounds.width / 2, Color::rgba(45, 55, 72, 30));
    let thumb_height = (bounds.height as usize * visible_rows / total_rows).max(12) as u32;
    let maximum_offset = total_rows.saturating_sub(visible_rows).max(1);
    let travel = bounds.height.saturating_sub(thumb_height);
    let thumb_y = travel as usize * offset.min(maximum_offset) / maximum_offset;
    painter.fill_rounded_rect(
        Rect::new(
            bounds.x,
            bounds.y + thumb_y as i32,
            bounds.width,
            thumb_height,
        ),
        bounds.width / 2,
        theme.accent,
    );
}
