# Repository Map

This repository keeps the upstream ArceOS workspace shape, with OSKernel
2026-specific evaluator integration layered on top.  The paths below are the
first places to inspect for most development work.

## Source layout

| Path | Developer meaning |
| --- | --- |
| `kernel/arch/axhal/` | Architecture and platform HAL.  Boot, trap, page-table, timer, and device-facing code lives here. |
| `kernel/runtime/axruntime/` | Runtime initialization and boot flow.  Changes here can affect all architectures. |
| `kernel/task/axtask/` | Tasking, scheduling, process/thread lifecycle support. |
| `kernel/fs/axfs/` | VFS and filesystem operations used by POSIX-facing syscalls and evaluator tests. |
| `kernel/net/axnet/` | TCP/UDP network stack integration. |
| `api/arceos_posix_api/` | Linux/POSIX syscall boundary, errno mapping, user-memory validation, process/FD/signal/futex/mmap semantics. |
| `api/arceos_api/`, `api/axfeat/` | Public ArceOS APIs and feature composition. |
| `ulib/axstd/`, `ulib/axlibc/` | User libraries and libc-facing support. |
| `examples/shell/` | Shell, user-program launcher, official evaluator group runner, and LTP runner. |
| `configs/platforms/` | Normal local platform configs. |
| `configs/remote-eval/` | Remote-evaluator-specific platform config, currently important for LoongArch submission builds. |
| `scripts/` | Build shims, make fragments, and log/parsing helpers such as `scripts/ltp_summary.py`. |
| `tools/bin/` | Checked-in helper commands preferred before user-installed tools. |
| `cargo-home/`, `vendor/` | Offline/vendor source support for remote-submission builds. |
| `docs/` | Compatibility notes, LTP campaign reports, final gates, and this developer guide. |
| `doc/` | Upstream-style ArceOS platform and build notes. |

## Evaluator integration hot path

For OSKernel work, the common path is:

```text
run-eval.sh
  -> Makefile run-rv / run-la
    -> kernel-rv / kernel-la build targets
      -> examples/shell with APP_FEATURES=auto-run-tests,uspace
        -> api/arceos_posix_api syscall/user-space layer
          -> kernel subsystems such as fs/task/mm/net/hal
```

`examples/shell` is therefore not just an example app.  It stages busybox/helper
resources, starts official test groups, chooses LTP cases, emits wrapper marker
lines, applies per-case timeouts, and cleans up user processes between cases.

## Generated and local-only artifacts

Do not treat these as source unless a task explicitly asks for generated
evidence:

- `kernel-rv`, `kernel-la`
- `sdcard-*.img`, `disk*.img`, `*.qcow2`
- `output*.md`, raw `*.log` files
- `.axconfig.toml`, `build/`, `target/`

When preserving evaluator evidence, prefer dated folders under `docs/` and keep
raw logs paired with `scripts/ltp_summary.py` output.

## Subsystem ownership hints

- Syscall return values, errno, ABI structs, and raw user pointers usually belong
  in `api/arceos_posix_api/`.
- Test selection, helper staging, wrapper output, and LTP case-list behavior
  usually belong in `examples/shell/src/cmd.rs`.
- VFS/FD behavior can cross `api/arceos_posix_api/`, `kernel/fs/axfs/`, and
  shell-side helper/resource staging.
- Process, wait, signal, and scheduler behavior can cross
  `api/arceos_posix_api/`, `kernel/task/axtask/`, and runtime/architecture trap
  code.
- LoongArch boot and remote/local address-map behavior can cross
  `kernel/arch/axhal/`, platform configs, and vendored platform/boot crates.
