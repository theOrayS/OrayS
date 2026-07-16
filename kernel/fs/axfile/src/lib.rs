//! Shared open-file identity and readiness notification primitives.
//!
//! This crate intentionally does not define filesystem operations. Callers own
//! their object-safe operation trait and place it behind [`OpenFile`].

#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::sync::{Arc, Weak};
use alloc::vec::Vec;
use axsync::Mutex;
use axsync::spin::SpinNoIrq as EventRegistryMutex;
use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use core::time::Duration;

static NEXT_OPEN_FILE_ID: AtomicU64 = AtomicU64::new(1);

/// Process-independent identity of one open file description.
///
/// The value is allocated once and is never derived from an FD-table slot or
/// an address. Exhaustion is reported rather than wrapping and reusing an ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OpenFileId(u64);

impl OpenFileId {
    /// Returns the integer representation for diagnostics and ordered keys.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Allocates a fresh identity for a legacy or object-backed description.
    pub fn allocate() -> Result<Self, IdExhausted> {
        NEXT_OPEN_FILE_ID
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map(Self)
            .map_err(|_| IdExhausted)
    }
}

/// Returned if the non-reusing open-file identity space is exhausted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdExhausted;

impl fmt::Display for IdExhausted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("open-file identity space exhausted")
    }
}

/// Shared lifetime and stable identity for every descriptor alias of one open
/// description, including entries still served by a legacy adapter.
///
/// The final alias closes `close_events`.  Long-lived consumers such as epoll
/// subscribe weakly and can remove their registration immediately instead of
/// retaining a stale map entry until the next readiness scan.
pub struct DescriptionIdentity {
    id: OpenFileId,
    close_events: EventSource,
}

impl DescriptionIdentity {
    pub fn new(id: OpenFileId) -> Self {
        Self {
            id,
            close_events: EventSource::new(),
        }
    }

    pub const fn id(&self) -> OpenFileId {
        self.id
    }

    pub const fn close_events(&self) -> &EventSource {
        &self.close_events
    }
}

impl Drop for DescriptionIdentity {
    fn drop(&mut self) {
        self.close_events.close();
    }
}

/// One shared open file description.
///
/// Cloning the outer [`Arc`] models `dup`/fork sharing. Descriptor-local flags
/// such as `FD_CLOEXEC` deliberately do not live here; applicable file status
/// flags do, so all aliases observe updates to the same description.
pub struct OpenFile<T: ?Sized> {
    id: OpenFileId,
    status_flags: AtomicU32,
    object: Arc<T>,
}

impl<T: ?Sized> OpenFile<T> {
    /// Creates a description around an already shared object.
    pub fn from_arc(object: Arc<T>, status_flags: u32) -> Result<Arc<Self>, IdExhausted> {
        Ok(Arc::new(Self {
            id: OpenFileId::allocate()?,
            status_flags: AtomicU32::new(status_flags),
            object,
        }))
    }

    /// Returns this description's stable identity.
    pub const fn id(&self) -> OpenFileId {
        self.id
    }

    /// Returns the current shared file status flags.
    pub fn status_flags(&self) -> u32 {
        self.status_flags.load(Ordering::Acquire)
    }

    /// Replaces the shared file status flags and returns the previous value.
    pub fn set_status_flags(&self, status_flags: u32) -> u32 {
        self.status_flags.swap(status_flags, Ordering::AcqRel)
    }

    /// Atomically updates the shared file status flags.
    pub fn update_status_flags(&self, update: impl Fn(u32) -> u32) -> u32 {
        let mut current = self.status_flags();
        loop {
            let next = update(current);
            match self.status_flags.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return next,
                Err(observed) => current = observed,
            }
        }
    }

    /// Borrows the underlying file-like object.
    pub fn object(&self) -> &T {
        &self.object
    }

    /// Clones the underlying object's shared pointer.
    pub fn object_arc(&self) -> Arc<T> {
        Arc::clone(&self.object)
    }
}

impl<T> OpenFile<T> {
    /// Creates a description around a new concrete object.
    pub fn new(object: T, status_flags: u32) -> Result<Arc<Self>, IdExhausted> {
        Self::from_arc(Arc::new(object), status_flags)
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for OpenFile<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenFile")
            .field("id", &self.id)
            .field("status_flags", &self.status_flags())
            .field("object", &self.object)
            .finish()
    }
}

/// Readiness bits shared by file objects and event consumers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ReadyEvents(u32);

