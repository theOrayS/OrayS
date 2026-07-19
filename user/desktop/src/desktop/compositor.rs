use crate::apps::ApplicationRegistry;
use crate::desktop::shell::DesktopShell;
use crate::desktop::theme::Theme;
use crate::desktop::window::{TITLE_BAR_HEIGHT, Window, WindowId};
use crate::desktop::window_manager::{WindowManager, close_rect, maximize_rect, minimize_rect};
use crate::graphics::geometry::{Point, Rect};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::shadow::ShadowCache;
use crate::graphics::surface::Surface;
use crate::graphics::text::draw_text;

pub struct Compositor {
    cursor: Point,
    shell: DesktopShell,
    shadows: ShadowCache,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            cursor: Point::new(0, 0),
            shell: DesktopShell::new(),
            shadows: ShadowCache::new(),
        }
    }

    pub const fn cursor(&self) -> Point {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: Point) -> Option<Rect> {
        if cursor == self.cursor {
            return None;
        }
        let damage = cursor_bounds(self.cursor).union(cursor_bounds(cursor));
        self.cursor = cursor;
        Some(damage)
    }

    pub const fn shell(&self) -> &DesktopShell {
        &self.shell
    }

    pub fn shell_mut(&mut self) -> &mut DesktopShell {
        &mut self.shell
    }

    pub fn render(&self, surface: &mut Surface, manager: &WindowManager, damage: &[Rect]) {
        self.render_internal(surface, manager, damage, None);
    }

    pub fn render_with_apps(
        &self,
        surface: &mut Surface,
        manager: &WindowManager,
        apps: &ApplicationRegistry,
        damage: &[Rect],
    ) {
        self.render_internal(surface, manager, damage, Some(apps));
    }

    fn render_internal(
        &self,
        surface: &mut Surface,
        manager: &WindowManager,
        damage: &[Rect],
        apps: Option<&ApplicationRegistry>,
    ) {
        let bounds = surface.bounds();
        for region in damage {
            let Some(clip) = region.intersection(bounds) else {
                continue;
            };
            self.render_region(surface, manager, clip, apps);
        }
    }

    pub fn render_full(&self, surface: &mut Surface, manager: &WindowManager) {
        let bounds = surface.bounds();
        self.render_region(surface, manager, bounds, None);
    }

    fn render_region(
        &self,
        surface: &mut Surface,
        manager: &WindowManager,
        clip: Rect,
        apps: Option<&ApplicationRegistry>,
    ) {
        let bounds = surface.bounds();
        let mut painter = Painter::new(surface, clip);
        self.shell.draw_background(&mut painter, bounds);

        let modal = manager.active_modal();
        for window in manager.windows().iter().filter(|window| window.visible()) {
            if modal == Some(window.id()) {
                painter.blend_rect(bounds, Color::rgba(5, 9, 18, 112));
            }
            let mut visual_window = window.clone();
            if let Some(visual_bounds) = manager.visual_bounds(window.id()) {
                visual_window.set_bounds(visual_bounds);
            }
            draw_window(
                &mut painter,
                &visual_window,
                manager.focused() == Some(window.id()),
                self.shell.theme(),
                &self.shadows,
                apps,
            );
        }
        self.shell.draw_overlay(&mut painter, bounds, &self.shadows);
        draw_cursor(&mut painter, self.cursor);
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

fn draw_window(
    painter: &mut Painter<'_>,
    window: &Window,
    focused: bool,
    theme: Theme,
    shadows: &ShadowCache,
    apps: Option<&ApplicationRegistry>,
) {
    let bounds = window.bounds();
    shadows.draw(painter, bounds, 9, 4);

    painter.fill_rounded_rect(bounds, 9, theme.window);
    let title = window.title_bar();
    painter.fill_rect(
        title,
        if focused {
            theme.title_active
        } else {
            theme.title_inactive
        },
    );
    painter.stroke_rect(
        bounds,
        1,
        if focused {
            Color::rgb(113, 155, 235)
        } else {
            Color::rgb(112, 122, 142)
        },
    );
    draw_text(
        painter,
        bounds.x + 12,
        bounds.y + 12,
        window.title(),
        Color::rgb(245, 248, 255),
        1,
    );

    let client = Rect::new(
        bounds.x + 1,
        bounds.y + TITLE_BAR_HEIGHT as i32,
        bounds.width.saturating_sub(2),
        bounds.height.saturating_sub(TITLE_BAR_HEIGHT + 1),
    );
    painter.fill_rect(client, theme.window);
    let rendered_app = apps.is_some_and(|registry| registry.draw(painter, window, client, theme));
    if !rendered_app {
        let accent = window_accent(window.id());
        painter.fill_rect(
            Rect::new(
                client.x + 18,
                client.y + 20,
                5,
                client.height.saturating_sub(40),
            ),
            accent,
        );
        draw_text(
            painter,
            client.x + 38,
            client.y + 24,
            "ORAYS WORKSPACE",
            Color::rgb(45, 56, 78),
            1,
        );
        draw_text(
            painter,
            client.x + 38,
            client.y + 48,
            "DAMAGE COMPOSITED WINDOW",
            Color::rgb(85, 96, 116),
            1,
        );
    }

    draw_control(painter, minimize_rect(window), Control::Minimize, false);
    if window.resizable() {
        draw_control(painter, maximize_rect(window), Control::Maximize, false);
    }
    if window.closable() {
        draw_control(painter, close_rect(window), Control::Close, true);
    }
}

enum Control {
    Minimize,
    Maximize,
    Close,
}

fn draw_control(painter: &mut Painter<'_>, rect: Rect, control: Control, destructive: bool) {
    painter.blend_rect(
        rect,
        if destructive {
            Color::rgba(202, 67, 76, 205)
        } else {
            Color::rgba(255, 255, 255, 28)
        },
    );
    let line = if destructive {
        Color::rgb(255, 235, 237)
    } else {
        Color::rgb(225, 232, 245)
    };
    match control {
        Control::Minimize => painter.fill_rect(
            Rect::new(rect.x + 6, rect.bottom() - 7, rect.width - 12, 1),
            line,
        ),
        Control::Maximize => painter.stroke_rect(
            Rect::new(rect.x + 6, rect.y + 6, rect.width - 12, rect.height - 12),
            1,
            line,
        ),
        Control::Close => {
            for offset in 0..10 {
                painter.fill_rect(
                    Rect::new(rect.x + 7 + offset, rect.y + 7 + offset, 1, 1),
                    line,
                );
                painter.fill_rect(
                    Rect::new(rect.x + 16 - offset, rect.y + 7 + offset, 1, 1),
                    line,
                );
            }
        }
    }
}

fn window_accent(id: WindowId) -> Color {
    let seed = id.0.wrapping_mul(0x9e37_79b9);
    Color::rgb(
        68u8.saturating_add((seed & 63) as u8),
        104u8.saturating_add(((seed >> 8) & 63) as u8),
        180u8.saturating_add(((seed >> 16) & 55) as u8),
    )
}

fn cursor_bounds(point: Point) -> Rect {
    Rect::new(point.x - 2, point.y - 2, 18, 24)
}

fn draw_cursor(painter: &mut Painter<'_>, point: Point) {
    let outline = Color::rgb(12, 16, 24);
    let fill = Color::rgb(250, 252, 255);
    for row in 0i32..16 {
        let width = (row / 2 + 1).min(8) as u32;
        painter.fill_rect(Rect::new(point.x, point.y + row, width + 2, 1), outline);
        if width > 1 {
            painter.fill_rect(Rect::new(point.x + 1, point.y + row, width, 1), fill);
        }
    }
    painter.fill_rect(Rect::new(point.x + 6, point.y + 12, 4, 9), outline);
    painter.fill_rect(Rect::new(point.x + 7, point.y + 13, 2, 7), fill);
}
