# Next Session Prompt: LTP stable115 → stable150 aggressive promotion

今天是 2026-05-22。请在 `/root/oskernel2026-orays` 继续提高 LTP stable 测试成绩，使用 **Ultragoal + Team** 分阶段执行。

请先读取并遵循仓库 `AGENTS.md`。Leader 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`；Team workers 只提供任务结果和证据，**不得直接 checkpoint Ultragoal**。

## 文档目录规则

本轮新文档保存到：

- `docs/ltp-score-improvement-2026-05-22-phase-c/`

不要把历史证据目录改名到今天；不要新建未来日期目录。上一轮证据保留在：

- `docs/ltp-score-improvement-2026-05-22-phase-b/`
- `docs/ltp-score-improvement-2026-05-22-phase-a/`
- `docs/ltp-score-improvement-2026-05-21-phase-*`

如果这是同一个 Codex thread 延续旧 Ultragoal，请先确认旧 Codex goal 已 clear；新 session 通常可直接创建新的 Ultragoal plan。

## 当前基线

上一轮 phase-b 已完成：

- stable 从 **101 → 115 cases / libc / arch**。
- 最低成功线 stable115 已达成；stable120/125 未 promotion 是因为剩余候选存在真实 blocker。
- final full evaluator gate 已完成：
  - `cargo fmt --all -- --check`: rerun exit 0
  - `make A=examples/shell ARCH=riscv64`: rerun exit 0
  - `./run-eval.sh la 2>&1 | tee output_la.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_la.md`: exit 0
  - `./run-eval.sh 2>&1 | tee output_rv.md`: exit 0
  - `python3 -B scripts/ltp_summary.py output_rv.md`: exit 0
- LA final stable：PASS LTP CASE 230, FAIL LTP CASE 0, ltp-musl 115/0, ltp-glibc 115/0。
- RV final stable：PASS LTP CASE 230, FAIL LTP CASE 0, ltp-musl 115/0, ltp-glibc 115/0。
- LA/RV internal：`TFAIL=0`, `TBROK=0`, `TCONF=4`。
- `TCONF=4` 来自已知 `read02` pass_with_tconf；必须继续透明记录，不得宣称 clean pass。
- LTP stable group timeout=0, ENOSYS=0, panic/trap=0。
- full output 仍可能出现非 LTP markers，例如 busybox `which ls fail`、libcbench futex unexpected error code、`iperf-glibc ... end: fail`。这些不属于 stable LTP promotion 成功条件，但最终报告必须透明说明。
- `/root/oskernel2026-orays-remote` 已同步 phase-b source changes，并保留 remote-only address-mapping differences。

phase-b 新增 stable cases：

```text
dup202
mkdirat01
openat01
pipe04
pipe05
pread01
pwrite01
sysinfo01
faccessat01
getgroups01
setrlimit02
sched_get_priority_max01
sched_get_priority_min01
sched_rr_get_interval01
```

## 本轮目标：一口气大冲 stable150，但 promotion gate 不能放松

- **主目标：stable150**，即从 stable115 再新增 35 个真实可验证 stable cases。
- **阶段目标：stable130 → stable140 → stable150**。
- **最低成功线：stable130**，即至少新增 15 个 clean cases。
- **Stretch：stable155+**，如果 targeted evidence 足够 clean，继续推进。

原则：

1. targeted batch 可以很大，promotion gate 不能大意。
2. 可以一次 targeted 验证 80-120 个候选，但只有 LA/RV × musl/glibc 全 clean 的 case 才能加入 stable。
3. 不伪造 PASS，不 hardcode case name，不把真实失败静默转 SKIP。
4. timeout 必须单独计数，且不能算 PASS。
5. 不只看 `run-eval` exit code；必须用 `scripts/ltp_summary.py` 读取 LTP 内部 TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap。
6. 先 targeted batch，再 promotion，再 stable targeted gate，最后 full gate；不要一开始跑完整 `./run-eval.sh la` / `./run-eval.sh`。
7. 每次 promotion 必须说明：新增 case 列表、为什么可加入 stable、LA/RV × musl/glibc 证据、internal TFAIL/TBROK/TCONF、timeout、ENOSYS、panic/trap 是否为 0 或被透明记录。
8. 如果低于 stable130、stable140 或 stable150，必须说明真实 blocker；不能因为保守或时间原因提前停止。

## 请先读取这些文件

必读：

- `AGENTS.md`
- `.omx/ultragoal/goals.json`
- `.omx/ultragoal/ledger.jsonl`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/stable115-promotion-gate-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-quality-gate.json`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-output-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-output-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-ai-slop-cleaner-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/final-gate-code-review-report.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a2-rv-matrix.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-a2-la-confirmation-matrix.md`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-b-near-clean-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-b-promotion-la-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-b/wave-c-signal-wait-rv-summary.txt`
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
- `examples/shell/src/uspace/credentials.rs`

## 当前已知 blocker / 不得直接 promotion

