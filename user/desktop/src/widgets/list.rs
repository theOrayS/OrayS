use alloc::string::String;
use alloc::vec::Vec;

use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

pub const ROW_HEIGHT: u32 = 24;

pub fn hit_row(bounds: Rect, point: Point, offset: usize, length: usize) -> Option<usize> {
    if !bounds.contains(point) {
        return None;
    }
    let row = ((point.y - bounds.y) as u32 / ROW_HEIGHT) as usize + offset;
    (row < length).then_some(row)
}

pub fn draw_rows(
    painter: &mut Painter<'_>,
    bounds: Rect,
    rows: &[String],
    selected: Option<usize>,
    offset: usize,
    theme: Theme,
) {
    let visible = (bounds.height / ROW_HEIGHT) as usize;
    for (visible_index, (index, row)) in rows
        .iter()
        .enumerate()
        .skip(offset)
        .take(visible)
        .enumerate()
    {
        let row_bounds = Rect::new(
            bounds.x,
            bounds.y + (visible_index as u32 * ROW_HEIGHT) as i32,
            bounds.width,
            ROW_HEIGHT,
        );
        if selected == Some(index) {
            painter.fill_rect(
                row_bounds,
                Color::rgba(theme.accent.r, theme.accent.g, theme.accent.b, 72),
            );
        }
        let available = bounds.width.saturating_sub(16) as usize / 6;
        let label = row.get(..row.len().min(available)).unwrap_or(row);
        draw_text(
            painter,
            row_bounds.x + 8,
            row_bounds.y + 9,
            label,
            Color::rgb(42, 52, 70),
            1,
        );
    }
}

pub fn labels(values: impl IntoIterator<Item = String>) -> Vec<String> {
    values.into_iter().collect()
}
