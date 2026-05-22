# 本地/远程评测统一改造简要报告（2026-05-22）

## 结论

已完成单分支双评测模式改造：本地仍使用 `./run-eval.sh` / `./run-eval.sh la`；远程提交入口 `make all` 会生成 `kernel-rv` 与采用远程 LoongArch 地址映射的 `kernel-la`。

## 关键变更

- `Makefile`：新增 `REMOTE_LA_PLAT_CONFIG`；仅 `make all` 的 LoongArch 构建传入远程地址映射配置；本地 `kernel-la` / `run-la` 不变。
- `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`：新增远程 LoongArch 地址映射配置。
- `kernel/arch/axhal/build.rs`：监听 `AX_CONFIG_PATH` 和配置文件变化，避免本地/远程映射切换后复用旧链接脚本。
- `examples/shell/src/cmd.rs`：LTP 成功格式为远程可计分的 `FAIL LTP CASE <case> : 0`；失败/超时仍输出真实状态。
- `AGENTS.md`：更新为单分支双评测模式说明。
- `docs/remote-local-eval-unification-2026-05-22/repair-plan.md`：保存详细修复方案。

## 验证结果

- `make -n all`：确认 `make all` 的 LA 构建使用 `configs/remote-eval/axplat-loongarch64-qemu-virt.toml`。
- `make -n kernel-la`：确认本地 LA 构建继续使用平台包默认本地配置。
- `cargo fmt --all -- --check`：通过。
- `make all`：通过，生成 ELF `kernel-rv` / `kernel-la`。
- `./run-eval.sh la`：通过；LTP musl/glibc 各 157 passed, 0 failed, 0 timed out。
- `./run-eval.sh`：通过；LTP musl/glibc 各 157 passed, 0 failed, 0 timed out。
- `scripts/ltp_summary.py`：LA/RV 均为 314 wrapper pass、0 wrapper fail、0 timeout、0 ENOSYS、0 panic/trap；`read02` 保留为 pass_with_tconf（未隐藏）。

## 质量门

- ai-slop-cleaner changed-files-only：通过，无 masking fallback / fake PASS / case-name hardcode 新增。
- code review：APPROVE，Architectural Status: CLEAR。

## 未运行项

- 未在真实远程评测机运行；本地已验证 `make all` 远程提交构建入口与 RV/LA 本地完整评测。
