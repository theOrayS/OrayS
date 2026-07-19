use alloc::string::ToString;

use crate::apps::AppResponse;
use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::text::draw_text;
use crate::platform::input::InputEvent;
use crate::platform::system;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MonitorSnapshot {
    pub elapsed_ms: u64,
    pub window_count: usize,
    pub input_events: u64,
}

pub struct SystemMonitor {
    snapshot: MonitorSnapshot,
    refresh_elapsed_ms: u32,
    initialized: bool,
}

impl SystemMonitor {
    pub const fn new() -> Self {
        Self {
            snapshot: MonitorSnapshot {
                elapsed_ms: 0,
                window_count: 0,
                input_events: 0,
            },
            refresh_elapsed_ms: 0,
            initialized: false,
        }
    }

    pub const fn snapshot(&self) -> MonitorSnapshot {
        self.snapshot
    }

    pub const fn cpu_usage_supported(&self) -> bool {
        system::CPU_USAGE_SUPPORTED
    }

    pub const fn memory_usage_supported(&self) -> bool {
        system::MEMORY_USAGE_SUPPORTED
    }

    pub fn tick(&mut self, elapsed_ms: u32, snapshot: MonitorSnapshot) -> bool {
        if !self.initialized {
            self.initialized = true;
            self.snapshot = snapshot;
            self.refresh_elapsed_ms = 0;
            return true;
        }
        self.refresh_elapsed_ms = self.refresh_elapsed_ms.saturating_add(elapsed_ms);
        if self.refresh_elapsed_ms < 1000 {
            return false;
        }
        self.refresh_elapsed_ms %= 1000;
        let changed = self.snapshot != snapshot;
        self.snapshot = snapshot;
        changed
    }

    pub fn handle_input(&mut self, _event: InputEvent, _bounds: Rect) -> AppResponse {
        AppResponse::ignored()
    }

    pub fn draw(&self, painter: &mut Painter<'_>, bounds: Rect, theme: Theme) {
        painter.fill_rect(bounds, theme.window);
        let metric_y = metric_baselines(bounds);
        draw_metric(
            painter,
            bounds.x + 20,
            metric_y[0],
            "DESKTOP UPTIME MS",
            &self.snapshot.elapsed_ms.to_string(),
            theme,
        );
        draw_metric(
            painter,
            bounds.x + 20,
            metric_y[1],
            "WINDOWS",
            &self.snapshot.window_count.to_string(),
            theme,
        );
        draw_metric(
            painter,
            bounds.x + 20,
            metric_y[2],
            "INPUT EVENTS",
            &self.snapshot.input_events.to_string(),
            theme,
        );
        draw_metric(
            painter,
            bounds.x + 20,
            metric_y[3],
            "CPU USAGE",
            "UNSUPPORTED",
            theme,
        );
        draw_metric(
            painter,
            bounds.x + 20,
            metric_y[4],
            "MEMORY USAGE",
            "UNSUPPORTED",
            theme,
        );
        if let Some(footer_y) = footer_baseline(bounds, metric_y[4]) {
            draw_text(
                painter,
                bounds.x + 20,
                footer_y,
                "NO SYNTHETIC METRICS",
                Color::rgb(112, 124, 142),
                1,
            );
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

fn draw_metric(painter: &mut Painter<'_>, x: i32, y: i32, label: &str, value: &str, theme: Theme) {
    draw_text(painter, x, y, label, Color::rgb(70, 82, 102), 1);
    draw_text(painter, x + 170, y, value, theme.accent, 1);
}

fn metric_baselines(bounds: Rect) -> [i32; 5] {
    let first = bounds.y + 26;
    [first, first + 40, first + 80, first + 120, first + 160]
}

fn footer_baseline(bounds: Rect, last_metric_y: i32) -> Option<i32> {
    let footer_y = bounds.bottom() - 22;
    (footer_y >= last_metric_y + 16).then_some(footer_y)
}

#[cfg(test)]
mod tests {
    use super::{footer_baseline, metric_baselines};
    use crate::graphics::geometry::Rect;

    #[test]
    fn compact_monitor_layout_keeps_footer_clear_of_metrics() {
        let bounds = Rect::new(0, 0, 410, 236);
        let metrics = metric_baselines(bounds);
        let footer = footer_baseline(bounds, metrics[4]).expect("footer should fit");
        assert!(footer - metrics[4] >= 16);

        let too_short = Rect::new(0, 0, 410, 198);
        let metrics = metric_baselines(too_short);
        assert_eq!(footer_baseline(too_short, metrics[4]), None);
    }
}