以下 case 在 phase-b 或 phase-a 有真实 blocker 或证据不足，必须先 targeted 修复验证，不能直接加入 stable：

```text
getrlimit03        # RV clean; LA ENOSYS / legacy getrlimit wrapper gap
unlinkat01         # RV clean; LA glibc TBROK/order pollution
sched_getscheduler02 # RV clean; LA blocker/history not clean
access02           # TFAIL, execute-file setup/ENOENT semantics
access04           # TBROK, tmpfs mount EINVAL in harness
getrlimit03
setrlimit01        # RV TFAIL/timeout under 20s targeted gate
dup03              # TFAIL, dup unexpectedly succeeded in negative case
pipe02             # TFAIL, child signal/pipe kill semantics
lseek02            # mkfifo/fixture ENOSYS
readlinkat01
readlinkat02       # invalid input / ENOTDIR semantics still failing
chmod02
fchmod02
truncate01
truncate02
ftruncate03
waitpid01
waitpid04
waitpid05
waitpid10
waitpid11
waitpid12
waitpid13
kill02
kill05
pause01
sigpending02
rt_sigpending01
sigaltstack01
sigaltstack02
sigwait01
sigtimedwait01
setitimer01
getitimer01
nanosleep01
nanosleep02
clock_nanosleep02
clock_gettime01
clock_gettime03
clock_gettime04
clock_getres01
getrusage02
gettimeofday02
```

不要因为这些曾经接近 clean 就直接 promotion；必须有 fresh LA/RV × musl/glibc clean evidence。

## Wave A：大批 targeted，先找 clean / near-clean

优先从以下池中选 **80-120 cases** 做 RV targeted；若无 panic/trap/大面积 timeout，立即 LA 对照。可以按 2-3 个 batch 跑，避免单次过长。

### proc/sched/wait/getter/rlimit

```text
sched_getscheduler02
sched_getparam02
sched_get_priority_max02
sched_get_priority_min02
sched_rr_get_interval02
getpgid01
getpgid02
getpgrp01
getgroups02
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
wait401
wait402
getrusage02
getrusage03
gettimeofday02
gettimeofday03
getpriority03
setpriority01
setpriority02
setpriority03
times02
getrlimit03
setrlimit01
setrlimit03
prlimit01
prlimit02
```

### fd/pipe/dup/lseek/open/access/readlink

```text
access02
access04
faccessat02
open04
open05
open06
openat02
close08
close09
dup03
dup05
dup201
pipe02
pipe06
pipe07
lseek02
lseek03
pread02
pwrite02
readlink02
readlink03
readlinkat01
readlinkat02
readlinkat03
```

### fs metadata/link/rename/statfs/sysinfo/chmod/truncate

```text
link01
link02
link03
linkat01
linkat02
unlink01
unlink02
unlink05
unlinkat01
unlinkat02
rename01
rename02
renameat01
renameat02
mkdir01
mkdir02
rmdir02
stat03
stat04
fstat01
fstat02
lstat02
statfs01
statfs02
fstatfs01
fstatfs02
statvfs01
fstatvfs01
sysinfo02
chmod02
chmod03
fchmod02
fchmodat01
truncate01
truncate02
ftruncate02
ftruncate03
```

