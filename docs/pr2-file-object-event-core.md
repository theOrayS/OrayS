# PR2 file-object and readiness-event core

## Scope and compatibility boundary

PR2 keeps all existing `FdEntry` discriminants. Only `Pipe`, `EventFd`, and
`TimerFd` carry `Arc<OpenFile<dyn FileObject>>`; every other entry continues to
use its existing implementation through the `FdTable::poll_entry`, read/write,
and fcntl branches. This payload boundary is the smallest adapter that lets one
FD table contain migrated and legacy descriptions without converting regular
files, sockets, signalfd, pidfd, message queues, or synthetic files.

The current variants are:

`Stdin`, `Stdout`, `Stderr`, `DevNull`, `DevZero`, `DevRandom`,
`DevCpuDmaLatency`, `BlockDevice`, `Rtc`, `File`, `Directory`, `ProcFdDir`,
`SyntheticDir`, `Path`, `MemoryFile`, `Memfd`, `ProcPagemap`, `ProcTimerSlack`,
`Pipe`, `Socket`, `LocalSocket`, `EventFd`, `Inotify`, `Epoll`, `TimerFd`,
`SignalFd`, `PidFd`, `PosixMq`, `ProcMqQueuesMax`, and `ProcSysFile`.

## Object and ownership relationships

```text
FdTable slot
  |-- fd_flags (descriptor-local FD_CLOEXEC)
  |-- Arc<DescriptionIdentity>
  |     |-- OpenFileId (stable, preserved by dup/fork)
  |     `-- final-alias close EventSource
  `-- FdEntry
       |-- migrated: Arc<OpenFile<dyn FileObject>>
       |     |-- OpenFileId
       |     |-- atomic shared status flags
       |     `-- Arc<pipe/eventfd/timerfd object>
       `-- legacy: existing FdEntry payload and adapter branches

EpollState
  `-- BTreeMap<RegistrationKey(fd, OpenFileId), Registration>
        |-- Weak<OpenFile> / Weak<EpollState> / Arc<LegacyEpollTarget>
        |-- per-registration ready observer + subscription
        `-- versioned close observer + subscription

EventSource --weak--> EventObserver (poll/select waiter or epoll registration)
```

`OpenFileId` is allocated monotonically and is never derived from an FD slot or
an address. Allocation reports exhaustion instead of wrapping. An epoll
registration therefore cannot attach to a replacement object merely because
the replacement reused the same integer FD.

The object, event source, observer, and subscription graph has no strong cycle.
Migrated and nested epoll targets are weak. A legacy registration owns a small
compatibility snapshot of the existing payload so a forked epoll can still
query it after the waiting process closes its local alias; that snapshot has no
back-reference to epoll. Its raw POSIX socket duplicate, if any, is closed when
the registration is retired. Observers hold a weak epoll owner and
subscriptions hold a weak source. Dropping or explicitly unregistering a
subscription is idempotent. `DescriptionIdentity` is shared by all descriptor
aliases, and its final drop emits a close hint that removes the exact
registration version and its compatibility snapshot immediately.

## Descriptor and shared state

`FdSlot::fd_flags` contains `FD_CLOEXEC` and is copied or cleared according to
the individual dup operation. `OpenFile::status_flags` contains applicable
shared flags such as `O_NONBLOCK` for the three migrated object types. All dup
and non-`CLONE_FILES` fork copies preserve the same `DescriptionIdentity` and
clone the same `Arc<OpenFile>`. Closing one alias only removes that slot; the
object and pipe endpoint lifetime end after the last alias is dropped.

Pipe `O_ASYNC` owner, signal, and enable state belongs to the `PipeEndpoint`
object. A dup shares that endpoint and therefore shares its open-description
state. The read and write descriptions created by `pipe2`, and separate FIFO
opens, each receive a different state object. The shared pipe buffer retains
only weak listener references, so it neither merges distinct descriptions nor
creates an ownership cycle.