impl ReadyEvents {
    pub const EMPTY: Self = Self(0);
    pub const READABLE: Self = Self(1 << 0);
    pub const WRITABLE: Self = Self(1 << 1);
    pub const PRIORITY: Self = Self(1 << 2);
    pub const ERROR: Self = Self(1 << 3);
    pub const HANGUP: Self = Self(1 << 4);
    pub const INVALID: Self = Self(1 << 5);
    pub const ALL: Self = Self(
        Self::READABLE.0
            | Self::WRITABLE.0
            | Self::PRIORITY.0
            | Self::ERROR.0
            | Self::HANGUP.0
            | Self::INVALID.0,
    );

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }
}

impl BitOr for ReadyEvents {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ReadyEvents {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for ReadyEvents {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for ReadyEvents {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for ReadyEvents {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Delivery state for one poll-style registration.
///
/// `notification` is a per-registration monotonic hint generation.  Combining
/// it with the last sampled level preserves a complete not-ready/ready cycle
/// that occurs between two consumer scans without treating the notification
/// itself as readiness.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EventDeliveryState {
    last_ready: u32,
    last_notification: u64,
    disabled: bool,
}

impl EventDeliveryState {
    pub const fn is_disabled(self) -> bool {
        self.disabled
    }

    pub const fn should_emit(self, ready: u32, notification: u64, edge: bool) -> bool {
        ready != 0
            && (!edge || ready & !self.last_ready != 0 || notification != self.last_notification)
    }

    /// Commits a level query after it has either produced an event or was
    /// observed not ready.  A ready event that could not fit in the caller's
    /// output buffer must not be committed, so it remains pending.
    pub fn commit(&mut self, ready: u32, notification: u64, delivered: bool, one_shot: bool) {
        self.last_ready = ready;
        self.last_notification = notification;
        if delivered && one_shot {
            self.disabled = true;
        }
    }

    /// Atomically decides and commits one delivery while the caller holds its
    /// registration lock.
    ///
    /// A ready event that does not fit remains uncommitted and can be claimed
    /// by a later scan. The return value means this caller owns one delivery.
    pub fn claim(
        &mut self,
        ready: u32,
        notification: u64,
        edge: bool,
        one_shot: bool,
        has_capacity: bool,
    ) -> bool {
        if self.disabled {
            return false;
        }
        let should_emit = self.should_emit(ready, notification, edge);
        if should_emit && !has_capacity {
            return false;
        }
        self.commit(ready, notification, should_emit && has_capacity, one_shot);
        should_emit && has_capacity
    }
}

/// Result of resolving a fresh level query against a deadline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LevelWaitDecision {
    Ready,
    TimedOut,
    Wait,
}

/// Readiness wins over an already reached deadline because consumers must
/// always perform the final level query after waking.
pub const fn decide_level_wait(ready: bool, deadline_reached: bool) -> LevelWaitDecision {
    if ready {
        LevelWaitDecision::Ready
    } else if deadline_reached {
        LevelWaitDecision::TimedOut
    } else {
        LevelWaitDecision::Wait
    }
}

/// Computes the next delay for an object whose readiness can change solely
/// through passage of time, such as timerfd.
pub fn readiness_deadline_delay(
    already_ready: bool,
    deadline: Option<Duration>,
    now: Duration,
) -> Option<Duration> {
    if already_ready {
        Some(Duration::ZERO)
    } else {
        deadline.map(|deadline| deadline.saturating_sub(now))
    }
}

/// Prevents synchronous observer graphs from recursively re-entering one
/// notification walk and coalesces concurrent attempts into another walk.
///
/// A rejected entry marks a pending round.  The active caller must invoke
/// [`ReentrancyGuard::finish_round`] after every notification walk; it either
/// claims that pending round or atomically releases the gate.  This avoids the
/// release-versus-pending race that a pair of independent booleans would have.
#[derive(Debug, Default)]
pub struct ReentrancyGate {
    state: AtomicU32,
}

impl ReentrancyGate {
    const IDLE: u32 = 0;
    const ACTIVE: u32 = 1;
    const PENDING: u32 = 2;

    pub const fn new() -> Self {
        Self {
            state: AtomicU32::new(Self::IDLE),
        }
    }

