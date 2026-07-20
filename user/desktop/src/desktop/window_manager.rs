use alloc::vec::Vec;

use crate::desktop::window::{
    RESIZE_BORDER, TITLE_BAR_HEIGHT, Window, WindowId, WindowSpec, WindowState,
};
use crate::graphics::damage::DamageTracker;
use crate::graphics::geometry::{Point, Rect, Size};

const CONTROL_SIZE: u32 = 24;
const CONTROL_GAP: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowError {
    InvalidBounds,
    UnknownWindow,
    UnknownModalParent,
    BlockedByModal,
    NotClosable,
    NotResizable,
    IdExhausted,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResizeEdges(u8);

impl ResizeEdges {
    const LEFT: u8 = 1 << 0;
    const RIGHT: u8 = 1 << 1;
    const TOP: u8 = 1 << 2;
    const BOTTOM: u8 = 1 << 3;

    pub const fn left(self) -> bool {
        self.0 & Self::LEFT != 0
    }

    pub const fn right(self) -> bool {
        self.0 & Self::RIGHT != 0
    }

    pub const fn top(self) -> bool {
        self.0 & Self::TOP != 0
    }

    pub const fn bottom(self) -> bool {
        self.0 & Self::BOTTOM != 0
    }

    const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTarget {
    Desktop,
    ModalBackdrop(WindowId),
    Client(WindowId),
    TitleBar(WindowId),
    Close(WindowId),
    Minimize(WindowId),
    Maximize(WindowId),
    Resize(WindowId, ResizeEdges),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowAnimationKind {
    Open,
    Close,
    Minimize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WindowAnimation {
    id: WindowId,
    kind: WindowAnimationKind,
    progress: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PointerOperation {
    Move {
        id: WindowId,
        origin: Point,
        initial: Rect,
    },
    Resize {
        id: WindowId,
        edges: ResizeEdges,
        origin: Point,
        initial: Rect,
    },
}

pub struct WindowManager {
    workspace: Rect,
    windows: Vec<Window>,
    focused: Option<WindowId>,
    next_id: Option<u32>,
    pointer_operation: Option<PointerOperation>,
    damage: DamageTracker,
    animations: Vec<WindowAnimation>,
}

impl WindowManager {
    pub fn new(workspace: Rect) -> Result<Self, WindowError> {
        if workspace.is_empty() {
            return Err(WindowError::InvalidBounds);
        }
        let mut damage = DamageTracker::new(48);
        damage.add(workspace);
        Ok(Self {
            workspace,
            windows: Vec::new(),
            focused: None,
            next_id: Some(1),
            pointer_operation: None,
            damage,
            animations: Vec::new(),
        })
    }

    pub const fn workspace(&self) -> Rect {
        self.workspace
    }

    pub fn windows(&self) -> &[Window] {
        &self.windows
    }

    pub const fn focused(&self) -> Option<WindowId> {
        self.focused
    }

    pub fn window(&self, id: WindowId) -> Option<&Window> {
        self.windows.iter().find(|window| window.id() == id)
    }

    pub fn damage(&self) -> &[Rect] {
        self.damage.regions()
    }

    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }

    pub fn add_damage(&mut self, region: Rect) {
        self.damage.add(region);
    }

    pub fn resize_workspace(&mut self, workspace: Rect) -> Result<bool, WindowError> {
        if workspace.is_empty() {
            return Err(WindowError::InvalidBounds);
        }
        if workspace == self.workspace {
            return Ok(false);
        }
        let previous = self.workspace;
        self.workspace = workspace;
        self.pointer_operation = None;
        self.damage.add(previous);
        self.damage.add(workspace);
        for window in &mut self.windows {
            let old = window.decorated_bounds();
            let restore =
                constrain_to_workspace(window.restore_bounds(), window.minimum_size(), workspace);
            let bounds = if matches!(window.state(), WindowState::Maximized) {
                workspace
            } else {
                constrain_to_workspace(window.bounds(), window.minimum_size(), workspace)
            };
            window.set_bounds(bounds);
            window.set_restore_bounds(restore);
            self.damage.add(old);
            self.damage.add(window.decorated_bounds());
        }
        Ok(true)
    }

    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    pub fn animate_open(&mut self, id: WindowId) -> Result<(), WindowError> {
        let bounds = self.window(id).ok_or(WindowError::UnknownWindow)?.bounds();
        self.animations.retain(|animation| animation.id != id);
        self.animations.push(WindowAnimation {
            id,
            kind: WindowAnimationKind::Open,
            progress: 0,
        });
        self.damage.add(decorated_bounds(bounds));
        Ok(())
    }

    pub fn animate_close(&mut self, id: WindowId) -> Result<(), WindowError> {
        self.ensure_action_allowed(id)?;
        if !self.window(id).unwrap().closable() {
            return Err(WindowError::NotClosable);
        }
        self.animations.retain(|animation| animation.id != id);
        self.animations.push(WindowAnimation {
            id,
            kind: WindowAnimationKind::Close,
            progress: 0,
        });
        self.pointer_operation = None;
        Ok(())
    }

    pub fn animate_minimize(&mut self, id: WindowId) -> Result<(), WindowError> {
        self.ensure_action_allowed(id)?;
        self.animations.retain(|animation| animation.id != id);
        self.animations.push(WindowAnimation {
            id,
            kind: WindowAnimationKind::Minimize,
            progress: 0,
        });
        self.pointer_operation = None;
        Ok(())
    }

    pub fn tick_animations(&mut self, elapsed_ms: u32) -> bool {
        if self.animations.is_empty() {
            return false;
        }
        let step = (elapsed_ms.saturating_mul(1000) / 160).min(1000) as u16;
        let mut completed = Vec::new();
        for animation in &mut self.animations {
            let Some(window) = self
                .windows
                .iter()
                .find(|window| window.id() == animation.id)
            else {
                completed.push((animation.id, animation.kind));
                continue;
            };
            let old = animated_bounds(window.bounds(), *animation, self.workspace);
            animation.progress = animation.progress.saturating_add(step).min(1000);
            let new = animated_bounds(window.bounds(), *animation, self.workspace);
            self.damage.add(decorated_bounds(old));
            self.damage.add(decorated_bounds(new));
            if animation.progress == 1000 {
                completed.push((animation.id, animation.kind));
            }
        }
        self.animations
            .retain(|animation| !completed.iter().any(|(id, _)| *id == animation.id));
        for (id, kind) in completed {
            match kind {
                WindowAnimationKind::Open => {}
                WindowAnimationKind::Close => {
                    let _ = self.close(id);
                }
                WindowAnimationKind::Minimize => {
                    let _ = self.minimize(id);
                }
            }
        }
        true
    }

    pub fn visual_bounds(&self, id: WindowId) -> Option<Rect> {
        let window = self.window(id)?;
        Some(
            self.animations
                .iter()
                .find(|animation| animation.id == id)
                .map_or(window.bounds(), |animation| {
                    animated_bounds(window.bounds(), *animation, self.workspace)
                }),
        )
    }

    pub fn create(&mut self, spec: WindowSpec<'_>) -> Result<WindowId, WindowError> {
        if spec.bounds.is_empty() || spec.minimum_size.width == 0 || spec.minimum_size.height == 0 {
            return Err(WindowError::InvalidBounds);
        }
        if self.active_modal().is_some() && spec.modal_for.is_none() {
            return Err(WindowError::BlockedByModal);
        }
        if let Some(parent) = spec.modal_for
            && self.window(parent).is_none()
        {
            return Err(WindowError::UnknownModalParent);
        }
        let id = allocate_window_id(&mut self.next_id)?;
        let bounds = constrain_bounds(spec.bounds, spec.minimum_size, self.workspace);
        let window = Window::from_spec(id, spec, bounds);
        self.damage.add(window.decorated_bounds());
        self.windows.push(window);
        self.set_focused(Some(id));
        Ok(id)
    }

    pub fn close(&mut self, id: WindowId) -> Result<(), WindowError> {
        let Some(window) = self.window(id) else {
            return Err(WindowError::UnknownWindow);
        };
        if !window.closable() {
            return Err(WindowError::NotClosable);
        }
        if let Some(modal) = self.active_modal()
            && modal != id
            && !self.is_descendant_of(modal, id)
        {
            return Err(WindowError::BlockedByModal);
        }

        let mut removed = Vec::new();
        collect_descendants(&self.windows, id, &mut removed);
        removed.push(id);
        for window in &self.windows {
            if removed.contains(&window.id()) {
                self.damage.add(window.decorated_bounds());
            }
        }
        self.windows
            .retain(|window| !removed.contains(&window.id()));
        self.animations
            .retain(|animation| !removed.contains(&animation.id));
        self.pointer_operation = None;
        self.set_focused(self.topmost_focusable());
        Ok(())
    }

    pub fn focus(&mut self, id: WindowId) -> Result<(), WindowError> {
        let Some(index) = self.windows.iter().position(|window| window.id() == id) else {
            return Err(WindowError::UnknownWindow);
        };
        if !self.windows[index].visible() {
            return Err(WindowError::UnknownWindow);
        }
        if let Some(modal) = self.active_modal()
            && modal != id
        {
            return Err(WindowError::BlockedByModal);
        }
        let window = self.windows.remove(index);
        self.damage.add(window.decorated_bounds());
        self.windows.push(window);
        self.set_focused(Some(id));
        Ok(())
    }

    pub fn minimize(&mut self, id: WindowId) -> Result<(), WindowError> {
        self.ensure_action_allowed(id)?;
        let index = self.index_of(id)?;
        let old = self.windows[index].decorated_bounds();
        self.windows[index].set_state(WindowState::Minimized);
        self.animations.retain(|animation| animation.id != id);
        self.damage.add(old);
        self.pointer_operation = None;
        self.set_focused(self.topmost_focusable());
        Ok(())
    }

    pub fn maximize(&mut self, id: WindowId) -> Result<(), WindowError> {
        self.ensure_action_allowed(id)?;
        let index = self.index_of(id)?;
        if !self.windows[index].resizable() {
            return Err(WindowError::NotResizable);
        }
        let old = self.windows[index].decorated_bounds();
        self.windows[index].maximize(self.workspace);
        let new = self.windows[index].decorated_bounds();
        self.damage.add(old);
        self.damage.add(new);
        self.focus(id)
    }

    pub fn restore(&mut self, id: WindowId) -> Result<(), WindowError> {
        self.ensure_action_allowed(id)?;
        let index = self.index_of(id)?;
        let old = self.windows[index].decorated_bounds();
        self.windows[index].restore();
        let new = self.windows[index].decorated_bounds();
        self.damage.add(old);
        self.damage.add(new);
        self.focus(id)
    }

    pub fn alt_tab(&mut self, reverse: bool) -> Option<WindowId> {
        if let Some(modal) = self.active_modal() {
            self.set_focused(Some(modal));
            return Some(modal);
        }
        let visible: Vec<WindowId> = self
            .windows
            .iter()
            .filter(|window| window.visible())
            .map(Window::id)
            .collect();
        if visible.is_empty() {
            self.set_focused(None);
            return None;
        }
        let current = self
            .focused
            .and_then(|id| visible.iter().position(|candidate| *candidate == id))
            .unwrap_or(visible.len() - 1);
        let next = if reverse {
            (current + 1) % visible.len()
        } else if current == 0 {
            visible.len() - 1
        } else {
            current - 1
        };
        let id = visible[next];
        let _ = self.focus(id);
        Some(id)
    }

    pub fn active_modal(&self) -> Option<WindowId> {
        self.windows
            .iter()
            .rev()
            .find(|window| window.visible() && window.modal_for().is_some())
            .map(Window::id)
    }

    pub fn hit_test(&self, point: Point) -> HitTarget {
        let active_modal = self.active_modal();
        for window in self.windows.iter().rev().filter(|window| window.visible()) {
            if self.animation_blocks_input(window.id()) {
                continue;
            }
            if active_modal.is_some_and(|modal| modal != window.id()) {
                continue;
            }
            if !window.bounds().contains(point)
                && !expanded(window.bounds(), RESIZE_BORDER).contains(point)
            {
                continue;
            }
            let edges = resize_edges(window.bounds(), point);
            if window.resizable()
                && matches!(window.state(), WindowState::Normal)
                && !edges.is_empty()
            {
                return HitTarget::Resize(window.id(), edges);
            }
            if close_rect(window).contains(point) && window.closable() {
                return HitTarget::Close(window.id());
            }
            if maximize_rect(window).contains(point) && window.resizable() {
                return HitTarget::Maximize(window.id());
            }
            if minimize_rect(window).contains(point) {
                return HitTarget::Minimize(window.id());
            }
            if window.title_bar().contains(point) {
                return HitTarget::TitleBar(window.id());
            }
            if window.bounds().contains(point) {
                return HitTarget::Client(window.id());
            }
        }
        active_modal.map_or(HitTarget::Desktop, HitTarget::ModalBackdrop)
    }

    pub fn pointer_press(&mut self, point: Point) -> Result<HitTarget, WindowError> {
        let target = self.hit_test(point);
        match target {
            HitTarget::Client(id) => self.focus(id)?,
            HitTarget::TitleBar(id) => {
                self.focus(id)?;
                let initial = self.window(id).unwrap().bounds();
                if matches!(self.window(id).unwrap().state(), WindowState::Normal) {
                    self.pointer_operation = Some(PointerOperation::Move {
                        id,
                        origin: point,
                        initial,
                    });
                }
            }
            HitTarget::Resize(id, edges) => {
                self.focus(id)?;
                let initial = self.window(id).unwrap().bounds();
                self.pointer_operation = Some(PointerOperation::Resize {
                    id,
                    edges,
                    origin: point,
                    initial,
                });
            }
            HitTarget::Close(id) => self.animate_close(id)?,
            HitTarget::Minimize(id) => self.animate_minimize(id)?,
            HitTarget::Maximize(id) => {
                if matches!(self.window(id).unwrap().state(), WindowState::Maximized) {
                    self.restore(id)?;
                } else {
                    self.maximize(id)?;
                }
            }
            HitTarget::ModalBackdrop(id) => {
                let _ = self.focus(id);
            }
            HitTarget::Desktop => {
                self.set_focused(None);
            }
        }
        Ok(target)
    }

    pub fn pointer_move(&mut self, point: Point) -> Result<(), WindowError> {
        let Some(operation) = self.pointer_operation else {
            return Ok(());
        };
        match operation {
            PointerOperation::Move {
                id,
                origin,
                initial,
            } => {
                let bounds = initial.translate(point.x - origin.x, point.y - origin.y);
                self.update_bounds(id, constrain_move(bounds, self.workspace))
            }
            PointerOperation::Resize {
                id,
                edges,
                origin,
                initial,
            } => {
                let minimum = self
                    .window(id)
                    .ok_or(WindowError::UnknownWindow)?
                    .minimum_size();
                let bounds = resize_bounds(
                    initial,
                    edges,
                    point.x - origin.x,
                    point.y - origin.y,
                    minimum,
                );
                self.update_bounds(id, constrain_resize(bounds, minimum, self.workspace))
            }
        }
    }

    pub fn pointer_release(&mut self) {
        self.pointer_operation = None;
    }

    fn update_bounds(&mut self, id: WindowId, bounds: Rect) -> Result<(), WindowError> {
        let index = self.index_of(id)?;
        let old = self.windows[index].decorated_bounds();
        self.windows[index].set_bounds(bounds);
        let new = self.windows[index].decorated_bounds();
        self.damage.add(old);
        self.damage.add(new);
        Ok(())
    }

    fn set_focused(&mut self, focused: Option<WindowId>) -> bool {
        if self.focused == focused {
            return false;
        }
        if let Some(window) = self.focused.and_then(|id| self.window(id)) {
            self.damage.add(window.decorated_bounds());
        }
        self.focused = focused;
        if let Some(window) = focused.and_then(|id| self.window(id)) {
            self.damage.add(window.decorated_bounds());
        }
        true
    }

    fn ensure_action_allowed(&self, id: WindowId) -> Result<(), WindowError> {
        self.index_of(id)?;
        if self.active_modal().is_some_and(|modal| modal != id) {
            return Err(WindowError::BlockedByModal);
        }
        Ok(())
    }

    fn index_of(&self, id: WindowId) -> Result<usize, WindowError> {
        self.windows
            .iter()
            .position(|window| window.id() == id)
            .ok_or(WindowError::UnknownWindow)
    }

    fn topmost_focusable(&self) -> Option<WindowId> {
        self.active_modal().or_else(|| {
            self.windows
                .iter()
                .rev()
                .find(|window| window.visible())
                .map(Window::id)
        })
    }

    fn is_descendant_of(&self, mut id: WindowId, ancestor: WindowId) -> bool {
        while let Some(parent) = self.window(id).and_then(Window::modal_for) {
            if parent == ancestor {
                return true;
            }
            id = parent;
        }
        false
    }

    fn animation_blocks_input(&self, id: WindowId) -> bool {
        self.animations.iter().any(|animation| {
            animation.id == id
                && matches!(
                    animation.kind,
                    WindowAnimationKind::Close | WindowAnimationKind::Minimize
                )
        })
    }
}

fn allocate_window_id(next_id: &mut Option<u32>) -> Result<WindowId, WindowError> {
    let value = next_id.ok_or(WindowError::IdExhausted)?;
    *next_id = value.checked_add(1);
    Ok(WindowId(value))
}

pub fn close_rect(window: &Window) -> Rect {
    control_rect(window, 0)
}

pub fn maximize_rect(window: &Window) -> Rect {
    control_rect(window, 1)
}

pub fn minimize_rect(window: &Window) -> Rect {
    control_rect(window, 2)
}

fn control_rect(window: &Window, index_from_right: i32) -> Rect {
    let right = window.bounds().right()
        - CONTROL_GAP
        - (CONTROL_SIZE as i32 + CONTROL_GAP) * index_from_right;
    Rect::new(
        right - CONTROL_SIZE as i32,
        window.bounds().y + ((TITLE_BAR_HEIGHT - CONTROL_SIZE) / 2) as i32,
        CONTROL_SIZE,
        CONTROL_SIZE,
    )
}

fn collect_descendants(windows: &[Window], parent: WindowId, output: &mut Vec<WindowId>) {
    for child in windows
        .iter()
        .filter(|window| window.modal_for() == Some(parent))
    {
        collect_descendants(windows, child.id(), output);
        output.push(child.id());
    }
}

fn constrain_bounds(bounds: Rect, minimum: Size, workspace: Rect) -> Rect {
    let width = bounds.width.max(minimum.width).min(workspace.width);
    let height = bounds.height.max(minimum.height).min(workspace.height);
    constrain_move(Rect::new(bounds.x, bounds.y, width, height), workspace)
}

fn constrain_to_workspace(bounds: Rect, minimum: Size, workspace: Rect) -> Rect {
    let width = bounds.width.max(minimum.width).min(workspace.width);
    let height = bounds.height.max(minimum.height).min(workspace.height);
    let max_x = workspace.right().saturating_sub(width as i32);
    let max_y = workspace.bottom().saturating_sub(height as i32);
    Rect::new(
        bounds.x.clamp(workspace.x, max_x.max(workspace.x)),
        bounds.y.clamp(workspace.y, max_y.max(workspace.y)),
        width,
        height,
    )
}

fn constrain_move(bounds: Rect, workspace: Rect) -> Rect {
    let title_visible = 64i32.min(bounds.width as i32);
    let min_x = workspace
        .x
        .saturating_sub(bounds.width as i32 - title_visible);
    let max_x = workspace.right().saturating_sub(title_visible);
    let min_y = workspace.y;
    let max_y = workspace.bottom().saturating_sub(TITLE_BAR_HEIGHT as i32);
    Rect::new(
        bounds.x.clamp(min_x, max_x.max(min_x)),
        bounds.y.clamp(min_y, max_y.max(min_y)),
        bounds.width,
        bounds.height,
    )
}

fn constrain_resize(bounds: Rect, minimum: Size, workspace: Rect) -> Rect {
    let x0 = bounds.x.max(workspace.x);
    let y0 = bounds.y.max(workspace.y);
    let right = bounds.right().min(workspace.right());
    let bottom = bounds.bottom().min(workspace.bottom());
    let width = (right - x0).max(minimum.width as i32) as u32;
    let height = (bottom - y0).max(minimum.height as i32) as u32;
    Rect::new(
        x0,
        y0,
        width.min(workspace.width),
        height.min(workspace.height),
    )
}

fn resize_edges(bounds: Rect, point: Point) -> ResizeEdges {
    let mut edges = 0;
    if point.x >= bounds.x - RESIZE_BORDER && point.x < bounds.x + RESIZE_BORDER {
        edges |= ResizeEdges::LEFT;
    }
    if point.x <= bounds.right() + RESIZE_BORDER && point.x > bounds.right() - RESIZE_BORDER {
        edges |= ResizeEdges::RIGHT;
    }
    if point.y >= bounds.y - RESIZE_BORDER && point.y < bounds.y + RESIZE_BORDER {
        edges |= ResizeEdges::TOP;
    }
    if point.y <= bounds.bottom() + RESIZE_BORDER && point.y > bounds.bottom() - RESIZE_BORDER {
        edges |= ResizeEdges::BOTTOM;
    }
    ResizeEdges(edges)
}

fn resize_bounds(initial: Rect, edges: ResizeEdges, dx: i32, dy: i32, minimum: Size) -> Rect {
    let mut left = initial.x;
    let mut top = initial.y;
    let mut right = initial.right();
    let mut bottom = initial.bottom();
    if edges.left() {
        left = (left + dx).min(right - minimum.width as i32);
    }
    if edges.right() {
        right = (right + dx).max(left + minimum.width as i32);
    }
    if edges.top() {
        top = (top + dy).min(bottom - minimum.height as i32);
    }
    if edges.bottom() {
        bottom = (bottom + dy).max(top + minimum.height as i32);
    }
    Rect::new(left, top, (right - left) as u32, (bottom - top) as u32)
}

fn expanded(rect: Rect, margin: i32) -> Rect {
    Rect::new(
        rect.x.saturating_sub(margin),
        rect.y.saturating_sub(margin),
        rect.width.saturating_add((margin * 2) as u32),
        rect.height.saturating_add((margin * 2) as u32),
    )
}

fn decorated_bounds(rect: Rect) -> Rect {
    expanded(rect, 21)
}

fn animated_bounds(bounds: Rect, animation: WindowAnimation, workspace: Rect) -> Rect {
    let target = match animation.kind {
        WindowAnimationKind::Open | WindowAnimationKind::Close => centered_scale(bounds, 72),
        WindowAnimationKind::Minimize => Rect::new(
            workspace.x + (workspace.width.saturating_sub(64) / 2) as i32,
            workspace.bottom() - 20,
            64,
            32,
        ),
    };
    match animation.kind {
        WindowAnimationKind::Open => lerp_rect(target, bounds, animation.progress),
        WindowAnimationKind::Close | WindowAnimationKind::Minimize => {
            lerp_rect(bounds, target, animation.progress)
        }
    }
}

fn centered_scale(bounds: Rect, percent: u32) -> Rect {
    let width = bounds.width.saturating_mul(percent) / 100;
    let height = bounds.height.saturating_mul(percent) / 100;
    Rect::new(
        bounds.x + ((bounds.width - width) / 2) as i32,
        bounds.y + ((bounds.height - height) / 2) as i32,
        width,
        height,
    )
}

fn lerp_rect(from: Rect, to: Rect, progress: u16) -> Rect {
    let progress = progress as i64;
    let interpolate_i32 = |start: i32, end: i32| -> i32 {
        (start as i64 + (end as i64 - start as i64) * progress / 1000) as i32
    };
    let interpolate_u32 = |start: u32, end: u32| -> u32 {
        (start as i64 + (end as i64 - start as i64) * progress / 1000).max(1) as u32
    };
    Rect::new(
        interpolate_i32(from.x, to.x),
        interpolate_i32(from.y, to.y),
        interpolate_u32(from.width, to.width),
        interpolate_u32(from.height, to.height),
    )
}

#[cfg(test)]
mod id_tests {
    use super::{WindowError, allocate_window_id};
    use crate::desktop::window::WindowId;

    #[test]
    fn final_window_id_is_issued_once_and_then_exhausts() {
        let mut next = Some(u32::MAX);
        assert_eq!(allocate_window_id(&mut next), Ok(WindowId(u32::MAX)));
        assert_eq!(allocate_window_id(&mut next), Err(WindowError::IdExhausted));
    }
}
