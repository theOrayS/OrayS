use alloc::string::String;
use alloc::vec::Vec;

use crate::desktop::launcher::AppId;
use crate::desktop::theme::{Theme, ThemeKind};
use crate::desktop::window::{TITLE_BAR_HEIGHT, Window, WindowId};
use crate::graphics::geometry::Rect;
use crate::graphics::painter::Painter;
use crate::platform::filesystem::{FsError, FsErrorKind};
use crate::platform::input::InputEvent;

pub mod file_manager;
pub mod image_viewer;
pub mod settings;
pub mod system_monitor;
pub mod terminal;
pub mod text_editor;

use file_manager::FileManager;
use image_viewer::ImageViewer;
use settings::Settings;
use system_monitor::{MonitorSnapshot, SystemMonitor};
use terminal::Terminal;
use text_editor::TextEditor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppAction {
    None,
    OpenEditor(String),
    OpenImage(String),
    SetTheme(ThemeKind),
    SetWallpaper(u8),
    CloseWindow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppResponse {
    pub consumed: bool,
    pub action: AppAction,
}

impl AppResponse {
    pub const fn ignored() -> Self {
        Self {
            consumed: false,
            action: AppAction::None,
        }
    }

    pub const fn consumed(action: AppAction) -> Self {
        Self {
            consumed: true,
            action,
        }
    }
}

pub enum Application {
    Terminal(Terminal),
    Files(FileManager),
    Editor(TextEditor),
    Images(ImageViewer),
    Monitor(SystemMonitor),
    Settings(Settings),
}

impl Application {
    fn new(id: AppId) -> Self {
        match id {
            AppId::Terminal => Self::Terminal(Terminal::new("/")),
            AppId::Files => Self::Files(FileManager::new("/")),
            AppId::Editor => Self::Editor(TextEditor::new()),
            AppId::Images => Self::Images(ImageViewer::new()),
            AppId::Monitor => Self::Monitor(SystemMonitor::new()),
            AppId::Settings => Self::Settings(Settings::new()),
        }
    }

    fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        match self {
            Self::Terminal(app) => app.draw(painter, bounds, theme),
            Self::Files(app) => app.draw(painter, bounds, theme),
            Self::Editor(app) => app.draw(painter, bounds, theme),
            Self::Images(app) => app.draw(painter, bounds, theme),
            Self::Monitor(app) => app.draw(painter, bounds, theme),
            Self::Settings(app) => app.draw(painter, bounds, theme),
        }
    }

    fn handle_input(&mut self, event: InputEvent, bounds: Rect) -> AppResponse {
        match self {
            Self::Terminal(app) => app.handle_input(event, bounds),
            Self::Files(app) => app.handle_input(event, bounds),
            Self::Editor(app) => app.handle_input(event, bounds),
            Self::Images(app) => app.handle_input(event, bounds),
            Self::Monitor(app) => app.handle_input(event, bounds),
            Self::Settings(app) => app.handle_input(event, bounds),
        }
    }

    fn request_close(&mut self) -> bool {
        match self {
            Self::Editor(editor) => editor.request_close(),
            _ => false,
        }
    }
}

struct AppInstance {
    window: WindowId,
    application: Application,
}

pub struct ApplicationRegistry {
    instances: Vec<AppInstance>,
    elapsed_ms: u64,
    input_events: u64,
}

impl ApplicationRegistry {
    pub const fn new() -> Self {
        Self {
            instances: Vec::new(),
            elapsed_ms: 0,
            input_events: 0,
        }
    }

    pub fn attach(&mut self, window: WindowId, id: AppId) {
        self.instances.retain(|instance| instance.window != window);
        self.instances.push(AppInstance {
            window,
            application: Application::new(id),
        });
    }

    pub fn attach_settings(&mut self, window: WindowId, theme: ThemeKind, wallpaper: u8) {
        self.attach_application(
            window,
            Application::Settings(Settings::from_appearance(theme, wallpaper)),
        );
    }

    pub fn sync_appearance(&mut self, theme: ThemeKind, wallpaper: u8) {
        for instance in &mut self.instances {
            if let Application::Settings(settings) = &mut instance.application {
                settings.sync_appearance(theme, wallpaper);
            }
        }
    }

