use alloc::collections::VecDeque;
use alloc::string::{String, ToString};

use crate::desktop::theme::Theme;
use crate::graphics::geometry::Rect;
use crate::graphics::painter::{Color, Painter};
use crate::graphics::shadow::ShadowCache;
use crate::graphics::text::draw_text;

const MAX_NOTIFICATIONS: usize = 4;

#[derive(Debug, Clone)]
struct Notification {
    message: String,
    remaining_ms: u32,
}

pub struct NotificationCenter {
    notifications: VecDeque<Notification>,
}

impl NotificationCenter {
    pub const fn new() -> Self {
        Self {
            notifications: VecDeque::new(),
        }
    }

    pub fn push(&mut self, message: &str) {
        if self.notifications.len() == MAX_NOTIFICATIONS {
            self.notifications.pop_front();
        }
        self.notifications.push_back(Notification {
            message: message.to_string(),
            remaining_ms: 3_500,
        });
    }

    pub fn tick(&mut self, elapsed_ms: u32) -> bool {
        let old_len = self.notifications.len();
        for notification in &mut self.notifications {
            notification.remaining_ms = notification.remaining_ms.saturating_sub(elapsed_ms);
        }
        self.notifications
            .retain(|notification| notification.remaining_ms != 0);
        old_len != self.notifications.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notifications.is_empty()
    }

    pub fn bounds(&self, desktop: Rect) -> Rect {
        let width = desktop.width.saturating_sub(24).min(330);
        let height = self.notifications.len() as u32 * 58;
        Rect::new(
            desktop.right() - width as i32 - 12,
            desktop.y + 50,
            width,
            height,
        )
    }

    pub fn draw(
        &self,
        painter: &mut Painter<'_>,
        desktop: Rect,
        theme: Theme,
        shadows: &ShadowCache,
    ) {
        let area = self.bounds(desktop);
        for (index, notification) in self.notifications.iter().rev().enumerate() {
            let card = Rect::new(area.x, area.y + index as i32 * 58, area.width, 48);
            shadows.draw(painter, card, 12, 4);
            painter.blend_rounded_rect(card, 12, theme.panel);
            painter.stroke_rect(card, 1, theme.panel_border);
            painter.fill_rounded_rect(Rect::new(card.x + 12, card.y + 13, 5, 22), 2, theme.accent);
            draw_text(
                painter,
                card.x + 28,
                card.y + 20,
                &notification.message,
                theme.text,
                1,
            );
            if notification.remaining_ms < 500 {
                painter.blend_rounded_rect(card, 12, Color::rgba(0, 0, 0, 36));
            }
        }
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}
