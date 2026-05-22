# Next Session Prompt: LTP stable101 → stable120/125 aggressive promotion

今天是 2026-05-22。请在 `/root/oskernel2026-orays` 继续提高 LTP stable 测试成绩，使用 **Ultragoal + Team** 分阶段执行。

请先读取并遵循仓库 `AGENTS.md`。Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`；Team workers 只提供任务结果和证据，**不得直接 checkpoint Ultragoal**。

## 文档目录规则

本轮新文档保存到：

- `docs/ltp-score-improvement-2026-05-22-phase-b/`

不要把历史证据目录改名到今天；不要新建未来日期目录。上一轮证据保留在：

- `docs/ltp-score-improvement-2026-05-22-phase-a/`
- `docs/ltp-score-improvement-2026-05-21-phase-*`

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。

## 当前基线

上一轮 phase-a 已完成：

- stable 从 **85 → 101 cases / libc / arch**。
- Ultragoal 已 complete：68/68。
- Team 已 shutdown/reconciled；后续 Stop hook 只需确认 team absent，不再分配任务。
- final full evaluator gate 已完成：
  - `cargo fmt --all -- --check`: exit 0
  - `make A=examples/shell ARCH=riscv64`: exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_rv.md`: exit 0
- LA final stable：PASS LTP CASE 202, FAIL LTP CASE 0, ltp-musl 101/0, ltp-glibc 101/0。
- RV final stable：PASS LTP CASE 202, FAIL LTP CASE 0, ltp-musl 101/0, ltp-glibc 101/0。
- LA/RV internal：`TFAIL=0`, `TBROK=0`, `TCONF=4`。
- `TCONF=4` 来自已知 `read02` pass_with_tconf；必须继续透明记录，不得宣称 clean pass。
- LTP stable group timeout=0, ENOSYS=0, panic/trap=0。
- full output 仍可能出现非 LTP markers，例如 busybox `which ls fail`、libcbench futex unexpected error code、`iperf-glibc ... end: fail`。这些不属于 stable LTP promotion 成功条件，但最终报告必须透明说明。

phase-a 新增 stable cases：

```text
sched_getparam01
getpriority01
getpriority02
waitpid03
rt_sigprocmask01
rt_sigprocmask02
sigaction02
sigprocmask01
sigsuspend01
dup04
fchmod03
pipe03
waitpid06
waitpid07
waitpid08
waitpid09
```

## 本轮目标：更激进，但不降低 gate

本轮不要只保守冲 stable105。

- **主目标：stable120**，即从 stable101 再新增 19 个真实可验证 stable cases。
- **最低成功线：stable115**，即至少新增 14 个 clean cases。
- **Stretch：stable125+**，如果 targeted evidence 足够 clean，继续推进。

原则：

1. 批次可以激进，promotion gate 不能激进。
2. 可以一次 targeted 验证 35-60 个候选，但只有 LA/RV × musl/glibc 全 clean 的 case 才能加入 stable。
3. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
4. timeout 必须单独计数，且不能算 PASS。
5. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
6. 先 targeted batch，再 promotion，再 stable targeted gate，最后 full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
7. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。
8. 如果 promotion 低于 stable115，必须说明真实 blocker；不能因为保守或时间原因提前停止。

## 请先读取这些文件

必读：

- `AGENTS.md`
- `.omx/ultragoal/goals.json`
- `.omx/ultragoal/ledger.jsonl`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-a/stable101-promotion-gate-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-a/stable103-targeted-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-a/stable103-promotion-matrix.md`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-ai-slop-cleaner-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-a/final-gate-code-review-report.md`
- `scripts/ltp_summary.py`
- `examples/shell/src/cmd.rs`
- `examples/shell/src/uspace/process_abi.rs`
- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/signal_abi.rs`
- `examples/shell/src/uspace/system_info.rs`
- `examples/shell/src/uspace/synthetic_fs.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`
- `examples/shell/src/uspace/task_context.rs`
- `examples/shell/src/uspace/task_registry.rs`
- `examples/shell/src/uspace/time_abi.rs`
- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/fd_pipe.rs`

## 当前已知 blocker / 不得直接 promotion

以下 case 在 phase-a 有真实 blocker 或证据不足，必须先 targeted 修复验证，不能直接加入 stable：

