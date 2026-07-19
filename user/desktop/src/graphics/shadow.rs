use super::geometry::Rect;
use super::painter::{Color, Painter};

pub struct ShadowCache {
    alpha: [u8; 16],
}

impl ShadowCache {
    pub const fn new() -> Self {
        let mut alpha = [0; 16];
        let mut index = 0;
        while index < alpha.len() {
            let remaining = (alpha.len() - index) as u16;
            alpha[index] = ((remaining * remaining * 42) / 256) as u8;
            index += 1;
        }
        Self { alpha }
    }

    pub fn draw(&self, painter: &mut Painter<'_>, rect: Rect, radius: u32, offset_y: i32) {
        for (distance, alpha) in self.alpha.iter().copied().enumerate().rev() {
            if alpha == 0 {
                continue;
            }
            let distance = distance as i32 + 1;
            let expanded = Rect::new(
                rect.x - distance,
                rect.y - distance + offset_y,
                rect.width.saturating_add((distance * 2) as u32),
                rect.height.saturating_add((distance * 2) as u32),
            );
            painter.blend_rounded_rect(
                expanded,
                radius.saturating_add(distance as u32),
                Color::rgba(0, 0, 0, alpha),
            );
        }
    }
}

impl Default for ShadowCache {
    fn default() -> Self {
        Self::new()
    }
}