    pub fn try_enter(&self) -> Option<ReentrancyGuard<'_>> {
        loop {
            let state = self.state.load(Ordering::Acquire);
            if state == Self::IDLE {
                if self
                    .state
                    .compare_exchange(
                        Self::IDLE,
                        Self::ACTIVE,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    return Some(ReentrancyGuard {
                        gate: self,
                        active: true,
                    });
                }
            } else if state == Self::ACTIVE {
                if self
                    .state
                    .compare_exchange(
                        Self::ACTIVE,
                        Self::ACTIVE | Self::PENDING,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    return None;
                }
            } else {
                debug_assert_eq!(state, Self::ACTIVE | Self::PENDING);
                return None;
            }
        }
    }
}

/// RAII release for [`ReentrancyGate`].
pub struct ReentrancyGuard<'a> {
    gate: &'a ReentrancyGate,
    active: bool,
}

impl ReentrancyGuard<'_> {
    /// Completes one notification walk.
    ///
    /// Returns `true` when a nested or concurrent caller requested another
    /// walk. Returns `false` only after atomically returning the gate to idle.
    pub fn finish_round(&mut self) -> bool {
        debug_assert!(self.active);
        loop {
            let state = self.gate.state.load(Ordering::Acquire);
            if state == ReentrancyGate::ACTIVE {
                if self
                    .gate
                    .state
                    .compare_exchange(
                        ReentrancyGate::ACTIVE,
                        ReentrancyGate::IDLE,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    self.active = false;
                    return false;
                }
            } else {
                debug_assert_eq!(state, ReentrancyGate::ACTIVE | ReentrancyGate::PENDING);
                if self
                    .gate
                    .state
                    .compare_exchange(
                        ReentrancyGate::ACTIVE | ReentrancyGate::PENDING,
                        ReentrancyGate::ACTIVE,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    return true;
                }
            }
        }
    }
}

impl Drop for ReentrancyGuard<'_> {
    fn drop(&mut self) {
        if self.active {
            // Early-return and unwind safety. Normal notification users call
            // finish_round(), which performs the race-free release above.
            self.gate
                .state
                .store(ReentrancyGate::IDLE, Ordering::Release);
        }
    }
}

fn with_ordered_arc_pair<T, R>(
    first: &Arc<T>,
    second: &Arc<T>,
    operation: impl FnOnce(&Arc<T>, &Arc<T>, bool) -> R,
) -> Option<R> {
    if Arc::ptr_eq(first, second) {
        return None;
    }
    if Arc::as_ptr(first) < Arc::as_ptr(second) {
        Some(operation(first, second, false))
    } else {
        Some(operation(second, first, true))
    }
}

/// Locks two distinct `Arc<Mutex<T>>` values in a process-wide canonical order
/// while presenting the values to `operation` in caller order.
///
/// This is intended for reciprocal operations such as `tee(A, B)` and
/// `tee(B, A)`: both calls acquire the same pair in the same order, but each
/// closure still receives its own source first and destination second. `None`
/// rejects attempts to lock the same mutex twice.
pub fn with_ordered_arc_mutex_pair<T, R>(
    first: &Arc<Mutex<T>>,
    second: &Arc<Mutex<T>>,
    operation: impl FnOnce(&mut T, &mut T) -> R,
) -> Option<R> {
    with_ordered_arc_pair(first, second, |lower, higher, caller_order_reversed| {
        let mut lower_guard = lower.lock();
        let mut higher_guard = higher.lock();
        if caller_order_reversed {
            operation(&mut higher_guard, &mut lower_guard)
        } else {
            operation(&mut lower_guard, &mut higher_guard)
        }
    })
}

/// Logical ownership of a process's descriptor table within a CLONE_FILES
/// sharing object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTableGroup {
    Base,
    Private(i32),
}

/// Table-map update required after a process leaves its sharing group.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileTableDetach {
    Keep,
    DropBase,
    DropPrivate(i32),
    MovePrivate { from: i32, to: i32 },
}

/// Explicit CLONE_FILES membership tracking.
///
/// Reference counts cannot decide whether a table is semantically shared: a
/// new Arc may be created from the process object immediately after a count is
/// sampled.  Callers mutate this tracker while holding the same lock used to
/// clone the shared table handle, making clone, exec split, and exit linear.
#[derive(Debug, Default)]
pub struct FileTableShareTracker {
    base_users: BTreeSet<i32>,
    private_owner: BTreeMap<i32, i32>,
}

impl FileTableShareTracker {
    pub fn register_base(&mut self, pid: i32) {
        if !self.private_owner.contains_key(&pid) {
            self.base_users.insert(pid);
        }
    }

    pub fn group(&self, pid: i32) -> FileTableGroup {
        self.private_owner
            .get(&pid)
            .copied()
            .map_or(FileTableGroup::Base, FileTableGroup::Private)
    }

