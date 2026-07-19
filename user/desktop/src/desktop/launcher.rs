use crate::desktop::theme::Theme;
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::shadow::ShadowCache;
use crate::graphics::text::{draw_text, glyph_pattern};

const ICON_MASK_SIZE: usize = 40;
type IconMask = [u64; ICON_MASK_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppId {
    Terminal,
    Files,
    Editor,
    Images,
    Monitor,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub struct AppEntry {
    pub id: AppId,
    pub label: &'static str,
    pub short_label: &'static str,
}

pub const APP_ENTRIES: [AppEntry; 6] = [
    AppEntry {
        id: AppId::Terminal,
        label: "TERMINAL",
        short_label: "TERM",
    },
    AppEntry {
        id: AppId::Files,
        label: "FILES",
        short_label: "FILES",
    },
    AppEntry {
        id: AppId::Editor,
        label: "TEXT EDITOR",
        short_label: "EDIT",
    },
    AppEntry {
        id: AppId::Images,
        label: "IMAGES",
        short_label: "IMAGE",
    },
    AppEntry {
        id: AppId::Monitor,
        label: "SYSTEM MONITOR",
        short_label: "STATS",
    },
    AppEntry {
        id: AppId::Settings,
        label: "SETTINGS",
        short_label: "SET",
    },
];

pub struct Launcher {
    target_open: bool,
    progress: u16,
}

impl Launcher {
    pub const fn new() -> Self {
        Self {
            target_open: false,
            progress: 0,
        }
    }

    pub const fn is_open(&self) -> bool {
        self.target_open
    }

    pub const fn is_fully_open(&self) -> bool {
        self.target_open && self.progress == 1000
    }

    pub const fn is_visible(&self) -> bool {
        self.progress != 0
    }

    pub fn toggle(&mut self) {
        self.target_open = !self.target_open;
    }

    pub fn close(&mut self) {
        self.target_open = false;
    }

    pub fn tick(&mut self, elapsed_ms: u32) -> bool {
        let step = (elapsed_ms.saturating_mul(1000) / 180).min(1000) as u16;
        let old = self.progress;
        if self.target_open {
            self.progress = self.progress.saturating_add(step).min(1000);
        } else {
            self.progress = self.progress.saturating_sub(step);
        }
        old != self.progress
    }

    pub fn full_rect(&self, bounds: Rect) -> Rect {
        let width = bounds.width.saturating_sub(40).min(620);
        let height = bounds.height.saturating_sub(160).min(420);
        Rect::new(
            bounds.x + ((bounds.width - width) / 2) as i32,
            bounds.y + 56,
            width,
            height,
        )
    }

    pub fn visible_rect(&self, bounds: Rect) -> Rect {
        let full = self.full_rect(bounds);
        let height = full.height.saturating_mul(self.progress as u32) / 1000;
        Rect::new(full.x, full.y, full.width, height)
    }

    pub fn hit_test(&self, bounds: Rect, point: Point) -> Option<AppId> {
        if self.progress < 700 {
            return None;
        }
        APP_ENTRIES
            .iter()
            .enumerate()
            .find(|(index, _)| app_rect(self.full_rect(bounds), *index).contains(point))
            .map(|(_, entry)| entry.id)
    }

    pub fn contains(&self, bounds: Rect, point: Point) -> bool {
        self.visible_rect(bounds).contains(point)
    }

    pub fn draw(
        &self,
        painter: &mut Painter<'_>,
        bounds: Rect,
        theme: Theme,
        shadows: &ShadowCache,
    ) {
        if !self.is_visible() {
            return;
        }
        let visible = self.visible_rect(bounds);
        shadows.draw(painter, visible, 18, 6);
        painter.blend_rounded_rect(visible, 18, theme.panel);
        painter.stroke_rect(visible, 1, theme.panel_border);
        if self.progress < 500 {
            return;
        }
        draw_text(
            painter,
            visible.x + 28,
            visible.y + 26,
            "APPLICATIONS",
            theme.text,
            2,
        );
        draw_text(
            painter,
            visible.x + 30,
            visible.y + 52,
            "SUPER + SPACE",
            theme.muted_text,
            1,
        );
        for (index, entry) in APP_ENTRIES.iter().enumerate() {
            let card = app_rect(self.full_rect(bounds), index);
            if card.bottom() > visible.bottom() {
                continue;
            }
            painter.blend_rounded_rect(card, 12, Color::rgba(255, 255, 255, 22));
            draw_app_icon(
                painter,
                Rect::new(card.x + 16, card.y + 14, 40, 40),
                entry.id,
                theme,
            );
            draw_text(
                painter,
                card.x + 68,
                card.y + 30,
                entry.label,
                theme.text,
                1,
            );
        }
    }
}

impl Default for Launcher {
    fn default() -> Self {
        Self::new()
    }
}

pub fn draw_app_icon(painter: &mut Painter<'_>, rect: Rect, id: AppId, theme: Theme) {
    let color = app_color(id, theme);
    painter.fill_rounded_rect(rect, 10, color);
    let mark = Color::rgb(248, 251, 255);
    let mask = app_icon_mask(id);
    let height = rect.height.min(ICON_MASK_SIZE as u32) as usize;
    let width = rect.width.min(ICON_MASK_SIZE as u32) as usize;
    for (y, bits) in mask.iter().copied().take(height).enumerate() {
        let mut x = 0usize;
        while x < width {
            if bits & (1u64 << x) == 0 {
                x += 1;
                continue;
            }
            let start = x;
            while x < width && bits & (1u64 << x) != 0 {
                x += 1;
            }
            painter.fill_rect(
                Rect::new(
                    rect.x + start as i32,
                    rect.y + y as i32,
                    (x - start) as u32,
                    1,
                ),
                mark,
            );
        }
    }
}

static APP_ICON_MASKS: [IconMask; APP_ENTRIES.len()] = build_app_icon_masks();

fn app_icon_mask(id: AppId) -> &'static IconMask {
    &APP_ICON_MASKS[id as usize]
}

const fn build_app_icon_masks() -> [IconMask; APP_ENTRIES.len()] {
    let mut masks = [[0u64; ICON_MASK_SIZE]; APP_ENTRIES.len()];

    set_glyph(
        &mut masks[AppId::Terminal as usize],
        9,
        16,
        glyph_pattern(b'>'),
    );
    set_glyph(
        &mut masks[AppId::Terminal as usize],
        15,
        16,
        glyph_pattern(b'_'),
    );

    set_rect(&mut masks[AppId::Files as usize], 8, 13, 24, 18, true);
    set_rect(&mut masks[AppId::Files as usize], 10, 9, 10, 6, true);

    set_rect(&mut masks[AppId::Editor as usize], 10, 8, 20, 24, true);
    let mut row = 0usize;
    while row < 3 {
        set_rect(
            &mut masks[AppId::Editor as usize],
            14,
            14 + row * 5,
            12,
            1,
            false,
        );
        row += 1;
    }

    set_rect(&mut masks[AppId::Images as usize], 8, 9, 24, 22, true);
    set_rect(&mut masks[AppId::Images as usize], 12, 22, 16, 6, false);

    let mut column = 0usize;
    while column < 4 {
        set_rect(
            &mut masks[AppId::Monitor as usize],
            8 + column * 7,
            28 - column * 4,
            4,
            5 + column * 4,
            true,
        );
        column += 1;
    }

    set_rounded_rect(&mut masks[AppId::Settings as usize], 9, 9, 22, 22, 11, true);
    set_rounded_rect(
        &mut masks[AppId::Settings as usize],
        15,
        15,
        10,
        10,
        5,
        false,
    );

    masks
}

const fn set_glyph(mask: &mut IconMask, x: usize, y: usize, glyph: [u8; 7]) {
    let mut row = 0usize;
    while row < glyph.len() {
        let mut column = 0usize;
        while column < 5 {
            if glyph[row] & (1 << (4 - column)) != 0 {
                set_pixel(mask, x + column, y + row, true);
            }
            column += 1;
        }
        row += 1;
    }
}

const fn set_rect(
    mask: &mut IconMask,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    value: bool,
) {
    let mut row = 0usize;
    while row < height {
        let mut column = 0usize;
        while column < width {
            set_pixel(mask, x + column, y + row, value);
            column += 1;
        }
        row += 1;
    }
}

const fn set_rounded_rect(
    mask: &mut IconMask,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    radius: usize,
    value: bool,
) {
    let radius = min_usize(radius, min_usize(width / 2, height / 2));
    let mut row = 0usize;
    while row < height {
        let edge_distance = min_usize(row, height - 1 - row);
        let inset = if edge_distance >= radius {
            0
        } else {
            circle_inset(radius, edge_distance)
        };
        set_rect(mask, x + inset, y + row, width - inset * 2, 1, value);
        row += 1;
    }
}

const fn circle_inset(radius: usize, edge_distance: usize) -> usize {
    let center = radius.saturating_sub(1);
    let y = center.saturating_sub(edge_distance);
    let radius_squared = radius * radius;
    let mut x = 0usize;
    while x < radius {
        let distance = radius - x;
        if distance * distance + y * y <= radius_squared {
            return x;
        }
        x += 1;
    }
    radius
}

const fn min_usize(left: usize, right: usize) -> usize {
    if left < right { left } else { right }
}

const fn set_pixel(mask: &mut IconMask, x: usize, y: usize, value: bool) {
    if x >= ICON_MASK_SIZE || y >= ICON_MASK_SIZE {
        return;
    }
    if value {
        mask[y] |= 1u64 << x;
    } else {
        mask[y] &= !(1u64 << x);
    }
}

fn app_rect(panel: Rect, index: usize) -> Rect {
    let column = index % 2;
    let row = index / 2;
    let gap = 14u32;
    let content_width = panel.width.saturating_sub(56);
    let width = content_width.saturating_sub(gap) / 2;
    Rect::new(
        panel.x + 28 + (column as u32 * (width + gap)) as i32,
        panel.y + 82 + row as i32 * 76,
        width,
        62,
    )
}

fn app_color(id: AppId, theme: Theme) -> Color {
    match id {
        AppId::Terminal => Color::rgb(53, 66, 88),
        AppId::Files => theme.accent,
        AppId::Editor => Color::rgb(120, 87, 205),
        AppId::Images => Color::rgb(42, 151, 139),
        AppId::Monitor => Color::rgb(224, 114, 58),
        AppId::Settings => Color::rgb(88, 108, 139),
    }
}

#[cfg(test)]
mod tests {
    use super::{APP_ENTRIES, AppId, app_icon_mask};

    #[test]
    fn app_icons_are_reused_from_the_static_atlas() {
        let first = app_icon_mask(AppId::Terminal);
        let second = app_icon_mask(AppId::Terminal);
        assert!(core::ptr::eq(first, second));
        assert!(first.iter().any(|row| *row != 0));
    }

    #[test]
    fn every_app_has_a_nonempty_distinct_cached_mask() {
        for (index, entry) in APP_ENTRIES.iter().enumerate() {
            let mask = app_icon_mask(entry.id);
            assert!(mask.iter().any(|row| *row != 0));
            for previous in APP_ENTRIES.iter().take(index) {
                assert_ne!(mask, app_icon_mask(previous.id));
            }
        }
    }
}
