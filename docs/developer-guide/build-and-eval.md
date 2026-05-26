# 构建与评测流程

这个分支有两条必须明确区分的评测路径：本地 QEMU 验证，以及远程提交产物构建。

## 本地评测运行

验证本地 sdcard 镜像时，使用 wrapper 脚本：

```bash
./run-eval.sh rv
./run-eval.sh la
```

脚本会检查 `cargo`、`qemu-img` 和对应 QEMU system binary，然后调用：

```bash
make run-rv ARCH=riscv64
make run-la ARCH=loongarch64
```

默认镜像路径在仓库根目录：

```text
sdcard-rv.img
sdcard-la.img
```

需要时可以覆盖：

```bash
RV_TESTSUITE_IMG=/path/to/sdcard-rv.img ./run-eval.sh rv
LA_TESTSUITE_IMG=/path/to/sdcard-la.img ./run-eval.sh la
```

`run-rv` 和 `run-la` 会在 `/tmp` 下创建临时 qcow2 overlay。不要并行运行共享同一 overlay 路径的 evaluator QEMU 任务，否则得到的证据可能被污染。

## 远程提交构建

远程评测期望仓库根目录存在两个产物：

```bash
make all
# 生成 ./kernel-rv 和 ./kernel-la
```

也可以分别构建：

```bash
make kernel-rv
make kernel-la
```

关键区别：

- `make all` 构建 LoongArch 提交内核时使用 `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`。
- 本地 `kernel-la` / `run-la` 默认使用常规 package/platform 配置，除非显式覆盖 `PLAT_CONFIG`。

不要把本地 LoongArch 地址映射和远程提交地址映射混用。一个内核可以在本地启动，但因为 boot page-table 或平台地址假设错误而在远程失败。

## 离线和 helper 工具行为

远程/提交构建不应该依赖网络下载。Makefile 会优先使用仓库内提供的 helper 路径：

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

如果依赖闭包发生变化，需要同步 `cargo-home/`、`vendor/` 和 helper shim。需要时，`scripts/ensure-cargo-vendor.sh` 会从归档恢复 `vendor/cargo/`。

## 常规 ArceOS app 构建

非评测 app 工作仍然使用标准 Make 接口：

```bash
make A=examples/helloworld ARCH=riscv64 run
make A=examples/httpserver ARCH=aarch64 LOG=info SMP=4 run NET=y
make A=examples/shell ARCH=riscv64 build
```

`ARCH` 必须是 `x86_64`、`riscv64`、`aarch64`、`loongarch64` 之一。常见变量包括 `A`/`APP`、`FEATURES`、`APP_FEATURES`、`LOG`、`SMP`、`MODE`、`PLAT_CONFIG`、`TARGET_DIR`、`BLK`、`NET`、`GRAPHIC`、`MEM` 和 `DISK_IMG`。

`NET=y`、`BLK=y` 这类 QEMU 参数配置的是运行时设备；它们不会自动开启 Rust feature。