    pub fn share(&mut self, parent_pid: i32, child_pid: i32) {
        match self.group(parent_pid) {
            FileTableGroup::Base => {
                self.base_users.insert(parent_pid);
                self.base_users.insert(child_pid);
            }
            FileTableGroup::Private(owner) => {
                self.private_owner.entry(owner).or_insert(owner);
                self.private_owner.insert(child_pid, owner);
            }
        }
    }

    pub fn is_shared(&self, pid: i32) -> bool {
        match self.group(pid) {
            FileTableGroup::Base => self.base_users.len() > 1,
            FileTableGroup::Private(owner) => {
                self.private_owner
                    .values()
                    .filter(|&&candidate| candidate == owner)
                    .take(2)
                    .count()
                    > 1
            }
        }
    }

    /// Commits a split after the caller has successfully copied the table.
    pub fn split(&mut self, pid: i32) -> Option<(i32, i32)> {
        let owner_move = match self.group(pid) {
            FileTableGroup::Base => {
                self.base_users.remove(&pid);
                None
            }
            FileTableGroup::Private(owner) => self.remove_private_member(pid, owner),
        };
        self.private_owner.insert(pid, pid);
        owner_move
    }

    pub fn detach(&mut self, pid: i32) -> FileTableDetach {
        match self.group(pid) {
            FileTableGroup::Base => {
                let was_member = self.base_users.remove(&pid);
                if was_member && self.base_users.is_empty() {
                    FileTableDetach::DropBase
                } else {
                    FileTableDetach::Keep
                }
            }
            FileTableGroup::Private(owner) => {
                let owner_move = self.remove_private_member(pid, owner);
                if self
                    .private_owner
                    .values()
                    .any(|&candidate| candidate == owner)
                {
                    FileTableDetach::Keep
                } else if let Some((from, to)) = owner_move {
                    FileTableDetach::MovePrivate { from, to }
                } else {
                    FileTableDetach::DropPrivate(owner)
                }
            }
        }
    }

    fn remove_private_member(&mut self, pid: i32, owner: i32) -> Option<(i32, i32)> {
        self.private_owner.remove(&pid);
        if pid != owner {
            return None;
        }
        let new_owner = self
            .private_owner
            .iter()
            .find_map(|(&member, &candidate)| (candidate == owner).then_some(member))?;
        for candidate in self.private_owner.values_mut() {
            if *candidate == owner {
                *candidate = new_owner;
            }
        }
        self.private_owner.insert(new_owner, new_owner);
        Some((owner, new_owner))
    }
}

/// A persistent registration key that cannot be retargeted by FD-slot reuse.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegistrationKey {
    pub registered_fd: i32,
    pub open_file_id: OpenFileId,
}

impl RegistrationKey {
    pub const fn new(registered_fd: i32, open_file_id: OpenFileId) -> Self {
        Self {
            registered_fd,
            open_file_id,
        }
    }
}

/// A readiness consumer. Notifications are hints to query current readiness.
pub trait EventObserver: Send + Sync {
    fn on_event(&self, events: ReadyEvents);
}

/// Stable token for one observer registration within an [`EventSource`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventToken(u64);

impl EventToken {
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// State sampled immediately after an observer has been registered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventSnapshot {
    pub generation: u64,
    pub closed: bool,
}

/// Failure to add an event observer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistrationError {
    SourceClosed,
    TokenExhausted,
}

struct EventSourceInner {
    observers: EventRegistryMutex<BTreeMap<EventToken, Weak<dyn EventObserver>>>,
    next_token: AtomicU64,
    generation: AtomicU64,
    closed: AtomicBool,
}

/// Readiness notification source with weak, explicitly removable observers.
///
/// Registering returns a generation sampled after insertion. A waiter uses the
/// query/register/recheck protocol: query readiness, subscribe, query again,
/// then block only while both readiness and the observed generation are
/// unchanged. Callbacks run after the registry lock has been released.
pub struct EventSource {
    inner: Arc<EventSourceInner>,
}

