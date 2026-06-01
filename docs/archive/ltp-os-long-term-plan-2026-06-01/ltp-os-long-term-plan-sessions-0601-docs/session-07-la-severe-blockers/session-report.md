# Session 7 report: LA severe blockers

Commit SHA: to be recorded after this session commit is created.
Previous session commit: `c73d323a` (Session 6 futex/process/IPC).

## Goal

对 LA-only blacklist 只做小范围、可复现的 severe-blocker 解除评估：每次挑 1~3 个条目，单 case 跑到可终止；只有证明它们已经退化为普通 PASS/FAIL/TIMEOUT marker，才从 LA blacklist 中移除。

## Changes

- 从 `docs/ltp-full-sweep-blacklist-2026-05-30-arch/blacklist-la.txt` 移除 2 个 LA-only 条目：
  - `creat07`
  - `tcp4-uni-basic01`
- 保留其余 LA network family、allocator/resource/hang 条目；没有把普通 FAIL 计为 PASS，也没有 stable promotion。
- 未修改源码、testsuite、evaluator 或 stable list。

## Evidence summary

- Active LA supplemental blacklist count: `376 -> 374`，common blacklist `5`、RV blacklist `1` 不变。
- `creat07` LA LTP-only targeted run：`PASS 0`、`FAIL 2`、internal `{'TBROK': 2}`、timeout/ENOSYS/panic/trap 均为 0，run status 0；从 severe hang/incomplete 降级为普通 closed FAIL。
- `tcp4-uni-basic01` LA LTP-only targeted run：`PASS 0`、`FAIL 2`、internal `{'TCONF': 2}`、timeout/ENOSYS/panic/trap 均为 0，run status 0；从 network resource blocker 列表中单项解除。
- Removal shard (`creat07,tcp4-uni-basic01`)：`PASS 0`、`FAIL 4`、internal `{'TBROK': 2, 'TCONF': 2}`、timeout/ENOSYS/panic/trap 均为 0，run status 0。
- 两个较早的 240s 非 LTP-only host-timeout 尝试在进入 LTP 前被基础组耗尽，不作为移除证据；真实移除证据只使用 `OSCOMP_TEST_GROUPS=ltp` 的三份日志。

## Result

Session 7 is complete. LA-only blacklist 减少 2 个条目，但没有 stable promotion；这两个 case 仍是普通 LTP FAIL/TBROK/TCONF，不能计 PASS，也不能作为 score/stable 证据。

## Risks / limitations

- 只移除了一个 network-family 代表 case；剩余 network family 仍因原 full-sweep 资源污染证据保持 LA blacklist。
- 未运行完整 `LTP_CASES=blacklist` LA full sweep；Session 8 final gate 需要重新确认 active blacklist 计数和 stable gate。
- `creat07` 仍 FAIL/TBROK，说明真实 VFS/permission/errno 语义仍需后续修复；本 session 只证明它不再是 severe sweep blocker。

## Next session entry

Session 8 进入整合与最终门禁：应 live 复核 stable count `506/506/0`，运行 RV/LA × musl/glibc stable gate，汇总 blacklist diff，并产出 final report 与下一轮 prompt。
