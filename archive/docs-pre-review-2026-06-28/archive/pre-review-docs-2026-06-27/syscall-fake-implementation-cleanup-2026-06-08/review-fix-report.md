# Syscall fake-implementation cleanup report

Date: 2026-06-08
Scope: follow-up to the syscall/LTP fake-implementation code review.  This report records source-level repairs only; it does not promote, demote, or edit `LTP_STABLE_CASES`.

## Hard boundaries preserved

- No LTP case name, process name, PASS marker, or evaluator-only success path was added to syscall/kernel/user-memory code.
- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` remains unchanged; the live baseline re-counted in this change set is 1000 total / 1000 unique / 0 duplicate entries.
- Unsupported Linux/POSIX-visible behavior must fail explicitly (`EOPNOTSUPP`, `EINVAL`, `EPERM`, `ENODEV`, etc.) rather than returning success without state or backing implementation.
- Synthetic `/proc`, `/dev`, `/etc`, metadata, and musl runtime patching remain compatibility boundaries, not promotion proof.

## Review findings addressed

| Severity | Area | Source-level outcome |
| --- | --- | --- |
| HIGH | Block-filesystem `mount(2)` | `vfat/msdos/fat/ext2/ext3/ext4` no longer aliases `/dev/vd*`/`sd*`/`xvd*` to `/`; after normal pointer/target validation it returns `EOPNOTSUPP` until a real block-device mount path exists. |
| HIGH | `fsync(2)` | Regular files now call the real `axfs` file flush path regardless of open mode; `memfd` is documented as RAM-backed; synthetic/device/socket/pipe/proc-like entries return `EINVAL` instead of generic success. |
| HIGH | `SIOCSIFFLAGS` | The ioctl now validates the ifreq buffer, enforces root-only mutation semantics, and returns `EOPNOTSUPP` for privileged callers because no netdev mutation backend exists. |
| MEDIUM | `log_read_cstr_efault` and `user_trace!` | Empty local trace macros were removed; a single gated `user_trace!` macro now formats trace arguments, and `log_read_cstr_efault` records pid, pointer, fault address, mapped bit, and reason when `USER_TRACE` is enabled. |
| MEDIUM | `sys_syslog` | Open/close/read-clear/clear/console actions now fail explicitly: non-root callers receive `EPERM`, and root callers receive `EOPNOTSUPP` until a real kernel ring-buffer/console-control backend exists. Empty read/read-all operations still expose an empty log with normal user-buffer validation. |
| MEDIUM | `times(2)`/`getrusage(2)` accounting | The arbitrary half-user/half-system split was removed.  The code now exposes real monotonic process lifetime ticks as user time and leaves system time at 0 because the kernel has no per-mode CPU accounting yet. |
| MEDIUM | scheduler policy state | `SCHED_DEADLINE` via `sched_setscheduler`/`sched_setattr` is now rejected with `EOPNOTSUPP` once user structures are read and validated enough to identify the policy, instead of storing deadline parameters with no deadline scheduler backend.  FIFO/RR remain priority-backed through `axtask::set_task_priority`, not strict Linux RT scheduling. |
| LOW | musl runtime byte patches | Existing G008 manifest/gate docs remain the boundary: patches are temporary compatibility bridges and require raw syscall + musl + glibc + RV/LA parser-clean evidence before promotion. |
| LOW | synthetic proc/dev/config and runner modes | Existing G006/G008/G010 guards plus this report keep synthetic capability and selection-mode caveats explicit; they are not treated as stable promotion evidence. |

## Remaining explicit limitations

- There is still no real block-device filesystem attach implementation for `mount(2)`.
- There is still no mutable network-interface control plane for `SIOCSIFFLAGS`.
- There is still no kernel ring-buffer export or console-control backend for `syslog(2)`; read/read-all expose an empty log, while state-changing actions return explicit unsupported errors instead of storing write-only fake state.
- There is still no per-task user-vs-kernel CPU accounting; `tms_stime`/`ru_stime` stay 0 rather than being fabricated.
- FIFO/RR scheduler policy support is limited to stored POSIX state plus backend priority mapping; strict Linux real-time ordering is not claimed.
- The existing `/tmp/ltp-work` bootstrap directory remains a known harness compatibility path and was not added by this change.

## Verification log

Commands run from `/root/oskernel2026-orays`:

- `python3 scripts/test_g012_syscall_review_hotspots.py` -> PASS (`Ran 13 tests`, `OK`).
- `python3 scripts/check_g002_fake_success.py` -> PASS.
- `python3 scripts/check_g003_stat_metadata.py` -> PASS.
- `python3 scripts/check_g004_rlimit_fd.py` -> PASS.
- `python3 scripts/check_g005_runner_parser.py` -> PASS.
- `python3 scripts/check_g006_synthetic_capabilities.py` -> PASS.
- `python3 scripts/check_g007_socket_time_mempolicy.py` -> PASS.
- `python3 scripts/check_g008_musl_patch_stable.py` -> PASS.
- `python3 scripts/check_g009_post_review_semantics.py` -> PASS.
- `python3 scripts/check_g010_real_kernel_semantics.py` -> PASS.
- `python3 scripts/check_g011_empty_shells.py` -> PASS.
- `python3 scripts/check_g012_syscall_review_hotspots.py` -> PASS.
- Targeted hardcoding scan over `examples/shell/src/uspace kernel api ulib` and the new G012 scripts -> no new LTP case/PASS marker hardcoding; only the pre-existing `process_lifecycle.rs` `/tmp/ltp-work` bootstrap path was reported.
- Live `LTP_STABLE_CASES` recount -> `total 1000 unique 1000 duplicates 0`.
- `git diff --check -- <changed source/script/report files>` -> PASS.
- `make A=examples/shell ARCH=riscv64` -> PASS (`exit=0`); this Make target built both `kernel-rv` and `kernel-la`.  Build warnings are pre-existing-style warnings in `vendor/smoltcp`, `axnet`, and unrelated uspace modules (`posix_mq`, `fd_pipe`, `fd_table`, `process_lifecycle`, `resource_sched`, `signal_abi`, `time_abi`).
- `df -h / /root` before and after build -> `/dev/vda2` remained at 59% used / 24G available.

Not run:

- QEMU/evaluator LTP runtime was not run for this cleanup; this change is source/static/build validated only and does not claim new LTP PASS evidence.
- Full `cargo fmt --all -- --check` is not claimed as a passing gate because this repository currently has unrelated formatting differences outside the touched scope; touched Rust files were formatted with targeted `rustfmt --edition 2024` and then checked with `git diff --check`.
