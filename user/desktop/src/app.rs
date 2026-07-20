use alloc::string::{String, ToString};

use crate::apps::{AppAction, AppResponse, ApplicationRegistry};
use crate::desktop::compositor::Compositor;
use crate::desktop::launcher::AppId;
use crate::desktop::shell::{DesktopShell, ShellAction, SystemAction};
use crate::desktop::window::{WindowId, WindowSpec};
use crate::desktop::window_manager::{HitTarget, WindowError, WindowManager};
use crate::graphics::damage::DamageTracker;
use crate::graphics::geometry::{Rect, Size};
use crate::graphics::painter::{Color, Painter};
use crate::graphics::surface::{Surface, SurfaceError};
use crate::graphics::text::draw_text;
use crate::platform::display::{DisplayBackend, DisplayError};
use crate::platform::input::{InputEvent, KeyState, PointerButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppError {
    Display(DisplayError),
    Surface(SurfaceError),
    Window(WindowError),
}

impl From<DisplayError> for AppError {
    fn from(value: DisplayError) -> Self {
        Self::Display(value)
    }
}

impl From<SurfaceError> for AppError {
    fn from(value: SurfaceError) -> Self {
        Self::Surface(value)
    }
}

impl From<WindowError> for AppError {
    fn from(value: WindowError) -> Self {
        Self::Window(value)
    }
}

pub struct DesktopApp<D: DisplayBackend> {
    display: D,
    back_buffer: Surface,
    damage: DamageTracker,
    frame_checksum: u64,
}

pub struct WindowedDesktop<D: DisplayBackend> {
    display: D,
    back_buffer: Surface,
    windows: WindowManager,
    compositor: Compositor,
    apps: ApplicationRegistry,
    frame_checksum: u64,
    pending_system_action: Option<SystemAction>,
    pending_action_marker: Option<String>,
}

impl<D: DisplayBackend> WindowedDesktop<D> {
    pub fn new(display: D) -> Result<Self, AppError> {
        let info = display.info();
        let back_buffer = Surface::new(info.width, info.height, info.width)?;
        let desktop_bounds = Rect::new(0, 0, info.width, info.height);
        let mut windows = WindowManager::new(DesktopShell::workspace(desktop_bounds))?;
        windows.add_damage(desktop_bounds);
        Ok(Self {
            display,
            back_buffer,
            windows,
            compositor: Compositor::new(),
            apps: ApplicationRegistry::new(),
            frame_checksum: 0,
            pending_system_action: None,
            pending_action_marker: None,
        })
    }

    pub fn create_window(&mut self, spec: WindowSpec<'_>) -> Result<WindowId, AppError> {
        let id = self.windows.create(spec)?;
        self.windows.animate_open(id)?;
        Ok(id)
    }

    pub fn render_pending(&mut self) -> Result<bool, AppError> {
        self.sync_display_geometry()?;
        if self.windows.damage().is_empty() {
            return Ok(false);
        }
        let damage = self.windows.damage().to_vec();
        self.compositor
            .render_with_apps(&mut self.back_buffer, &self.windows, &self.apps, &damage);
        self.display.present(&self.back_buffer, &damage)?;
        self.frame_checksum = self.back_buffer.checksum64();
        self.windows.clear_damage();
        Ok(true)
    }

    fn sync_display_geometry(&mut self) -> Result<bool, AppError> {
        let info = self.display.refresh_info()?;
        if self.back_buffer.width() == info.width && self.back_buffer.height() == info.height {
            return Ok(false);
        }
        let desktop_bounds = Rect::new(0, 0, info.width, info.height);
        let workspace = DesktopShell::workspace(desktop_bounds);
        let back_buffer = Surface::new(info.width, info.height, info.width)?;
        self.windows.resize_workspace(workspace)?;
        let cursor = self.compositor.cursor();
        let clamped = crate::graphics::geometry::Point::new(
            cursor.x.clamp(0, info.width.saturating_sub(1) as i32),
            cursor.y.clamp(0, info.height.saturating_sub(1) as i32),
        );
        self.compositor.set_cursor(clamped);
        self.back_buffer = back_buffer;
        self.windows.add_damage(desktop_bounds);
        Ok(true)
    }

    pub fn handle_input(&mut self, event: InputEvent) -> Result<bool, AppError> {
        if matches!(event, InputEvent::StateReset) {
            self.windows.pointer_release();
            return self.render_pending();
        }
        let bounds = self.back_buffer.bounds();
        let launcher_was_open = self.compositor.shell().launcher_open();
        let theme_was = self.compositor.shell().theme_kind();
        let response = self.compositor.shell_mut().handle_input(event, bounds);
        if launcher_was_open != self.compositor.shell().launcher_open() {
            self.pending_action_marker = Some(
                if self.compositor.shell().launcher_open() {
                    "LAUNCHER OPEN"
                } else {
                    "LAUNCHER CLOSED"
                }
                .to_string(),
            );
        } else if theme_was != self.compositor.shell().theme_kind() {
            self.apps.sync_appearance(
                self.compositor.shell().theme_kind(),
                self.compositor.shell().wallpaper(),
            );
            self.pending_action_marker = Some(alloc::format!(
                "THEME {:?}",
                self.compositor.shell().theme_kind()
            ));
        }
        if let Some(damage) = response.damage {
            self.windows.add_damage(damage);
        }
        match response.action {
            ShellAction::Launch(app) => {
                self.launch_application(app)?;
            }
            ShellAction::AltTab { reverse } => {
                self.windows.alt_tab(reverse);
                self.pending_action_marker = Some(alloc::format!("ALT_TAB reverse={reverse}"));
            }
            ShellAction::System(action) => {
                self.pending_system_action = Some(action);
                self.pending_action_marker = Some(alloc::format!("SYSTEM {action:?}"));
            }
            ShellAction::None => {}
        }

        if !response.consumed {
            match event {
                InputEvent::PointerMoved { position, .. } => {
                    if let Some(cursor_damage) = self.compositor.set_cursor(position) {
                        self.windows.add_damage(cursor_damage);
                    }
                    self.windows.pointer_move(position)?;
                }
                InputEvent::PointerButton {
                    button: PointerButton::Left,
                    state: KeyState::Pressed,
                    position,
                } => {
                    let target = self.windows.hit_test(position);
                    if let HitTarget::Close(id) = target
                        && self.apps.request_close(id)
                    {
                        if let Some(window) = self.windows.window(id) {
                            self.windows.add_damage(window.bounds());
                        }
                    } else {
                        self.windows.pointer_press(position)?;
                        if let HitTarget::Client(id) = target {
                            self.route_app_input(id, event)?;
                        }
                    }
                }
                InputEvent::PointerButton {
                    button: PointerButton::Left,
                    state: KeyState::Released,
                    ..
                } => self.windows.pointer_release(),
                InputEvent::Key { .. } => {
                    if let Some(id) = self.windows.focused() {
                        self.route_app_input(id, event)?;
                    }
                }
                InputEvent::Scroll { position, .. } => {
                    if let HitTarget::Client(id) = self.windows.hit_test(position) {
                        self.route_app_input(id, event)?;
                    }
                }
                _ => {}
            }
        }
        self.render_pending()
    }

    pub fn tick(&mut self, elapsed_ms: u32) -> Result<bool, AppError> {
        self.windows.tick_animations(elapsed_ms);
        self.apps.retain_windows(self.windows.windows());
        let changed_apps = self.apps.tick(elapsed_ms, self.windows.windows().len());
        let changed_bounds = changed_apps
            .into_iter()
            .filter_map(|id| {
                self.windows
                    .window(id)
                    .map(ApplicationRegistry::client_bounds)
            })
            .collect::<alloc::vec::Vec<_>>();
        for bounds in changed_bounds {
            self.windows.add_damage(bounds);
        }
        if let Some(damage) = self
            .compositor
            .shell_mut()
            .tick(elapsed_ms, self.back_buffer.bounds())
        {
            self.windows.add_damage(damage);
        }
        self.render_pending()
    }

    pub const fn frame_checksum(&self) -> u64 {
        self.frame_checksum
    }

    pub fn surface(&self) -> &Surface {
        &self.back_buffer
    }

    pub fn windows(&self) -> &WindowManager {
        &self.windows
    }

    pub const fn shell(&self) -> &DesktopShell {
        self.compositor.shell()
    }

    pub const fn apps(&self) -> &ApplicationRegistry {
        &self.apps
    }

    pub fn take_system_action(&mut self) -> Option<SystemAction> {
        self.pending_system_action.take()
    }

    pub fn take_action_marker(&mut self) -> Option<String> {
        self.pending_action_marker.take()
    }

    pub fn launch_application(&mut self, app: AppId) -> Result<WindowId, AppError> {
        let index = self.windows.windows().len() as i32;
        let offset = (index % 5) * 26;
        let (title, width, height) = match app {
            AppId::Terminal => ("TERMINAL", 560, 340),
            AppId::Files => ("FILES", 620, 390),
            AppId::Editor => ("TEXT EDITOR", 640, 420),
            AppId::Images => ("IMAGES", 580, 390),
            AppId::Monitor => ("SYSTEM MONITOR", 560, 360),
            AppId::Settings => ("SETTINGS", 520, 350),
        };
        let id = self.create_window(WindowSpec::new(
            title,
            Rect::new(72 + offset, 72 + offset, width, height),
        ))?;
        if app == AppId::Settings {
            self.apps.attach_settings(
                id,
                self.compositor.shell().theme_kind(),
                self.compositor.shell().wallpaper(),
            );
        } else {
            self.apps.attach(id, app);
        }
        self.pending_action_marker = Some(alloc::format!("LAUNCH {app:?}"));
        Ok(id)
    }

    pub fn launch_file_manager_at(&mut self, path: &str) -> Result<WindowId, AppError> {
        let id = self.create_window(WindowSpec::new("FILES", Rect::new(80, 170, 620, 390)))?;
        self.apps.attach_files(id, path);
        Ok(id)
    }

    pub fn launch_editor_path(&mut self, path: &str) -> Result<WindowId, AppError> {
        let id = self.create_window(WindowSpec::new(
            "TEXT EDITOR",
            Rect::new(540, 120, 640, 420),
        ))?;
        self.apps.attach_editor(id, path);
        Ok(id)
    }

    fn route_app_input(&mut self, id: WindowId, event: InputEvent) -> Result<(), AppError> {
        let Some(bounds) = self
            .windows
            .window(id)
            .map(ApplicationRegistry::client_bounds)
        else {
            return Ok(());
        };
        let response = self.apps.handle_input(id, event, bounds);
        if response.consumed {
            self.windows.add_damage(bounds);
            self.apply_app_response(id, response)?;
        }
        Ok(())
    }

    fn apply_app_response(
        &mut self,
        source: WindowId,
        response: AppResponse,
    ) -> Result<(), AppError> {
        match response.action {
            AppAction::None => {}
            AppAction::OpenEditor(path) => {
                let id = self
                    .create_window(WindowSpec::new("TEXT EDITOR", Rect::new(126, 94, 640, 420)))?;
                self.apps.attach_editor(id, &path);
                self.pending_action_marker = Some("OPEN EDITOR FILE".to_string());
            }
            AppAction::OpenImage(path) => {
                let id =
                    self.create_window(WindowSpec::new("IMAGES", Rect::new(138, 102, 580, 390)))?;
                self.apps.attach_image(id, &path);
                self.pending_action_marker = Some("OPEN IMAGE FILE".to_string());
            }
            AppAction::SetTheme(kind) => {
                self.compositor.shell_mut().set_theme(kind);
                self.apps.sync_appearance(
                    self.compositor.shell().theme_kind(),
                    self.compositor.shell().wallpaper(),
                );
                self.pending_action_marker = Some(alloc::format!("THEME {kind:?}"));
                self.windows.add_damage(self.back_buffer.bounds());
            }
            AppAction::SetWallpaper(wallpaper) => {
                self.compositor.shell_mut().set_wallpaper(wallpaper);
                self.apps.sync_appearance(
                    self.compositor.shell().theme_kind(),
                    self.compositor.shell().wallpaper(),
                );
                self.pending_action_marker = Some(alloc::format!("WALLPAPER {wallpaper}"));
                self.windows.add_damage(self.back_buffer.bounds());
            }
            AppAction::CloseWindow => {
                self.windows.animate_close(source)?;
                self.pending_action_marker = Some("CLOSE CONFIRMED".to_string());
            }
        }
        Ok(())
    }
}

pub fn default_window_specs(width: u32, height: u32) -> [WindowSpec<'static>; 2] {
    let workspace = Size::new(width.max(640), height.max(480));
    let first_width = (workspace.width * 46 / 100).clamp(300, 620);
    let first_height = (workspace.height * 52 / 100).clamp(220, 430);
    let second_width = (workspace.width * 42 / 100).clamp(280, 560);
    let second_height = (workspace.height * 45 / 100).clamp(210, 390);
    [
        WindowSpec::new(
            "FILES",
            Rect::new(
                (width / 12) as i32,
                (height / 7) as i32,
                first_width,
                first_height,
            ),
        ),
        WindowSpec::new(
            "SYSTEM MONITOR",
            Rect::new(
                (width * 42 / 100) as i32,
                (height * 28 / 100) as i32,
                second_width,
                second_height,
            ),
        ),
    ]
}

impl<D: DisplayBackend> DesktopApp<D> {
    pub fn new(display: D) -> Result<Self, AppError> {
        let info = display.info();
        let back_buffer = Surface::new(info.width, info.height, info.width)?;
        Ok(Self {
            display,
            back_buffer,
            damage: DamageTracker::new(32),
            frame_checksum: 0,
        })
    }

    pub fn render_boot_frame(&mut self) -> Result<(), AppError> {
        let info = self.display.refresh_info()?;
        if self.back_buffer.width() != info.width || self.back_buffer.height() != info.height {
            self.back_buffer = Surface::new(info.width, info.height, info.width)?;
            self.damage.clear();
        }
        render_checkpoint2_scene(&mut self.back_buffer, &mut self.damage);
        self.display
            .present(&self.back_buffer, self.damage.regions())?;
        self.frame_checksum = self.back_buffer.checksum64();
        self.damage.clear();
        Ok(())
    }

    pub const fn frame_checksum(&self) -> u64 {
        self.frame_checksum
    }

    pub fn surface(&self) -> &Surface {
        &self.back_buffer
    }
}

pub fn render_checkpoint2_scene(surface: &mut Surface, damage: &mut DamageTracker) {
    let bounds = surface.bounds();
    let mut painter = Painter::new(surface, bounds);
    painter.fill_vertical_gradient(bounds, Color::rgb(18, 28, 48), Color::rgb(53, 72, 112));

    let top_bar = Rect::new(0, 0, bounds.width, 38);
    painter.blend_rect(top_bar, Color::rgba(12, 17, 28, 222));
    draw_text(&mut painter, 18, 14, "ORAYS", Color::rgb(235, 241, 255), 2);
    draw_text(
        &mut painter,
        bounds.width.saturating_sub(148) as i32,
        14,
        "HEADLESS  10:24",
        Color::rgb(208, 219, 240),
        1,
    );

    let card_width = bounds.width.min(620);
    let card_height = bounds.height.min(310);
    let card = Rect::new(
        ((bounds.width - card_width) / 2) as i32,
        ((bounds.height - card_height) / 2) as i32,
        card_width,
        card_height,
    );
    painter.blend_rect(card.translate(8, 10), Color::rgba(4, 7, 14, 92));
    painter.blend_rect(card, Color::rgba(28, 36, 56, 236));
    painter.stroke_rect(card, 2, Color::rgba(138, 167, 225, 132));
    draw_text(
        &mut painter,
        card.x + 34,
        card.y + 40,
        "ORAYS DESKTOP",
        Color::rgb(245, 248, 255),
        3,
    );
    draw_text(
        &mut painter,
        card.x + 36,
        card.y + 92,
        "PURE RUST SOFTWARE COMPOSITOR",
        Color::rgb(168, 190, 232),
        1,
    );
    draw_text(
        &mut painter,
        card.x + 36,
        card.y + 126,
        "RV64 + LA64 / STRIDE / CLIPPING / DAMAGE",
        Color::rgb(195, 207, 231),
        1,
    );

    let button = Rect::new(card.x + 36, card.bottom() - 72, 164, 38);
    painter.fill_rect(button, Color::rgb(78, 125, 224));
    painter.stroke_rect(button, 1, Color::rgb(149, 185, 255));
    draw_text(
        &mut painter,
        button.x + 20,
        button.y + 14,
        "OPEN LAUNCHER",
        Color::rgb(255, 255, 255),
        1,
    );

    let dock_width = bounds.width.min(430);
    let dock = Rect::new(
        ((bounds.width - dock_width) / 2) as i32,
        bounds.bottom() - 68,
        dock_width,
        48,
    );
    painter.blend_rect(dock, Color::rgba(12, 17, 28, 210));
    painter.stroke_rect(dock, 1, Color::rgba(172, 194, 235, 105));
    for index in 0..6 {
        let icon = Rect::new(dock.x + 22 + index * 66, dock.y + 10, 28, 28);
        let color = Color::rgb(78 + index as u8 * 18, 120 + index as u8 * 9, 210);
        painter.fill_rect(icon, color);
    }

    damage.add(bounds);
}
