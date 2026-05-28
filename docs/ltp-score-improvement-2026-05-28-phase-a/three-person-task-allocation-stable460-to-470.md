# 三人协作任务分配：stable460 -> stable470+

Date: 2026-05-28
Branch baseline: `score/best`
Primary goal: stable470（+10 个真实 clean case）
Stretch goal: stable480（仅在低风险 clean pool 足够且 final-gate 预算允许时推进）

## 当前事实

启动本轮前已经 live 复核：

```text
examples/shell/src/cmd.rs::LTP_STABLE_CASES = 460 total / 460 unique / 0 duplicates
```

可信交付证据仍以 `docs/archive/ltp-score-improvement/ltp-score-improvement-2026-05-27-phase-a/stable460-delivery-report.md` 为准：

- RV stable460 final gate 002：`PASS LTP CASE 920`，`FAIL 0`，musl/glibc 各 460/0。
- LA stable460 final gate 002：`PASS LTP CASE 920`，`FAIL 0`，musl/glibc 各 460/0。
- 已知 caveat：aggregate 内部只有既有 `read02` O_DIRECT/tmpfs `TCONF=4/arch`，不要写成 internal-TCONF-clean。
- clean reserves 仅有 `mknod08`、`mknodat01`、`rename14`；它们仍需 fresh RV+LA x musl+glibc targeted gate 后才能使用。
- `kill02` targeted clean 曾被 LA aggregate TBROK 推翻；`readlinkat02` LA musl TFAIL，二者不得从旧 targeted 证据直接推广。

## 分支和合并模型

- `score/best`：保护当前最高分 baseline；不要直接放实验性补丁。
- 建议负责人创建冻结点：`release/stage-460`（branch 或 tag 均可）。
- 若三人并行需要集成缓冲，使用 `dev`；否则每个修复分支完成后由负责人串行合入 `score/best`。
- 个人分支建议：
  - 负责人：`dev` 或保持 `score/best` 做集成/gate。
  - 人员 B：`fix/vfs-path-stable470`。
  - 人员 C：`fix/fd-fcntl-pipe-stable470`。

## 人员 A：负责人 / gate owner / 集成

### 目标

负责人不追单个 syscall 修复，负责让证据可信、串行使用 QEMU/sdcard、最终决定是否 promotion。

### 具体任务

1. 启动检查：
   - `git checkout score/best`
   - `git status --short`
   - `df -h / /root`
   - `du -sh /root/.codex`
   - 重新计算 `LTP_STABLE_CASES` total/unique/duplicates。
2. 建立本轮目录：`docs/ltp-score-improvement-2026-05-28-phase-a/`，raw logs 放 `raw/`。
3. 创建/维护：
   - `candidate-matrix-stable460-to-470.md`
   - `promotion-gate-stable470-report.md`
   - 每个 worker 的报告索引。
4. 串行运行 targeted gate。没有隔离镜像时，B/C 不并发跑默认 evaluator。
5. 只有满足 RV+LA x musl+glibc wrapper PASS 且 zero new internal `TFAIL/TBROK/TCONF`、timeout、ENOSYS、panic/trap、marker-prefix bad lines，才编辑 `LTP_STABLE_CASES`。
6. 每次 stable-list edit 后检查 total/unique/duplicates。
7. 最终 gate：

```bash
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 timeout 120m ./run-eval.sh rv
OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 140m ./run-eval.sh la
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-28-phase-a/raw/<rv-log>
python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-28-phase-a/raw/<la-log>
```

### 第一批 targeted case files

负责人先准备两个 case file，交给 gate 串行使用：

```text
raw/cases-vfs-reserve.txt:
mknod08
mknodat01
rename14
```

```text
raw/cases-fd-ownership-scout.txt:
pipe07
fcntl19
fcntl19_64
fcntl20
fcntl20_64
fcntl21
fcntl21_64
fcntl22
fcntl22_64
fchown04
fchownat02
chown04
```

第二个文件是 RV-first scout，不是 promotion promise；B/C source review 可以删减后再跑。

## 人员 B：VFS/path/mknod/rename lane

### 主线目标

先把最接近 stable470 的 3 个 clean reserve 重新证明干净；如果失败，做窄修复；如果成功，再找同族低风险补位。

### 第一优先级

- `mknod08`
- `mknodat01`
- `rename14`

任务：读取 LTP source 和本地 `examples/shell/src/uspace/fd_table.rs` / path metadata 相关实现，确认每个 case 的期望 syscall、errno、flag、权限和 symlink/type 行为。把结论写入：

```text
docs/ltp-score-improvement-2026-05-28-phase-a/worker-b-vfs-path-report.md
```

### 第二优先级（只有第一批不足 +10 时启用）

RV-first scout 候选：

