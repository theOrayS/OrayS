# 本地与远程评测模式

只在任务涉及本地 QEMU、远程 evaluator、`make all`、`kernel-rv`、`kernel-la`、RV/LA 启动或平台地址映射时读取本文件。

## 单一工作分支原则

`/root/oskernel2026-orays` 是本地 QEMU 验证和远程提交构建共用的维护 checkout。当前最高分维护线使用 `score/best`；原 `refactor/moss_kernel_like` 已改名为 `score/best`。不要另建或维护单独 remote 分支作为交付目标，除非用户提出新的明确要求。

历史 `refactor/moss_kernel_like_remote` 分支和 sibling checkout 只能作为只读参考；不要为了常规交付去修改或同步它。

## 本地验证

本地 evaluator：

```bash
./run-eval.sh rv
./run-eval.sh la
```

这些命令使用本地 QEMU command line 和 package default LoongArch platform address map。

## 远程提交构建

远程提交验证由 `make all` 代表，必须生成根目录 ELF-format：

```text
kernel-rv
kernel-la
```

`kernel-la` 的远程提交构建使用：

```text
configs/remote-eval/axplat-loongarch64-qemu-virt.toml
```

不要把这个 remote config 用于本地 `run-la`，除非任务明确是在测试远程提交构建行为。

## LoongArch 地址映射注意

LoongArch boot page-table setup 必须从 `KERNEL_BASE_VADDR` 推导 L0 slot，不能假设 high-half index `0`。

- 本地 QEMU 当前使用 `0xffff_0000_8000_0000`。
- 远程 evaluator 使用 `0xffff_8000_8000_0000`。

硬编码 `BOOT_PT_L0[0]` 可能本地能启动，但远程 instruction-fetch fault 循环。

## Offline / vendor-first 约束

远程 evaluator 可能网络不可靠或离线。Submission builds 不应在 `make all` 期间下载 crates 或安装工具。依赖闭包变化时保持这些路径同步：

```text
tools/bin/
configs/platforms/
cargo-home/
vendor/cargo-vendor.tar.gz
```

`scripts/ensure-cargo-vendor.sh` 会恢复 `vendor/cargo/`。

如扩展 helper 行为，验证：

```bash
make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all
```

不能运行离线验证时要报告缺口。

## 报告要求

涉及 evaluator-mode 的最终报告必须说明：

- local `./run-eval.sh rv` 是否通过；
- local `./run-eval.sh la` 是否通过；
- `make all` 是否仍能生成 remote-submission `kernel-rv`/`kernel-la`；
- local-only 与 remote-submission 地址映射规则是否有变化。
