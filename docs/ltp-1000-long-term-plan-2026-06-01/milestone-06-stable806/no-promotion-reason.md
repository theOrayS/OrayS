# milestone-06 current no-promotion reason

This is an interim stable806 checkpoint. The current baseline remains stable756 because the two fresh RV scouting batches produced zero RV × musl/glibc parser-clean candidates.

Reasons promotion is blocked at this checkpoint:

1. The old archived 4/4 clean-not-stable seed list has already been exhausted by earlier milestones; no remaining old clean seed exists outside current stable756.
2. The proc/synthetic/sched scout has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows, and the RV-only promotion report lists `Promotion candidates: 0`.
3. The time/fd/signal scout likewise has visible `TFAIL`, `TBROK`, `TCONF`, `ENOSYS`, and timeout rows, and the RV-only promotion report lists `Promotion candidates: 0`.
4. No LA gate was run because RV evidence is already insufficient; running LA on known-dirty candidates would not create promotion evidence.
5. No blacklist/SKIP/status0/full-sweep partial TPASS evidence is counted.

Next safe slices:

- Implement generic timerslack semantics (`PR_SET_TIMERSLACK`, `PR_GET_TIMERSLACK`, and real `/proc/self/timerslack_ns`) before retesting `prctl08`/`prctl09`.
- Isolate `nice04` musl errno behavior against `setpriority`/`nice` semantics before touching shared priority code.
- Avoid POSIX timer rows (`timer_create` family) as easy promotions unless the project accepts a real timer-object implementation.
- For network/socket milestone-06 intent, run a small `accept/bind/connect/getsockopt/setsockopt/send/recv` RV scout next, but keep LA network severe-blocker history in mind and do not use blacklist-skipped rows as promotion evidence.
