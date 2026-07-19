use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

use orays_desktop::app::{WindowedDesktop, default_window_specs};
use orays_desktop::desktop::launcher::AppId;
use orays_desktop::desktop::window::WindowSpec;
use orays_desktop::graphics::geometry::{Point, Rect};
use orays_desktop::platform::display::MemoryDisplay;
use orays_desktop::platform::input::{InputEvent, KeyState, Modifiers, PointerButton};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: render-scene OUTPUT.ppm SCENE [WIDTH HEIGHT]")?;
    let scene = env::args().nth(2).unwrap_or_else(|| "boot".into());
    let width = env::args()
        .nth(3)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(1280);
    let height = env::args()
        .nth(4)
        .map(|value| value.parse())
        .transpose()?
        .unwrap_or(720);

    let render_started = Instant::now();
    let display = MemoryDisplay::new(width, height, width as usize * 4)
        .map_err(|error| std::io::Error::other(format!("display: {error:?}")))?;
    let mut desktop = WindowedDesktop::new(display)
        .map_err(|error| std::io::Error::other(format!("desktop: {error:?}")))?;
    if scene == "applications" {
        let fixture_root = PathBuf::from("test/output/desktop/application-scene-files");
        if fixture_root.exists() {
            std::fs::remove_dir_all(&fixture_root)?;
        }
        std::fs::create_dir_all(&fixture_root)?;
        std::fs::write(
            fixture_root.join("README.txt"),
            "REAL FILE FROM THE HOST FILESYSTEM\n",
        )?;
        std::fs::write(
            fixture_root.join("sample.ppm"),
            b"P6\n2 1\n255\n\xff\x00\x00\x00\x80\xff",
        )?;
        desktop
            .launch_application(AppId::Monitor)
            .map_err(|error| std::io::Error::other(format!("monitor: {error:?}")))?;
        desktop
            .launch_file_manager_at(fixture_root.to_string_lossy().as_ref())
            .map_err(|error| std::io::Error::other(format!("file manager: {error:?}")))?;
        desktop
            .launch_editor_path(fixture_root.join("README.txt").to_string_lossy().as_ref())
            .map_err(|error| std::io::Error::other(format!("editor: {error:?}")))?;
    } else {
        for spec in default_window_specs(width, height) {
            desktop
                .create_window(spec)
                .map_err(|error| std::io::Error::other(format!("window: {error:?}")))?;
        }
    }
    desktop
        .tick(180)
        .map_err(|error| std::io::Error::other(format!("open animation: {error:?}")))?;
    apply_scene(&mut desktop, &scene).map_err(std::io::Error::other)?;
    desktop
        .render_pending()
        .map_err(|error| std::io::Error::other(format!("render: {error:?}")))?;
    let render_elapsed_us = render_started.elapsed().as_micros();

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "P6\n{width} {height}\n255\n")?;
    for y in 0..height {
        for x in 0..width {
            let pixel = desktop.surface().get(x, y).ok_or("pixel out of bounds")?;
            writer.write_all(&[pixel.r, pixel.g, pixel.b])?;
        }
    }
    writer.flush()?;
    println!(
        "{} scene={} checksum={:016x} render_elapsed_us={}",
        output.display(),
        scene,
        desktop.frame_checksum(),
        render_elapsed_us
    );
    Ok(())
}

fn apply_scene(desktop: &mut WindowedDesktop<MemoryDisplay>, scene: &str) -> Result<(), String> {
    match scene {
        "boot" | "windows" => {}
        "launcher" => {
            desktop
                .handle_input(InputEvent::Key {
                    code: 57,
                    state: KeyState::Pressed,
                    modifiers: Modifiers {
                        super_key: true,
                        ..Modifiers::default()
                    },
                    text: None,
                })
                .map_err(|error| format!("launcher input: {error:?}"))?;
            desktop
                .tick(180)
                .map_err(|error| format!("launcher animation: {error:?}"))?;
        }
        "light" => {
            desktop
                .handle_input(InputEvent::Key {
                    code: 20,
                    state: KeyState::Pressed,
                    modifiers: Modifiers {
                        super_key: true,
                        ..Modifiers::default()
                    },
                    text: None,
                })
                .map_err(|error| format!("theme input: {error:?}"))?;
        }
        "power" => {
            desktop
                .handle_input(InputEvent::PointerButton {
                    button: PointerButton::Left,
                    state: KeyState::Pressed,
                    position: Point::new(desktop.surface().width() as i32 - 55, 20),
                })
                .map_err(|error| format!("power input: {error:?}"))?;
        }
        "overlap" => {
            desktop
                .create_window(WindowSpec::new(
                    "TEXT EDITOR",
                    Rect::new(340, 170, 600, 390),
                ))
                .map_err(|error| format!("overlap window: {error:?}"))?;
            desktop
                .tick(180)
                .map_err(|error| format!("overlap animation: {error:?}"))?;
        }
        "applications" => {
            desktop
                .tick(1000)
                .map_err(|error| format!("monitor refresh: {error:?}"))?;
        }
        _ => return Err(format!("unknown desktop scene: {scene}")),
    }
    Ok(())
}