```text
mknod01,mknod03,mknod04,mknod07,mknod09,mknodat02,
rename03,rename04,rename05,openat02,openat03
```

`readlinkat02` 只做 LA-musl TFAIL 诊断，不进 promotion batch，除非先修复并 fresh 四路 clean。

### 允许修改的范围

优先只碰：

- `examples/shell/src/uspace/fd_table.rs`
- 必要时 `examples/shell/src/uspace/linux_abi.rs` 或路径/metadata helper 所在文件

不要改 testsuite，不要在 shell runner 里按 case 名 special-case。

### 验证要求

代码变更至少运行：

```bash
cargo fmt -p arceos-shell -- --check
git diff --check
python3 -B scripts/test_ltp_summary.py
make A=examples/shell ARCH=riscv64
```

targeted RV/LA evaluator 由负责人串行跑；如果 B 有隔离环境，也必须把 raw log/summary/status/marker-prefix 路径交给负责人复核。

## 人员 C：FD/fcntl/pipe/ownership lane

### 主线目标

从 FD/fcntl/pipe/ownership 相邻区域找 7 个左右低风险候选，补足 stable470；不得抢跑高风险 process/signal/mmap。

### 第一优先级：source scout + 分类

先读 LTP source，再把候选分成 `run-now` / `needs-repair` / `blocked`：

- `pipe07`：重点查 `/proc/self/fd`、`getdtablesize()`、`RLIMIT_NOFILE`、`EMFILE`。
- `fcntl19`/`fcntl19_64`、`fcntl20`/`fcntl20_64`、`fcntl21`/`fcntl21_64`、`fcntl22`/`fcntl22_64`：重点查 file-region lock、unlock、跨进程冲突语义。
- `fchown04`、`fchownat02`、`chown04`：重点查 `EPERM`、`EBADF`、`EROFS`、`AT_SYMLINK_NOFOLLOW`、symlink ownership metadata。

报告路径：

```text
docs/ltp-score-improvement-2026-05-28-phase-a/worker-c-fd-fcntl-ownership-report.md
```

### 禁止第一批占用预算

- `pipe02`：已有 RV musl panic/trap 史。
- `dup05`、`select01`-`select04`、`pselect01`、`close_range*`：先做 source diagnosis，不作为第一批 promotion。
- 大范围 fcntl lock redesign：如果需要重写锁模型，降级为 blocker report，不在 stable470 主线里硬推。

### 允许修改的范围

优先只碰：

- `examples/shell/src/uspace/fd_table.rs`
- FD/proc/self/fd/rlimit/fcntl lock 直接相关 helper

不要改 parser、wrapper marker、testsuite 或通过 case 名绕过。

### 验证要求

代码变更至少运行：

```bash
cargo fmt -p arceos-shell -- --check
git diff --check
python3 -B scripts/test_ltp_summary.py
make A=examples/shell ARCH=riscv64
```

targeted evaluator 同样由负责人串行纳入 promotion matrix。

## Promotion 节奏

1. **Reserve gate**：先跑 `mknod08,mknodat01,rename14`。如果三者全 clean，可形成 stable463。
2. **FD/ownership RV scout**：只在 RV x musl+glibc clean 且无 internal failure 后，才花 LA 预算。
3. **补位原则**：优先同族小语义 case；遇到 setup-heavy、TCONF、timeout、ENOSYS、panic/trap 立即 demote。
4. **stable470 edit**：只由负责人编辑 `LTP_STABLE_CASES`，B/C 不直接改 stable list。
5. **final gate**：stable470 必须 RV+LA aggregate PASS；目标 parser 形态：`PASS LTP CASE 940`、`FAIL 0`、musl/glibc 各 470/0，并显式披露既有 `read02` TCONF caveat。

## Do-not-first 清单

本轮第一批不要投入这些目标，除非已经完成 blocker root-cause report：

```text
kill02, readlinkat02, pipe02, inode02,
poll02, getcpu01, gethostid01, gethostname02,
times03, getpgid01, fork13, fork14, kill05, kill10,
mmap04, mmap05, munmap01, mprotect01, mprotect02,
mount/fs_bind/fanotify/inotify/bpf/keyctl/io_uring/ptrace/quota/landlock
```

## 当天停止条件

满足任一条件就停止 promotion，写报告而不是继续硬冲：

- 无法凑齐 10 个 fresh 四路 clean case。
- 新增 case 让 stable aggregate 出现除既有 `read02` 之外的 `TFAIL/TBROK/TCONF`。
- 任一 arch 出现 timeout、ENOSYS、panic/trap 或 marker-prefix 污染。
- 修复需要跨 VFS、FD、process、signal、scheduler 多层大改。
- 磁盘空间进入风险区，或 QEMU/sdcard 资源被并发污染。