- `clock_nanosleep02`, `nanosleep01`: 曾进入 stable103 candidate，但 RV stable103 targeted 在 musl/glibc 均出现真实 TFAIL，已剔除。
- `sched_getscheduler02`: RV clean，但 LA musl TFAIL。
- `clock_gettime01`, `clock_nanosleep01`: timeout，不能 promotion。
- `clock_getres01`: TCONF，不能 clean promotion。
- `setpriority01`, `setpriority02`: 仍有 TCONF/TFAIL 或权限/errno 语义缺口。
- `waitpid01`: wait status/child-state 仍有 TFAIL。
- `getrusage02`: 若仍 TCONF，不得作为 clean promotion。
- `gettimeofday02`: 若 timeout，不得计 PASS。
- FS/metadata/statfs/sysinfo/access/link/rename/unlink/mkdir/lseek/pipe/dup 方向必须修真实 ABI/errno；不要伪造文件系统/内存信息。

## 候选方向：一次多冲一些

### Wave A：广撒网 targeted，先找 clean/near-clean

优先从以下池中选 **35-60 cases** 做 RV targeted，若无 panic/trap/大面积 timeout，立即 LA 对照：

proc/sched/wait/getter：

```text
sched_getscheduler02
sched_getparam02
sched_get_priority_max01
sched_get_priority_min01
sched_rr_get_interval01
getpgid01
getpgid02
getpgrp01
getgroups01
getgroups03
gettid02
waitpid01
waitpid02
waitpid04
waitpid05
waitpid10
waitpid11
waitpid12
waitpid13
getrusage02
getrusage03
gettimeofday02
gettimeofday03
getpriority03
setpriority01
setpriority02
setpriority03
times01
times02
getrlimit03
setrlimit01
setrlimit02
```

time/signal：

```text
clock_gettime01
clock_gettime03
clock_gettime04
clock_getres01
clock_nanosleep01
clock_nanosleep02
nanosleep01
nanosleep02
kill02
kill05
pause01
sigpending02
rt_sigpending01
sigaltstack01
sigaltstack02
sigwait01
sigtimedwait01
alarm01
alarm02
setitimer01
getitimer01
```

fd/pipe/dup/lseek/open/access：

```text
access02
access04
faccessat01
open01
open02
openat01
close01
close02
dup01
dup02
dup03
dup05
dup201
dup202
pipe01
pipe02
pipe04
pipe05
lseek01
lseek02
pread01
pwrite01
readlink01
readlinkat01
readlinkat02
```

fs metadata/link/rename/statfs/sysinfo：

```text
link01
link02
linkat01
unlink01
unlink05
unlinkat01
rename01
renameat01
mkdir01
mkdir02
mkdirat01
rmdir01
stat01
stat02
fstat01
lstat01
statfs01
statfs02
fstatfs01
fstatfs02
statvfs01
fstatvfs01
sysinfo01
chmod01
chmod02
fchmod02
truncate01
truncate02
ftruncate01
ftruncate03
```

### Wave B：near-clean 修复

从 Wave A 结果中优先修近似 clean 的 case。建议优先：

```text
sched_getscheduler02
setpriority01
setpriority02
waitpid01
clock_nanosleep02
nanosleep01
dup03
pipe02
lseek02
access02
access04
link02
rename01
unlink05
mkdir02
statfs01
statvfs01
fstatfs01
sysinfo01
```

修复要求：

- 只修真实 syscall/ABI/errno/time/signal/FS 语义。
- 不根据 case 名称返回成功。
- 不把真实失败改成 skip/conf。
- raw user pointer/copy-in/copy-out 必须保持显式校验。
- ABI-visible struct layout 和 errno 变化必须在报告中说明。

### Wave C：分批 promotion

不要等所有修复完成。

- 每攒够 8-12 个 LA/RV × musl/glibc clean cases，就做一次 promotion。
- 第一批目标：stable101 → stable112/115。
- 第二批目标：stable115 → stable120/125。
- 每次 promotion 后必须跑 LA/RV targeted stable batch，并保存 raw log + summary。

## Team 分工建议

建议启动 7-worker Team；资源不足时 6-worker fallback。

Leader：

