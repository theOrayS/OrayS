# 三人长期 LTP 冲分板块负责人制

Date: 2026-05-28
Baseline branch: `score/best`
Current live baseline: `LTP_STABLE_CASES = 460 total / 460 unique / 0 duplicates`
Scope: 长期 LTP/score 提升，不限于 stable470 一轮冲刺。

## 设计目标

三个人不再按“本轮谁跑哪一批 case”临时分工，而是长期拥有不同板块：

1. 每个人维护一个子系统候选池、失败知识库和修复路线。
2. 每个人对自己板块内的真实 Linux/POSIX 语义负责。
3. `LTP_STABLE_CASES` promotion 仍然要经过公共 gate，不允许板块 owner 自己单独推广。
4. 任何一轮冲分都从各板块拿“已证明或接近证明”的候选，组合成 targeted batch，再做 RV+LA final gate。

公共红线不变：不 hardcode testcase、不 fake PASS、不改 testsuite、不隐藏 `TCONF`/timeout/`ENOSYS`/panic/trap，不用单 arch/libc 或 targeted-only 证据冒充 promotion proof。

## 长期角色总览

| 人员 | 长期板块 | 主要收益来源 | 常用分支前缀 | 常用报告 |
| --- | --- | --- | --- | --- |
| 人员 A | VFS / Path / Metadata / Permissions | `access`、`chmod`、`stat/statx`、`open/openat`、`rename`、`link/symlink`、`mknod`、`getdents/statfs` | `fix/vfs-*`, `feat/vfs-*` | `owner-a-vfs-path-metadata-report.md` |
| 人员 B | FD / IO / Pipe / Fcntl / Vector IO | `read/write`、`readv/writev`、`preadv/pwritev`、`sendfile`、`pipe/pipe2`、`fcntl/flock`、fd limits | `fix/fd-*`, `feat/fd-*` | `owner-b-fd-io-pipe-report.md` |
| 人员 C | Process / Signal / MM / Time / Resource | `fork/clone/wait`、`kill/signal`、`mmap/mprotect/munmap`、`rlimit`、`sched/time/rusage` | `fix/proc-*`, `fix/mm-*`, `exp/proc-*` | `owner-c-proc-signal-mm-report.md` |

说明：人员 C 的板块风险最高，长期价值大，但短期 promotion 要更保守；遇到 scheduler、namespace、time accounting、catchable SIGSEGV 等大缺口时，应输出 blocker/design report，不要硬塞 stable list。

## 公共机制：轮值 Gate Captain

不设置永久“负责人”板块。每一轮 promotion 由一名 Gate Captain 轮值，职责是集成证据而不是拥有所有模块。

建议轮值：

```text
stable470: 人员 A 轮值 gate
stable480: 人员 B 轮值 gate
stable500 或下一阶段: 人员 C 轮值 gate
之后按 A -> B -> C 循环
```

Gate Captain 职责：

- 从三个板块收候选和报告。
- 写当轮 `candidate-matrix-*.md` 和 `promotion-gate-*.md`。
- 串行跑默认 QEMU/sdcard targeted gate 和 final gate。
- 编辑 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`。
- 检查 total/unique/duplicates。
- 汇总未验证项、demoted case 和下一轮 backlog。

板块 owner 不得在自己的分支直接修改 stable list；promotion commit 应由 Gate Captain 或合并 owner 统一完成。

## 人员 A：VFS / Path / Metadata / Permissions Owner

### 长期边界

负责路径解析、目录权限、文件 metadata、所有权/权限位、创建/删除/重命名、link/symlink、stat/statx/statfs/getdents 等。

### 常见代码区域

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/metadata.rs`
- `examples/shell/src/uspace/linux_abi.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`

### 长期候选池

优先维护这些 case 家族：

```text
access*, chmod*, fchmod*, fchmodat*, chown*, fchown*, fchownat*, lchown*,
stat*, statx*, statfs*, fstatfs*, statvfs*, getdents*, getcwd*,
open*, openat*, creat*, mkdir*, rmdir*, unlink*, rename*, link*, symlink*, readlinkat*,
mknod*, mknodat*
```

### 当前短期入口

