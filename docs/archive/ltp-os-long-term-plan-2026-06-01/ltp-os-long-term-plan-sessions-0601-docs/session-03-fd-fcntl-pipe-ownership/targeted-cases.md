# Session 3 targeted cases

单位：LTP case（每个 case 在 evaluator 内分别跑 musl/glibc）。

## Scout / diagnosis

- `fcntl11`
- `fcntl14`
- `fcntl19`
- `fcntl22`
- `fcntl30`（保持 blocked：依赖 `/proc/sys/fs/pipe-max-size`）
- `pipe07`（保持 blocked：依赖 `/proc/self/fd`）
- `pipe15`（保持 blocked：依赖 `/proc/sys/fs/pipe-user-pages-soft`）
- `writev03`（保持 TCONF/设备类，不 promotion）
- `pwritev03`（保持 TBROK/ENOSPC 设备创建问题，不 promotion）

## Promotion gate candidates

- `fcntl11`
- `fcntl14`
- `fcntl19`
- `fcntl22`

## Adjacent regression subset

- `fcntl07`
- `fcntl12`
- `fcntl13`
- `fcntl18`
- `fcntl29`
- `pipe02`
- `pipe08`
- `pipe2_02`
- `dup05`