- 创建新的 Ultragoal plan。
- 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`。
- 控制 targeted → promotion → stable targeted → final full gate 顺序。
- 只把 clean evidence 的 case 加入 stable。
- 最终运行 ai-slop-cleaner、verification、code-review，并用 quality-gate JSON 完成 Ultragoal。

Discovery/Matrix lane：

- 从 sdcard-rv.img / sdcard-la.img、phase-a artifacts、现有 docs 中枚举 80-120 个候选。
- 生成 candidate matrix，区分 clean pass / pass_with_tconf / timeout / TFAIL / TBROK / ENOSYS / panic/trap。
- 先产出 Wave A 35-60 cases。

Proc/Sched/Wait lane：

- 修 scheduler/getpriority/setpriority/wait/getter 真实 ABI/errno。
- 优先：`sched_getscheduler02`, `setpriority01`, `setpriority02`, `waitpid01`, `getpgid01`, `getgroups01`。

Time/Signal lane：

- 修 nanosleep/clock/signal blockers。
- 优先：`clock_nanosleep02`, `nanosleep01`, `nanosleep02`, `pause01`, `kill05`, `sigpending02`。
- timeout case 必须单独统计，不能 promotion。

FD/Pipe/Open lane：

- 修 `dup03`, `pipe02`, `lseek02`, `access02`, `access04`, `open/openat` 近邻。
- 保持 errno 与 fd table 行为真实。

FS/Metadata lane：

- 修 access/link/rename/unlink/mkdir/statfs/statvfs/fstatfs/sysinfo 真实 ABI/errno。
- 不伪造文件系统/内存信息。

Hard-blocker/Runtime lane：

- 单独调查 timeout、futex abort、panic/trap、RV memory pressure、非 LTP benchmark markers。
- 不让单个 hard blocker 卡住整批 promotion。

Verification/Review lane：

- 审核是否存在 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS。
- 审核 `LTP_STABLE_CASES` 是否只加入 clean evidence cases。
- 最终做 code-review + quality gate。

建议命令：

```bash
omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-22-phase-b/plan-stable101-to-125.md
omx team 7:executor "aggressively continue LTP stable score improvement from stable101 toward stable120/125; use wide targeted waves but strict promotion gate; Ultragoal state is leader-owned only"
```

资源不足 fallback：

```bash
omx team 6:executor "aggressively continue LTP stable score improvement from stable101 toward stable120; wide targeted validation first; Ultragoal state is leader-owned only"
```

## 推荐执行顺序

1. 创建 `docs/ltp-score-improvement-2026-05-22-phase-b/plan-stable101-to-125.md`。
2. 创建 `.omx/context/ltp-score-improvement-stable101-to-125-*.md`，总结 stable101 baseline、phase-a blocker、phase-b candidate pools。
3. 创建新的 Ultragoal plan。
4. 启动 Team。
5. Discovery/Matrix 先产出 80-120 candidate matrix，并选择 Wave A 35-60 cases。
6. Wave A targeted：先 RV，若无 panic/trap/大面积 timeout，立即 LA。
7. 选 clean subset 直接 promotion；near-clean 分派修复。
8. Wave B 修复并 targeted 重测。
9. 第一批 promotion 到 stable112/115；跑 LA/RV targeted stable batch。
10. 如果 evidence 允许，Wave C 继续到 stable120/125；跑 LA/RV targeted stable batch。
11. 最终 full gate 前运行 guardrail/code review/ai-slop-cleaner。
12. 最终 full gate 后完成 quality gate JSON 和 Ultragoal checkpoint。

## 最终交付前必须跑

```bash
cargo fmt --all -- --check
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 -B scripts/ltp_summary.py output_la.md
python3 -B scripts/ltp_summary.py output_rv.md
```

如果修改 shell/uspace 代码，至少还要跑：

```bash
make A=examples/shell ARCH=riscv64
```

如果目标交付本地分支代码，最终还需说明是否同步 `/root/oskernel2026-orays-remote`，并保留 remote-only address-mapping differences。

## 最终报告必须包括

- 修改文件。
- 修改函数/常量。
- 每项修复的预期行为。
- 实际验证命令和 exit code。
- LA/RV pass/fail 汇总。
- internal TFAIL/TBROK/TCONF。
- timeout / ENOSYS / panic/trap。
- stable batch 新增 case。
- 不纳入 stable 的 blocked cases 及原因。
- 如果低于 stable115 或 stable120，说明为什么继续 promotion 被真实 blocker 阻止。
- 未完成风险和下一批建议。
- 是否同步 `/root/oskernel2026-orays-remote`，以及保留了哪些 remote-only address-mapping differences。
- 是否有 syscall / errno / ABI-visible 行为变化；如果没有，明确说明没有。

## 停止/降级条件

以下情况必须停止 promotion 或降级目标，而不是污染 stable：

- LTP stable targeted 出现 timeout。
- targeted case 内部 TFAIL/TBROK 非 0。
- panic/trap 或 RV memory pressure 影响稳定运行。
- ENOSYS 仍存在且不是明确可接受的 TCONF。
- 只能通过 case-name hardcode、fake PASS、silent SKIP 才能变绿。
- FS/sysinfo/statfs 只能靠伪造数据通过，而非真实 ABI/语义修复。
