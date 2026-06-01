# Session 4 promotion candidates

## Promoted to stable

以下 8 个 case 已加入 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`：

```text
fpathconf01
pathconf01
rename14
mknod08
mknodat01
getxattr01
listxattr01
statx03
```

推广依据：

- RV final combined gate 覆盖新增 8 cases + 27 个相邻 stable 回归 case：`PASS LTP CASE 70`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- LA promotion-clean gate 覆盖新增 8 cases + 26 个相邻 stable 回归 case：`PASS LTP CASE 68`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- live stable count：`474 total / 474 unique / 0 duplicate`。
- 这些 case 的通过来自真实 syscall/errno/value/path 行为改动或四路 clean 复核；没有依赖 blacklist/SKIP/status0。

## Explicitly not promoted

- `readlinkat02`：RV musl/glibc clean，LA glibc clean，但 LA musl 仍 `FAIL/TFAIL=1`，不能四路推广。
- `getdents01`：`getdents64` symlink overlay 已改善，但 legacy raw `getdents` 仍出现 `TCONF/ENOSYS`，case 不 clean。
- `getdents02`：初筛有 `TCONF`/ENOSYS 迹象，不是 parser-clean PASS。
- `readlink03`：父目录搜索权限和部分 errno 修复后仍有组件级 `ELOOP` 等 TFAIL。
- `statx01`、`statx04`、`statfs01`：初筛仍有 TBROK/失败，未进入本 session promotion。

## Stable-list boundary

Session 4 不改 blacklist，不把普通 FAIL 转成 blacklist，也不把 `readlinkat02/getdents01/readlink03` 的部分修复当作 stable promotion 证据。
