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


## Additional times() behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/mod.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/time_abi.rs`

Behavior:

- `times()` now fills `struct tms` self and waited-child counters instead of returning all-zero `tms_*` fields.
- The `times()` return value now uses `USER_HZ` clock ticks rather than raw milliseconds.
- `wait4`/`waitid` account waited-child self plus descendant ticks before child teardown.

POSIX/Linux-visible impact:

- `tms_utime`, `tms_stime`, `tms_cutime`, and `tms_cstime` become monotonic, nonzero after real process lifetime/child work, and visible to both musl and glibc callers.
- Accounting is still coarse: self user/system ticks are wall-clock-derived and split between user/system time rather than scheduler-precise CPU sampling. This is more truthful than the prior all-zero stub but remains a maintenance boundary for future scheduler-level CPU accounting.
- FD, signal delivery, futex, mmap permissions, and user-pointer copy semantics are unchanged.
- Resource/lifetime risk: low. Added counters are per-process atomics and are only accumulated when a child is actually waited/reaped; regression subset protects existing time anchors on RV and LA.


## Additional MAP_LOCKED / VmLck behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/mod.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/memory_map.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`

Behavior:

- `mmap(..., MAP_LOCKED, ...)` now records the created VMA as locked metadata and eagerly populates the pages instead of leaving the mapping lazy.
- `munmap` and `MAP_FIXED` range replacement remove/split the locked metadata together with the mmap range; `mprotect` preserves the locked state while changing visible protection bits.
- `/proc/self/status` now includes `VmLck:\t<N> kB` computed from the current process's locked mmap ranges.

POSIX/Linux-visible impact:

- Syscall/flag surface affected: `mmap` with `MAP_LOCKED`; `munmap`, `mprotect`, and `MAP_FIXED` preserve/update the associated synthetic metadata.
- Procfs ABI affected: `/proc/<pid>/status` and `/proc/self/status` now expose `VmLck`, enabling generic locked-memory introspection.
- Errno impact: none in this follow-up; unsupported flags and existing mmap error paths are unchanged.
- FD impact: none; no descriptor allocation or lifetime behavior changes.
- Signal/futex/user-pointer copy impact: none.
- mmap/resource caveat: this is locked-range accounting plus eager population in a no-swap teaching kernel. It does not yet implement full Linux `mlock(2)`/`RLIMIT_MEMLOCK` enforcement or page-reclaim interaction.
- Resource/lifetime risk: moderate-low. Metadata is per-process, cloned on fork, cleared on exec, and reduced on unmap; the RV/LA regression subset protects stable mmap/mincore/mprotect anchors.

## Additional /proc/self/pagemap behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`
- `examples/shell/src/uspace/metadata.rs`

Behavior:

- `/proc/self/pagemap` and `/proc/<pid>/pagemap` are now exposed as read-only synthetic procfs files.
- A new fd entry supports sparse `lseek` and `read` for pagemap offsets without materializing a huge file.
- Reads return native-endian `u64` pagemap entries with bit 63 (`present`) set for pages that are present in a snapshot taken at open time across the executable approximation, heap, stack, and tracked mmap ranges.
- PFN, soft-dirty, swapped, file/shared, exclusive, and other Linux pagemap bits are intentionally not exposed; those fields remain zero.

POSIX/Linux-visible impact:

- Procfs ABI affected: `open/read/lseek/stat` on `/proc/self/pagemap` and `/proc/<pid>/pagemap` now work for existing process ids. O_PATH/stat path lookup is supported through the existing synthetic-path machinery.
- Errno impact: missing proc pids still fail as absent synthetic paths; write/truncate/create modes remain rejected by the existing read-only synthetic procfs open checks.
- FD impact: a pagemap fd has its own seek offset and is duplicated on fork like other fd table entries.
- mmap/resource impact: `MAP_POPULATE`-visible present pages can now be observed through pagemap. This is a snapshot-on-open model, not a live kernel pagemap; callers that expect Linux's continuously changing pagemap bits remain outside the current compatibility boundary.
- Signal/futex/user-pointer copy impact: none.
- Resource/lifetime risk: moderate-low. The fd stores coalesced present page-index ranges rather than a full address-space-sized byte vector; the RV/LA regression subset protects existing mmap/mincore/mprotect/proc maps anchors.

## Additional setgid-directory create metadata behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/fd_table.rs`

Behavior:

- New filesystem nodes created via `open(O_CREAT)`/`creat()`, `mkdirat()`, and `mknodat()` now inspect the parent directory metadata before recording synthetic uid/gid/mode metadata.
- If the parent directory has `S_ISGID`, the new node's recorded gid inherits the parent directory gid instead of the caller's current `fs_gid()`.
- New subdirectories under a setgid parent also retain the setgid bit in the recorded mode after applying umask. Non-setgid parents keep the previous process-gid behavior.

POSIX/Linux-visible impact:

