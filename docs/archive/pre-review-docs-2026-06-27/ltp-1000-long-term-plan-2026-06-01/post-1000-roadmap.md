# post-1000 roadmap

## Immediate quality work

1. Schedule a full `LTP_CASES=stable` RV and LA sweep for the 1000-case list, preferably sharded to avoid long single-run timeouts and to isolate order/resource interactions.
2. Add a focused exec-failure atomicity regression: start from a live process image, attempt failing exec paths (`EACCES`, `ENOENT`, `ENOTDIR`, `EFAULT`, `ENOEXEC`), then prove the caller image still runs and address-space state remains valid.
3. Add a vfork exec wake regression covering child successful exec and child exit paths so parent wake semantics stay explicit.
4. Reduce LA fcntl stress BadAddress log noise while preserving true user fault behavior and parser-clean outcomes.
5. Add a focused FD table regression for `CLOSE_RANGE_UNSHARE` followed by `clone(CLONE_FILES)` while older processes still share the original base table.

## Semantic expansion lanes

- VFS/path/xattr/statfs: continue from full-sweep clean candidates only.
- FD/fcntl/pipe/io: expand after shared-offset/O_APPEND/SIGPIPE/EINTR and FD-table alias confidence is documented.
- time/signal/process: handle jitter-prone and signal-delivery cases with strict errno/siginfo boundaries.
- mmap/mm/resource: continue file-backed shared mmap, msync, mprotect split/merge, mincore, and teardown work.
- futex/thread/IPC: stress wakeup lifetime and timeout/EINTR semantics.
- proc/socket/syntheticfs: improve field truthfulness and socket readiness/teardown.

## Promotion rule after 1000

No new stable promotion should count blacklist/SKIP/status0/full-sweep partial PASS. Keep RV + LA x musl + glibc parser-clean evidence and document ABI/resource/lifetime boundaries per milestone.
