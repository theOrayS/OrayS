use std::sync::atomic::{AtomicUsize, Ordering};

use orays_desktop::app::WindowedDesktop;
use orays_desktop::apps::file_manager::FileManager;
use orays_desktop::apps::image_viewer::{ImageOpenError, ImageViewer, MAX_IMAGE_FILE_BYTES};
use orays_desktop::apps::settings::Settings;
use orays_desktop::apps::system_monitor::{MonitorSnapshot, SystemMonitor};
use orays_desktop::apps::terminal::Terminal;
use orays_desktop::apps::text_editor::TextEditor;
use orays_desktop::apps::{AppAction, Application};
use orays_desktop::desktop::launcher::AppId;
use orays_desktop::desktop::theme::ThemeKind;
use orays_desktop::desktop::window_manager::close_rect;
use orays_desktop::graphics::geometry::Point;
use orays_desktop::graphics::image::{Bitmap, ImageError};
use orays_desktop::platform::display::MemoryDisplay;
use orays_desktop::platform::filesystem::{self, FsErrorKind, MAX_TEXT_BYTES};
use orays_desktop::platform::input::{InputEvent, KeyState, Modifiers, PointerButton};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(1);

struct TempDirectory {
    path: String,
}

impl TempDirectory {
    fn new(label: &str) -> Self {
        let path = format!(
            "/tmp/orays-desktop-{label}-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        );
        std::fs::create_dir(&path).unwrap();
        Self { path }
    }

    fn join(&self, name: &str) -> String {
        format!("{}/{name}", self.path)
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        assert!(self.path.starts_with("/tmp/orays-desktop-"));
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn filesystem_rejects_unsafe_names_and_reports_real_missing_paths() {
    assert_eq!(
        filesystem::join("/tmp", "../escape").unwrap_err().kind,
        FsErrorKind::InvalidInput
    );
    assert_eq!(filesystem::parent("/"), "/");
    let missing = filesystem::read_text("/tmp/orays-desktop-definitely-missing").unwrap_err();
    assert_eq!(missing.kind, FsErrorKind::NotFound);
}

#[test]
fn bounded_file_reads_reject_oversized_text_and_images_before_loading() {
    let temp = TempDirectory::new("bounded-read");
    let text_path = temp.join("oversized.txt");
    std::fs::File::create(&text_path)
        .unwrap()
        .set_len((MAX_TEXT_BYTES + 1) as u64)
        .unwrap();
    assert_eq!(
        filesystem::read_text(&text_path).unwrap_err().kind,
        FsErrorKind::TooLarge
    );

    let image_path = temp.join("oversized.ppm");
    std::fs::File::create(&image_path)
        .unwrap()
        .set_len((MAX_IMAGE_FILE_BYTES + 1) as u64)
        .unwrap();
    let mut viewer = ImageViewer::new();
    assert!(matches!(
        viewer.open(&image_path),
        Err(ImageOpenError::Filesystem(error)) if error.kind == FsErrorKind::TooLarge
    ));
}

#[test]
fn terminal_executes_real_filesystem_commands_and_preserves_errors() {
    let temp = TempDirectory::new("terminal");
    let mut terminal = Terminal::new(&temp.path);
    terminal.execute_command("write note.txt hello desktop");
    assert_eq!(
        std::fs::read_to_string(temp.join("note.txt")).unwrap(),
        "hello desktop"
    );
    terminal.execute_command("ls");
    assert!(
        terminal
            .output()
            .iter()
            .any(|line| line.contains("note.txt"))
    );
    terminal.execute_command("cat note.txt");
    assert!(terminal.output().iter().any(|line| line == "hello desktop"));
    terminal.execute_command("cat /tmp/orays-desktop-no-such-file");
    assert!(
        terminal
            .output()
            .iter()
            .any(|line| line.contains("NOT FOUND"))
    );
    terminal.execute_command("not-a-command");
    assert!(
        terminal
            .output()
            .iter()
            .any(|line| line.contains("NOT SUPPORTED"))
    );
}

#[test]
fn file_manager_performs_create_rename_delete_and_open_on_real_entries() {
    let temp = TempDirectory::new("files");
    std::fs::write(temp.join("note.txt"), "real contents").unwrap();
    let mut files = FileManager::new(&temp.path);
    assert!(files.error().is_none());

    files.create_directory("folder").unwrap();
    let folder = files
        .entries()
        .iter()
        .position(|entry| entry.name == "folder")
        .unwrap();
    assert!(files.select(folder));
    files.rename_selected("renamed").unwrap();
    assert!(std::path::Path::new(&temp.join("renamed")).is_dir());
    let renamed = files
        .entries()
        .iter()
        .position(|entry| entry.name == "renamed")
        .unwrap();
    files.select(renamed);
    files.delete_selected().unwrap();
    assert!(!std::path::Path::new(&temp.join("renamed")).exists());

    let note = files
        .entries()
        .iter()
        .position(|entry| entry.name == "note.txt")
        .unwrap();
    files.select(note);
    assert_eq!(
        files.open_selected(),
        AppAction::OpenEditor(temp.join("note.txt"))
    );
    files.refresh().unwrap();
    assert_eq!(
        files.delete_selected().unwrap_err().kind,
        FsErrorKind::InvalidInput
    );
}

#[test]
fn editor_opens_edits_saves_and_requires_unsaved_confirmation() {
    let temp = TempDirectory::new("editor");
    let path = temp.join("draft.txt");
    std::fs::write(&path, "draft").unwrap();
    let mut editor = TextEditor::new();
    editor.open(&path).unwrap();
    assert_eq!(editor.contents(), "draft");
    assert!(editor.insert('!'));
    assert!(editor.dirty());
    assert!(editor.request_close());
    assert!(editor.close_prompt_visible());
    editor.save().unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "draft!");
    assert!(!editor.dirty());
    assert!(!editor.request_close());

    let mut missing = TextEditor::new();
    assert_eq!(
        missing
            .open("/tmp/orays-desktop-missing-editor")
            .unwrap_err()
            .kind,
        FsErrorKind::NotFound
    );
}

#[test]
fn ppm_viewer_decodes_real_pixels_clamps_zoom_and_rejects_truncation() {
    let temp = TempDirectory::new("image");
    let path = temp.join("sample.ppm");
    let ppm = b"P6\n2 1\n255\n\xff\x00\x00\x00\xff\x00";
    std::fs::write(&path, ppm).unwrap();
    let bitmap = Bitmap::parse_ppm(ppm).unwrap();
    assert_eq!((bitmap.width(), bitmap.height()), (2, 1));
    assert_eq!(bitmap.pixel(0, 0).unwrap().r, 255);
    assert_eq!(
        Bitmap::parse_ppm(b"P6\n2 1\n255\n\xff").unwrap_err(),
        ImageError::Truncated
    );

    let mut viewer = ImageViewer::new();
    viewer.open(&path).unwrap();
    for _ in 0..20 {
        viewer.zoom_in();
    }
    assert_eq!(viewer.zoom_percent(), 400);
    for _ in 0..20 {
        viewer.zoom_out();
    }
    assert_eq!(viewer.zoom_percent(), 25);
    assert!(matches!(
        viewer.open("/tmp/orays-desktop-missing-image"),
        Err(ImageOpenError::Filesystem(_))
    ));
}

#[test]
fn monitor_updates_only_from_real_counters_and_marks_unavailable_metrics() {
    let mut monitor = SystemMonitor::new();
    let snapshot = MonitorSnapshot {
        elapsed_ms: 1000,
        window_count: 3,
        input_events: 7,
    };
    assert!(monitor.tick(1, snapshot));
    assert_eq!(monitor.snapshot(), snapshot);
    assert!(!monitor.tick(999, snapshot));
    assert!(!monitor.tick(1, snapshot));
    assert!(!monitor.cpu_usage_supported());
    assert!(!monitor.memory_usage_supported());
}

#[test]
fn settings_emit_live_theme_and_wallpaper_actions_without_fixed_success() {
    let mut settings = Settings::new();
    assert_eq!(
        settings.toggle_theme(),
        AppAction::SetTheme(ThemeKind::Light)
    );
    assert_eq!(settings.cycle_wallpaper(), AppAction::SetWallpaper(1));
    assert_eq!(settings.cycle_wallpaper(), AppAction::SetWallpaper(2));
    assert_eq!(settings.cycle_wallpaper(), AppAction::SetWallpaper(0));
}

#[test]
fn settings_pointer_action_changes_the_live_shell_theme() {
    let display = MemoryDisplay::new(1280, 720, 1280 * 4).unwrap();
    let mut desktop = WindowedDesktop::new(display).unwrap();
    desktop.launch_application(AppId::Settings).unwrap();
    desktop.render_pending().unwrap();
    assert_eq!(desktop.shell().theme_kind(), ThemeKind::Dark);
    desktop
        .handle_input(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            position: Point::new(300, 180),
        })
        .unwrap();
    assert_eq!(desktop.shell().theme_kind(), ThemeKind::Light);

    let second = desktop.launch_application(AppId::Settings).unwrap();
    let Application::Settings(settings) = desktop.apps().application(second).unwrap() else {
        panic!("second settings window did not own a settings model");
    };
    assert_eq!(settings.theme(), ThemeKind::Light);
}

#[test]
fn window_runtime_routes_text_to_editor_and_guards_dirty_close() {
    let display = MemoryDisplay::new(1280, 720, 1280 * 4).unwrap();
    let mut desktop = WindowedDesktop::new(display).unwrap();
    let id = desktop.launch_application(AppId::Editor).unwrap();
    desktop.render_pending().unwrap();
    desktop
        .handle_input(InputEvent::Key {
            code: 45,
            state: KeyState::Pressed,
            modifiers: Modifiers::default(),
            text: Some('x'),
        })
        .unwrap();
    let Application::Editor(editor) = desktop.apps().application(id).unwrap() else {
        panic!("editor window did not own an editor model");
    };
    assert!(editor.dirty());

    let close = close_rect(desktop.windows().window(id).unwrap());
    desktop
        .handle_input(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            position: Point::new(close.x + 4, close.y + 4),
        })
        .unwrap();
    assert!(desktop.windows().window(id).is_some());
    let Application::Editor(editor) = desktop.apps().application(id).unwrap() else {
        panic!("editor model disappeared while close prompt was open");
    };
    assert!(editor.close_prompt_visible());
}
