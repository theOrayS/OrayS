# Agents Guidelines for ArceOS / OSKernel 2026

This file is the local working contract for AI agents editing this repository.
It is intentionally stricter and more operational than the public `README.md`.
When repository docs and this file disagree, follow the repository's actual code,
Makefile, scripts, and CI configuration.

This tree is an ArceOS-based experimental modular OS/unikernel written in Rust,
with local OSKernel 2026 evaluator support layered on top. The checkout may
contain generated kernels, large disk images, run logs, and in-progress user
changes. Work incrementally and avoid broad cleanup unless the task explicitly
asks for it.

## Repository Layout

| Path | Purpose |
| ---- | ------- |
| `kernel/` | Core runtime and subsystems: arch/hal, config, diagnostics/logging, drivers, FS, memory, namespace, net, runtime, SMP/IPI, sync, task. |
| `api/` | Public ArceOS APIs and POSIX-facing APIs. `api/arceos_posix_api` is the user-space/Linux-compat boundary. |
| `ulib/` | User-facing libraries: `axstd` for Rust apps and `axlibc` for C/POSIX-facing apps. |
| `examples/` | Rust and C example applications used by local builds and CI (`helloworld`, `httpclient`, `httpserver`, `shell`, plus C variants). |
| `configs/` | Default, dummy, and custom platform configuration files. |
| `scripts/` | Make helpers, build/QEMU glue, linker wrapper scripts, and network helper scripts. |
| `tools/` | Board-specific and utility tools. |
| `doc/` | Upstream-style build/platform documentation and figures. |
| `docs/` | Local compatibility/progress notes for this fork. |
| `vendor/` | Vendored/patched third-party crates (`axcpu`, `axfs_ramfs`, `rust-fatfs`, `smoltcp`). |
| `.github/workflows/` | CI definitions for build, docs, and app/unit tests. |
| `build/`, `target/` | Generated artifacts; avoid manual edits. |

Important generated or local-only root files can include `kernel-rv`, `kernel-la`,
`sdcard-*.img`, `disk*.img`, `output*.md`, `*.log`, `.axconfig.toml`, and the
`build/kernels/` tree. Do not edit or commit generated artifacts unless the task
is explicitly about those artifacts. `run-eval` is a local symlink to
`run-eval.sh` when present.

## Disk Space and Commit Hygiene

- Check disk space at the start and end of long-running tasks, and before/after
  commands that can create large artifacts such as full evaluator runs, `make
  all`, QEMU logs, Docker builds, vendoring, or broad test sweeps. Use at least
  `df -h / /root` and, when Codex state/cache growth is relevant, `du -sh
  /root/.codex`.
- If `/` is near full (roughly 85%+ used or less than 10 GiB free), pause new
  heavy builds/tests and clean low-value generated content first: stale build
  outputs, old raw logs, temporary files, abandoned worktree artifacts, and
  disposable `.codex`/OMX caches or logs. Preserve user-supplied evidence,
  memory files, active `.omx` state, source files, and anything needed to
  reproduce current validation unless the user explicitly says otherwise.
- When cleaning `.codex`, prefer old rollout summaries, transient logs, caches,
  and inactive session artifacts. Do not delete installed skills, prompts,
  agents, memory entries, credentials, or active session state unless the user
  explicitly requests that exact cleanup.
- After completing and verifying a task that changes tracked source,
  documentation, or durable project state, create a Git commit automatically
  unless the user explicitly says not to, validation is still failing, or the
  worktree contains unrelated changes that cannot be safely separated. Stage
  only agent-owned changes; leave user-provided logs, generated kernels, disk
  images, and unrelated dirty files uncommitted by default.
- Automatic commits must follow the Lore Commit Protocol below and must report
  the commit SHA in the final response. If a task cannot be committed safely,
  report the exact blocker and the files left uncommitted.

## Build and Run

Run commands from the repository root (`/root/oskernel2026-orays` in this
container) unless a task clearly spans a sibling checkout.

```bash
# Default target: build remote-submission kernels (kernel-rv and kernel-la).
# The LoongArch kernel built by `make all` uses the remote evaluator address map.
make

# Build a Rust example for one architecture
make A=examples/helloworld ARCH=x86_64

# Build and run an example in QEMU
make A=examples/shell ARCH=riscv64 run

# Build local evaluator kernels used by this fork
make kernel-rv
make kernel-la

# Run testsuite-backed evaluator boots when the sdcard images/QEMU are available
./run-eval.sh rv
./run-eval.sh la
# Equivalent lower-level targets:
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64

# Lint, format, docs, and unit tests
make clippy
make fmt
make fmt_c
make doc_check_missing
make unittest_no_fail_fast

# Docker helper path
make docker-image
make docker
```

