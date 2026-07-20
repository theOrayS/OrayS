use std::cell::Cell;
use std::rc::Rc;

use orays_desktop::app::WindowedDesktop;
use orays_desktop::desktop::compositor::Compositor;
use orays_desktop::desktop::shell::DesktopShell;
use orays_desktop::desktop::window::{WindowSpec, WindowState};
use orays_desktop::desktop::window_manager::{HitTarget, WindowError, WindowManager};
use orays_desktop::graphics::geometry::{Point, Rect, Size};
use orays_desktop::graphics::surface::Surface;
use orays_desktop::platform::display::{
    DisplayBackend, DisplayDescriptor, DisplayError, MemoryDisplay, PixelFormat,
};
use orays_desktop::platform::input::{InputEvent, KeyState, Modifiers, PointerButton};

fn manager() -> WindowManager {
    WindowManager::new(Rect::new(0, 36, 800, 520)).unwrap()
}

#[test]
fn create_focus_z_order_alt_tab_and_close_are_consistent() {
    let mut manager = manager();
    let first = manager
        .create(WindowSpec::new("FILES", Rect::new(40, 70, 320, 240)))
        .unwrap();
    let second = manager
        .create(WindowSpec::new("TERMINAL", Rect::new(180, 110, 360, 260)))
        .unwrap();
    assert_eq!(manager.focused(), Some(second));
    assert_eq!(manager.windows().last().unwrap().id(), second);

    assert_eq!(manager.alt_tab(false), Some(first));
    assert_eq!(manager.windows().last().unwrap().id(), first);
    assert_eq!(manager.alt_tab(true), Some(second));

    manager.close(second).unwrap();
    assert_eq!(manager.focused(), Some(first));
    assert!(manager.window(second).is_none());
}

#[test]
fn pointer_move_and_resize_respect_workspace_and_minimum_size() {
    let mut manager = manager();
    let mut spec = WindowSpec::new("EDITOR", Rect::new(100, 100, 300, 220));
    spec.minimum_size = Size::new(220, 160);
    let id = manager.create(spec).unwrap();
    manager.clear_damage();

    assert_eq!(
        manager.pointer_press(Point::new(150, 112)).unwrap(),
        HitTarget::TitleBar(id)
    );
    manager.pointer_move(Point::new(250, 182)).unwrap();
    manager.pointer_release();
    assert_eq!(
        manager.window(id).unwrap().bounds(),
        Rect::new(200, 170, 300, 220)
    );
    assert!(!manager.damage().is_empty());

    let bottom_right = Point::new(500, 390);
    assert!(matches!(
        manager.pointer_press(bottom_right).unwrap(),
        HitTarget::Resize(candidate, _) if candidate == id
    ));
    manager.pointer_move(Point::new(270, 230)).unwrap();
    manager.pointer_release();
    assert_eq!(manager.window(id).unwrap().bounds().width, 220);
    assert_eq!(manager.window(id).unwrap().bounds().height, 160);
}

#[test]
fn maximize_minimize_restore_preserve_normal_bounds() {
    let mut manager = manager();
    let original = Rect::new(80, 90, 320, 230);
    let id = manager
        .create(WindowSpec::new("SETTINGS", original))
        .unwrap();

    manager.maximize(id).unwrap();
    assert_eq!(manager.window(id).unwrap().state(), WindowState::Maximized);
    assert_eq!(manager.window(id).unwrap().bounds(), manager.workspace());
    manager.restore(id).unwrap();
    assert_eq!(manager.window(id).unwrap().bounds(), original);

    manager.minimize(id).unwrap();
    assert_eq!(manager.window(id).unwrap().state(), WindowState::Minimized);
    assert_eq!(manager.focused(), None);
    manager.restore(id).unwrap();
    assert_eq!(manager.window(id).unwrap().bounds(), original);
    assert_eq!(manager.focused(), Some(id));
}

