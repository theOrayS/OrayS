# Session 5 report: mmap/mm/resource

Commit SHA: to be recorded after this session commit is created.
Previous session commit: `6a476cc2` (Session 4 VFS/metadata/path).

## Goal

把 mmap/mm/resource lane 中低风险候选转成真实语义增量，优先闭合 `mincore01` 的 `ENOSYS`，并只把 RV/LA × musl/glibc parser-clean 的 case 推广到 stable。

## Changes

- 新增 `mincore(2)` syscall dispatch 和最小真实语义实现：
  - `len == 0` 返回 0。
  - `addr` 非页对齐返回 `EINVAL`。
  - 地址溢出、超出用户空间或覆盖未映射页返回 `ENOMEM`。
  - `vec` 不可写返回 `EFAULT`。
  - 对已映射页 copy-out residency byte `1`，不伪造 LTP 输出。
- 将 11 个四路 clean case 加入 `LTP_STABLE_CASES`：`diotest1`、`diotest2`、`diotest3`、`diotest5`、`diotest6`、`mprotect05`、`mmap001`、`mmap15`、`mmap17`、`mmap19`、`mincore01`。
- 保留 `diotest4`、`mprotect01`、`mprotect02` 的真实失败边界；不修改 blacklist、testsuite 或 evaluator。

## Evidence summary

- live stable count after promotion: `485 total / 485 unique / 0 duplicate`。
- RV initial mmap/mm scout：`PASS LTP CASE 20`、`FAIL 8`、internal `{'TCONF': 4, 'TFAIL': 18, 'TBROK': 4}`、timeout `0`、ENOSYS `8`、panic/trap `0`；只作分类，不作 promotion。
- RV `mincore01` postfix：`PASS LTP CASE 2`、`FAIL 0`、internal `{}`、timeout/ENOSYS/panic/trap 均为 0。
- RV final combined mmap/mm gate：`PASS LTP CASE 44`、`FAIL 0`、internal `{}`、timeout `0`、ENOSYS `0`、panic/trap `0`。
- LA final combined mmap/mm gate：`PASS LTP CASE 44`、`FAIL 0`、internal `{}`、timeout `0`、ENOSYS `0`、panic/trap `0`。
- Build、guardrail scan、parser summary 和 checksum 见 `validation.md`。

## Result

Session 5 is complete. Stable 从 474 推进到 485，新增 11 个 mmap/mm/resource stable case，并用真实 `mincore` errno/residency/copy-out 语义闭合 `mincore01`。

## Risks / limitations

- `mincore` 当前返回“已映射即 resident”的最小模型；对本内核无换页/回收模型时这是可解释的真实驻留语义，但不是完整 Linux VM residency 统计。
- `mprotect01` 仍暴露 `ENOMEM/EACCES` 边界不足；`mprotect02` 仍需要 SIGSEGV handler/恢复语义；`diotest4` 暴露 read/write 用户指针校验问题。三者均未推广。
- 本 session 没有跑完整 stable485 四路 gate；只跑了 mmap/mm promotion+相邻回归子集。
- LA 资源指标只来自 parser case matrix 的 free-frame before/after 字段，没有新增 allocator instrumentation。

## Next session entry

Session 6 进入 futex/process/IPC lane。建议优先从 `futex` wait/wake timeout/EINTR/key、waitid/zombie/reparent 和低风险 IPC/clone 缺口切入；若先发现 stable485 回归，应停止扩张并回滚/修复回归。