### Build Notes

- `ARCH` must be one of `x86_64`, `riscv64`, `aarch64`, or `loongarch64`.
- The Makefile accepts `A`/`APP`, `FEATURES`, `APP_FEATURES`, `LOG`, `SMP`,
  `MODE`, `MYPLAT`, `PLAT_CONFIG`, `TARGET_DIR`, and QEMU options such as
  `BLK`, `NET`, `GRAPHIC`, `BUS`, `MEM`, `DISK_IMG`, `NET_DEV`, and `ACCEL`.
- Runtime QEMU device flags such as `NET=y`, `BLK=y`, and `GRAPHIC=y` affect
  the QEMU invocation; they are not a substitute for compile-time feature flags.
- Running apps or app tests requires QEMU. The evaluator path expects
  `qemu-system-riscv64` and/or `qemu-system-loongarch64` plus matching sdcard
  images.
- Building C examples requires the appropriate musl cross toolchains and
  `libclang`/`clang` tooling described in `README.md`.
- `make testsuite-sdcard` expects the sibling testsuite checkout configured by
  `TESTSUITE_DIR` (default `../testsuits-for-oskernel`).

## Local and Remote Evaluation Modes

- This checkout, `/root/oskernel2026-orays`, is the single maintained working
  branch for both local QEMU validation and remote-evaluator submission builds.
  Do not maintain a separate remote branch as the delivery target unless a newer
  user request explicitly reintroduces that workflow.
- Local validation remains `./run-eval.sh` for RISC-V and `./run-eval.sh la` for
  LoongArch. These targets use the local QEMU command lines and the package
  default LoongArch platform address map.
- Remote submission validation is represented by `make all`, which must generate
  root-level ELF-format `kernel-rv` and `kernel-la`. The `kernel-la` produced by
  `make all` uses `configs/remote-eval/axplat-loongarch64-qemu-virt.toml` to
  match the remote evaluator's LoongArch address map; do not use that remote
  config for local `run-la` unless specifically testing remote-submission build
  behavior.
- LoongArch boot page-table setup must derive the L0 slot from
  `KERNEL_BASE_VADDR`, not assume high-half index `0`. Local QEMU currently uses
  `0xffff_0000_8000_0000`, while the remote evaluator uses
  `0xffff_8000_8000_0000`; hardcoding `BOOT_PT_L0[0]` can boot locally but loop
  on remote instruction-fetch faults.
- Treat the remote evaluator as network-unreliable/offline. Submission builds
  must not depend on `cargo install` or downloading crates during `make all`.
  Keep repo-local build helpers under `tools/bin/`, platform config fallbacks
  under `configs/platforms/`, and the non-hidden Cargo home `cargo-home/` plus
  `vendor/cargo-vendor.tar.gz` source archive in sync when dependency closure changes; `scripts/ensure-cargo-vendor.sh` restores the working `vendor/cargo/` directory during builds.
- The historical `refactor/moss_kernel_like_remote` branch and sibling checkout
  may be used only as read-only references for remote-evaluator behavior. Do not
  modify that branch or sync source into it for normal local-branch tasks.
- Keep local-only and remote-submission address mapping rules explicitly named in
  code, docs, and reports. Do not hide real evaluator failures with fake PASS,
  case-name hardcoding, or converting real failures into SKIP/TCONF.

## Toolchain

- Rust toolchain is pinned in `rust-toolchain.toml` to `nightly-2025-05-20`.
- Rust edition is 2024.
- Required Rust components: `rust-src`, `llvm-tools`, `rustfmt`, and `clippy`.
- Configured Rust targets:
  - `x86_64-unknown-none`
  - `riscv64gc-unknown-none-elf`
  - `aarch64-unknown-none-softfloat`
  - `loongarch64-unknown-none-softfloat`
- Build helpers used by the Makefile include `cargo-binutils`/`rust-objcopy`,
  `axconfig-gen`, and `cargo-axplat`.
- For remote/offline submission builds, prefer the checked-in helper shims
  `tools/bin/cargo-axplat`, `tools/bin/axconfig-gen`, and
  `tools/bin/rust-objcopy` before any user-installed tools. If helper behavior
  is extended, validate both `make all` and an offline build with
  `CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH`.
