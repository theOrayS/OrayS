use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;

pub fn draw(painter: &mut Painter<'_>, bounds: Rect, text: &str, color: Color) {
    draw_text(painter, bounds.x, bounds.y, text, color, 1);
}
