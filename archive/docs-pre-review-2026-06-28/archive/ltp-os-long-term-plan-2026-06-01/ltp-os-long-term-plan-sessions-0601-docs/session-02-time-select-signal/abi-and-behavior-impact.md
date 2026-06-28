# ABI and behavior impact - Session 2

## Syscall / errno / flag 行为变化

- 新增 `getitimer(2)` syscall 分发：`__NR_getitimer` 现在读取当前 interval timer 状态并 copy-out `struct itimerval`；非法 `which` 返回 `-EINVAL`，无效用户指针返回 `-EFAULT`。
- `setitimer(2)` 从仅接受 `ITIMER_REAL` 扩展为接受 `ITIMER_REAL`、`ITIMER_VIRTUAL`、`ITIMER_PROF`：
  - `ITIMER_REAL` 保持原有 SIGALRM delivery 模型。
  - `ITIMER_VIRTUAL` / `ITIMER_PROF` 现在保存并回报 `it_value`/`it_interval`，不再直接 `-EINVAL`；当前仍不实现 CPU-time 计时和 SIGVTALRM/SIGPROF delivery。
  - `new_value == NULL` 仍沿用现有兼容行为：清零/disarm，而不是强制 `EFAULT`；后续如需更严格 Linux 语义应单独评估。
- `ppoll(2)` / `pselect6(2)` 现在在等待期间应用临时 signal mask 并在返回时恢复，覆盖 pending signal race 与 `EINTR` 可见语义。
- `poll(2)`/`ppoll(2)` revents 修正：普通 readable FD 不再因为 requested `POLLPRI` 而把 `POLLPRI` 当成 `POLLIN` 一起回报；`POLLPRI` 只从 exceptional readiness 路径产生。

## Struct layout / user pointer

- 未修改用户可见结构体 layout；使用既有 `linux_raw_sys::general::itimerval`、`timespec`、kernel sigset 表示。
- 新增/扩展 copy-in/copy-out：`getitimer` copy-out `itimerval`；`setitimer` 对三个 timer 类别 copy-in/copy-out；`ppoll`/`pselect6` 读取用户 sigset / pselect sigmask pair。

## FD / signal / mmap / futex 影响

- FD：只影响 poll readiness flags；不修改 FD 表、继承、CLOEXEC、offset 或 ownership。
- Signal：新增临时 signal mask guard；目标是 ppoll/pselect 等待窗口内真实 `EINTR`，不改变 rt_sigprocmask 的持久 mask 语义。
- mmap/futex/资源：无直接行为变化。

## 已知限制

- `ITIMER_VIRTUAL` / `ITIMER_PROF` 当前是可查询的 wall-clock based timer slot，不是完整 Linux CPU accounting；本 session 只把原来的 `ENOSYS/EINVAL` 缺口推进到 get/set 可见语义，SIGVTALRM/SIGPROF delivery 留给后续 signal/time lane。
- `select02` 仍因 legacy `__NR_select` TCONF 和 pselect6 子阶段超过 wrapper 15s timeout 未推广。
