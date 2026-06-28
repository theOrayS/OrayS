# G006 self-check WATCH resolution report

Date: 2026-06-19
Base commit: `bbf13542` (`Expose self-check violations instead of hiding them`)
Branch: `fix/self-check-compliance-20260618`
Scope: resolve the final review WATCH items recorded for G005 without weakening `self-check.md` constraints.

## Result summary

G006 addresses two implementation WATCH items and records explicit provenance for the two non-source WATCH items:

1. **Raw syscall copy-in/copy-out boundary:** resolved for the reviewed syscall surfaces by centralizing scalar/slice/buffer user-pointer access in `api/arceos_posix_api/src/utils.rs`, including a follow-up conversion of `epoll_ctl` event copy-in and `getaddrinfo` hints copy-in.
2. **pthread mutex attr/type model:** resolved from "trylock is ENOSYS / mutex type ignored" to a minimal real model covering normal, recursive, and errorcheck mutex types, plus attr get/set wrappers.
3. **Dirty worktree provenance:** documented as pre-existing/unrelated and intentionally not staged by this lane.
4. **Live QEMU/remote evaluator evidence:** still not run in this G006 source-compliance lane; this remains a visible `Not-tested` boundary, not hidden score evidence.

No testsuite/evaluator bypasses were added. No stable LTP case name/path/process-name/runner-output special cases were introduced.

## Implementation changes

### 1. Shared user-pointer boundary

`api/arceos_posix_api/src/utils.rs` now defines the shared syscall boundary helpers:

- `read_user_value<T: Copy>` and `write_user_value<T>` for scalar ABI values (`utils.rs:34-58`).
- `readable_user_buffer` / `writable_user_buffer` for byte buffers (`utils.rs:60-94`).
- `readable_user_slice` / `writable_user_slice` for typed arrays (`utils.rs:69-112`).

Converted syscall surfaces include:

- `read`, `write`, `writev` (`api/arceos_posix_api/src/imp/io.rs`).
- `stat`, `fstat`, `lstat`, `getcwd` (`api/arceos_posix_api/src/imp/fs.rs`).
- `select` fd-set/timeval copy-in/out (`api/arceos_posix_api/src/imp/io_mpx/select.rs`).
- `epoll_ctl` event copy-in and `epoll_wait` event array output (`api/arceos_posix_api/src/imp/io_mpx/epoll.rs`).
- socket address, socklen, `getaddrinfo` hints/result, send/recv buffers (`api/arceos_posix_api/src/imp/net.rs`).
- `getrlimit` output and `setrlimit` input (`api/arceos_posix_api/src/imp/resources.rs`).
- `clock_gettime` / `nanosleep` timespec input/output (`api/arceos_posix_api/src/imp/time.rs`).
- pthread creation/join output pointers (`api/arceos_posix_api/src/imp/pthread/mod.rs`).

Remaining raw primitives from the scan are intentionally constrained:

- `utils.rs` owns `read_unaligned`, `write_unaligned`, `from_raw_parts`, and `from_raw_parts_mut` as the central boundary.
- `net.rs` uses `addr_of_mut!` to wire a kernel-owned `getaddrinfo` result buffer before publishing the head pointer, and `Vec::from_raw_parts` only in `freeaddrinfo` to reclaim that same kernel-owned allocation.
- `pthread/mutex.rs::from_user` still obtains an in-place reference to a user-provided `pthread_mutex_t`; this is required because POSIX mutex lock state lives in the ABI object. It is no longer used to hide unsupported paths: null is rejected and unsupported attr bits return real errors.

`scripts/check_g013_user_copy_boundary.py` and `scripts/test_g013_user_copy_boundary.py` now guard this boundary against regression: syscall implementation files must not reintroduce raw memory copy/slice primitives, `core::ptr`/`ptr` read/write/copy calls, common raw pointer method writes, or ordinary same-line/multiline unsafe user-pointer derefs outside the documented helper/ownership exceptions.

### 2. pthread mutex attr/type model

`api/arceos_posix_api/src/imp/pthread/mutex.rs` now stores explicit mutex state in the ABI-sized object:

- `owner`, `lock_count`, `kind`, and reserved words (`mutex.rs:17-23`).
- `from_attr` reads `pthread_mutexattr_t`, supports low-bit normal/recursive/errorcheck type values, and returns `EOPNOTSUPP` for unsupported extra attr bits (`mutex.rs:39-53`).
- `lock` handles first acquisition, recursive acquisition, and self-lock detection without panic (`mutex.rs:70-95`).
- `try_lock` returns real success/`EBUSY`/error behavior instead of `ENOSYS` (`mutex.rs:97-114`).
- `unlock` enforces ownership and decrements recursive count (`mutex.rs:116-143`).

`ulib/axlibc` now exposes and uses the model:

