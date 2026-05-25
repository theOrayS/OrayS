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

Key paths: `kernel/` for runtime/subsystems, `api/arceos_posix_api` for the
Linux/POSIX boundary, `ulib/` for user libraries, `examples/shell` for evaluator
integration, `configs/` for platform configs, `scripts/` and `tools/` for build
helpers, `docs/` for local progress notes, and `vendor/`/`cargo-home/` for
offline-capable dependencies.

Generated or local-only artifacts include `kernel-rv`, `kernel-la`,
`sdcard-*.img`, `disk*.img`, `output*.md`, `*.log`, `.axconfig.toml`,
`build/`, and `target/`. Do not edit or commit them unless the task explicitly
targets generated evidence. `run-eval` may be a local symlink to `run-eval.sh`.

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
make                         # remote-submission kernels: kernel-rv/kernel-la
make kernel-rv && make kernel-la
./run-eval.sh rv             # local RISC-V evaluator path
./run-eval.sh la             # local LoongArch evaluator path
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
make A=examples/shell ARCH=riscv64 run
make clippy
make fmt && make fmt_c
make doc_check_missing
make unittest_no_fail_fast
```

### Build Notes

- `ARCH` must be one of `x86_64`, `riscv64`, `aarch64`, or `loongarch64`.
- Common Make variables include `A`/`APP`, `FEATURES`, `APP_FEATURES`, `LOG`,
  `SMP`, `MODE`, `PLAT_CONFIG`, `TARGET_DIR`, and QEMU flags such as `BLK`,
  `NET`, `GRAPHIC`, `MEM`, and `DISK_IMG`.
- QEMU runtime flags are not compile-time feature flags. Evaluator runs require
  the matching QEMU binary plus sdcard image.
- C examples require the musl cross toolchains and `libclang`/`clang` described
  in `README.md`.
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
  must not download crates or install tools during `make all`. Keep
  `tools/bin/`, `configs/platforms/`, `cargo-home/`, and
  `vendor/cargo-vendor.tar.gz` in sync when dependency closure changes;
  `scripts/ensure-cargo-vendor.sh` restores `vendor/cargo/`.
- The historical `refactor/moss_kernel_like_remote` branch and sibling checkout
  may be used only as read-only references for remote-evaluator behavior. Do not
  modify that branch or sync source into it for normal local-branch tasks.
- Keep local-only and remote-submission address mapping rules explicitly named in
  code, docs, and reports. Do not hide real evaluator failures with fake PASS,
  case-name hardcoding, or converting real failures into SKIP/TCONF.

## LTP Contest-Oriented Test Selection Policy

LTP work in this repository is contest-score oriented. The current objective is
not maximum upstream Linux LTP coverage; it is maximizing OS contest score per
unit of development time while preserving honest Linux-compatible behavior.
Prioritize high-score, high-yield tests that are strongly related to subsystems
already implemented or recently repaired. Do not blindly run, triage, or repair
all LTP cases with equal effort.

When source-level LTP evidence is needed, prefer the contest evaluator baseline
`oscomp/testsuits-for-oskernel@pre-2025` and its `ltp-full-20240524` tree over
upstream LTP master. Use upstream LTP only as supporting context when the
contest tree is unavailable or ambiguous.

When choosing LTP cases, sort candidates in this order:

1. Existing gaps in the current contest score table where `pass < all`.
2. Same-subsystem expansion cases near already high-scoring passes, especially
   `access`, `chmod`, `stat`, `open`, `pipe`, `signal`, `wait`, `read`, and
   `write` families.
3. Not-yet-run upstream LTP cases whose source contains many internal cases,
   loops, variants, or forked child processes.
4. Hidden-test defense cases for common Linux compatibility semantics, such as
   `mmap`, `mprotect`, `statx`, `openat`, and `waitid`.
5. Complex, heavy, or low-yield families only after the higher-yield groups are
   exhausted or specifically required. Examples include `bpf`, `fanotify`,
   `inotify`, `keyctl`, `landlock`, `io_uring`, `perf_event_open`, `ptrace`,
   `mount`/`swap`, `quota`, and broad `xattr` coverage.

For broad LTP selection beyond the current stable list, do not rank candidates
by raw runtest size alone. Use the evaluator's actual `ltp/runtest` files as
the counting baseline, then apply contest ROI. The current high-value
`syscalls`, `mm`, and `fs` expansion order is:

1. `syscalls`: prefer `statx`, `mmap`, `fcntl`, `open`/`openat`, `rename`,
   `link`, `unlink`, `readlinkat`, `preadv`/`pwritev`, `writev`, `sendfile`,
   `waitid`, `kill`, `fork`/`clone`, `pipe`, `access`, `chmod`/`fchmod`, and
   `chown`/`fchown` before broader missing subsystems. These families either
   have large remaining runtest coverage, directly reuse current VFS/FD/process
   work, or defend common hidden Linux-compatibility checks.
2. `mm`: prefer base `mm`/`page`/`mem`, `mmap10*`, `vma*`, and then
   `mmapstress*` or `shmt*` after basic mapping and SysV-shm behavior is
   understood. Treat `ksm*`, `oom*`, `thp*`, `overcommit_memory*`, `cpuset*`,
   and `swapping*` as low-ROI unless the kernel gains the corresponding Linux
   VM controls.
3. `fs`: prefer `fs_perms*`, `ftest*`, `rwtest*`, `stream*`, `openfile01`,
   `writetest01`, `iogen01`, `fs_inod01`, and `inode*` before generic stress
   runs. Run `gf*` only in smaller batches after basic file semantics are
   stable. Treat `fs_bind*` and `test_robind*` as low-ROI until mount, bind
   mount, and namespace semantics are real targets.

Large runtest families such as `fs_bind*`, `test_robind*`, `ksm*`,
`fanotify*`, `inotify*`, `bpf*`, `keyctl*`, `ptrace*`, `mount*`, `quotactl*`,
and namespace-specific `ioctl*` tests must not displace smaller adjacent cases
that exercise already-implemented VFS, FD, process, signal, pipe, or memory-map
behavior.

Before recommending an LTP candidate, inspect the current score gap, the
evaluator/upstream `runtest` entry, the matching source under
`testcases/kernel/{syscalls,mem,fs}/` or its corresponding directory, and
source-level yield signals such as `tcases[]`, `ARRAY_SIZE`, `.test_variants`,
loops, forks, `TST_EXP_PASS`, `TST_EXP_FAIL`, and `tst_res(TPASS)` counts.
Always name the required subsystems: syscall dispatch, VFS, permissions, FDs,
pipes, process lifecycle, wait/fork, signals, memory management, or user-memory
copying.

Rank candidates with this model:

```text
priority_score =
  potential_score_or_case_count
  * relevance_to_existing_work
  * hidden_test_value
  / implementation_cost
  / regression_risk
