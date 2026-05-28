# 编码边界与高风险区域

只在修改 Rust、syscall、POSIX ABI、用户态边界、内核 runtime 或高风险路径时读取本文件。

## 通用改动边界

- 优先最小、局部、可回滚补丁。
- 优先复用已有工具和模式，不新增 speculative abstraction。
- 不做跨 `kernel/`、`api/`、`ulib/`、`examples/` 的无关重构。
- 保留平台和 feature 结构；不要为了简化控制流合并架构差异。
- 不做 repo-wide search/replace、机械 rename、import normalization 或批量格式化，除非任务明确要求。
- capability 不支持时要显式失败并说明理由，不要 stub success。

## Rust 规则

- 跟随 touched file 的现有风格。
- 低层模块已有 `unsafe`，不要强加 blanket ban；新增 `unsafe` 要尽量窄。
- 不明显的 `unsafe` invariant 用 `// SAFETY:` 注明。
- 运行时、syscall、filesystem、network、用户输入路径避免无根据 `unwrap()`/`expect()`。
- 大集成文件中优先小 helper 和 early return，避免深层嵌套。

## POSIX / 用户态边界

原始用户指针、长度和 ABI-visible struct 都是不可信输入。必须先验证，再转 slice/string/struct。copy-in/copy-out 行为要显式，不要静默扩大信任边界。

修改 syscall、errno、struct layout、FD、signal、futex、networking、process/task、mmap 等 Linux/POSIX 可见语义时，最终报告必须列出可见行为变化；无变化也要明确写“无预期 ABI/POSIX 可见变化”。

`api/arceos_posix_api/src/uspace.rs` 是大型集成文件，覆盖 ELF loading、memory layout、FDs、signals、futexes、syscall handling；避免宽泛重写。

## 日志与输出

`kernel/`、`api/`、`ulib/` 中优先用现有 logging（如 `axlog` macros），不要随手 `println!`。`examples/` 和 evaluator scripts 中 stdout/stderr 可以作为可见接口的一部分。

## 高风险路径

### `api/arceos_posix_api/`

修改时必须检查并报告：syscall 号、返回值、errno、flag、struct layout；raw user pointer 是否验证；是否影响 FD/process/signal/futex/mmap/ELF loading；musl 与 glibc 是否都可能受影响。

### `examples/shell/`

这是 evaluator 集成面，不只是 demo。修改时确认 wrapper marker 格式未破坏、内部 LTP 输出未隐藏、case selection/timeout/cleanup 有真实理由，且不是为了某个 case 名字假通过。

### `kernel/arch/axhal/`、`kernel/runtime/axruntime/`、`kernel/task/axtask/`

这些路径影响 boot、trap、scheduler、user-task flow。保留跨架构 cfg；说明 RV/LA 影响；先 build 再 QEMU/evaluator；不要把本地 LA 地址映射假设误用于远程提交。

### `vendor/`、`cargo-home/`、`tools/bin/`

除非任务明确是 offline build、依赖闭包或 helper 行为，否则不要改。必须改时验证 `make all` 和 offline-style `make all`；无法运行就报告。
