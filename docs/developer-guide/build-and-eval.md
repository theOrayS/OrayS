# Build and Evaluation Workflow

This branch has two evaluator modes that must stay distinct: local QEMU
validation and remote-submission artifact generation.

## Local evaluator runs

Use the wrapper script when validating against local sdcard images:

```bash
./run-eval.sh rv
./run-eval.sh la
```

The script checks for `cargo`, `qemu-img`, and the matching QEMU binary, then
invokes:

```bash
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
```

By default it expects root-level images:

```text
sdcard-rv.img
sdcard-la.img
```

Override them when needed:

```bash
RV_TESTSUITE_IMG=/path/to/sdcard-rv.img ./run-eval.sh rv
LA_TESTSUITE_IMG=/path/to/sdcard-la.img ./run-eval.sh la
```

The direct `run-rv` and `run-la` targets create temporary qcow2 overlays under
`/tmp`.  Do not run parallel evaluator QEMU jobs that share the same overlay
paths; the resulting evidence can be contaminated.

## Remote-submission build

The remote evaluator expects root-level artifacts:

```bash
make all
# produces ./kernel-rv and ./kernel-la
```

Per-architecture build targets are also available:

```bash
make kernel-rv
make kernel-la
```

Important distinction:

- `make all` builds the LoongArch submission kernel with
  `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`.
- Local `kernel-la` / `run-la` use the normal package/platform defaults unless
  `PLAT_CONFIG` is explicitly overridden.

Do not silently mix the local LoongArch address map with the remote submission
address map.  A kernel can boot locally and still fail remotely if boot
page-table or platform-address assumptions are wrong.

## Offline and helper-tool behavior

Remote/submission builds should not depend on network downloads.  The Makefile
prefers repository-provided helper paths:

```text
scripts/axconfig-gen.py
scripts/cargo-axplat.sh
scripts/rust-objcopy.sh
tools/bin/axconfig-gen
tools/bin/cargo-axplat
tools/bin/rust-objcopy
cargo-home/config.toml
vendor/cargo-vendor.tar.gz
```

If dependency closure changes, keep `cargo-home/`, `vendor/`, and helper shims
in sync.  `scripts/ensure-cargo-vendor.sh` restores `vendor/cargo/` from the
archive when needed.

## Normal ArceOS app builds

For non-evaluator app work, use the standard Make interface:

```bash
make A=examples/helloworld ARCH=riscv64 run
make A=examples/httpserver ARCH=aarch64 LOG=info SMP=4 run NET=y
make A=examples/shell ARCH=riscv64 build
```

`ARCH` must be one of `x86_64`, `riscv64`, `aarch64`, or `loongarch64`.
Common variables include `A`/`APP`, `FEATURES`, `APP_FEATURES`, `LOG`, `SMP`,
`MODE`, `PLAT_CONFIG`, `TARGET_DIR`, `BLK`, `NET`, `GRAPHIC`, `MEM`, and
`DISK_IMG`.

QEMU options such as `NET=y` and `BLK=y` configure runtime devices; they do not
by themselves enable Rust feature flags.