Legacy entries retain their existing payload behavior. Their stable
`description_id` is already carried by the coherent slot and is used by poll
and epoll to reject reuse, but moving all legacy status flags into `OpenFile`
is intentionally follow-up work rather than a flag-day rewrite.

`FileTableShareTracker` records base/private membership under the same
`ProcessFdTableState` lock that publishes a shared table Arc. No sharing
decision depends on `Arc::strong_count`. A non-sharing fork copies slots while
retaining open-description identities; its failure path closes every raw POSIX
socket duplicate already created. FD insertion and `dup3` target-close failure
paths likewise discard any prepared raw socket duplicate that never became an
installed slot. A `CLONE_FILES` child is registered using its allocated pid
before any rollback can run.

Exec builds the complete replacement image first. It then prepares the
fallible table copy and commits the membership split, still before swapping the
live address space. That swap and the remaining image-state updates are
infallible; `FD_CLOEXEC` closure applies only to the executing process after the
split. A loader or table-copy error therefore returns to the intact old image
and leaves the formerly shared table relationship unchanged.

## Readiness and waiting protocol

`ReadyEvents` is a level query. Pipe operations and peer close, eventfd counter
transitions, timerfd arm/disarm/read, and object drop are readiness producers.
poll, select, epoll, and blocking object reads/writes are consumers.

Notifications are hints only. poll/select perform this protocol for migrated
targets:

1. snapshot strong `OpenFile` references while briefly holding the FD-table
   lock;
2. register a weak observer with each distinct source;
3. sample the waiter's generation;
4. query current readiness with no FD-table lock held for migrated objects;
5. block only if the generation still equals the sample;
6. re-query after every wakeup or timeout.

The generation check closes the notification-versus-block race. After every
wake helper return, poll, select, and epoll return to a level query before
testing their deadline; readiness wins when readiness and timeout become true
together. Legacy targets retain the existing bounded 1 ms rescan adapter.
timerfd exposes a non-mutating relative next timeout. An already pending or
passed expiry returns zero and forces the outer loop to query readiness; wait
profiling never silently consumes an expiry.

epoll installs one weak source subscription and one notification generation per
migrated or nested registration. Source hints carry the readiness classes that
may have changed, so an `EPOLLIN|EPOLLET` watch does not retrigger merely because
the same pipe became more writable. Its wait path samples the epoll generation
before scanning and blocks on the epoll wait queue only while that sample
remains current. Target callbacks update the registration generation, then
advance the epoll generation and notify waiters/nested epolls without holding
the registration map. `EventDeliveryState` always re-queries the current level;
for `EPOLLET`, a changed notification generation preserves a complete
not-ready/ready cycle between scans. Output-capacity failure does not consume an
edge, and `EPOLLONESHOT` disables only after actual delivery. `MOD` installs a
fresh version and rearms it. Delivery decision and commit occur under the
registration lock through `EventDeliveryState::claim`, so concurrent waiters
cannot both consume one edge/one-shot. Stale scan cleanup compares the captured
version before removal and therefore cannot delete a newer `MOD` replacement.

Nested-graph validation and insertion are one serialized transaction, so the
published graph is acyclic. `ReentrancyGate` flattens synchronous propagation:
a callback arriving during a fan-out atomically marks a pending round, and the
active caller performs that round before releasing the gate. Thus concurrent
child transitions cannot be discarded merely because a parent notification is
already in progress.

## Locks and callbacks

The intended ordering and non-nesting rules are:

1. FD-table lock is used for slot validation, identity snapshots, allocation,
   legacy adapter queries, and `epoll_ctl` setup. It is never held across a
   blocking wait or migrated-object I/O.
2. The global epoll graph lock is taken after the FD-table lock and spans both
   nested-graph validation and registration mutation. This closes the
   check-versus-insert race across forked tables sharing `EpollState`.
3. Object-state, event-source registry, and epoll registration locks protect
   one local transition or snapshot. A callback is invoked only after the
   source registry lock is released; readiness callbacks do not enter the
   registration map.
