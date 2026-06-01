# Final report: 2026-06-01 LTP OS long-term improvement plan

## Executive summary

本轮完成主计划 Session 1~8。最高可信 stableN 从提示词创建时的 `460 total / 460 unique / 0 duplicate` 推进到 `506 total / 506 unique / 0 duplicate`，净增 `46` 个 stable case。最终 RV/LA × musl/glibc stable gate 全部 wrapper PASS，parser 无 FAIL、timeout、ENOSYS、panic/trap；唯一 parser-visible caveat 是 stable460 已知的 `read02` TCONF。

本轮达到最低完成定义：stable 约 500（实际 506）；至少 2 个核心 lane 有真实语义修复并四路验证；LA severe-blocker 有减少；final report 与下一轮 prompt 均写入 docs。

未达到理想完成定义：stable520 未达成；Optional Session 9/10 未执行；LA-only blacklist 仍有大量 network/resource/allocator blockers。

## Session and commit ledger

| Session | Scope | Commit | Stable count after session | Summary |
| --- | --- | --- | ---: | --- |
| 00 | Orchestration ledger | `c37dd315` | 460 | 建立 ultragoal brief 与文档根目录 |
| 01 | Baseline candidate matrix | `cd15c930` | 460 | 从 rv-arch002/la-arch012 full-sweep summary 生成候选矩阵；no promotion |
| 02 | time/select/signal | `c1c5dcd5` | 462 | 修复/推广 `getitimer01`, `ppoll01` |
| 03 | FD/fcntl/pipe/ownership | `15950e13` | 466 | 推广 `fcntl11`, `fcntl14`, `fcntl19`, `fcntl22` |
| 04 | VFS/metadata/path | `6a476cc2` | 474 | 推广 8 个 VFS/metadata/path cases |
| 05 | mmap/mm/resource | `1578a684` | 485 | 实现真实 `mincore(2)` 并推广 11 个 mmap/mm cases |
| 06 | futex/process/IPC | `c73d323a` | 506 | 推广 21 个 futex/process/IPC cases |
| 07 | LA severe blockers | `8b00b494` | 506 | LA-only blacklist `376 -> 374`；no stable promotion |
| 08 | Integration final gate | this commit | 506 | RV/LA stable506 final gate；final docs and next prompt |

## Stable progression

Live/source-backed counts:

```text
e3d43365 baseline: 460 460 0
c1c5dcd5 Session 2: 462 462 0
15950e13 Session 3: 466 466 0
6a476cc2 Session 4: 474 474 0
1578a684 Session 5: 485 485 0
c73d323a Session 6: 506 506 0
8b00b494 Session 7: 506 506 0
Session 8 final: 506 506 0
```

## Core semantic lanes completed

- time/select/signal: `getitimer01` / `ppoll01` 第一批修复与四路推广。
- FD/fcntl/pipe/ownership: record lock / fcntl cases 四路 clean 推广。
- VFS/metadata/path: statx/xattr/getdents/readlink 等低风险 metadata 增量四路 clean。
- mmap/mm/resource: 新增真实 `mincore(2)` syscall 行为，按用户地址、长度、vec copy-out、ENOMEM/EFAULT/EINVAL 边界处理；相关 mmap/mm cases 四路 clean。
- futex/process/IPC: 21 个 futex/process/IPC 候选通过 parser-backed RV/LA × musl/glibc gate 后推广。

## Final stable gate evidence

Gate command shape：`OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable ./run-eval.sh <arch>`。该形状只运行 LTP stable list，避免基础组噪声混入 final gate；结果仍以 `scripts/ltp_summary.py` parser summary 为准。

| Arch | PASS LTP CASE | FAIL LTP CASE | Suite summaries | Internal caveat | timeout | ENOSYS | panic/trap | Evidence |
| --- | ---: | ---: | --- | --- | ---: | ---: | ---: | --- |
| RV | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | inherited `read02` TCONF only (`TCONF 4`) | 0 | 0 | 0 | `target/ltp-long-term-session8/session8-rv-stable506-summary.{txt,json}` |
| LA | 1012 | 0 | `ltp-musl 506/0`, `ltp-glibc 506/0` | inherited `read02` TCONF only (`TCONF 4`) | 0 | 0 | 0 | `target/ltp-long-term-session8/session8-la-stable506-summary.{txt,json}` |

Checksums and raw/summary paths are recorded in `validation.md`.

## Blacklist status

Session 8 made no blacklist edits. Current active counts:

```text
blacklist-common.txt 5
blacklist-rv.txt     1
blacklist-la.txt     374
```

Session 7 removed two LA-only severe blockers (`creat07`, `tcp4-uni-basic01`) after LTP-only LA targeted runs proved they now close as ordinary FAIL/TBROK/TCONF without timeout/ENOSYS/panic/trap. They remain non-PASS and are not stable evidence.

## Quality and noise checks

- `scripts/ltp_summary.py` parser summaries exist for both final logs.
- Marker-prefix audit found `0` non-prefix `LTP CASE` lines in RV and LA final logs; no marker glue found.
- QEMU/evaluator pre/post disk checks stayed at `/dev/vda2 59G used 23G avail 34G use 41%`.
- Raw logs were not committed; summary/checksum/path evidence is committed instead.
- `git diff --check` is part of the final commit validation.

## Not completed / next high-ROI work

- Stable520 target remains open; current trusted value is stable506.
- Optional Session 9 should focus on network/socket and proc/syntheticfs semantics, especially LA network family blockers.
- Optional Session 10 should run all-minus-blacklist or shard sweep quality audit to prove `incomplete_count=0`, panic/trap=0, and resource failure=0 after Session 7 blacklist reduction.
- `read02` remains a known TCONF caveat inherited from stable460; future work should either make it clean or keep reporting it explicitly.

## Final conclusion

主计划 Session 1~8 已完成到最低交付标准。本轮可交付的最高可信 score/stable artifact 是 stable506 final gate；不宣称 stable520 或 full-sweep clean。