- Syscall/flag surface affected: `open`/`openat` with `O_CREAT`, `creat`, `mkdir`/`mkdirat`, and `mknod`/`mknodat` for regular/FIFO nodes in setgid directories.
- `stat`/`fstatat`/`lstat` visible metadata becomes more Linux-compatible for `st_gid` and directory setgid inheritance.
- Errno impact: none; existing permission and existence checks run before creation as before.
- FD impact: no new descriptor semantics; this only changes recorded metadata after successful creation.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: low. The change reuses existing per-process path metadata maps and only broadens the values recorded for successful generic create paths; RV/LA regression protects adjacent stable open/creat/chmod/chown/mkdir/mknod anchors.


## Additional synthetic group database behavior change on 2026-06-02

File:

- `examples/shell/src/uspace/linux_abi.rs`

Behavior:

- The default synthetic `/etc/group` file now contains `daemon:x:1:` and `users:x:100:` in addition to `root` and `nogroup`.
- The change is generic filesystem-visible data for libc name-service lookups; it does not special-case LTP binaries, process names, paths beyond the existing `/etc/group` synthetic file, or wrapper output.

POSIX/Linux-visible impact:

- Synthetic filesystem ABI affected: reads/stat/open of `/etc/group` expose two additional conventional group entries.
- Syscall/errno impact: none for kernel syscalls directly; successful libc `getgrnam("users")` / `getgrnam("daemon")` can change user-space setup paths that previously treated the groups as absent.
- FD impact: none beyond the existing read-only synthetic-file fd behavior.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: low. This is static byte content consumed by existing synthetic-file plumbing; RV/LA regression protects chmod/chown/open/creat anchors.

## Additional tmpfs read-only mount metadata behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/mod.rs`
- `examples/shell/src/uspace/mount_abi.rs`
- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/credentials.rs`
- `examples/shell/src/uspace/fd_table.rs`

Behavior:

- Per-process mount table entries now store a source root and a read-only flag instead of only the source path string.
- `mount(..., MS_REMOUNT|MS_RDONLY, ...)` is accepted for existing mount points and updates the recorded read-only state; `MS_REMOUNT` on a non-mounted target still fails with `EINVAL`.
- Write-like file metadata paths (`access(..., W_OK)`, `truncate`, `fchmod`, `fchmodat`, `chown`/`fchownat`, open/create parent write checks) detect when the resolved path sits under a read-only mount and return `EROFS`.
- Chown path handling now preserves Linux errno ordering for inaccessible path prefixes by returning `EACCES` before ownership permission checks.

POSIX/Linux-visible impact:

- Syscall/flag surface affected: `mount` flag handling for `MS_RDONLY|MS_REMOUNT`, `access`/`faccessat`, `chmod`/`fchmod`/`fchmodat`, `chown`/`fchownat`, `truncate`, and open/create permission checks.
- Errno impact: write-like operations below a read-only mounted subtree now return `EROFS` instead of succeeding or falling through to `EPERM`; inaccessible parent prefixes for chown-style path operations return `EACCES`.
- FD impact: existing fd paths are used to identify read-only mount membership for `fchmod` and fd-relative `fchmodat`; descriptor allocation/dup/close semantics are unchanged.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: moderate-low. Mount state is still per-process shared metadata, now with one extra boolean and source-root string per mount point. The RV/LA regression subset protects adjacent stable VFS permission and metadata cases.
- Maintenance boundary: this is read-only mount metadata sufficient for VFS errno semantics. It does not implement a full Linux VFS superblock layer, shared mount namespaces, or bind-mount read-only propagation beyond the current synthetic mount table model.


## Additional `/proc/self/fd` directory behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/metadata.rs`

Behavior:

- `/proc/self/fd`, `/proc/<current-pid>/fd`, and `/dev/fd` are now exposed as read-only synthetic directories.
- `getdents64` on those directory fds returns `.`, `..`, and a dynamic snapshot of the process's currently open fd numbers; numeric fd rows use directory cookies for continuation and `DT_LNK` type to match Linux procfs expectations.
- Directory fd operations reuse the current directory behavior: file reads and file-offset seeks return directory errors, while `getdents64` advances the fd-directory enumeration cursor and `fstat`/path stat report a read-only directory mode.
- `O_PATH` opens of the directory paths use the existing synthetic path entry so later metadata operations can identify the path without creating a readable fd object.

POSIX/Linux-visible impact:

