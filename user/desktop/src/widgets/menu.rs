use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

pub const ITEM_HEIGHT: u32 = 28;

pub fn hit(bounds: Rect, point: Point, item_count: usize) -> Option<usize> {
    if !bounds.contains(point) {
        return None;
    }
    let index = ((point.y - bounds.y) as u32 / ITEM_HEIGHT) as usize;
    (index < item_count).then_some(index)
}

pub fn draw(painter: &mut Painter<'_>, bounds: Rect, items: &[&str], theme: Theme) {
    painter.fill_rounded_rect(bounds, 7, theme.window);
    painter.stroke_rect(bounds, 1, theme.panel_border);
    for (index, item) in items.iter().enumerate() {
        draw_text(
            painter,
            bounds.x + 10,
            bounds.y + 10 + (index as u32 * ITEM_HEIGHT) as i32,
            item,
            Color::rgb(40, 50, 68),
            1,
        );
    }
}