4. Reciprocal pipe buffer operations use `with_ordered_arc_mutex_pair`, which
   ranks the two `Arc` allocations once and acquires the same order for
   `tee(A,B)` and `tee(B,A)`. No wait or notification occurs with either buffer
   guard held.

Legacy epoll target retirement is returned through `EpollCtlUpdate`; the
syscall drops it only after its FD-table guard has gone out of scope. This keeps
raw socket close and local-socket endpoint destruction out of the FD-table
critical section.

`epoll_wait` snapshots the `EpollEntry` under the table lock and then scans its
self-contained weak/object, nested, or compatibility targets without that
guard. No migrated readiness method or nested epoll query is called while the
FD table is locked.

Migrated-object paths do not hold the FD-table, object-state, source-registry,
or epoll-registration lock across a blocking wait or user-memory copy.
`F_GETOWN_EX` snapshots pipe async ownership and releases its lock before the
user write. Legacy adapter branches retain their pre-PR2 locking behavior;
removing every such legacy critical section belongs to the later per-variant
migrations. Producers drop object-state locks before calling
`EventSource::notify`. Epoll readiness queries are performed from registration
snapshots and committed under a later short lock guarded by a registration
version. A tested `ReentrancyGate` bounds synchronous nested-epoll callback
stack depth and replays a coalesced round before release.

## Close and FD reuse

Close detaches a slot while holding the table lock but returns a `ClosedFd`
owner. The syscall drops that owner only after the lock guard is gone, so final
object and `DescriptionIdentity` callbacks cannot re-enter FD/event code under
the table lock. If dup/fork aliases remain, readiness and registrations continue
to refer to the same description. The final alias closes the identity source;
the versioned close observer removes and drops the registration outside its map
lock. A new object in the old slot receives a new `OpenFileId`; `ADD`, `MOD`,
and `DEL` address only its distinct `RegistrationKey`.

Legacy poll targets and legacy epoll registrations also retain the slot's
stable description ID. A legacy epoll registration queries its compatibility
snapshot rather than looking up the original integer slot. Closing an original
FD in one process therefore cannot delete or retarget the shared watch while a
dup or fork alias remains anywhere. Final `DescriptionIdentity` close removes
the snapshot; reuse of the integer FD creates a different identity and cannot
retarget it.

## Validation and regression coverage

The `axfile` suite executes 25 semantic tests. In addition to the original core
coverage, it now exercises explicit CLONE_FILES split/rollback state,
final-alias close, a fork-surviving legacy target lease, per-registration ET
and one-shot delivery, timeout-versus-readiness resolution, timer deadline
profiling, readiness-hint classes, concurrent one-shot claiming, notification
coalescing, canonical two-lock ordering, and state shared only by aliases of one
description.

`scripts/check_pr2_file_event_core.py` binds those tested primitives to the
actual exec, FD-table, epoll, poll/select, timerfd, and pipe call sites.
`scripts/test_pr2_file_event_core.py` applies 22 targeted mutations plus a clean
repository case; each confirmed review issue has a mutation that must be
rejected. The `pr2-check` Make target runs both scripts and the Rust suite, and
both `unittest` variants depend on it, so the existing Test CI executes them.

The external runtime inventory contains pipe, dup3, epoll, eventfd, and timerfd
LTP cases. Their presence is not counted as a pass: behavioral runtime evidence
requires the evaluator disk images and QEMU run on each architecture.

No new dependency or `unsafe` was introduced by this design. The core crate is
`no_std`; it uses the repository's existing allocation and synchronization
facilities.

## Follow-up boundary

Regular files, directories, sockets, signalfd, pidfd, memfd, message queues,
and synthetic entries remain behind the adapter. Future migrations can replace
one payload with `OpenFileRef`, implement `FileObject`, and remove only that
variant's legacy branches. Broad mmap/VM integration, page cache work, and
signal/process-model redesign are not required by this core.
