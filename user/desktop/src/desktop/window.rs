use alloc::string::{String, ToString};

use crate::graphics::geometry::{Rect, Size};

pub const TITLE_BAR_HEIGHT: u32 = 32;
pub const RESIZE_BORDER: i32 = 7;
pub const SHADOW_MARGIN: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowSpec<'a> {
    pub title: &'a str,
    pub bounds: Rect,
    pub minimum_size: Size,
    pub modal_for: Option<WindowId>,
    pub closable: bool,
    pub resizable: bool,
}

impl<'a> WindowSpec<'a> {
    pub const fn new(title: &'a str, bounds: Rect) -> Self {
        Self {
            title,
            bounds,
            minimum_size: Size::new(180, 120),
            modal_for: None,
            closable: true,
            resizable: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    id: WindowId,
    title: String,
    bounds: Rect,
    restore_bounds: Rect,
    minimum_size: Size,
    state: WindowState,
    modal_for: Option<WindowId>,
    closable: bool,
    resizable: bool,
}

impl Window {
    pub(crate) fn from_spec(id: WindowId, spec: WindowSpec<'_>, bounds: Rect) -> Self {
        Self {
            id,
            title: spec.title.to_string(),
            bounds,
            restore_bounds: bounds,
            minimum_size: spec.minimum_size,
            state: WindowState::Normal,
            modal_for: spec.modal_for,
            closable: spec.closable,
            resizable: spec.resizable,
        }
    }

    pub const fn id(&self) -> WindowId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn bounds(&self) -> Rect {
        self.bounds
    }

    pub const fn restore_bounds(&self) -> Rect {
        self.restore_bounds
    }

    pub const fn minimum_size(&self) -> Size {
        self.minimum_size
    }

    pub const fn state(&self) -> WindowState {
        self.state
    }

    pub const fn modal_for(&self) -> Option<WindowId> {
        self.modal_for
    }

    pub const fn closable(&self) -> bool {
        self.closable
    }

    pub const fn resizable(&self) -> bool {
        self.resizable
    }

    pub const fn visible(&self) -> bool {
        !matches!(self.state, WindowState::Minimized)
    }

    pub fn title_bar(&self) -> Rect {
        Rect::new(
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            TITLE_BAR_HEIGHT.min(self.bounds.height),
        )
    }

    pub fn decorated_bounds(&self) -> Rect {
        let margin = SHADOW_MARGIN;
        Rect::new(
            self.bounds.x.saturating_sub(margin),
            self.bounds.y.saturating_sub(margin),
            self.bounds.width.saturating_add((margin * 2) as u32),
            // The cached shadow is offset downward, so cover one additional
            // bottom row beyond the symmetric outer margin.
            self.bounds.height.saturating_add((margin * 2 + 1) as u32),
        )
    }

    pub(crate) fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
        if matches!(self.state, WindowState::Normal) {
            self.restore_bounds = bounds;
        }
    }

    pub(crate) fn set_restore_bounds(&mut self, bounds: Rect) {
        self.restore_bounds = bounds;
    }

    pub(crate) fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    pub(crate) fn maximize(&mut self, workspace: Rect) {
        if !matches!(self.state, WindowState::Maximized) {
            if !matches!(self.state, WindowState::Minimized) {
                self.restore_bounds = self.bounds;
            }
            self.bounds = workspace;
            self.state = WindowState::Maximized;
        }
    }

    pub(crate) fn restore(&mut self) {
        self.bounds = self.restore_bounds;
        self.state = WindowState::Normal;
    }
}