### time/signal

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
kill04
kill05
pause01
sigpending02
rt_sigpending01
sigaltstack01
sigaltstack02
sigwait01
sigtimedwait01
alarm01
setitimer01
getitimer01
```

## Wave B：优先修 near-clean，高性价比冲 stable130/140

第一优先级，目标 stable130：

```text
getrlimit03
unlinkat01
sched_getscheduler02
getpgrp01
getgroups03
open04
open05
close08
close09
dup201
pipe06
lseek03
stat03
fstat01
lstat02
link01
unlink01
rename01
mkdir01
chmod02
ftruncate02
```

第二优先级，目标 stable140：

```text
access02
access04
readlinkat01
readlinkat02
dup03
pipe02
lseek02
fchmod02
truncate01
truncate02
ftruncate03
statfs01
statvfs01
fstatfs01
sysinfo02
```

第三优先级，目标 stable150：

```text
waitpid01
waitpid04
waitpid05
waitpid10
kill02
kill05
pause01
sigpending02
sigwait01
sigtimedwait01
nanosleep02
clock_gettime03
clock_gettime04
getrusage02
gettimeofday02
```

修复要求：

- 只修真实 syscall/ABI/errno/time/signal/FS 语义。
- 不根据 case 名称返回成功。
- 不把真实失败改成 skip/conf。
- raw user pointer/copy-in/copy-out 必须保持显式校验。
- ABI-visible struct layout 和 errno 变化必须在报告中说明。

## Wave C：分批 promotion

不要等所有修复完成。

- 每攒够 10-15 个 LA/RV × musl/glibc clean cases，就做一次 promotion。
- 第一批目标：stable115 → stable130。
- 第二批目标：stable130 → stable140。
- 第三批目标：stable140 → stable150。
- 每次 promotion 后必须跑 LA/RV targeted stable batch，并保存 raw log + summary。

## Team 分工建议

建议启动 7-worker Team；资源不足时 6-worker 或 4-worker fallback，但需要明确 lane 覆盖。

Leader：

- 创建新的 Ultragoal plan。
- 维护 `.omx/ultragoal/goals.json` / `ledger.jsonl`。
- 控制 targeted → promotion → stable targeted → final full gate 顺序。
- 只把 clean evidence 的 case 加入 stable。
- 最终运行 ai-slop-cleaner、verification、code-review，并用 quality-gate JSON 完成 Ultragoal。

Discovery/Matrix lane：

- 从 sdcard-rv.img / sdcard-la.img、phase-b artifacts、现有 docs 中枚举 120-180 个候选。
- 生成 candidate matrix，区分 clean pass / pass_with_tconf / timeout / TFAIL / TBROK / ENOSYS / panic/trap。
- 先产出 Wave A 80-120 cases。

Proc/Sched/Wait/Rlimit lane：

- 优先 legacy LA `getrlimit/setrlimit/prlimit` wrapper、`sched_getscheduler02`、waitpid status/child-state。
- 不把权限/errno 缺口硬改成成功。

FD/Pipe/Open/Access lane：

- 优先 `dup03`, `dup201`, `pipe02`, `pipe06`, `lseek02/03`, `access02/04`, open/close 近邻。
- 保持 errno 与 fd table 行为真实。

FS/Metadata lane：

- 优先 `unlinkat01`, link/rename/mkdir/stat/fstat/lstat/statfs/statvfs/fstatfs/sysinfo/chmod/truncate。
- 不伪造文件系统/内存信息。

Time/Signal lane：

- 优先 nanosleep/clock/signal blockers。
- timeout case 必须单独统计，不能 promotion。

Hard-blocker/Runtime lane：

- 单独调查 timeout、futex abort、panic/trap、RV memory pressure、非 LTP benchmark markers。
- 不让单个 hard blocker 卡住整批 promotion。

Verification/Review lane：

- 审核是否存在 fake PASS、case-name hardcode、silent SKIP、timeout 被算 PASS。
- 审核 `LTP_STABLE_CASES` 是否只加入 clean evidence cases。
- 最终做 code-review + quality gate。

建议命令：

```bash
omx ultragoal create-goals --brief-file docs/ltp-score-improvement-2026-05-22-phase-c/plan-stable115-to-150.md
omx team 7:executor "aggressively continue LTP stable score improvement from stable115 toward stable150; use wide targeted waves but strict promotion gate; Ultragoal state is leader-owned only"
```

资源不足 fallback：

```bash
omx team 6:executor "continue LTP stable score improvement from stable115 toward stable150; wide targeted validation first; strict promotion gate; Ultragoal state is leader-owned only"
```

或 4-worker fallback：

```bash
omx team 4:executor "continue LTP stable score improvement from stable115 toward stable150; lanes: discovery/matrix, proc-rlimit-wait, fd-fs metadata, time-signal-verification; strict promotion gate; Ultragoal leader-owned only"
```

## 推荐执行顺序

1. 创建 `docs/ltp-score-improvement-2026-05-22-phase-c/plan-stable115-to-150.md`。
2. 创建 `.omx/context/ltp-score-improvement-stable115-to-150-*.md`，总结 stable115 baseline、phase-b blocker、phase-c candidate pools。
3. 创建新的 Ultragoal plan。
4. 启动 Team。
5. Discovery/Matrix 先产出 120-180 candidate matrix，并选择 Wave A 80-120 cases。
6. Wave A targeted：先 RV，若无 panic/trap/大面积 timeout，立即 LA。
7. 选 clean subset 直接 promotion；near-clean 分派修复。
8. Wave B 修复并 targeted 重测。
9. 第一批 promotion 到 stable130；跑 LA/RV targeted stable batch。
10. 第二批 promotion 到 stable140；跑 LA/RV targeted stable batch。
11. 第三批 promotion 到 stable150；跑 LA/RV targeted stable batch。
12. 如果 evidence 允许，继续 stable155+。
13. 最终 full gate 前运行 guardrail/code review/ai-slop-cleaner。
14. 最终 full gate 后完成 quality gate JSON 和 Ultragoal checkpoint。

## 最终交付前必须跑

```bash
cargo fmt --all -- --check
make A=examples/shell ARCH=riscv64
./run-eval.sh la 2>&1 | tee output_la.md
./run-eval.sh 2>&1 | tee output_rv.md
python3 -B scripts/ltp_summary.py output_la.md
python3 -B scripts/ltp_summary.py output_rv.md
```

如果目标交付本地分支代码，最终还需同步 `/root/oskernel2026-orays-remote`，并保留 remote-only address-mapping differences。

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
- 如果低于 stable130 / stable140 / stable150，说明为什么继续 promotion 被真实 blocker 阻止。
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
