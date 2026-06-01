# milestone-02-stable606 ABI and behavior impact preflight

## Code changes in this preflight

File: `examples/shell/src/uspace/fd_socket.rs`

`sys_socket_bridge` now distinguishes unsupported AF_INET raw sockets from invalid AF_INET socket types:

- `SOCK_RAW` with unsupported AF_INET protocol returns `EPROTONOSUPPORT`.
- Other invalid AF_INET socket types return `EINVAL`.
- Existing AF_UNIX, AF_INET stream, and AF_INET datagram paths are unchanged.

This is a generic socket errno correction, not an LTP case/path/process/output special case.

## POSIX/Linux-visible impact

- Syscall surface affected: `socket(domain=AF_INET, type=..., protocol=...)`.
- Errno impact:
  - Before: unsupported/invalid AF_INET socket types generally returned `ESOCKTNOSUPPORT`.
  - After: invalid type composition returns `EINVAL`; unsupported raw type returns `EPROTONOSUPPORT`.
- FD/lifetime impact: none. No new descriptor is allocated on these error paths.
- Signal/futex/mmap/user-pointer impact: none.
- Resource/lifetime risk: low; adjacent socket regression (`accept01`, `listen01`, `socket02`, `socketpair02`) is parser-clean on RV + LA x musl + glibc.

## Deliberately rejected shortcut: nice04

`nice04` was not fixed by changing kernel `setpriority()` errno from `EACCES` to `EPERM`.

Reason:

- Linux `setpriority(2)` distinguishes `EACCES` for lowering the numeric nice value without privilege from `EPERM` for target ownership/capability mismatch.
- Linux/POSIX `nice()` wrappers may map `EACCES` to `EPERM` for `nice(-10)`; local host probing showed `setpriority(..., -10)` returns `EACCES` while `nice(-10)` returns `EPERM` after dropping privileges.
- A kernel-only remap would make direct `setpriority()` less Linux-compatible, so this preflight leaves `nice04` blocked rather than taking a score-only shortcut.

References used for the semantic boundary:

- Linux man-pages `setpriority(2)`: https://www.man7.org/linux/man-pages/man2/setpriority.2.html
- Linux man-pages `nice(2)`: https://man7.org/linux/man-pages/man2/nice.2.html
- glibc `nice.c` behavior maps `EACCES` to `EPERM`: https://codebrowser.dev/glibc/glibc/sysdeps/posix/nice.c.html
- musl patch discussion for the same `nice()` errno mapping: https://www.openwall.com/lists/musl/2021/06/29/1

## Stable-list ABI impact

None in this preflight. `LTP_STABLE_CASES` is unchanged at 556/556/0.

## Additional procfs/mmap behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/mod.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/memory_map.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`

Behavior:

- `UserProcess` now tracks user-created mmap regions as synthetic procfs metadata.
- `/proc/self/maps` now includes parseable dynamic mmap ranges with current `rwx` protection bits and `p/s` private/shared state.
- `MAP_FIXED`, `munmap`, `mprotect`, `exec`, and `fork` update/preserve the synthetic map list.

POSIX/Linux-visible impact:

- `/proc/self/maps` becomes more truthful for anonymous mmap, vma-adjacency, and protection-display tests.
- No actual page-table permissions are weakened; `mmap_prot_to_flags` still keeps implementation-internal read access where needed, while procfs prints the requested Linux-visible protection bits.
- FD, signal, futex, and user-pointer copy semantics are unchanged.
- Resource/lifetime risk: moderate-low. Metadata is per-process and cleared on exec; the regression subset protects existing stable mmap/mincore/mprotect anchors on both RV and LA.
