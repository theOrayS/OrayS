use crate::desktop::launcher::{APP_ENTRIES, AppId, draw_app_icon};
use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::shadow::ShadowCache;

pub const DOCK_RESERVED_HEIGHT: u32 = 78;

pub struct Dock;

impl Dock {
    pub const fn new() -> Self {
        Self
    }

    pub fn panel_rect(bounds: Rect) -> Rect {
        let width = bounds.width.saturating_sub(28).min(520);
        Rect::new(
            bounds.x + ((bounds.width - width) / 2) as i32,
            bounds.bottom() - 66,
            width,
            52,
        )
    }

    pub fn hit_test(&self, bounds: Rect, point: Point) -> Option<AppId> {
        let panel = Self::panel_rect(bounds);
        APP_ENTRIES
            .iter()
            .enumerate()
            .find(|(index, _)| icon_rect(panel, *index).contains(point))
            .map(|(_, entry)| entry.id)
    }

    pub fn draw(
        &self,
        painter: &mut Painter<'_>,
        bounds: Rect,
        theme: Theme,
        shadows: &ShadowCache,
    ) {
        let panel = Self::panel_rect(bounds);
        shadows.draw(painter, panel, 17, 5);
        painter.blend_rounded_rect(panel, 17, theme.panel);
        painter.stroke_rect(panel, 1, theme.panel_border);
        for (index, entry) in APP_ENTRIES.iter().enumerate() {
            let icon = icon_rect(panel, index);
            draw_app_icon(painter, icon, entry.id, theme);
            painter.fill_rounded_rect(
                Rect::new(icon.x + 13, panel.bottom() - 5, 10, 2),
                1,
                Color::rgba(theme.accent.r, theme.accent.g, theme.accent.b, 170),
            );
        }
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

fn icon_rect(panel: Rect, index: usize) -> Rect {
    let icon_size = 36u32;
    let total = APP_ENTRIES.len() as u32 * icon_size;
    let gap = panel.width.saturating_sub(total) / (APP_ENTRIES.len() as u32 + 1);
    Rect::new(
        panel.x + gap as i32 + index as i32 * (icon_size + gap) as i32,
        panel.y + 8,
        icon_size,
        icon_size,
    )
}
