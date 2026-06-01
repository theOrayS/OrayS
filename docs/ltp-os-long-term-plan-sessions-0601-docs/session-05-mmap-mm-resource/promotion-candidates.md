# Session 5 promotion candidates

## Promoted to stable

以下 11 个 case 已加入 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`：

```text
diotest1
diotest2
diotest3
diotest5
diotest6
mprotect05
mmap001
mmap15
mmap17
mmap19
mincore01
```

推广依据：

- RV final combined gate 覆盖新增 11 cases + 11 个相邻 stable 回归 case：`PASS LTP CASE 44`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- LA final combined gate 覆盖同一 22 cases：`PASS LTP CASE 44`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- live stable count：`485 total / 485 unique / 0 duplicate`。
- `mincore01` 的通过来自真实 syscall/errno/copy-out 行为；其他推广 case 来自四路 parser-clean evidence。没有依赖 blacklist/SKIP/status0。

## Explicitly not promoted

- `diotest4`：RV scout 仍 `FAIL`，内部 `TCONF/TFAIL` 指向非对齐 buffer 与 non-existent user-buffer read/write 边界；不能推广。
- `mprotect01`：RV scout 仍 `FAIL/TFAIL`，`addr=0`、read-only shared mapping 等 `ENOMEM/EACCES` 边界未闭合；不能推广。
- `mprotect02`：RV scout 仍 `FAIL/TBROK`，child 以 SIGSEGV/139 退出，用户态 SIGSEGV handler 恢复语义未闭合；不能推广。
- 未列入 final combined 的其他 mmap/mm/resource sweep 候选：本 session 没有四路 parser-clean gate，不能推广。

## Stable-list boundary

Session 5 不改 blacklist，不把普通 FAIL 转成 blacklist，也不把 scout/status0/部分架构结果当作 promotion 证据。