#[test]
fn modal_window_blocks_owner_and_closes_with_owner() {
    let mut manager = manager();
    let owner = manager
        .create(WindowSpec::new("EDITOR", Rect::new(80, 80, 420, 300)))
        .unwrap();
    let mut dialog = WindowSpec::new("SAVE CHANGES", Rect::new(180, 150, 280, 160));
    dialog.modal_for = Some(owner);
    let modal = manager.create(dialog).unwrap();

    assert_eq!(manager.active_modal(), Some(modal));
    assert_eq!(manager.focus(owner), Err(WindowError::BlockedByModal));
    assert_eq!(manager.minimize(owner), Err(WindowError::BlockedByModal));
    assert_eq!(manager.alt_tab(false), Some(modal));
    assert_eq!(
        manager.hit_test(Point::new(90, 90)),
        HitTarget::ModalBackdrop(modal)
    );

    manager.close(modal).unwrap();
    assert_eq!(manager.focused(), Some(owner));
    let mut second_dialog = WindowSpec::new("CONFIRM", Rect::new(190, 160, 260, 150));
    second_dialog.modal_for = Some(owner);
    let child = manager.create(second_dialog).unwrap();
    manager.close(owner).unwrap();
    assert!(manager.window(owner).is_none());
    assert!(manager.window(child).is_none());
}

#[test]
fn damage_render_matches_fresh_full_composition() {
    let mut manager = manager();
    let first = manager
        .create(WindowSpec::new("FILES", Rect::new(45, 75, 330, 245)))
        .unwrap();
    manager
        .create(WindowSpec::new("ABOUT", Rect::new(260, 150, 300, 210)))
        .unwrap();
    let compositor = Compositor::new();
    let mut incremental = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut incremental, &manager);
    manager.clear_damage();

    manager.focus(first).unwrap();
    manager.pointer_press(Point::new(100, 87)).unwrap();
    manager.pointer_move(Point::new(170, 127)).unwrap();
    manager.pointer_release();
    let damage = manager.damage().to_vec();
    assert!(!damage.is_empty());
    compositor.render(&mut incremental, &manager, &damage);

    let mut full = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut full, &manager);
    assert_surfaces_equal(&incremental, &full);
}

#[test]
fn creating_a_non_overlapping_window_damages_both_focus_decorations() {
    let mut manager = manager();
    manager
        .create(WindowSpec::new("FILES", Rect::new(40, 70, 260, 200)))
        .unwrap();
    let compositor = Compositor::new();
    let mut incremental = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut incremental, &manager);
    manager.clear_damage();

    manager
        .create(WindowSpec::new("TERMINAL", Rect::new(470, 280, 280, 210)))
        .unwrap();
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
}

#[test]
fn closing_or_minimizing_the_focused_window_damages_the_new_focus() {
    for minimize in [false, true] {
        let mut manager = manager();
        manager
            .create(WindowSpec::new("FILES", Rect::new(40, 70, 260, 200)))
            .unwrap();
        let focused = manager
            .create(WindowSpec::new("TERMINAL", Rect::new(470, 280, 280, 210)))
            .unwrap();
        let compositor = Compositor::new();
        let mut incremental = Surface::new(800, 600, 800).unwrap();
        compositor.render_full(&mut incremental, &manager);
        manager.clear_damage();

        if minimize {
            manager.minimize(focused).unwrap();
        } else {
            manager.close(focused).unwrap();
        }
        render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
    }
}

#[test]
fn clicking_the_desktop_to_clear_focus_damages_the_old_decoration() {
    let mut manager = manager();
    manager
        .create(WindowSpec::new("FILES", Rect::new(40, 70, 260, 200)))
        .unwrap();
    let compositor = Compositor::new();
    let mut incremental = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut incremental, &manager);
    manager.clear_damage();

    assert_eq!(
        manager.pointer_press(Point::new(760, 530)).unwrap(),
        HitTarget::Desktop
    );
    assert_eq!(manager.focused(), None);
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
}

