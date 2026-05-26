# OSKernel 2026 ArceOS Evaluation Branch

This repository is an ArceOS-based experimental modular OS/unikernel, adapted
for the OSKernel 2026 evaluation flow.  It keeps the upstream ArceOS workspace
structure, but the maintained branch is focused on bootable evaluator kernels,
Linux/POSIX compatibility in the shell/user-space path, and honest LTP result
reporting for both local QEMU validation and remote submission builds.

The current working branch is the single maintained source tree for both modes:

- **Local validation:** `./run-eval.sh rv` and `./run-eval.sh la` run the
  RISC-V and LoongArch QEMU evaluator paths against local sdcard images.
- **Remote submission build:** `make all` builds the root-level `kernel-rv` and
  `kernel-la` artifacts expected by the remote evaluator.  The LoongArch
  submission build uses `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`
  so its address map matches the remote environment.
- **Offline-friendly dependencies:** helper shims under `scripts/` and
  `tools/bin/`, the non-hidden `cargo-home/`, and `vendor/cargo-vendor.tar.gz`
  keep submission builds from depending on network access.

ArceOS itself was inspired by [Unikraft](https://github.com/unikraft/unikraft).
This branch is still experimental and under active compatibility work.

## Current feature focus

- Architectures: `x86_64`, `riscv64`, `aarch64`, `loongarch64`.
- QEMU platforms: pc-q35 for x86_64 and virt platforms for
  RISC-V/AArch64/LoongArch.
- Kernel subsystems: multitasking, FIFO/RR/CFS schedulers, SMP scheduling,
  VirtIO block/network/display, file systems, and a smoltcp-based TCP/UDP stack.
- User-space boundary: `api/arceos_posix_api` plus `examples/shell` provide the
  POSIX/Linux-facing evaluator integration, ELF loading, process lifecycle,
  file descriptors, signals, futexes, memory mapping, and syscall dispatch.
- Evaluator harness: `examples/shell` can auto-run official groups and LTP cases
  from the mounted test images.
- LTP stable set: `examples/shell/src/cmd.rs::LTP_STABLE_CASES` currently lists
  383 unique cases.  The runner executes the selected list for both `/musl` and
  `/glibc`, so the stable default is 766 LTP case executions per architecture.

## Repository layout

| Path | Purpose |
| --- | --- |
| `kernel/` | Core ArceOS runtime, HAL, memory, tasking, drivers, fs, net, sync, and SMP modules. |
| `api/arceos_posix_api/` | Linux/POSIX ABI-facing syscall and user-space integration layer. |
| `ulib/` | User libraries, including `axstd` and `axlibc`. |
| `examples/shell/` | Interactive shell and OSKernel evaluator integration point. |
| `configs/` | Default, platform, custom, and remote-evaluator platform configs. |
| `scripts/` | Build helper shims, QEMU/build make fragments, and LTP summary tools. |
| `tools/bin/` | Repo-provided helper executables preferred by offline/submission builds. |
| `cargo-home/` | Non-hidden Cargo home for vendored/offline source replacement. |
| `vendor/` | Local crate patches and `cargo-vendor.tar.gz` restore archive. |
| `docs/` | Local compatibility notes, remote/local eval unification evidence, LTP plans, raw summaries, and final gates. |
| `eval-reports/` | Archived evaluator result bundles, when present. |
| `doc/` | Upstream-style ArceOS documentation and platform notes. |

Generated/local artifacts such as `kernel-rv`, `kernel-la`, `sdcard-*.img`,
`disk*.img`, `output*.md`, `*.log`, `build/`, and `target/` may exist in a
working checkout.  They are not source-of-truth files unless a task explicitly
asks to preserve generated evidence.

## Prerequisites

The Rust toolchain is pinned by `rust-toolchain.toml`:

- channel: `nightly-2025-05-20`
- components: `rust-src`, `llvm-tools`, `rustfmt`, `clippy`
- targets: `x86_64-unknown-none`, `riscv64gc-unknown-none-elf`,
  `aarch64-unknown-none-softfloat`, `loongarch64-unknown-none-softfloat`

On Debian/Ubuntu, install the host packages needed for direct builds and QEMU
runs:

```bash
sudo apt-get update
sudo apt-get install -y build-essential make git wget ca-certificates \
    python3 python3-venv pkg-config libclang-dev qemu-system qemu-utils
```

The Makefile prefers repository helper shims before user-installed tools, but a
normal development host can also install the upstream Cargo helpers:

```bash
cargo install cargo-binutils axconfig-gen cargo-axplat
```

C examples and libc-oriented rebuilds need the appropriate musl cross toolchains
for the target architecture.  See the upstream toolchain notes in this file's
history or the contest environment setup if you need to rebuild C user apps.

## Quick start

### Build remote-submission kernels

The default target is `all`, which builds both evaluator artifacts:

```bash
make all
# outputs:
#   ./kernel-rv
#   ./kernel-la
```

Equivalent per-architecture targets are available:

```bash
make kernel-rv
make kernel-la
```

`kernel-rv` is wrapped from the RISC-V binary through
`scripts/make/riscv64-kernel-wrap.lds`; `kernel-la` is copied from the
LoongArch ELF output.  `make all` intentionally uses the remote LoongArch
platform config, while local `kernel-la`/`run-la` keep the package default
LoongArch config unless you override `PLAT_CONFIG`.

### Run local evaluator images

Place or point to the evaluator sdcard images, then run:

```bash
./run-eval.sh rv
./run-eval.sh la
```

By default the script looks for `sdcard-rv.img` and `sdcard-la.img` in the
repository root.  You can override paths with:

```bash
RV_TESTSUITE_IMG=/path/to/sdcard-rv.img ./run-eval.sh rv
LA_TESTSUITE_IMG=/path/to/sdcard-la.img ./run-eval.sh la
```

The script checks for `cargo`, `qemu-img`, and the matching QEMU system binary
before launching the run.

### Run QEMU targets directly

```bash
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
```

The direct targets build `kernel-rv`/`kernel-la`, create temporary qcow2 overlays
under `/tmp`, and boot QEMU with the mounted evaluator image.  Avoid running RV
or LA evaluator QEMU jobs in parallel unless their run-image paths are isolated.

### Build or run a normal ArceOS app

```bash
make A=examples/helloworld ARCH=riscv64 run
make A=examples/httpserver ARCH=aarch64 LOG=info SMP=4 run NET=y
make A=examples/shell ARCH=riscv64 build
```

`ARCH` must be one of `x86_64`, `riscv64`, `aarch64`, or `loongarch64`.
Common Make variables include `A`/`APP`, `FEATURES`, `APP_FEATURES`, `LOG`,
`SMP`, `MODE`, `PLAT_CONFIG`, `TARGET_DIR`, `BLK`, `NET`, `GRAPHIC`, `MEM`, and
`DISK_IMG`.  QEMU flags such as `NET=y` or `BLK=y` affect runtime device
configuration, not the Rust feature set by themselves.

## LTP and evaluator workflow

The shell evaluator path is selected with `APP_FEATURES=auto-run-tests,uspace`
by the kernel build targets.  LTP execution is controlled in
`examples/shell/src/cmd.rs`:

- `LTP_CORE_CASES` is the small smoke set.
- `LTP_STABLE_CASES` is the current high-confidence contest set.
- `LTP_CASE_BATCHES` contains named targeted batches.
- `/ltp_cases.txt` or `/tmp/ltp_cases.txt` inside the guest overrides the case
  selection at runtime.
- Build-time `LTP_CASES` can select `stable`, `core`, `batch:<name>`,
  `file:<path>`, or an inline comma/space-separated case list.
- `/ltp_case_timeout_secs` or build-time `LTP_CASE_TIMEOUT_SECS` can override
  the default per-case timeout.

The runner keeps the remote evaluator's wrapper wire format for completed LTP
cases: `FAIL LTP CASE <case> : <status>`, where status `0` is a wrapper-level
pass and non-zero is a failure.  Internal LTP output (`TFAIL`, `TBROK`, `TCONF`),
timeouts, ENOSYS, and panic/trap signals remain visible in the raw log.

Always summarize evaluator logs with the parser before promoting a result:

```bash
python3 scripts/ltp_summary.py output_rv.md
python3 scripts/ltp_summary.py output_la.md
python3 scripts/ltp_summary.py --promotion-candidates rv.log la.log
```

Do not rely on the outer QEMU/run-eval exit code alone.  A clean promotion needs
wrapper pass counts plus no unexpected internal failure, timeout, ENOSYS, or
panic/trap signals for the selected scope.

## Validation commands

Use the smallest check that proves your change:

```bash
# Formatting / static checks
make fmt
make fmt_c
make clippy
make doc_check_missing
make unittest_no_fail_fast

# Build-only evaluator artifacts
make kernel-rv
make kernel-la
make all

# Local evaluator gates when images and QEMU are available
./run-eval.sh rv
./run-eval.sh la
```

For POSIX/user-space changes, at least build the shell for the affected target;
for evaluator behavior changes, follow build validation with RV and LA evaluator
runs when the images and QEMU are available.

## Docker workflow

A Dockerfile is provided for a development container with the pinned Rust
workflow and QEMU/tooling assumptions:

```bash
docker build -t orays-arceos-dev -f Dockerfile .
docker run --rm -it -v "$(pwd):/work" -w /work orays-arceos-dev bash
```

The Makefile also provides `make docker-image` and `make docker`; the latter
uses the Makefile's built-in `/code/arceos` mount convention, so the manual
command above is safer when this checkout is not literally named `arceos`.

## Writing ArceOS apps

Rust applications should be `no_std`/`no_main` crates that depend on `axstd`:

```toml
[dependencies]
axstd = { path = "/path/to/arceos/ulib/axstd", features = ["..."] }
```

Annotate the entry point with `#[unsafe(no_mangle)]`, then build through this
repository:

```bash
make -C /path/to/arceos A=$(pwd) ARCH=<arch> run
```

C applications can provide `axbuild.mk` and `features.txt` like the examples in
`examples/`, then use the same Make interface.

## Platform-specific builds

Use `MYPLAT` for platform crates or `PLAT_CONFIG` for explicit TOML configs:

```bash
make MYPLAT=axplat-aarch64-raspi SMP=4 A=examples/helloworld
make PLAT_CONFIG=$(pwd)/configs/custom/x86_64-pc-oslab.toml \
    A=examples/httpserver FEATURES=page-alloc-4g,driver-ixgbe,driver-ramdisk SMP=4
```

Repository configs are under `configs/platforms/`, `configs/custom/`, and
`configs/remote-eval/`.

## Documentation and evidence

- Long-running LTP score work is recorded in dated folders such as
  `docs/ltp-score-improvement-YYYY-MM-DD-phase-x/`.
- Compatibility notes live in `docs/`, for example FD/socket progress and
  network loopback behavior.
- Upstream-style platform docs remain under `doc/`.

Keep evaluator reports honest: do not hardcode test names, fake `TPASS`, hide
`TCONF`/timeouts, or edit testsuite sources to manufacture a pass.

## Licenses

This workspace inherits the upstream ArceOS licensing files in the repository
root: Apache-2.0, GPLv3, MulanPSL2, and MulanPubL2.
