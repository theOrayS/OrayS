use core::any::Any;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;

use axerrno::LinuxError;
use axfile::{EventObserver, EventSource, EventSubscription, OpenFile, OpenFileId, ReadyEvents};
use axtask::WaitQueue;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::vec::Vec;

use super::UserProcess;
use super::signal_abi::current_unblocked_signal_pending;

/// Shared operation boundary for the first file-object migration slice.
///
/// The descriptor table stores one `Arc<OpenFile<_>>` per open description.
/// Legacy `FdEntry` variants remain outside this trait until a later migration.
pub(super) trait FileObject: Any + Send + Sync {
    fn read(
        &self,
        description: &OpenFile<dyn FileObject>,
        process: &UserProcess,
        dst: &mut [u8],
    ) -> Result<usize, LinuxError>;

    fn write(
        &self,
        description: &OpenFile<dyn FileObject>,
        process: &UserProcess,
        src: &[u8],
    ) -> Result<usize, LinuxError>;

    fn readiness(&self) -> ReadyEvents;

    fn event_source(&self) -> &EventSource;

    /// Returns the remaining delay until readiness may change without a
    /// producer notification (currently timerfd).  A relative duration keeps
    /// realtime and monotonic timerfd clock domains out of poll's clock domain.
    fn next_timeout(&self) -> Option<Duration> {
        None
    }

    fn as_any(&self) -> &dyn Any;
}

pub(super) type OpenFileRef = Arc<OpenFile<dyn FileObject>>;

pub(super) fn new_open_file<T: FileObject + 'static>(
    object: T,
    status_flags: u32,
) -> Result<OpenFileRef, LinuxError> {
    let object: Arc<dyn FileObject> = Arc::new(object);
    OpenFile::from_arc(object, status_flags).map_err(|_| LinuxError::ENFILE)
}

pub(super) fn object_as<T: FileObject + 'static>(
    description: &OpenFileRef,
) -> Result<&T, LinuxError> {
    description
        .object()
        .as_any()
        .downcast_ref::<T>()
        .ok_or(LinuxError::EBADF)
}

/// One fan-in waiter for any number of migrated readiness sources.
///
/// Sources keep only weak observer references.  Subscriptions keep weak source
/// references, so this object cannot form a source/consumer ownership cycle.
struct ObjectWaiter {
    generation: AtomicU64,
    wait: WaitQueue,
}

impl ObjectWaiter {
    const fn new() -> Self {
        Self {
            generation: AtomicU64::new(0),
            wait: WaitQueue::new(),
        }
    }
}

impl EventObserver for ObjectWaiter {
    fn on_event(&self, _events: axfile::ReadyEvents) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        self.wait.notify_all(false);
    }
}

/// RAII subscriptions used by poll/select's query/register/recheck protocol.
pub(super) struct ObjectWaitSet {
    waiter: Arc<ObjectWaiter>,
    _subscriptions: Vec<EventSubscription>,
}

impl ObjectWaitSet {
    pub(super) fn subscribe(files: &[OpenFileRef]) -> Result<Self, LinuxError> {
        let waiter = Arc::new(ObjectWaiter::new());
        let observer: Arc<dyn EventObserver> = waiter.clone();
        let mut seen = BTreeSet::<OpenFileId>::new();
        let mut subscriptions = Vec::new();
        subscriptions
            .try_reserve_exact(files.len())
            .map_err(|_| LinuxError::ENOMEM)?;
        for file in files {
            if !seen.insert(file.id()) {
                continue;
            }
            match file.object().event_source().subscribe(&observer) {
                Ok((_snapshot, subscription)) => subscriptions.push(subscription),
                // A live OpenFile owns its source, so SourceClosed is treated as
                // a hint to re-query.  Token exhaustion is a real resource error.
                Err(axfile::RegistrationError::SourceClosed) => {
                    waiter.on_event(axfile::ReadyEvents::ALL)
                }
                Err(axfile::RegistrationError::TokenExhausted) => {
                    return Err(LinuxError::ENFILE);
                }
            }
        }
        Ok(Self {
            waiter,
            _subscriptions: subscriptions,
        })
    }

    /// Sample this before the readiness recheck.  Blocking is allowed only if
    /// the sample remains current after that recheck.
    pub(super) fn generation(&self) -> u64 {
        self.waiter.generation.load(Ordering::Acquire)
    }

    pub(super) fn wait_for_change(&self, process: &UserProcess, sample: u64, timeout: Duration) {
        if timeout.is_zero() || self.generation() != sample {
            return;
        }
        process.set_syscall_wait_blocked(true);
        self.waiter.wait.wait_timeout_until(timeout, || {
            self.generation() != sample
                || process.eval_watchdog_expired()
                || current_unblocked_signal_pending()
        });
        process.set_syscall_wait_blocked(false);
    }
}