#[test]
fn cursor_damage_is_bounded_and_incremental() {
    let mut manager = manager();
    manager
        .create(WindowSpec::new("TERMINAL", Rect::new(100, 100, 360, 240)))
        .unwrap();
    let mut compositor = Compositor::new();
    let mut incremental = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut incremental, &manager);
    let cursor_damage = compositor.set_cursor(Point::new(420, 260)).unwrap();
    compositor.render(&mut incremental, &manager, &[cursor_damage]);

    let mut full = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut full, &manager);
    assert_surfaces_equal(&incremental, &full);
    assert!(cursor_damage.width <= 440 && cursor_damage.height <= 300);
}

#[test]
fn open_minimize_and_close_animations_finish_in_bounded_ticks() {
    let mut manager = manager();
    let id = manager
        .create(WindowSpec::new("TERMINAL", Rect::new(100, 100, 360, 240)))
        .unwrap();
    let normal = manager.window(id).unwrap().bounds();
    manager.animate_open(id).unwrap();
    assert_eq!(manager.animation_count(), 1);
    assert_ne!(manager.visual_bounds(id), Some(normal));
    assert!(manager.tick_animations(80));
    assert!(manager.tick_animations(80));
    assert_eq!(manager.animation_count(), 0);
    assert_eq!(manager.visual_bounds(id), Some(normal));

    manager.animate_minimize(id).unwrap();
    assert!(manager.tick_animations(160));
    assert_eq!(manager.window(id).unwrap().state(), WindowState::Minimized);
    manager.restore(id).unwrap();

    manager.animate_close(id).unwrap();
    assert!(manager.tick_animations(10_000));
    assert!(manager.window(id).is_none());
    assert_eq!(manager.animation_count(), 0);
    assert!(!manager.tick_animations(16));
}

#[test]
fn animated_damage_matches_fresh_composition_at_each_transition() {
    let mut manager = manager();
    let id = manager
        .create(WindowSpec::new("EDITOR", Rect::new(120, 100, 420, 300)))
        .unwrap();
    manager.animate_open(id).unwrap();
    let compositor = Compositor::new();
    let mut incremental = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut incremental, &manager);
    manager.clear_damage();

    manager.tick_animations(80);
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
    manager.tick_animations(80);
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);

    manager.animate_minimize(id).unwrap();
    manager.tick_animations(80);
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
    manager.tick_animations(80);
    render_incremental_and_compare(&compositor, &mut incremental, &mut manager);
    assert_eq!(manager.window(id).unwrap().state(), WindowState::Minimized);
}

#[test]
fn interactive_desktop_commits_pointer_drags_to_the_display() {
    let display = MemoryDisplay::new(640, 480, 640 * 4).unwrap();
    let mut desktop = WindowedDesktop::new(display).unwrap();
    let id = desktop
        .create_window(WindowSpec::new("FILES", Rect::new(60, 70, 300, 220)))
        .unwrap();
    assert!(desktop.render_pending().unwrap());
    let boot_checksum = desktop.frame_checksum();

    desktop
        .handle_input(InputEvent::PointerMoved {
            position: Point::new(120, 82),
            delta_x: 120,
            delta_y: 82,
        })
        .unwrap();
    desktop
        .handle_input(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Pressed,
            position: Point::new(120, 82),
        })
        .unwrap();
    desktop
        .handle_input(InputEvent::PointerMoved {
            position: Point::new(200, 122),
            delta_x: 80,
            delta_y: 40,
        })
        .unwrap();
    desktop
        .handle_input(InputEvent::PointerButton {
            button: PointerButton::Left,
            state: KeyState::Released,
            position: Point::new(200, 122),
        })
        .unwrap();

    assert_eq!(
        desktop.windows().window(id).unwrap().bounds(),
        Rect::new(140, 110, 300, 220)
    );
    assert_ne!(desktop.frame_checksum(), boot_checksum);

    let no_alt = desktop
        .handle_input(InputEvent::Key {
            code: 15,
            state: KeyState::Pressed,
            modifiers: Modifiers::default(),
            text: Some('\t'),
        })
        .unwrap();
    assert!(!no_alt);
}

struct ResizableDisplay {
    descriptor: Rc<Cell<DisplayDescriptor>>,
    frames: Rc<Cell<u64>>,
}

impl DisplayBackend for ResizableDisplay {
    fn info(&self) -> DisplayDescriptor {
        self.descriptor.get()
    }

