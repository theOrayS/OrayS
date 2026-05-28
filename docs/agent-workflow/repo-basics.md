# 仓库基础与卫生

只在需要理解仓库布局、生成物边界、磁盘/worktree 卫生或清理策略时读取本文件。

## 仓库性质

本树是 ArceOS-based experimental modular OS/unikernel，叠加了 OSKernel 2026 evaluator 支持。
工作方式是比赛冲分导向：增量、局部、可验证，不做无关大清理。

## 关键路径

- `kernel/`：runtime、arch、task、drivers 等内核子系统。
- `api/arceos_posix_api/`：Linux/POSIX syscall 与用户态边界。
- `ulib/`：用户库。
- `examples/shell/`：shell 示例，也是 evaluator/LTP 集成点。
- `configs/`：平台和远程评测配置。
- `scripts/`、`tools/`：构建、评测、辅助工具。
- `docs/`：阶段报告、开发者文档、工作流规则。
- `vendor/`、`cargo-home/`：远程/offline 构建依赖闭包。

## 不要随手编辑的内容

除非任务明确目标就是这些文件，否则不要编辑或提交：

```text
kernel-rv
kernel-la
sdcard-*.img
disk*.img
output*.md
*.log
.axconfig.toml
build/
target/
```

`run-eval` 可能只是 `run-eval.sh` 的本地 symlink。不要为了整洁而删除本地证据、镜像或运行产物。

## Worktree 规则

- 默认 worktree 是 dirty 的；先看 `git status --short`。
- 只修改当前任务相关文件。
- 不要 revert、覆盖、格式化或暂存他人无关改动。
- 需要提交时只 stage agent 自己的改动；不能安全分离就报告 blocker。

## 磁盘规则

长构建、完整 evaluator、QEMU、大日志、Docker、vendoring 前后运行：

```bash
df -h / /root
```

涉及 Codex/OMX 缓存清理时再运行：

```bash
du -sh /root/.codex
```

如果 `/` 约 85%+ 使用或剩余小于约 10 GiB，先停下新的重任务，清理低价值生成物、临时文件、过期 runtime 缓存或旧 raw logs。保留源码、用户证据、活跃 `.omx` 状态、memory、credentials 和可复现实验所需文件。

## 清理边界

清理 `.codex` 或 `.omx` 时优先删可重建的 transient logs/cache/temp/session debris。不要删除 skills、prompts、agents、memories、auth、active session state、`.omx/logs` 中仍需复现的证据，除非用户明确要求。