- C formatting follows the repository `.clang-format`; there is no repo-local
  `rustfmt.toml`, so use the pinned toolchain's formatter.

## Hard Constraints

### General

- Work from the repository root, not the outer workspace, unless the task clearly
  spans sibling directories.
- Assume the Git worktree is dirty. Never revert unrelated user changes.
- Prefer minimal, subsystem-local patches. Do not refactor across `kernel/`,
  `api/`, `ulib/`, and `examples/` unless the task requires it.
- Do not hand-edit generated outputs in `build/`, `target/`, root-level kernels,
  sdcard/disk images, or logs.
- Avoid pseudo/fake implementations and hardcoded behavior. Prefer real
  implementations wired through existing configuration, feature flags, platform
  abstractions, or capability checks. If a capability is intentionally
  unsupported, fail explicitly with a clear error or documented rationale instead
  of stubbing success.
- Preserve platform and feature structure. This repo is intentionally built
  across four architectures, multiple platform configs, and many `#[cfg(...)]` /
  feature combinations.
- Do not perform repository-wide search/replace, mechanical renames, import
  normalization, or bulk formatting unless the task explicitly requires it.
- Avoid modifying `vendor/` unless the change is explicitly about a vendored
  crate, the remote/offline Cargo source archive `vendor/cargo-vendor.tar.gz`,
  or a local patch is necessary and documented.

### Rust

- Follow the style already present in the touched file; do not import style rules
  from other projects unless ArceOS already does so.
- `unsafe` already exists in low-level modules, runtime code, drivers, and the
  POSIX boundary. Do not impose blanket bans that the repository itself does not
  follow.
- When adding new `unsafe`, keep it narrow and explain the invariant with a
  `// SAFETY:` comment when the reason is not trivial from nearby code.
- Do not collapse architecture-specific code just to simplify control flow.
  Preserve `#[cfg(target_arch = ...)]` and feature-gated behavior.
- Avoid `unwrap()` and `expect()` on fallible runtime, syscall, filesystem,
  networking, or user-input paths unless the invariant is immediate and locally
  proven.
- Prefer small helpers and early returns over deeper nesting, especially in large
  integration files.

### POSIX and User-Space Boundary

- Treat raw user pointers, lengths, and ABI-visible structures as untrusted input.
- Validate before turning raw pointers into slices, strings, or structs.
- Keep copy-in/copy-out behavior explicit. Do not silently widen trust
  boundaries.
- Preserve Linux/POSIX-visible behavior when changing syscalls, errno mapping,
  struct layouts, file descriptor behavior, signals, futexes, networking, or
  process/task semantics.
- In `api/arceos_posix_api/src/uspace.rs`, avoid broad rewrites. It is a large
  integration file covering ELF loading, memory layout, FDs, signals, futexes,
  and syscall handling.
- If a change modifies syscall behavior, errno mapping, ABI-visible struct
  layout, user-visible return values, or other POSIX/Linux-observable semantics,
  the final summary must explicitly list the visible behavior changes. If there
  is no intended visible behavior change, say so clearly.

### Logging and Output

- In `kernel/`, `api/`, and `ulib/`, prefer existing logging facilities such as
  `axlog` macros over ad-hoc printing.
- In `examples/` and evaluator scripts, stdout/stderr-oriented behavior is
  acceptable when it is part of the visible interface.

## Validation Rules

Pick the smallest check set that proves the change:

- For local-branch code tasks, do not claim the target is delivered until both
  `./run-eval.sh` (the default RV path) and `./run-eval.sh la` have been run
  successfully from `/root/oskernel2026-orays`, unless a required host tool,
  image, or external dependency is unavailable. If either command cannot be run
  or fails, report the exact blocker/output instead of substituting build-only or
  smoke-only evidence.
- Documentation-only changes: inspect the rendered Markdown structure manually or
  run a lightweight text check; no kernel build is required unless examples or
  commands changed in a way that needs proof.
- Formatting-only or broad Rust edits: run `cargo fmt --all -- --check` after
  formatting, or `make fmt` if intentionally applying formatting.
- C formatting changes: run `make fmt_c` or the targeted `clang-format` command.
- Library/module changes: run `make clippy` or `make clippy ARCH=<arch>` for the
  affected target when architecture-specific code changed.
- Example changes: build the touched example for the affected architecture, e.g.
  `make A=examples/helloworld ARCH=riscv64`.
