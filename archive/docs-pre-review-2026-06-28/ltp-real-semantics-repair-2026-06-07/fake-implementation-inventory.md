# Fake implementation inventory (G001 baseline)

Date: 2026-06-07
Scope: read-only inventory for `G001-g001-phase-0-quarantine`; no fixes are applied here.

Severity key:

- **CRITICAL** — unimplemented or case-specific behavior can produce success-like results and must block promotion until fixed or quarantined.
- **HIGH** — behavior may be partial, synthetic, or weakly observable; promotion requires explicit capability/evidence linkage.
- **MEDIUM** — audit/manifest risk that can confuse evidence but is not by itself a syscall success path.

## Summary table

| ID | Severity | Domain | Evidence | Promotion risk | G001 status |
| --- | --- | --- | --- | --- | --- |
| FS-STAT-001 | CRITICAL | stat/lstat metadata | `api/arceos_posix_api/src/imp/fs.rs:44-59`, `api/arceos_posix_api/src/imp/fs.rs:196-206` | `File::stat` hard-codes inode/link/uid/gid/blksize fields; `sys_lstat` writes default `stat` and returns `Ok(0)`. | Quarantine until stat/lstat metadata has observable semantics and regression evidence. |
| FD-FCNTL-001 | CRITICAL | fd/fcntl | `api/arceos_posix_api/src/imp/fd_ops.rs:109-137` | Unsupported `fcntl` commands warn and return `Ok(0)`; `F_DUPFD_CLOEXEC` notes TODO for flags. | Quarantine unsupported fcntl success; require errno or real flag behavior. |
| RES-RLIMIT-001 | CRITICAL | rlimit | `api/arceos_posix_api/src/imp/resources.rs:31-68` | `getrlimit` can accept a resource and return success without writing a limit; `setrlimit` says resources are unsupported and returns `Ok(0)`. | Quarantine rlimit promotions until set/get plus enforcement are proved. |
| SYS-SYSCONF-001 | CRITICAL | sysconf | `api/arceos_posix_api/src/imp/sys.rs:10-41` | Unknown `sysconf` names return `0`, which can be read as a supported configuration value. | Require Linux-compatible unsupported handling or documented value semantics. |
| LIBC-UNISTD-001 | CRITICAL | libc filesystem/process wrappers | `ulib/axlibc/c/unistd.c:9-34`, `ulib/axlibc/c/unistd.c:57-124`, `ulib/axlibc/c/unistd.c:162-166` | Multiple TODO wrappers call `unimplemented()` then return success-like `0`, including identity/session/tty, `access`, `readlink`, `unlink`, `rmdir`, `fsync`, `fchown`, `ftruncate`, `chdir`, `truncate`, and `execve`. | Quarantine all case evidence relying on these wrappers until failure errno or real behavior is implemented. |
| LIBC-SIGNAL-001 | CRITICAL | libc signals | `ulib/axlibc/c/signal.c:6-15`, `ulib/axlibc/c/signal.c:36-85` | `sigaction_helper` discards action and returns success; `kill`, `raise`, `pthread_sigmask`, and `pthread_kill` are TODO but return `0`. | Require real signal state/delivery or honest unsupported errors before promotion. |
| LIBC-MMAP-001 | CRITICAL | libc memory mapping | `ulib/axlibc/c/mmap.c:12-38` | `munmap`, `mprotect`, and `madvise` are TODO yet return `0`. | Quarantine mmap-related success unless memory-map state changes are observable. |
| LIBC-PTHREAD-001 | CRITICAL | pthread/cancel/cond/mutex | `ulib/axlibc/c/pthread.c:9-78` | pthread cancel, trylock, name, cond signal/wait/broadcast paths are TODO/unimplemented but return success-like values. | Require pthread-semantics errors or real synchronization behavior. |
| LIBC-MISC-001 | CRITICAL | libc misc wrappers | `ulib/axlibc/c/fcntl.c:42-54`, `ulib/axlibc/c/ioctl.c:4-8`, `ulib/axlibc/c/select.c:9-14` | `posix_fadvise`, `sync_file_range`, `ioctl`, and `pselect` can return `0` despite TODO/ignored underlying results. | Quarantine wrapper-level PASS until each path has real semantics or honest errno. |
| RUNNER-SEL-001 | CRITICAL | runner selection | `Makefile:84-95`, `Makefile:313-328`, `examples/shell/src/cmd.rs:1238-1299` | Default `REMOTE_LTP_CASES ?= stable-plus-blacklist`, blacklist modes, and `/ltp_cases.txt`/env overrides can mix stable with all-minus-blacklist extras. | Reports must state selection mode, blacklist source/count, skipped count, and override status; these modes are not promotion proof. |
| RUNNER-CASE-001 | CRITICAL | case-name specialization | `examples/shell/src/cmd.rs:1922-1947` | `chdir01` receives `LTP_FORCE_SINGLE_FS_TYPE=tmpfs` and `LTP_DEV_FS_TYPE=tmpfs`, changing the case environment for a named test. | Quarantine case-specific runner behavior until replaced by a general real mechanism or honest failure/TCONF. |
| SYNTH-PROC-001 | HIGH | synthetic `/proc` and config | `examples/shell/src/uspace/synthetic_fs.rs:835-844`, `examples/shell/src/uspace/synthetic_fs.rs:944-969` | Synthetic cmdline includes `ltp.oskernel2026=1`; synthetic kernel config advertises probe-oriented entries such as `CONFIG_EVENTFD=y`. | Treat as compatibility façade; promotion must link advertised capability to real implementation/test evidence. |
| SYNTH-DEV-001 | HIGH | synthetic block devices/config open paths | `examples/shell/src/uspace/fd_table.rs:8484-8496`, `examples/shell/src/uspace/fd_table.rs:8568-8576`, `examples/shell/src/uspace/fd_table.rs:8767-8784` | Kernel config and synthetic block device paths can be opened as entries even when full block-device semantics are not established. | Require declared metadata, size/ioctl, read/write, and mount semantics or honest error/TCONF. |
| FD-FCNTL-002 | HIGH | fd fallback | `examples/shell/src/uspace/fd_table.rs:5918-6062` | Some unhandled local-socket/socket/FD `fcntl` paths fall through to `Ok(0)`, and some `F_GETFL`/`F_SETFL` fallback cases use default success. | Require per-entry supported-command matrix and regression evidence. |
| SOCK-RECVMSG-001 | HIGH | socket recvmsg | `examples/shell/src/uspace/fd_socket.rs:1981-1989` | Success path leaves `msg_name` buffer untouched and comment notes basic LTP only asserts errno/return value. | Require peer-address/name-buffer behavior proof before promoting recvmsg semantics. |
| TIME-TIMER-001 | HIGH | timers/itimers | `examples/shell/src/uspace/time_abi.rs:562-568`, `examples/shell/src/uspace/time_abi.rs:916-925` | `SIGEV_THREAD` is accepted as non-delivering; virtual/prof itimers are tracked for get/set state but only real timer delivers `SIGALRM`. | Distinguish create/delete smoke from delivery semantics; require behavior proof or unsupported errno. |
| MEM-MPOLICY-001 | HIGH | NUMA/mempolicy | `examples/shell/src/uspace/memory_policy.rs:61-92` | `mbind` and `set_mempolicy` validate nodemask but do not implement policy effect; `get_mempolicy` writes a default policy. | Promotion requires observable policy behavior or Linux-compatible unsupported result. |
| MUSL-PATCH-001 | HIGH | runtime libc patching | `examples/shell/src/uspace/program_loader.rs:399-528`, `examples/shell/src/uspace/program_loader.rs:591-615`, `examples/shell/src/uspace/program_loader.rs:744-810`, `examples/shell/src/uspace/program_loader.rs:965-1135` | Loader patches musl wrappers at runtime for syscall behavior (`brk`, `sbrk`, sched/nice/gethostname/readlink/readlinkat paths). | Temporary shim only; require patch manifest and raw syscall + musl + glibc cross-checks before promotion. |
| PARSER-TRUTH-001 | HIGH | parser/reporting | `scripts/ltp_summary.py:1-10`, `scripts/ltp_summary.py:25-40`, `scripts/ltp_summary.py:110-121` | Runner markers are audit signals; numeric status and internal LTP quality signals decide PASS/FAIL/promotion blocking. | Promotion docs must cite parser-backed evidence and keep all caveats visible. |