- clean reserve fresh gate：`mknod08`, `mknodat01`, `rename14`。
- 可 scout 但需 source 复核：`mknod01/03/04/07/09`, `mknodat02`, `rename03/04/05`, `openat02/03`。
- 暂不 promotion：`readlinkat02`，除非先修复 LA musl `TFAIL=1` 并 fresh 四路 clean。

### 长期产物

每轮至少维护：

- 当前板块 candidate table：case、source expectation、local code path、blocking reason、next action。
- 已修语义清单：errno/flag/permission/stat layout 等。
- 回归保护清单：已进 stable 的 access/chmod/stat/open/rename/mknod 等 case。

## 人员 B：FD / IO / Pipe / Fcntl / Vector IO Owner

### 长期边界

负责 fd table、dup/close、offset/O_APPEND、pipe/pipe2、read/write/readv/writev、preadv/pwritev、sendfile、fcntl/flock、fd limit、`/proc/self/fd` 可见性等。

### 常见代码区域

- `examples/shell/src/uspace/fd_table.rs`
- `examples/shell/src/uspace/fd_pipe.rs`
- `examples/shell/src/uspace/syscall_dispatch.rs`
- `examples/shell/src/uspace/process_lifecycle.rs` 中与 fd 继承/close-on-exec 直接相关的路径

### 长期候选池

优先维护这些 case 家族：

```text
dup*, close*, close_range*, fcntl*, flock*, pipe*, pipe2*,
read*, write*, readv*, writev*, pread*, pwrite*, preadv*, pwritev*,
sendfile*, llseek*, lseek*, poll/select/pselect 中纯 fd readiness 子集
```

### 当前短期入口

- 第一批 source scout：`pipe07`, `fcntl19/19_64`, `fcntl20/20_64`, `fcntl21/21_64`, `fcntl22/22_64`。
- ownership-adjacent 与人员 A 协作：`fchown04`, `fchownat02`, `chown04`。如果主要问题是权限/metadata，由 A 接手；如果主要问题是 fd validity、fd-backed metadata 或 read-only fs fd，由 B 接手。
- 暂不第一批：`pipe02`, `dup05`, `select01-04`, `pselect01`, `close_range*`，除非先有 blocker/source report。

### 长期产物

每轮至少维护：

- FD/IO regression matrix：stable 中的 pipe/sendfile/preadv/pwritev/fcntl/flock 是否受新改动影响。
- fd-limit 与 `/proc/self/fd` 行为说明。
- fcntl/flock 锁语义缺口表：哪些只需 errno/flag 修复，哪些需要锁模型设计。

## 人员 C：Process / Signal / MM / Time / Resource Owner

### 长期边界

负责进程生命周期、fork/clone/wait/waitid、kill/signal/sigaction、mmap/mprotect/munmap、futex 交互、rlimit、sched、time、rusage、sysinfo 等。

### 常见代码区域

- `examples/shell/src/uspace/process_lifecycle.rs`
- `examples/shell/src/uspace/signal*.rs` 或 signal 相关模块
- `examples/shell/src/uspace/memory_map.rs`
- `examples/shell/src/uspace/resource_sched.rs`
- `examples/shell/src/uspace/time_abi.rs`
- `examples/shell/src/uspace/futex*.rs`
- `api/arceos_posix_api/src/uspace.rs`（只在确实跨 POSIX 边界时谨慎修改）

### 长期候选池

优先维护这些 case 家族：

```text
fork*, clone*, wait*, waitid*, kill*, signal*, sigaction*, sigprocmask*,
mmap*, mprotect*, munmap*, brk*, shmat/shm*,
getrlimit*, setrlimit*, prlimit*, getrusage*, times*, clock_gettime*, sysinfo*,
sched_*, nice*, setpriority*, getpriority*
```

### 当前短期入口

本轮 stable470 不建议 C 抢 promotion 预算。C 先做长期 blocker 分层：

- `kill02`：解释 LA aggregate child setup timeout/TBROK，确认是 testcase setup、scheduler、signal、wait 还是 timing 问题。
- `waitid07/08/10`、`getpgid01`、`times03`、`getrusage02-04`：做 source expectation + local implementation gap 表。
- `mmap04/05`, `mprotect01/02`, `munmap01`：延续 guardrail，不做 broad VM redesign。

