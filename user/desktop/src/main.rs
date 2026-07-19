#![no_std]
#![no_main]

#[macro_use]
extern crate axstd as std;

use orays_desktop::app::WindowedDesktop;
use orays_desktop::desktop::launcher::AppId;
use orays_desktop::desktop::shell::SystemAction;
use orays_desktop::platform::display::OraysDisplay;
use orays_desktop::platform::input::{InputTranslator, initialize_orays_input, poll_orays_raw};
use orays_desktop::platform::system;

#[unsafe(no_mangle)]
fn main() {
    let display = OraysDisplay::new().expect("desktop display unavailable");
    let mut app = WindowedDesktop::new(display).expect("desktop initialization failed");
    app.launch_application(AppId::Files)
        .expect("desktop file manager creation failed");
    app.launch_application(AppId::Monitor)
        .expect("desktop monitor creation failed");
    app.render_pending().expect("desktop boot frame failed");
    println!(
        "ORAYS_DESKTOP_DISPLAY width={} height={}",
        app.surface().width(),
        app.surface().height()
    );
    initialize_orays_input();
    println!("ORAYS_DESKTOP_FRAME boot {}", app.frame_checksum());
    while let Some(marker) = app.take_action_marker() {
        println!("ORAYS_DESKTOP_ACTION {}", marker);
    }

    let mut input = InputTranslator::<128>::new(app.surface().width(), app.surface().height());
    let mut previous_tick = std::time::Instant::now();
    let mut launcher_was_fully_open = app.shell().launcher_fully_open();

    loop {
        input.resize(app.surface().width(), app.surface().height());
        let mut raw_count = 0;
        while raw_count < 64 {
            let Some(raw) = poll_orays_raw() else {
                break;
            };
            input.feed(raw);
            raw_count += 1;
        }

        let mut event_count = 0;
        while let Some(event) = input.pop() {
            if app
                .handle_input(event)
                .expect("desktop input handling failed")
            {
                println!("ORAYS_DESKTOP_FRAME input {}", app.frame_checksum());
            }
            // Input is acknowledged only after its synchronous handler and any
            // resulting display present have completed. Headless capture may
            // therefore use this marker as a processed-input barrier.
            println!("ORAYS_DESKTOP_INPUT {:?}", event);
            while let Some(marker) = app.take_action_marker() {
                println!("ORAYS_DESKTOP_ACTION {}", marker);
            }
            event_count += 1;
        }

        let idle_ms: u32 = if raw_count == 0 && event_count == 0 {
            8
        } else {
            1
        };
        std::thread::sleep(core::time::Duration::from_millis(idle_ms as u64));
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(previous_tick).as_millis().clamp(1, 1000) as u32;
        previous_tick = now;
        if app.tick(elapsed_ms).expect("desktop animation tick failed") {
            println!("ORAYS_DESKTOP_FRAME animation {}", app.frame_checksum());
        }
        let launcher_is_fully_open = app.shell().launcher_fully_open();
        if launcher_is_fully_open && !launcher_was_fully_open {
            println!("ORAYS_DESKTOP_STATE LAUNCHER OPEN_STABLE");
        }
        launcher_was_fully_open = launcher_is_fully_open;
        if let Some(action) = app.take_system_action() {
            println!("ORAYS_DESKTOP_SYSTEM_ACTION {:?}", action);
            match action {
                SystemAction::Shutdown => system::shutdown(),
                SystemAction::Restart => println!(
                    "ORAYS_DESKTOP_SYSTEM_ACTION_UNSUPPORTED restart_supported={}",
                    system::RESTART_SUPPORTED
                ),
            }
        }
    }
}