    fn present(&mut self, surface: &Surface, _damage: &[Rect]) -> Result<(), DisplayError> {
        let descriptor = self.descriptor.get();
        if surface.width() != descriptor.width || surface.height() != descriptor.height {
            return Err(DisplayError::InvalidGeometry);
        }
        self.frames.set(self.frames.get() + 1);
        Ok(())
    }
}

#[test]
fn runtime_display_resize_rebuilds_surface_workspace_and_window_constraints() {
    let descriptor = Rc::new(Cell::new(DisplayDescriptor {
        width: 900,
        height: 650,
        stride_bytes: 900 * 4,
        pixel_format: PixelFormat::Bgra8888,
    }));
    let frames = Rc::new(Cell::new(0));
    let display = ResizableDisplay {
        descriptor: Rc::clone(&descriptor),
        frames: Rc::clone(&frames),
    };
    let mut desktop = WindowedDesktop::new(display).unwrap();
    let normal = desktop
        .create_window(WindowSpec::new("FILES", Rect::new(580, 360, 300, 220)))
        .unwrap();
    assert!(desktop.render_pending().unwrap());

    descriptor.set(DisplayDescriptor {
        width: 640,
        height: 480,
        stride_bytes: 640 * 4,
        pixel_format: PixelFormat::Bgra8888,
    });
    assert!(desktop.tick(1).unwrap());
    assert_eq!(
        (desktop.surface().width(), desktop.surface().height()),
        (640, 480)
    );
    let workspace = DesktopShell::workspace(Rect::new(0, 0, 640, 480));
    assert_eq!(desktop.windows().workspace(), workspace);

    let normal_bounds = desktop.windows().window(normal).unwrap().bounds();
    assert!(workspace.contains(Point::new(normal_bounds.x, normal_bounds.y)));
    assert!(normal_bounds.right() <= workspace.right());
    assert!(normal_bounds.bottom() <= workspace.bottom());
    assert_eq!(frames.get(), 2);
}

#[test]
fn workspace_shrink_reconstrains_maximized_restore_bounds() {
    let initial_workspace = Rect::new(0, 36, 900, 540);
    let mut manager = WindowManager::new(initial_workspace).unwrap();
    let id = manager
        .create(WindowSpec::new("EDITOR", Rect::new(590, 350, 300, 220)))
        .unwrap();
    manager.maximize(id).unwrap();

    let smaller_workspace = Rect::new(0, 36, 640, 380);
    assert!(manager.resize_workspace(smaller_workspace).unwrap());
    let window = manager.window(id).unwrap();
    assert_eq!(window.state(), WindowState::Maximized);
    assert_eq!(window.bounds(), smaller_workspace);
    let restore_bounds = window.restore_bounds();
    assert!(smaller_workspace.contains(Point::new(restore_bounds.x, restore_bounds.y)));
    assert!(restore_bounds.right() <= smaller_workspace.right());
    assert!(restore_bounds.bottom() <= smaller_workspace.bottom());

    manager.restore(id).unwrap();
    assert_eq!(manager.window(id).unwrap().bounds(), restore_bounds);
}

fn assert_surfaces_equal(left: &Surface, right: &Surface) {
    let difference = left
        .pixels()
        .iter()
        .zip(right.pixels())
        .enumerate()
        .find(|(_, (left, right))| left != right);
    if let Some((index, (left_pixel, right_pixel))) = difference {
        panic!(
            "first surface difference at ({}, {}): {left_pixel:#010x} != {right_pixel:#010x}",
            index % left.stride_pixels(),
            index / left.stride_pixels(),
        );
    }
}

fn render_incremental_and_compare(
    compositor: &Compositor,
    incremental: &mut Surface,
    manager: &mut WindowManager,
) {
    let damage = manager.damage().to_vec();
    assert!(!damage.is_empty());
    compositor.render(incremental, manager, &damage);
    let mut full = Surface::new(800, 600, 800).unwrap();
    compositor.render_full(&mut full, manager);
    assert_surfaces_equal(incremental, &full);
    manager.clear_damage();
}
