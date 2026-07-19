use orays_desktop::app::DesktopApp;
use orays_desktop::graphics::damage::DamageTracker;
use orays_desktop::graphics::geometry::{Point, Rect};
use orays_desktop::graphics::painter::{Color, Painter};
use orays_desktop::graphics::surface::{Surface, SurfaceError};
use orays_desktop::platform::display::{DisplayBackend, MemoryDisplay};

#[test]
fn geometry_clips_and_unions_without_wraparound() {
    let left = Rect::new(-10, 4, 20, 10);
    let right = Rect::new(5, 0, 10, 20);
    assert_eq!(left.intersection(right), Some(Rect::new(5, 4, 5, 10)));
    assert_eq!(left.union(right), Rect::new(-10, 0, 25, 20));
    assert!(left.contains(Point::new(0, 8)));
    assert!(!left.contains(Point::new(10, 8)));
}

#[test]
fn surface_rejects_invalid_geometry() {
    assert!(matches!(Surface::new(0, 10, 10), Err(SurfaceError::Empty)));
    assert!(matches!(
        Surface::new(11, 10, 10),
        Err(SurfaceError::InvalidStride)
    ));
}

#[test]
fn painter_clips_and_alpha_blends() {
    let mut surface = Surface::new(4, 3, 6).unwrap();
    let bounds = surface.bounds();
    let mut painter = Painter::new(&mut surface, bounds);
    painter.fill_rect(bounds, Color::rgb(10, 20, 30));
    painter.blend_rect(Rect::new(3, 2, 5, 5), Color::rgba(210, 20, 30, 128));
    assert_eq!(surface.get(2, 2), Some(Color::rgb(10, 20, 30)));
    assert_eq!(surface.get(3, 2), Some(Color::rgb(110, 20, 30)));
}

#[test]
fn rounded_rect_leaves_corners_and_fills_center() {
    let mut surface = Surface::new(20, 16, 20).unwrap();
    let bounds = surface.bounds();
    let mut painter = Painter::new(&mut surface, bounds);
    painter.fill_rect(bounds, Color::rgb(4, 8, 12));
    painter.fill_rounded_rect(Rect::new(2, 2, 16, 12), 5, Color::rgb(90, 120, 180));
    assert_eq!(surface.get(2, 2), Some(Color::rgb(4, 8, 12)));
    assert_eq!(surface.get(10, 2), Some(Color::rgb(90, 120, 180)));
    assert_eq!(surface.get(10, 8), Some(Color::rgb(90, 120, 180)));
}

#[test]
fn damage_merges_touching_regions_and_collapses_budget() {
    let mut damage = DamageTracker::new(2);
    damage.add(Rect::new(0, 0, 4, 4));
    damage.add(Rect::new(4, 0, 4, 4));
    assert_eq!(damage.regions(), &[Rect::new(0, 0, 8, 4)]);
    damage.add(Rect::new(20, 20, 2, 2));
    damage.add(Rect::new(40, 40, 2, 2));
    assert_eq!(damage.regions(), &[Rect::new(0, 0, 42, 42)]);
}

#[test]
fn stride_copy_preserves_destination_padding() {
    let mut surface = Surface::new(2, 2, 3).unwrap();
    let bounds = surface.bounds();
    Painter::new(&mut surface, bounds).fill_rect(bounds, Color::rgb(1, 2, 3));
    let mut destination = [0xa5; 24];
    surface.copy_bgra8888_to(&mut destination, 12).unwrap();
    assert_eq!(&destination[0..4], &[3, 2, 1, 255]);
    assert_eq!(&destination[8..12], &[0xa5; 4]);
    assert_eq!(&destination[20..24], &[0xa5; 4]);
}

#[test]
fn damaged_copy_changes_only_clipped_regions() {
    let mut surface = Surface::new(3, 2, 3).unwrap();
    let bounds = surface.bounds();
    Painter::new(&mut surface, bounds).fill_rect(bounds, Color::rgb(8, 16, 24));
    let mut destination = [0xa5; 32];
    surface
        .copy_bgra8888_regions_to(&mut destination, 16, &[Rect::new(1, 0, 1, 1)])
        .unwrap();
    assert_eq!(&destination[4..8], &[24, 16, 8, 255]);
    assert_eq!(&destination[0..4], &[0xa5; 4]);
    assert_eq!(&destination[8..32], &[0xa5; 24]);
}

#[test]
fn app_commits_one_complete_frame_and_damage_record() {
    let display = MemoryDisplay::new(320, 200, 320 * 4 + 16).unwrap();
    let mut app = DesktopApp::new(display).unwrap();
    app.render_boot_frame().unwrap();
    assert_ne!(app.frame_checksum(), 0);
}

#[test]
fn memory_display_rejects_resolution_mismatch() {
    let mut display = MemoryDisplay::new(10, 10, 40).unwrap();
    let surface = Surface::new(9, 10, 9).unwrap();
    assert!(display.present(&surface, &[]).is_err());
}
