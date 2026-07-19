use orays_desktop::app::WindowedDesktop;
use orays_desktop::desktop::shell::{DesktopShell, SystemAction};
use orays_desktop::desktop::theme::ThemeKind;
use orays_desktop::desktop::window::WindowSpec;
use orays_desktop::graphics::geometry::{Point, Rect};
use orays_desktop::platform::display::MemoryDisplay;
use orays_desktop::platform::input::{InputEvent, KeyState, Modifiers, PointerButton};

fn key(code: u16, modifiers: Modifiers) -> InputEvent {
    InputEvent::Key {
        code,
        state: KeyState::Pressed,
        modifiers,
        text: None,
    }
}

fn click(x: i32, y: i32) -> InputEvent {
    InputEvent::PointerButton {
        button: PointerButton::Left,
        state: KeyState::Pressed,
        position: Point::new(x, y),
    }
}

fn desktop() -> WindowedDesktop<MemoryDisplay> {
    let display = MemoryDisplay::new(1280, 720, 1280 * 4).unwrap();
    WindowedDesktop::new(display).unwrap()
}

#[test]
fn workspace_reserves_shell_chrome_at_multiple_resolutions() {
    assert_eq!(
        DesktopShell::workspace(Rect::new(0, 0, 1280, 720)),
        Rect::new(0, 40, 1280, 602)
    );
    assert_eq!(
        DesktopShell::workspace(Rect::new(0, 0, 800, 600)),
        Rect::new(0, 40, 800, 482)
    );
}

#[test]
fn launcher_shortcut_animates_and_escape_closes_without_busy_frames() {
    let mut desktop = desktop();
    desktop.render_pending().unwrap();
    let modifiers = Modifiers {
        super_key: true,
        ..Modifiers::default()
    };
    assert!(!desktop.shell().launcher_fully_open());
    assert!(desktop.handle_input(key(57, modifiers)).unwrap());
    assert!(desktop.shell().launcher_open());
    assert!(!desktop.shell().launcher_fully_open());
    assert!(desktop.tick(90).unwrap());
    assert!(!desktop.shell().launcher_fully_open());
    assert!(desktop.tick(90).unwrap());
    assert!(desktop.shell().launcher_fully_open());
    assert!(!desktop.tick(16).unwrap());

    assert!(desktop.handle_input(key(1, Modifiers::default())).unwrap());
    assert!(!desktop.shell().launcher_open());
    assert!(!desktop.shell().launcher_fully_open());
    assert!(desktop.tick(180).unwrap());
    assert!(!desktop.tick(16).unwrap());
}

#[test]
fn dock_and_launcher_create_real_managed_windows() {
    let mut desktop = desktop();
    desktop
        .create_window(WindowSpec::new("WELCOME", Rect::new(80, 80, 420, 280)))
        .unwrap();
    desktop.render_pending().unwrap();
    let initial = desktop.windows().windows().len();

    // First Dock icon at 1280x720.
    assert!(desktop.handle_input(click(430, 675)).unwrap());
    assert_eq!(desktop.windows().windows().len(), initial + 1);
    assert_eq!(
        desktop.windows().windows().last().unwrap().title(),
        "TERMINAL"
    );

    let modifiers = Modifiers {
        super_key: true,
        ..Modifiers::default()
    };
    desktop.handle_input(key(57, modifiers)).unwrap();
    desktop.tick(180).unwrap();
    // First launcher card in the fully open 620x420 panel.
    assert!(desktop.handle_input(click(400, 160)).unwrap());
    assert_eq!(desktop.windows().windows().len(), initial + 2);
    assert!(!desktop.shell().launcher_open());
}

#[test]
fn theme_shortcut_recomposes_the_frame() {
    let mut desktop = desktop();
    desktop.render_pending().unwrap();
    let dark_checksum = desktop.frame_checksum();
    let modifiers = Modifiers {
        super_key: true,
        ..Modifiers::default()
    };
    assert!(desktop.handle_input(key(20, modifiers)).unwrap());
    assert_eq!(desktop.shell().theme_kind(), ThemeKind::Light);
    assert_ne!(desktop.frame_checksum(), dark_checksum);
}

#[test]
fn power_menu_reports_shutdown_and_restart_requests_without_faking_completion() {
    let mut desktop = desktop();
    desktop.render_pending().unwrap();
    assert!(desktop.handle_input(click(1220, 20)).unwrap());
    assert!(desktop.handle_input(click(1080, 115)).unwrap());
    assert_eq!(desktop.take_system_action(), Some(SystemAction::Shutdown));
    assert_eq!(desktop.take_system_action(), None);

    assert!(desktop.handle_input(click(1220, 20)).unwrap());
    assert!(desktop.handle_input(click(1180, 115)).unwrap());
    assert_eq!(desktop.take_system_action(), Some(SystemAction::Restart));
}