- Procfs ABI affected: `open`/`openat`, `getdents64`, `fstat`, `stat`/`fstatat`, and fd-relative path resolution for `/proc/self/fd`, `/proc/<pid>/fd` when `<pid>` is the current process, and `/dev/fd`.
- Errno impact: creating/truncating/writing those directory paths is rejected as a directory/read-only procfs operation; regular directory reads still return directory errors. Existing `/proc/self/fd/<n>` `readlink` handling is unchanged.
- FD impact: the new fd entry carries only the directory path and getdents cursor; descriptor allocation, dup/fork, close, pipe EOF/SIGPIPE, and file offset sharing semantics outside this synthetic directory are unchanged.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: moderate-low. `getdents64` snapshots open fd numbers at call time and stores only a cursor; it does not hold references to every fd entry. The RV/LA regression subset protects stable pipe, proc, readlink, and fcntl anchors.
- Maintenance boundary: this implements the fd directory surface needed for generic procfs enumeration. It does not yet provide full Linux semantics for every `/proc/<other-pid>/fd` namespace, permission, or per-fd `readlink` target type.

## Additional mknodat mode errno behavior change on 2026-06-02

File:

- `examples/shell/src/uspace/fd_table.rs`

Behavior:

- `mknod()`/`mknodat()` still create only regular files and FIFOs in the current synthetic filesystem model.
- Character and block device node requests still return `EPERM`, matching the existing no-device-node/no-`CAP_MKNOD` boundary.
- Invalid or nonsensical file-type encodings now return `EINVAL` instead of falling through to `EPERM`; this covers `S_IFMT`, directory, symlink, socket, and unknown type-bit combinations.

POSIX/Linux-visible impact:

- Syscall/flag surface affected: `mknod`/`mknodat` mode file-type validation and errno ordering.
- Errno impact: unsupported privileged device nodes remain `EPERM`; invalid mode type encodings now surface `EINVAL`.
- FD impact: none; successful regular/FIFO creation still uses the existing file creation and metadata recording path.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: low. The change only narrows the pre-create mode validation branch before any filesystem mutation; RV/LA targeted and VFS regression subsets protect adjacent mknod/open/creat/chmod/chown cases.
- Maintenance boundary: this is an errno/order compatibility fix, not a full Linux device-node implementation. Future device-node support must revisit the `EPERM` branch with real capability and special-file semantics.

## Additional fchownat symlink nofollow behavior change on 2026-06-02

File:

- `examples/shell/src/uspace/metadata.rs`

Behavior:

- `fchownat(..., AT_SYMLINK_NOFOLLOW)` now checks whether the final resolved path is a synthetic symlink and, when it is, applies ownership metadata to that symlink path rather than to the target path.
- Synthetic symlink `lstat` now overlays recorded owner/group metadata, so subsequent nofollow stat operations observe prior nofollow chown changes.
- Non-symlink paths keep the existing `fchownat` path-stat flow, and `AT_EMPTY_PATH` fd behavior is unchanged.

POSIX/Linux-visible impact:

- Syscall/flag surface affected: `fchownat` with `AT_SYMLINK_NOFOLLOW`; `lstat`/`newfstatat(..., AT_SYMLINK_NOFOLLOW)` visible uid/gid for synthetic symlinks.
- Errno impact: no new errno branches for successful symlink nofollow ownership changes; existing unsupported flags, empty-path, readonly-mount, and permission checks remain in place.
- FD impact: none for descriptor allocation/dup/close. FD-relative path resolution is reused for the nofollow path.
- Signal/futex/mmap/user-pointer copy impact: none.
- Resource/lifetime risk: low. The change reuses the existing per-process path owner map and only changes which path key is selected for final synthetic symlinks; RV/LA symlink/chown regression protects adjacent stable cases.
- Maintenance boundary: this models synthetic symlink ownership metadata. It does not implement a full Linux inode/symlink permission model or cross-process persistent filesystem metadata.

## Additional busybox applet exec fallback behavior change on 2026-06-02

Files:

- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/runtime_paths.rs`

Behavior:

- When `execve` targets a missing `/bin/<name>` or `/usr/bin/<name>` path and `<name>` is in the existing busybox applet allowlist, the loader falls back to the current libc root's busybox binary.
- `argv[0]` is normalized to the applet basename for the fallback, matching busybox applet dispatch expectations.
- Existing real files at the requested path still take precedence; `/busybox` and `/bin/busybox` compatibility is preserved.

POSIX/Linux-visible impact:

- Syscall/path surface affected: `execve`/`execvp`/`execlp` of standard busybox applet paths under `/bin` and `/usr/bin` when those files are absent from the guest root.
- Errno impact: a missing applet path that can be served by busybox now succeeds instead of returning `ENOENT`; names outside the allowlist and missing busybox binaries still fail as before.
- FD impact: successful exec still runs the existing `FD_CLOEXEC` close path; descriptor allocation/dup/close semantics are unchanged.
- Signal/futex/mmap/user-pointer copy impact: none beyond normal exec image replacement.
- Resource/lifetime risk: moderate-low. This broadens the standard utility exec surface but is constrained by an explicit busybox applet allowlist and by real-file precedence; RV/LA rlimit/exec/wait regression protects adjacent process behavior.
- Maintenance boundary: this is a compatibility shim for the bundled busybox root, not a general PATH resolver or package manager. Future additions should extend the applet allowlist deliberately.