- pthread wrappers return POSIX pthread-style positive error numbers via `pthread_ret` instead of syscall-style `-1`/`errno` (`ulib/axlibc/src/pthread.rs:5-73`).
- `pthread_mutex_trylock` calls the Rust syscall wrapper (`ulib/axlibc/c/pthread.c:35-38`).
- `pthread_mutexattr_init/destroy/gettype/settype` are implemented with explicit type validation (`ulib/axlibc/c/pthread.c:40-76`).
- mutex type constants/prototypes are declared in `ulib/axlibc/include/pthread.h:24-27,69-76`.

Behavior note: `PTHREAD_MUTEX_NORMAL` self-lock currently returns `EDEADLK` rather than silently hanging. This is a conservative, visible failure mode for this kernel model and avoids hidden deadlocks during compliance review; it is documented as a semantic tradeoff rather than claimed as full POSIX completeness.

### 3. Dirty worktree provenance

The following tracked paths are dirty but unrelated to G006 and were not staged by this lane:

- Deleted docs under `docs/ltp-long-term-collaboration-2026-05-28/`.
- Deleted `docs/ltp-score-improvement-2026-05-28-phase-a/three-person-task-allocation-stable460-to-470.md`.
- Modified `examples/shell/src/uspace/memory_map.rs`.
- Modified `examples/shell/src/uspace/task_context.rs`.
- Deleted `output_la.md` and `output_rv.md`.

This lane will stage only the implementation files above, the G013 guard/test scripts,
this report, and G006-owned raw evidence artifacts.

## Validation evidence

Fresh validation log: `docs/self-check-compliance-review-2026-06-19/raw/g006-fresh-validation.log`

Passed commands in that log:

- Current self-check guard scripts present in the repo: `check_g002` through `check_g012`.
- `python3 scripts/check_g013_user_copy_boundary.py` — user-copy boundary guard PASS.
- `python3 -m unittest discover -s scripts -p 'test_g00*.py'` — 75 tests OK.
- `python3 scripts/test_g013_user_copy_boundary.py` — 8 tests OK.
- `python3 scripts/test_ltp_summary.py` — 12 tests OK.
- `rustfmt +nightly-2025-05-20 --check --config skip_children=true` on modified Rust files.
- `gcc -fsyntax-only -DAX_CONFIG_MULTITASK -Iulib/axlibc/include -Iulib/axlibc/include/bits ulib/axlibc/c/pthread.c`.
- `cargo check -p arceos_posix_api --offline`.
- `cargo check -p arceos_posix_api --offline --features 'fd fs net pipe select epoll multitask uspace'`.
- `cargo check -p axlibc --offline`.
- `cargo check -p axlibc --offline --features 'fd fs net pipe select epoll multitask alloc'`.
- `git diff --check` on agent-owned paths.

Validation warnings are pre-existing/vendor warnings from `smoltcp`, `ulib/axlibc/src/net.rs`, `strftime.rs`, and `mktime.rs`; they are not introduced by the G006 changed files and are left visible in the log.

## Not tested / remaining risks

- No live QEMU LTP, non-LTP, or official remote evaluator run was performed in this G006 lane.
- The new helpers and G013 guard centralize the current single-address-space pointer model but do not add hardware-backed user-fault recovery or MMU copy isolation.
- pthread condition variables/cancellation remain honest `ENOSYS`/unimplemented paths where already exposed; G006 only changes mutex attr/type/trylock and pthread return-code mapping.
- `PTHREAD_MUTEX_NORMAL` self-lock behavior is intentionally fail-visible (`EDEADLK`) rather than a potentially indefinite deadlock.

## Self-check compliance conclusion for G006

This patch removes or narrows the two source-level WATCH items without adding score-specific shortcuts. The remaining QEMU/remote gap and unrelated worktree dirt are explicit delivery boundaries, not hidden evidence.

## Independent final review gate

Fresh post-fix review lanes were run after the G013 guard was hardened against `core::ptr`/`ptr` writes, raw pointer method writes, and multiline unsafe dereferences:

- `code-reviewer` agent `019edd95-e5b3-7860-ada3-28e504ac05a4` returned **APPROVE**. It re-ran `py_compile`, `scripts/check_g013_user_copy_boundary.py`, and `scripts/test_g013_user_copy_boundary.py`; it also reproduced that the previous blocker patterns now fail the guard.
- `architect` agent `019edd95-e9ce-7651-8fa7-7c7c29244283` returned **CLEAR** for the source-compliance gate. It explicitly kept live QEMU/remote and MMU-backed user-fault isolation as `Not-tested`, but did not treat those as blockers for this source-compliance lane.

Final G006 source-compliance gate result: **CLEAR / APPROVE**. Runtime evaluator and hardware isolation work remain separate follow-up scope, not hidden success evidence.
