# Session 6 ABI and behavior impact

## User-visible syscall/errno behavior changes

- 本 session 未修改 futex/process/IPC syscall 实现，因此没有新的 syscall/errno/flag/struct layout 行为变化。
- `LTP_STABLE_CASES` 新增 21 个 futex/process/IPC 相关 case；这是 evaluator 选择面的变化，不是内核运行时 ABI 变化。

## ABI / struct layout / copy-in-copy-out

- 未修改公开 C struct layout、`linux_raw_sys` ABI 常量或用户指针 copy-in/copy-out 逻辑。
- 未修改 futex key、wait/wake、signal delivery、SysV shm attach/detach 或 process lifecycle 数据结构。
- 没有硬编码 LTP case 名、路径、进程名或输出；case 名只出现在 stable list 与文档中。

## FD / signal / futex / mmap impact

- futex、signal、scheduler/process、SysV shm 行为保持 Session 5 后状态；本 session 只通过 targeted gate 确认 21 个 case 可作为 stable promotion。
- 不修改 FD、mmap、VFS 或 blacklist。

## Known limitations

- `futex_wait03/futex_wait05` 的 timeout/EINTR 边界仍未修复。
- `waitid07` 的 wait/zombie/reparent 语义仍未闭合。
- `clone02`、`execve01/05` 等仍需要独立诊断，不在本 session 中推广。