impl EventSource {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventSourceInner {
                observers: EventRegistryMutex::new(BTreeMap::new()),
                next_token: AtomicU64::new(1),
                generation: AtomicU64::new(0),
                closed: AtomicBool::new(false),
            }),
        }
    }

    /// Registers a weak observer and samples the generation after insertion.
    pub fn subscribe(
        &self,
        observer: &Arc<dyn EventObserver>,
    ) -> Result<(EventSnapshot, EventSubscription), RegistrationError> {
        let token = self.allocate_token()?;
        let mut observers = self.inner.observers.lock();
        if self.inner.closed.load(Ordering::Acquire) {
            return Err(RegistrationError::SourceClosed);
        }
        observers.insert(token, Arc::downgrade(observer));
        let snapshot = EventSnapshot {
            generation: self.inner.generation.load(Ordering::Acquire),
            closed: false,
        };
        drop(observers);
        Ok((
            snapshot,
            EventSubscription {
                source: Arc::downgrade(&self.inner),
                token: Some(token),
            },
        ))
    }

    /// Returns the current generation and closed state.
    pub fn snapshot(&self) -> EventSnapshot {
        EventSnapshot {
            generation: self.inner.generation.load(Ordering::Acquire),
            closed: self.inner.closed.load(Ordering::Acquire),
        }
    }

    /// Advances the generation and notifies all currently live observers.
    ///
    /// `events` identifies readiness classes that may have changed in either
    /// direction. It is a hint, never a replacement for a fresh level query.
    pub fn notify(&self, events: ReadyEvents) {
        if self.inner.closed.load(Ordering::Acquire) {
            return;
        }
        self.notify_observers(false, events);
    }

    /// Closes the source, wakes current observers once, and removes them.
    pub fn close(&self) {
        if !self.inner.closed.swap(true, Ordering::AcqRel) {
            self.notify_observers(true, ReadyEvents::ALL);
        }
    }

    /// Number of registration records, primarily for lifetime assertions.
    pub fn registration_count(&self) -> usize {
        self.inner.observers.lock().len()
    }

    fn allocate_token(&self) -> Result<EventToken, RegistrationError> {
        self.inner
            .next_token
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map(EventToken)
            .map_err(|_| RegistrationError::TokenExhausted)
    }

    fn notify_observers(&self, clear: bool, events: ReadyEvents) {
        self.inner.generation.fetch_add(1, Ordering::AcqRel);

        let observers = {
            let mut registry = self.inner.observers.lock();
            let mut live = Vec::with_capacity(registry.len());
            registry.retain(|_, observer| {
                if let Some(observer) = observer.upgrade() {
                    live.push(observer);
                    !clear
                } else {
                    false
                }
            });
            live
        };

        for observer in observers {
            observer.on_event(events);
        }
    }
}

impl Default for EventSource {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EventSource {
    fn drop(&mut self) {
        self.close();
    }
}

/// RAII registration. It keeps only a weak source reference, so source,
/// observed object, and consumer cannot form a strong-reference cycle.
pub struct EventSubscription {
    source: Weak<EventSourceInner>,
    token: Option<EventToken>,
}

impl EventSubscription {
    pub fn token(&self) -> Option<EventToken> {
        self.token
    }