```

Use contest score data, runtest counts, internal source case counts, loop/variant
counts, and fork fan-out for `potential_score_or_case_count`. Raise priority for
cases reusing existing `access`, `chmod`, `stat`, `pipe`, `fork`, `signal`,
`read`, or `write` logic, and for hidden-test defense. Lower priority for work
requiring broad VM, scheduler, networking, permission-model, or filesystem
redesign, or when it risks proven high-score cases.

Once the following high-yield case families pass, treat them as regression
protection targets:

- `access01` and the broader `access` family;
- `getpid01`, `fork`, and `wait` families;
- `pipe11` and the broader `pipe` family;
- `chmod01`, `stat`, and `statx` families;
- `signal03`, `signal04`, and the broader `signal` family;
- `read02`, `read`, `write`, `readv`, and `writev` families.

Any change to VFS, permissions, file descriptors, pipes, process lifecycle,
signals, user-memory checks, memory mappings, or errno return values must
consider whether these high-score families can regress.

LTP contest work must not:

- hardcode LTP test names;
- return fixed results based on test path, filename, or process name;
- modify LTP test source to pass;
- modify evaluator scripts to bypass real testing;
- make test programs fake-print `TPASS`;
- break general Linux semantics for one LTP case;
- chase a single-case pass without running the relevant high-score regression
  cases.

Every LTP analysis or fix report must include:

A. Current gap summary: case, `pass`/`all`, remaining gap, subsystem, priority.

B. Not-yet-run cases worth adding to self-test: source evidence, internal
case/loop/variant/fork counts, related syscalls, rationale, estimated cost, and
regression risk.

C. Next minimal execution plan: individual cases to run first, likely syscall /
errno / flag / boundary checks on failure, regression cases after fixes, and the
RISC-V/LoongArch plus glibc/musl finish gate.

## Toolchain

- Rust is pinned by `rust-toolchain.toml` to `nightly-2025-05-20`, edition
  2024, with `rust-src`, `llvm-tools`, `rustfmt`, and `clippy`.
- Supported Rust targets are `x86_64-unknown-none`,
  `riscv64gc-unknown-none-elf`, `aarch64-unknown-none-softfloat`, and
  `loongarch64-unknown-none-softfloat`.
- Makefile helper tools include `cargo-binutils`/`rust-objcopy`,
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

- Documentation-only changes: inspect Markdown structure and run a lightweight
  text check such as `git diff --check`.
- Formatting-only or broad Rust/C edits: use `make fmt`, `cargo fmt --all --
  --check`, `make fmt_c`, or targeted `clang-format` as appropriate.
- Library/module changes: run `make clippy` or `make clippy ARCH=<arch>` for the
  affected target; unit-testable code should also run `make unittest_no_fail_fast`.
- Example changes: build the touched example for the affected architecture.
- POSIX/user-space behavior changes: at minimum build `make A=examples/shell
  ARCH=riscv64`; add QEMU/evaluator validation when runtime behavior matters.
- Evaluator-kernel or local-branch behavior changes: build `make kernel-rv`
  and/or `make kernel-la`, then run both `./run-eval.sh rv` and
  `./run-eval.sh la` when QEMU and sdcard images are available. Do not claim
  delivery from build-only evidence if either evaluator run is required but
  missing or failing.

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

CI covers formatting, clippy across configured architectures, Rust/C example
builds, selected platform/config builds, docs checks, unit tests, and
QEMU-backed `arceos-apps` tests as defined in `.github/workflows/`. CI runs the
pinned nightly and some moving-nightly lanes; pinned-toolchain failures are
regressions even if moving-nightly failures are allowed to continue.

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