    pub fn attach_editor(&mut self, window: WindowId, path: &str) {
        let mut editor = TextEditor::new();
        let _ = editor.open(path);
        self.attach_application(window, Application::Editor(editor));
    }

    pub fn attach_files(&mut self, window: WindowId, path: &str) {
        self.attach_application(window, Application::Files(FileManager::new(path)));
    }

    pub fn attach_image(&mut self, window: WindowId, path: &str) {
        let mut viewer = ImageViewer::new();
        let _ = viewer.open(path);
        self.attach_application(window, Application::Images(viewer));
    }

    fn attach_application(&mut self, window: WindowId, application: Application) {
        self.instances.retain(|instance| instance.window != window);
        self.instances.push(AppInstance {
            window,
            application,
        });
    }

    pub fn contains(&self, window: WindowId) -> bool {
        self.instances
            .iter()
            .any(|instance| instance.window == window)
    }

    pub fn application(&self, window: WindowId) -> Option<&Application> {
        self.instances
            .iter()
            .find(|instance| instance.window == window)
            .map(|instance| &instance.application)
    }

    pub fn draw(
        &self,
        painter: &mut Painter<'_>,
        window: &Window,
        bounds: Rect,
        theme: Theme,
    ) -> bool {
        let Some(instance) = self
            .instances
            .iter()
            .find(|instance| instance.window == window.id())
        else {
            return false;
        };
        instance.application.draw(painter, bounds, theme);
        true
    }

    pub fn handle_input(
        &mut self,
        window: WindowId,
        event: InputEvent,
        bounds: Rect,
    ) -> AppResponse {
        self.input_events = self.input_events.saturating_add(1);
        self.instances
            .iter_mut()
            .find(|instance| instance.window == window)
            .map_or_else(AppResponse::ignored, |instance| {
                instance.application.handle_input(event, bounds)
            })
    }

    pub fn request_close(&mut self, window: WindowId) -> bool {
        self.instances
            .iter_mut()
            .find(|instance| instance.window == window)
            .is_some_and(|instance| instance.application.request_close())
    }

    pub fn tick(&mut self, elapsed_ms: u32, windows: usize) -> Vec<WindowId> {
        self.elapsed_ms = self.elapsed_ms.saturating_add(elapsed_ms as u64);
        let snapshot = MonitorSnapshot {
            elapsed_ms: self.elapsed_ms,
            window_count: windows,
            input_events: self.input_events,
        };
        let mut changed = Vec::new();
        for instance in &mut self.instances {
            if let Application::Monitor(monitor) = &mut instance.application
                && monitor.tick(elapsed_ms, snapshot)
            {
                changed.push(instance.window);
            }
        }
        changed
    }

    pub fn retain_windows(&mut self, windows: &[Window]) {
        self.instances
            .retain(|instance| windows.iter().any(|window| window.id() == instance.window));
    }

    pub fn client_bounds(window: &Window) -> Rect {
        let bounds = window.bounds();
        Rect::new(
            bounds.x + 1,
            bounds.y + TITLE_BAR_HEIGHT as i32,
            bounds.width.saturating_sub(2),
            bounds.height.saturating_sub(TITLE_BAR_HEIGHT + 1),
        )
    }
}

impl Default for ApplicationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn fs_error_label(error: &FsError) -> String {
    let kind = match error.kind {
        FsErrorKind::NotFound => "NOT FOUND",
        FsErrorKind::AlreadyExists => "ALREADY EXISTS",
        FsErrorKind::PermissionDenied => "PERMISSION DENIED",
        FsErrorKind::InvalidInput => "INVALID INPUT",
        FsErrorKind::IsDirectory => "IS A DIRECTORY",
        FsErrorKind::NotDirectory => "NOT A DIRECTORY",
        FsErrorKind::TooLarge => "TOO LARGE",
        FsErrorKind::Unsupported => "UNSUPPORTED",
        FsErrorKind::Other => "IO ERROR",
    };
    alloc::format!("{}: {} ({})", error.operation, error.path, kind)
}

pub fn uppercase_extension(path: &str) -> String {
    path.rsplit_once('.')
        .map(|(_, extension)| extension.to_ascii_uppercase())
        .unwrap_or_default()
}