    /// Removes this registration. Repeated calls are harmless.
    pub fn unregister(&mut self) -> bool {
        let Some(token) = self.token.take() else {
            return false;
        };
        let Some(source) = self.source.upgrade() else {
            return false;
        };
        source.observers.lock().remove(&token).is_some()
    }
}

impl Drop for EventSubscription {
    fn drop(&mut self) {
        self.unregister();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axsync::spin::SpinNoIrq as TestMutex;
    use core::sync::atomic::{AtomicBool, AtomicUsize};

    struct CountingObserver(AtomicUsize);

    impl EventObserver for CountingObserver {
        fn on_event(&self, _events: ReadyEvents) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn open_file_identity_is_stable_and_non_slot_based() {
        let first = OpenFile::new(10_u32, 0).unwrap();
        let alias = Arc::clone(&first);
        let replacement = OpenFile::new(20_u32, 0).unwrap();

        assert_eq!(first.id(), alias.id());
        assert_ne!(first.id(), replacement.id());
        assert_eq!(RegistrationKey::new(7, first.id()).registered_fd, 7);
        assert_ne!(
            RegistrationKey::new(7, first.id()),
            RegistrationKey::new(7, replacement.id())
        );
    }

    #[test]
    fn status_flags_are_shared_by_description_aliases() {
        let file = OpenFile::new((), 0x10).unwrap();
        let alias = Arc::clone(&file);

        alias.set_status_flags(0x20);
        assert_eq!(file.status_flags(), 0x20);
        assert_eq!(file.update_status_flags(|flags| flags | 0x04), 0x24);
        assert_eq!(alias.status_flags(), 0x24);
    }

    #[test]
    fn closing_one_alias_keeps_description_until_last_close() {
        let file = OpenFile::new((), 0).unwrap();
        let alias = Arc::clone(&file);
        let weak = Arc::downgrade(&file);

        drop(file);
        assert!(weak.upgrade().is_some());

        drop(alias);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn description_close_notification_waits_for_the_last_alias() {
        let identity = Arc::new(DescriptionIdentity::new(OpenFileId::allocate().unwrap()));
        let alias = Arc::clone(&identity);
        let observer = Arc::new(CountingObserver(AtomicUsize::new(0)));
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (_, _subscription) = identity.close_events().subscribe(&erased).unwrap();

        drop(identity);
        assert_eq!(observer.0.load(Ordering::Relaxed), 0);
        drop(alias);
        assert_eq!(observer.0.load(Ordering::Relaxed), 1);
    }

    struct DropTargetObserver(TestMutex<Option<Arc<()>>>);

    impl EventObserver for DropTargetObserver {
        fn on_event(&self, _events: ReadyEvents) {
            self.0.lock().take();
        }
    }

    #[test]
    fn legacy_target_lease_survives_fork_alias_and_drops_on_final_close() {
        let identity = Arc::new(DescriptionIdentity::new(OpenFileId::allocate().unwrap()));
        let fork_alias = Arc::clone(&identity);
        let target = Arc::new(());
        let weak_target = Arc::downgrade(&target);
        let observer = Arc::new(DropTargetObserver(TestMutex::new(Some(target))));
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (_, _subscription) = identity.close_events().subscribe(&erased).unwrap();

        drop(identity);
        assert!(weak_target.upgrade().is_some());

        drop(fork_alias);
        assert!(weak_target.upgrade().is_none());
    }

    #[test]
    fn subscribe_then_recheck_observes_generation_changes() {
        let source = EventSource::new();
        let observer = Arc::new(CountingObserver(AtomicUsize::new(0)));
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (registered, _subscription) = source.subscribe(&erased).unwrap();

        source.notify(ReadyEvents::READABLE);

        assert_ne!(source.snapshot().generation, registered.generation);
        assert_eq!(observer.0.load(Ordering::Relaxed), 1);
    }

    struct HintObserver(AtomicU32);

    impl EventObserver for HintObserver {
        fn on_event(&self, events: ReadyEvents) {
            self.0.fetch_or(events.bits(), Ordering::AcqRel);
        }
    }

    #[test]
    fn readiness_hints_preserve_the_changed_event_class() {
        let source = EventSource::new();
        let observer = Arc::new(HintObserver(AtomicU32::new(0)));
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (_, _subscription) = source.subscribe(&erased).unwrap();

        source.notify(ReadyEvents::WRITABLE);

        assert_eq!(
            observer.0.load(Ordering::Acquire),
            ReadyEvents::WRITABLE.bits()
        );
    }

    #[test]
    fn unregister_and_drop_are_idempotent() {
        let source = EventSource::new();
        let observer: Arc<dyn EventObserver> = Arc::new(CountingObserver(AtomicUsize::new(0)));
        let (_, mut subscription) = source.subscribe(&observer).unwrap();
        assert_eq!(source.registration_count(), 1);

        assert!(subscription.unregister());
        assert!(!subscription.unregister());
        drop(subscription);
        assert_eq!(source.registration_count(), 0);
    }

    #[test]
    fn dead_weak_observers_are_pruned_on_notification() {
        let source = EventSource::new();
        let observer: Arc<dyn EventObserver> = Arc::new(CountingObserver(AtomicUsize::new(0)));
        let (_, _subscription) = source.subscribe(&observer).unwrap();
        assert_eq!(source.registration_count(), 1);

        drop(observer);
        source.notify(ReadyEvents::READABLE);

        assert_eq!(source.registration_count(), 0);
    }

    struct ReentrantObserver {
        source: Weak<EventSource>,
        calls: AtomicUsize,
        reentered: AtomicBool,
    }

    impl EventObserver for ReentrantObserver {
        fn on_event(&self, events: ReadyEvents) {
            self.calls.fetch_add(1, Ordering::Relaxed);
            if !self.reentered.swap(true, Ordering::Relaxed) {
                self.source.upgrade().unwrap().notify(events);
            }
        }
    }

    #[test]
    fn callbacks_can_reenter_without_registry_lock() {
        let source = Arc::new(EventSource::new());
        let observer = Arc::new(ReentrantObserver {
            source: Arc::downgrade(&source),
            calls: AtomicUsize::new(0),
            reentered: AtomicBool::new(false),
        });
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (_, _subscription) = source.subscribe(&erased).unwrap();

        source.notify(ReadyEvents::READABLE);

        assert_eq!(observer.calls.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn closing_wakes_once_and_breaks_registration_lifetimes() {
        let source = EventSource::new();
        let observer = Arc::new(CountingObserver(AtomicUsize::new(0)));
        let erased: Arc<dyn EventObserver> = observer.clone();
        let (_, mut subscription) = source.subscribe(&erased).unwrap();

        source.close();

        assert_eq!(observer.0.load(Ordering::Relaxed), 1);
        assert!(source.snapshot().closed);
        assert_eq!(source.registration_count(), 0);
        assert!(!subscription.unregister());
        assert!(matches!(
            source.subscribe(&erased),
            Err(RegistrationError::SourceClosed)
        ));
    }

    #[test]
    fn edge_delivery_keeps_a_notification_between_level_scans() {
        let mut delivery = EventDeliveryState::default();
        assert!(delivery.should_emit(ReadyEvents::READABLE.bits(), 0, true));
        delivery.commit(ReadyEvents::READABLE.bits(), 0, true, false);

        // The object became not-ready and ready again between scans.  Its
        // current level equals the preceding sample, but the registration's
        // notification generation proves that a transition was observed.
        assert!(delivery.should_emit(ReadyEvents::READABLE.bits(), 2, true));
        delivery.commit(ReadyEvents::READABLE.bits(), 2, true, false);
        assert!(!delivery.should_emit(ReadyEvents::READABLE.bits(), 2, true));
    }

    #[test]
    fn one_shot_disables_only_after_an_event_is_delivered() {
        let mut delivery = EventDeliveryState::default();
        delivery.commit(ReadyEvents::READABLE.bits(), 1, false, true);
        assert!(!delivery.is_disabled());
        delivery.commit(ReadyEvents::READABLE.bits(), 1, true, true);
        assert!(delivery.is_disabled());
    }

    #[test]
    fn one_shot_claim_is_single_winner_across_concurrent_waiters() {
        let delivery = Arc::new(TestMutex::new(EventDeliveryState::default()));
        let barrier = Arc::new(std::sync::Barrier::new(3));
        let winners = Arc::new(AtomicUsize::new(0));
        let mut threads = Vec::new();

        for _ in 0..2 {
            let delivery = delivery.clone();
            let barrier = barrier.clone();
            let winners = winners.clone();
            threads.push(std::thread::spawn(move || {
                barrier.wait();
                if delivery
                    .lock()
                    .claim(ReadyEvents::READABLE.bits(), 1, true, true, true)
                {
                    winners.fetch_add(1, Ordering::AcqRel);
                }
            }));
        }
        barrier.wait();
        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(winners.load(Ordering::Acquire), 1);
    }

    #[test]
    fn full_output_does_not_consume_an_edge_sample() {
        let mut delivery = EventDeliveryState::default();
        assert!(delivery.should_emit(ReadyEvents::READABLE.bits(), 1, true));
        // The consumer deliberately does not call commit when its output is
        // full; the same edge is therefore still visible on the next scan.
        assert!(delivery.should_emit(ReadyEvents::READABLE.bits(), 1, true));
        delivery.commit(ReadyEvents::READABLE.bits(), 1, true, false);
        assert!(!delivery.should_emit(ReadyEvents::READABLE.bits(), 1, true));
    }

    #[test]
    fn final_level_query_wins_over_a_reached_deadline() {
        assert_eq!(decide_level_wait(true, true), LevelWaitDecision::Ready);
        assert_eq!(decide_level_wait(false, true), LevelWaitDecision::TimedOut);
        assert_eq!(decide_level_wait(false, false), LevelWaitDecision::Wait);
    }

    #[test]
    fn ready_or_expired_object_requests_an_immediate_rescan() {
        let now = Duration::from_secs(10);
        assert_eq!(
            readiness_deadline_delay(true, None, now),
            Some(Duration::ZERO)
        );
        assert_eq!(
            readiness_deadline_delay(false, Some(Duration::from_secs(9)), now),
            Some(Duration::ZERO)
        );
        assert_eq!(
            readiness_deadline_delay(false, Some(Duration::from_secs(12)), now),
            Some(Duration::from_secs(2))
        );
    }

    #[test]
    fn notification_reentrancy_is_bounded_and_released_by_drop() {
        let gate = ReentrancyGate::new();
        let outer = gate.try_enter().expect("first notification enters");
        assert!(gate.try_enter().is_none());
        drop(outer);
        assert!(gate.try_enter().is_some());
    }

    #[test]
    fn notification_reentrancy_coalesces_without_losing_a_round() {
        let gate = ReentrancyGate::new();
        let mut outer = gate.try_enter().expect("first notification enters");

        assert!(gate.try_enter().is_none());
        assert!(outer.finish_round(), "nested notification must be replayed");
        assert!(!outer.finish_round(), "quiescent gate must return to idle");
        assert!(gate.try_enter().is_some());
    }

    #[test]
    fn reciprocal_pair_operations_share_one_lock_order() {
        let first = Arc::new(TestMutex::new(0_u32));
        let second = Arc::new(TestMutex::new(0_u32));
        let start = Arc::new(std::sync::Barrier::new(3));
        let spawn_reciprocal = |source: Arc<TestMutex<u32>>, destination: Arc<TestMutex<u32>>| {
            let start = Arc::clone(&start);
            std::thread::spawn(move || {
                start.wait();
                for _ in 0..128 {
                    with_ordered_arc_pair(
                        &source,
                        &destination,
                        |lower, higher, caller_order_reversed| {
                            let mut lower_guard = lower.lock();
                            let mut higher_guard = higher.lock();
                            let destination_guard = if caller_order_reversed {
                                &mut lower_guard
                            } else {
                                &mut higher_guard
                            };
                            **destination_guard += 1;
                        },
                    )
                    .unwrap();
                }
            })
        };

        let forward = spawn_reciprocal(Arc::clone(&first), Arc::clone(&second));
        let reverse = spawn_reciprocal(Arc::clone(&second), Arc::clone(&first));
        start.wait();
        forward.join().unwrap();
        reverse.join().unwrap();

        assert_eq!(*first.lock(), 128);
        assert_eq!(*second.lock(), 128);
        assert!(with_ordered_arc_pair(&first, &first, |_, _, _| ()).is_none());
    }

    #[test]
    fn object_state_is_shared_only_by_description_aliases() {
        let description = OpenFile::new(TestMutex::new(0_u32), 0).unwrap();
        let alias = Arc::clone(&description);
        let independent = OpenFile::new(TestMutex::new(0_u32), 0).unwrap();

        *alias.object().lock() = 7;

        assert_eq!(*description.object().lock(), 7);
        assert_eq!(*independent.object().lock(), 0);
    }

    #[test]
    fn clone_and_exec_split_are_serialized_by_explicit_membership() {
        let mut sharing = FileTableShareTracker::default();
        sharing.register_base(10);
        sharing.share(10, 20);
        assert!(sharing.is_shared(10));
        assert!(sharing.is_shared(20));

        assert_eq!(sharing.split(10), None);
        assert_eq!(sharing.group(10), FileTableGroup::Private(10));
        assert_eq!(sharing.group(20), FileTableGroup::Base);
        assert!(!sharing.is_shared(10));
        assert!(!sharing.is_shared(20));

        // A child cloned after the split joins the parent's private table,
        // never the old base table.
        sharing.share(10, 30);
        assert!(sharing.is_shared(10));
        assert_eq!(sharing.group(30), FileTableGroup::Private(10));
    }

    #[test]
    fn failed_split_preparation_leaves_shared_membership_unchanged() {
        let mut sharing = FileTableShareTracker::default();
        sharing.register_base(10);
        sharing.share(10, 20);

        // A caller prepares a fallible table copy before committing `split`.
        // Simulate that copy failing by deliberately not calling `split`.
        assert!(sharing.is_shared(10));
        assert!(sharing.is_shared(20));
        assert_eq!(sharing.group(10), FileTableGroup::Base);
        assert_eq!(sharing.group(20), FileTableGroup::Base);
    }

    #[test]
    fn detaching_an_unknown_process_does_not_drop_the_base_table() {
        let mut sharing = FileTableShareTracker::default();
        sharing.register_base(10);
        assert_eq!(sharing.detach(99), FileTableDetach::Keep);
        assert_eq!(sharing.group(10), FileTableGroup::Base);
    }

    #[test]
    fn splitting_or_exiting_a_private_owner_transfers_the_table_key() {
        let mut sharing = FileTableShareTracker::default();
        sharing.register_base(1);
        sharing.share(1, 2);
        sharing.split(1);
        sharing.share(1, 3);

        assert_eq!(sharing.split(1), Some((1, 3)));
        assert_eq!(sharing.group(1), FileTableGroup::Private(1));
        assert_eq!(sharing.group(3), FileTableGroup::Private(3));
        assert_eq!(sharing.detach(3), FileTableDetach::DropPrivate(3));
        assert_eq!(sharing.detach(1), FileTableDetach::DropPrivate(1));
        assert_eq!(sharing.detach(2), FileTableDetach::DropBase);
    }
}