- `api/arceos_posix_api`, `ulib/axlibc`, or user-space behavior changes: at
  minimum build `make A=examples/shell ARCH=riscv64` or the closest relevant
  kernel target; add QEMU/evaluator validation when behavior is observable only
  at runtime.
- Unit-testable code: run `make unittest_no_fail_fast`.
- Evaluator-kernel changes: build `make kernel-rv` and/or `make kernel-la`; run
  `./run-eval.sh rv` / `./run-eval.sh la` when QEMU and sdcard images are
  available and runtime behavior is in scope.

For changes spanning tightly coupled boot, trap, scheduler, or user-task flow
code — especially across `kernel/runtime/axruntime`, `kernel/arch/axhal`,
`kernel/task/axtask`, and `api/arceos_posix_api/src/uspace.rs` — prefer staged
validation:

1. first run the smallest relevant build-only validation;
2. then run behavior/QEMU/evaluator validation after the build succeeds.

If QEMU, Docker, external testsuite checkouts, sdcard images, or cross toolchains
are unavailable, state exactly which checks could not be run instead of claiming
full verification.

## CI Facts

Current CI verifies at least the following:

- `cargo fmt --all -- --check` on the x86_64 clippy lane.
- `make clippy` and `make clippy ARCH=<arch>` across `x86_64`, `riscv64`,
  `aarch64`, and `loongarch64`.
- Rust example builds for `examples/helloworld`, `examples/httpclient`,
  `examples/httpserver`, and `examples/shell` on all four supported arches.
- C example builds for `examples/helloworld-c`, `examples/httpclient-c`, and
  `examples/httpserver-c` on all four supported arches after musl setup.
- Additional platform/config builds for `helloworld-myplat`, x86_64 custom
  config, Raspi4, BSTA1000B, Phytium Pi, and loongarch64/riscv64 QEMU virt
  paths.
- `make doc_check_missing` on Ubuntu and macOS doc lanes.
- `make unittest_no_fail_fast`.
- QEMU-backed app tests through `arceos-apps` at the revision pinned in
  `.github/workflows/test.yml`.

CI runs both the pinned nightly (`nightly-2025-05-20`) and a moving `nightly` in
several build lanes; moving-nightly failures may be marked `continue-on-error`,
but pinned-toolchain failures should be treated as regressions.

Any change that only works on one local architecture but breaks other configured
targets, feature combinations, or CI entry points should be treated as a
regression.

## Subsystem Notes

- `examples/shell` is not just a demo shell; it is also a practical integration
  point for testsuite and user-space flows in this tree.
- `api/arceos_posix_api` is ABI-sensitive. Avoid casual renames, layout changes,
  or behavior changes that leak through libc/POSIX-facing APIs.
- `kernel/runtime/axruntime`, `kernel/arch/axhal`, `kernel/task/axtask`, and
  `api/arceos_posix_api/src/uspace.rs` are tightly coupled in boot, trap,
  scheduler, and user-task flows. Cross-cutting changes there need extra care.
- `kernel-rv` is wrapped from the riscv64 binary through
  `scripts/make/riscv64-kernel-wrap.lds`; `kernel-la` is copied from the
  loongarch64 ELF path. Keep this Makefile behavior in mind when debugging
  evaluator boot differences.
- Local notes under `docs/` document compatibility work such as LTP FD/socket
  progress and network loopback behavior; update them when changing the behavior
  they describe.

## LTP Documentation Naming

- For LTP score-improvement campaigns, save durable artifacts under
  `docs/ltp-score-improvement-YYYY-MM-DD-phase-x/`.
- Use the local calendar date for the day the document is created. The first
  campaign/document set of that day uses `phase-a`; later same-day sets continue
  with `phase-b`, `phase-c`, and so on.
- Do not create future-dated LTP documentation directories. Keep historical
  evidence in its original date/phase directory and reference it from new plans
  instead of renaming old evidence forward.
- When a prompt or plan is moved to a new date/phase directory, update all
  self-references, OMX brief paths, and follow-up prompt text to the same
  date/phase naming.

## Change Summary Requirements

When reporting completed work, include:

- files changed;
- intent of each change;
- validation commands actually run;
- checks that could not be run, if any;
- for evaluator-mode changes, whether local `./run-eval.sh` and
  `./run-eval.sh la` passed, and whether `make all` still builds the
  remote-submission `kernel-rv`/`kernel-la` outputs;
- any user-visible behavior change;
- any syscall / errno / ABI-visible change, or an explicit statement that there
  was no intended visible ABI/POSIX behavior change.

Do not claim full verification unless the relevant checks were actually run.