如果 C 找到小而真实的修复，只能按 `fix/proc-*` 或 `fix/mm-*` 分支提交，并由其他人 review 后纳入 targeted gate。

### 长期产物

每轮至少维护：

- 高风险 blocker index：case -> 需要的 Linux 语义 -> 当前缺口 -> 是否值得本阶段修。
- process/signal/mm regression list：哪些 stable case 会因新改动回退。
- “可短修 vs 需设计”分类：避免把大架构缺口伪装成小修。

## 跨板块交接规则

- 权限/metadata 与 FD 交叉：A 负责 path/metadata/ownership 语义；B 负责 fd validity、fd flags、fd-backed operations。
- signal 与 pipe/FD 交叉：B 负责 pipe write/read 触发条件；C 负责信号投递、默认动作、线程退出时机。
- mmap 与 fd/file 交叉：B 负责 fd access mode/O_APPEND/file metadata；C 负责 VMA/protection/page fault 行为。
- process 与 fd 继承交叉：C 负责 fork/clone/wait 生命周期；B 负责 fd table clone、CLOEXEC、close_range。
- 争议处理：先写 10 行以内 handoff note，说明“现象、证据、建议 owner”，不要两个 owner 同时改同一文件。

## 每周 / 每阶段节奏

### 1. 板块例会式更新

每个 owner 更新一份短报告：

```text
owner-<a|b|c>-<area>-weekly.md
```

字段：

```text
New clean candidates:
Blocked candidates:
Repairs landed:
Regression risks:
Next 5 cases to test:
Needs cross-owner help:
```

### 2. 候选池合并

Gate Captain 合并三份报告，形成：

```text
candidate-matrix-stable<N>-to-stable<M>.md
```

每个候选必须标明：owner、subsystem、source evidence、current evidence、cost、risk、promotion state。

### 3. Targeted batch

先 RV scout，再 LA confirm；如果某 case 在 RV 任何 libc 出现 wrapper FAIL、内部 `TFAIL/TBROK/TCONF`、timeout、ENOSYS、panic/trap，直接 demote，不花 LA 预算。

### 4. Promotion batch

每次 promotion 建议 5-15 个 case。不要一次性塞入大量未聚类 case。推广后必须跑 stable aggregate gate。

## Branch / review 规则

- 每个 owner 从 `score/best` 开自己的 `fix/<area>-...` 或 `feat/<area>-...`。
- 高风险设计用 `exp/<name>-...`，默认不合进 `main` 或 `score/best`。
- 合并到 `score/best` 前，必须有另一名 owner review。
- 涉及 `api/arceos_posix_api/src/uspace.rs`、scheduler、trap、memory mapping 的改动，需要两名 owner review 或单独 design note。
- 每个 owner 的分支只提交自己板块的源码/报告；不要顺手改别人的板块。

## Promotion 证据标准

一个 case 进入 stable list 前必须满足：

```text
RV musl wrapper PASS + zero new internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap
RV glibc wrapper PASS + zero new internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap
LA musl wrapper PASS + zero new internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap
LA glibc wrapper PASS + zero new internal TFAIL/TBROK/TCONF/timeout/ENOSYS/panic/trap
marker-prefix bad lines = 0
```

已知 `read02` O_DIRECT/tmpfs TCONF 是历史 caveat，只能显式披露，不能把新增 TCONF 混进去。

## 当前建议路线

- stable470：A 的 VFS reserve + B 的 FD/fcntl/ownership 小池，C 主要做 blocker 分层。
- stable480：B 主导 FD/fcntl/pipe/vector IO 扩张，A 补 VFS/path reserve，C 尝试一个小型 process/resource 修复。
- stable500：只有当 A/B 低风险池变少时，C 的 process/signal/mm 板块才成为主要增长源；否则继续优先真实低风险语义。

## Stop rules

出现以下情况就停止 promotion，改写 blocker report：

- 某板块修复需要跨两个以上高风险子系统。
- targeted clean 被 aggregate gate 推翻，例如 `kill02` 类型问题。
- 证据只有单 arch/libc 或 wrapper exit code，没有 `scripts/ltp_summary.py` 解析。
- 新增 case 引入除已知 `read02` 之外的内部 `TFAIL/TBROK/TCONF`。
- 分支改动无法从其他 owner 的改动中安全分离。