## Domain notes

### 1. Fake success is a blocker, not a convenience

Any `return 0` / `Ok(0)` from an unimplemented Linux/POSIX-visible interface is a promotion blocker.  Success must mean one of the following is true:

- the requested state change occurred and can be read back;
- the behavior is observable through a follow-up Linux/POSIX operation;
- the interface is explicitly documented as a compatibility façade and is not used to claim real kernel support.

Otherwise the path must return an honest Linux-compatible error, usually `ENOSYS`, `EINVAL`, `ENOTSUP`, `ENOPROTOOPT`, `EPERM`, or the domain-specific errno required by the relevant API.

### 2. Synthetic files and devices are compatibility façades

Synthetic `/proc`, `/dev`, `/etc`, config, metadata, and block devices may exist to let the evaluator run.  They must not be presented as proof of real kernel support unless a capability map and tests connect each advertised feature to implemented behavior.

### 3. Runner behavior must not mutate kernel/user ABI truth

The runner may select cases, isolate test directories, and emit audit manifests.  It must not make a named LTP case pass by changing the kernel/user ABI surface for that case.  `chdir01`-specific env injection is therefore quarantined pending a general test-device/mount mechanism or honest TCONF/FAIL.

### 4. Runtime musl patching needs a manifest

Runtime byte patches may be a temporary compatibility bridge, but they are not a stable-promotion foundation.  Each patch needs a manifest with target libc, symbol, offset/hash, reason, raw syscall evidence, musl evidence, and glibc cross-check evidence.

## G001 status

This inventory is a baseline and quarantine input only.  No item above is fixed by this document, no source file was edited, and no case should be promoted because it appears in this inventory.
