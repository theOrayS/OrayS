use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use orays_desktop::app::DesktopApp;
use orays_desktop::platform::display::MemoryDisplay;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: render-golden OUTPUT.ppm [WIDTH HEIGHT]")?;
    let width = env::args()
        .nth(2)
        .map(|v| v.parse())
        .transpose()?
        .unwrap_or(1280);
    let height = env::args()
        .nth(3)
        .map(|v| v.parse())
        .transpose()?
        .unwrap_or(720);

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let display = MemoryDisplay::new(width, height, width as usize * 4)
        .map_err(|error| std::io::Error::other(format!("display: {error:?}")))?;
    let mut app = DesktopApp::new(display)
        .map_err(|error| std::io::Error::other(format!("app init: {error:?}")))?;
    app.render_boot_frame()
        .map_err(|error| std::io::Error::other(format!("render: {error:?}")))?;

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    write!(writer, "P6\n{width} {height}\n255\n")?;
    for y in 0..height {
        for x in 0..width {
            let pixel = app.surface().get(x, y).ok_or("pixel out of bounds")?;
            writer.write_all(&[pixel.r, pixel.g, pixel.b])?;
        }
    }
    writer.flush()?;
    println!("{} {:016x}", output.display(), app.frame_checksum());
    Ok(())
}
