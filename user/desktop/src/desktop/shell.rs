use crate::desktop::dock::{DOCK_RESERVED_HEIGHT, Dock};
use crate::desktop::launcher::{AppId, Launcher};
use crate::desktop::notifications::NotificationCenter;
use crate::desktop::shortcuts::{self, Shortcut};
use crate::desktop::theme::{Theme, ThemeKind};
use crate::desktop::top_bar::{TOP_BAR_HEIGHT, TopBar, TopBarTarget, uptime_label};
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::shadow::ShadowCache;
use crate::graphics::text::draw_text;
use crate::platform::input::{InputEvent, KeyState, PointerButton};
use crate::platform::time::ElapsedClock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemAction {
    Shutdown,
    Restart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellAction {
    None,
    Launch(AppId),
    AltTab { reverse: bool },
    System(SystemAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellResponse {
    pub consumed: bool,
    pub action: ShellAction,
    pub damage: Option<Rect>,
}

impl ShellResponse {
    const fn ignored() -> Self {
        Self {
            consumed: false,
            action: ShellAction::None,
            damage: None,
        }
    }

    const fn consumed(action: ShellAction, damage: Rect) -> Self {
        Self {
            consumed: true,
            action,
            damage: Some(damage),
        }
    }
}

pub struct DesktopShell {
    theme: Theme,
    launcher: Launcher,
    dock: Dock,
    top_bar: TopBar,
    notifications: NotificationCenter,
    power_menu: bool,
    clock: ElapsedClock,
    wallpaper: u8,
}

impl DesktopShell {
    pub fn new() -> Self {
        let mut notifications = NotificationCenter::new();
        notifications.push("DESKTOP READY");
        Self {
            theme: Theme::default(),
            launcher: Launcher::new(),
            dock: Dock::new(),
            top_bar: TopBar::new(),
            notifications,
            power_menu: false,
            clock: ElapsedClock::new(),
            wallpaper: 0,
        }
    }

    pub const fn theme_kind(&self) -> ThemeKind {
        self.theme.kind
    }

    pub const fn launcher_open(&self) -> bool {
        self.launcher.is_open()
    }

    pub const fn launcher_fully_open(&self) -> bool {
        self.launcher.is_fully_open()
    }

    pub fn set_theme(&mut self, kind: ThemeKind) {
        self.theme = Theme::for_kind(kind);
    }

    pub fn set_wallpaper(&mut self, wallpaper: u8) {
        self.wallpaper = wallpaper % 3;
    }

    pub const fn wallpaper(&self) -> u8 {
        self.wallpaper
    }

    pub fn workspace(bounds: Rect) -> Rect {
        Rect::new(
            bounds.x,
            bounds.y + TOP_BAR_HEIGHT as i32,
            bounds.width,
            bounds
                .height
                .saturating_sub(TOP_BAR_HEIGHT + DOCK_RESERVED_HEIGHT),
        )
    }

    pub fn tick(&mut self, elapsed_ms: u32, bounds: Rect) -> Option<Rect> {
        let previous_minute = self.clock.whole_minutes();
        self.clock.advance(elapsed_ms);
        let launcher_changed = self.launcher.tick(elapsed_ms);
        let old_notifications = self.notifications.bounds(bounds);
        let notifications_changed = self.notifications.tick(elapsed_ms);
        let minute_changed = previous_minute != self.clock.whole_minutes();
        if launcher_changed {
            return Some(expand(self.launcher.full_rect(bounds), 20));
        }
        if notifications_changed {
            return Some(expand(
                old_notifications.union(self.notifications.bounds(bounds)),
                18,
            ));
        }
        minute_changed.then(|| Rect::new(bounds.x, bounds.y, bounds.width, TOP_BAR_HEIGHT))
    }

    pub fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> ShellResponse {
        if let Some(shortcut) = shortcuts::resolve(event) {
            return match shortcut {
                Shortcut::ToggleLauncher => {
                    self.power_menu = false;
                    self.launcher.toggle();
                    ShellResponse::consumed(
                        ShellAction::None,
                        expand(self.launcher.full_rect(bounds), 20),
                    )
                }
                Shortcut::CloseLauncher => {
                    let was_visible = self.launcher.is_visible() || self.power_menu;
                    self.launcher.close();
                    self.power_menu = false;
                    if was_visible {
                        ShellResponse::consumed(ShellAction::None, bounds)
                    } else {
                        ShellResponse::ignored()
                    }
                }
                Shortcut::ToggleTheme => {
                    self.theme = Theme::for_kind(self.theme.kind.toggled());
                    ShellResponse::consumed(ShellAction::None, bounds)
                }
                Shortcut::AltTab { reverse } => ShellResponse::consumed(
                    ShellAction::AltTab { reverse },
                    Self::workspace(bounds),
                ),
            };
        }

        let InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            position,
        } = event
        else {
            return ShellResponse::ignored();
        };

        if self.power_menu {
            let menu = power_menu_rect(bounds);
            let action = if shutdown_rect(menu).contains(position) {
                Some(SystemAction::Shutdown)
            } else if restart_rect(menu).contains(position) {
                Some(SystemAction::Restart)
            } else {
                None
            };
            self.power_menu = false;
            if let Some(action) = action {
                return ShellResponse::consumed(ShellAction::System(action), expand(menu, 18));
            }
            return ShellResponse::consumed(ShellAction::None, expand(menu, 18));
        }

        if self.launcher.is_visible() {
            if let Some(app) = self.launcher.hit_test(bounds, position) {
                self.launcher.close();
                self.notifications.push(app_launched_message(app));
                return ShellResponse::consumed(ShellAction::Launch(app), bounds);
            }
            if self.launcher.contains(bounds, position) {
                return ShellResponse::consumed(
                    ShellAction::None,
                    expand(self.launcher.full_rect(bounds), 20),
                );
            }
            self.launcher.close();
            return ShellResponse::consumed(
                ShellAction::None,
                expand(self.launcher.full_rect(bounds), 20),
            );
        }

        if let Some(target) = self.top_bar.hit_test(bounds, position) {
            return match target {
                TopBarTarget::Launcher => {
                    self.launcher.toggle();
                    ShellResponse::consumed(
                        ShellAction::None,
                        expand(self.launcher.full_rect(bounds), 20),
                    )
                }
                TopBarTarget::Power => {
                    self.power_menu = true;
                    ShellResponse::consumed(ShellAction::None, expand(power_menu_rect(bounds), 18))
                }
            };
        }

        if let Some(app) = self.dock.hit_test(bounds, position) {
            self.notifications.push(app_launched_message(app));
            return ShellResponse::consumed(ShellAction::Launch(app), bounds);
        }
        ShellResponse::ignored()
    }

    pub fn draw_background(&self, painter: &mut Painter<'_>, bounds: Rect) {
        draw_wallpaper(painter, bounds, self.theme, self.wallpaper);
    }

    pub fn draw_overlay(&self, painter: &mut Painter<'_>, bounds: Rect, shadows: &ShadowCache) {
        self.top_bar.draw(
            painter,
            bounds,
            self.theme,
            &uptime_label(self.clock.milliseconds()),
        );
        self.dock.draw(painter, bounds, self.theme, shadows);
        self.launcher.draw(painter, bounds, self.theme, shadows);
        self.notifications
            .draw(painter, bounds, self.theme, shadows);
        if self.power_menu {
            draw_power_menu(painter, bounds, self.theme, shadows);
        }
    }

    pub const fn theme(&self) -> Theme {
        self.theme
    }
}

impl Default for DesktopShell {
    fn default() -> Self {
        Self::new()
    }
}

fn draw_wallpaper(painter: &mut Painter<'_>, bounds: Rect, theme: Theme, wallpaper: u8) {
    let clip = painter.clip();
    let denominator = bounds.height.saturating_sub(1).max(1);
    for y in clip.y.max(bounds.y)..clip.bottom().min(bounds.bottom()) {
        let row = (y - bounds.y) as u32;
        let mix = |top: u8, bottom: u8| -> u8 {
            ((top as u32 * (denominator - row) + bottom as u32 * row) / denominator) as u8
        };
        let (red, green, blue) = match wallpaper {
            1 => (
                mix(theme.wallpaper_top.r / 2, theme.wallpaper_bottom.r / 2),
                mix(theme.wallpaper_top.g / 2, theme.wallpaper_bottom.g / 2),
                mix(theme.wallpaper_top.b / 2, theme.wallpaper_bottom.b / 2),
            ),
            2 => (
                mix(
                    theme.wallpaper_top.r.saturating_add(35),
                    theme.wallpaper_bottom.r,
                ),
                mix(
                    theme.wallpaper_top.g.saturating_add(20),
                    theme.wallpaper_bottom.g,
                ),
                mix(
                    theme.wallpaper_top.b,
                    theme.wallpaper_bottom.b.saturating_sub(25),
                ),
            ),
            _ => (
                mix(theme.wallpaper_top.r, theme.wallpaper_bottom.r),
                mix(theme.wallpaper_top.g, theme.wallpaper_bottom.g),
                mix(theme.wallpaper_top.b, theme.wallpaper_bottom.b),
            ),
        };
        painter.fill_rect(
            Rect::new(bounds.x, y, bounds.width, 1),
            Color::rgb(red, green, blue),
        );
    }
    let orb = bounds.width.min(bounds.height) / 3;
    painter.blend_rounded_rect(
        Rect::new(bounds.right() - orb as i32 / 2, bounds.y + 80, orb, orb),
        orb / 2,
        Color::rgba(theme.accent.r, theme.accent.g, theme.accent.b, 20),
    );
}

fn power_menu_rect(bounds: Rect) -> Rect {
    Rect::new(bounds.right() - 238, bounds.y + 48, 220, 116)
}

fn shutdown_rect(menu: Rect) -> Rect {
    Rect::new(menu.x + 14, menu.y + 46, 90, 52)
}

fn restart_rect(menu: Rect) -> Rect {
    Rect::new(menu.x + 116, menu.y + 46, 90, 52)
}

fn draw_power_menu(painter: &mut Painter<'_>, bounds: Rect, theme: Theme, shadows: &ShadowCache) {
    let menu = power_menu_rect(bounds);
    shadows.draw(painter, menu, 14, 5);
    painter.blend_rounded_rect(menu, 14, theme.panel);
    painter.stroke_rect(menu, 1, theme.panel_border);
    draw_text(painter, menu.x + 16, menu.y + 20, "SYSTEM", theme.text, 1);
    let shutdown = shutdown_rect(menu);
    let restart = restart_rect(menu);
    painter.blend_rounded_rect(shutdown, 10, Color::rgba(203, 70, 78, 190));
    painter.blend_rounded_rect(restart, 10, Color::rgba(255, 255, 255, 24));
    draw_text(
        painter,
        shutdown.x + 16,
        shutdown.y + 22,
        "SHUTDOWN",
        theme.text,
        1,
    );
    draw_text(
        painter,
        restart.x + 20,
        restart.y + 22,
        "RESTART",
        theme.text,
        1,
    );
}

fn app_launched_message(app: AppId) -> &'static str {
    match app {
        AppId::Terminal => "TERMINAL OPENED",
        AppId::Files => "FILES OPENED",
        AppId::Editor => "EDITOR OPENED",
        AppId::Images => "IMAGES OPENED",
        AppId::Monitor => "MONITOR OPENED",
        AppId::Settings => "SETTINGS OPENED",
    }
}

fn expand(rect: Rect, margin: i32) -> Rect {
    Rect::new(
        rect.x.saturating_sub(margin),
        rect.y.saturating_sub(margin),
        rect.width.saturating_add((margin * 2) as u32),
        rect.height.saturating_add((margin * 2) as u32),
    )
}
