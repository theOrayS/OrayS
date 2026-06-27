# G010 full-evaluator runthrough panic fixes (2026-06-11/12)

## 目标与红线

目标是让当前内核在 `run-eval.sh` 全量评测路径下能持续跑到 runner 结束，而不是因为 kernel panic、内存分配 panic、资源泄漏或永久阻塞提前中断。这里的“跑通”只表示评测 runner 可以完整执行并诚实汇总结果；不表示每个 LTP / benchmark case 都通过。

红线：本次修复不使用 LTP case 名、路径、进程名或输出硬编码；不 fake PASS；不隐藏 `TCONF`、`TBROK`、`TFAIL`、`ENOSYS`、timeout、panic/trap。仍失败的 case 必须保留为真实失败。

## 已修复的真实问题

### 1. 退出进程/任务被强引用保留，长期运行后 OOM/panic

- `examples/shell/src/uspace/syscall_dispatch.rs`
  - syscall dispatch 不再为每次 syscall 额外克隆 `Arc<UserProcess>`；普通分支改为从 `UserTaskExt` 借用 `&UserProcess`。
  - `exit` / `exit_group` / pending watchdog 等 no-return 路径先处理，再进入普通 syscall dispatch，避免 no-return 前本地 `Arc` 永久滞留。
  - 只有确实需要跨 helper 生命周期持有进程的路径继续传 `&Arc<UserProcess>`，如 `clone`、`setitimer`、POSIX timer。
- `examples/shell/src/uspace/memory_map.rs`
  - page fault/no-return terminate 路径同样从当前 task ext 借用进程，避免故障终止路径保留额外 `Arc`。
- `examples/shell/src/uspace/process_lifecycle.rs`, `examples/shell/src/uspace/task_context.rs`
  - user task entry 不再通过 closure 捕获 `Arc<UserProcess>` / `UspaceContext`；初始上下文放在 `UserTaskExt.initial_context` 中由 task 启动时取走，减少退出后的强引用链。
  - 清理流程主动 `prune_exited_user_tasks()`、`axtask::reap_exited_tasks()`、futex prune，并添加仅 auto-run-tests 下的对象/强引用诊断。
- `examples/shell/src/uspace/signal_abi.rs`
  - `kill` / `tkill` / `tgkill` 给当前进程/线程发送致命信号时，先 `drop(entry)` 再进入 no-return `terminate_current_if_exit_group_pending()`。
  - 根因证据：`waitpid01` 的 self-signal 子进程会让 `EXITED_TASKS` 中 task `strong_count=2`，旧代码在 no-return 前保留 `UserThreadEntry`，导致 `user_process_retained` 从 3 跳到 25；修复后 targeted `waitpid01` 全部回到 0。
- `api/arceos_posix_api/src/imp/fd_ops.rs`, `api/arceos_posix_api/src/lib.rs`
  - 导出 FD 表占用数用于全量运行资源诊断，不改变 syscall 语义。

### 2. pidfd/timer/helper 持有进程强引用导致超时后无法释放

- `examples/shell/src/uspace/fd_table.rs`
  - `PidFdEntry` 的目标进程从 `Arc<UserProcess>` 改为 `Weak<UserProcess>`；目标已释放时返回 `ESRCH` / exited 语义，而不是用 pidfd 永久保活进程。
- `examples/shell/src/uspace/time_abi.rs`
  - real itimer / POSIX timer helper 改用 `Weak<UserProcess>`，helper loop 分片等待并在 teardown 时取消 timer。
  - helper kernel stack 缩小，避免每个 timer 默认 256 KiB kernel stack 在长期运行中放大内存压力。
- `kernel/task/axtask/src/timers.rs`, `kernel/task/axtask/src/wait_queue.rs`
  - wait queue 被唤醒/取消时同步取消对应 timer event；零时长 timed wait 直接 timeout，不再挂无意义 timer。

### 3. 大文件、mmap、ramfs 连续分配导致 OOM/panic

- `kernel/memory/axalloc/src/lib.rs`
  - `GlobalAlloc::alloc` 失败返回 null，让 `try_reserve*` 等 fallible 路径收到 OOM，而不是直接 kernel panic。
  - byte heap 扩容改为 capped + fallback；大分配/高对齐分配绕过 TLSF byte heap 直接走 page allocator，并在 dealloc 对称释放。
- `examples/shell/src/uspace/fd_table.rs`, `examples/shell/src/uspace/metadata.rs`, `examples/shell/src/uspace/memory_map.rs`, `examples/shell/src/uspace/mod.rs`
  - 常规文件物理 backing 限制为一个用户 I/O chunk，超出部分通过 sparse overlay / repeated-byte extent 维护逻辑内容。
  - `fallocate`、大范围同值写入、truncate、mmap fault 合成都基于真实逻辑大小和 sparse extent，而不是一次性预分配 100MB/300MB 级 `Vec`。
- `vendor/axfs_ramfs/src/file.rs`
  - ramfs truncate/write 使用 fallible reserve，无内存时返回 `StorageFull`。
- `kernel/fs/axfs/src/api/mod.rs`, `ulib/axstd/src/fs/mod.rs`
  - `read` / `read_to_string` 不再按 metadata size 预分配整个文件。
- `kernel/memory/axmm/src/backend/*`, `kernel/memory/axmm/src/lib.rs`
  - 增加 shared frame 诊断，用于确认 mmap/COW 长跑后引用数回落。

### 4. 长阻塞 syscall 与网络表资源积累导致 runner 卡死

- `examples/shell/src/uspace/fd_socket.rs`
  - 阻塞 `recv` / `accept` 在 eval watchdog 激活时短周期 poll，允许 case 诚实 timeout，而不是永久卡住。
- `kernel/net/axnet/src/smoltcp_impl/*`
  - listen/UDP loopback 表改为可清理/复用的 sparse 结构，降低 full run 中端口/loopback 状态积累。
- `examples/shell/src/uspace/futex.rs`, `examples/shell/src/uspace/task_registry.rs`
  - futex/registry 在清理时 prune 空项和已退出 task，避免长期扫描越来越慢或保留无效引用。

### 5. loader/runner 资源与日志噪声

- `examples/shell/src/uspace/program_loader.rs`
  - ELF/shebang/interpreter 镜像按 chunk fallible 读取到复用 buffer，限制单个 exec image 大小，避免每次 exec 重新分配/长期保留大镜像。
- `examples/shell/src/cmd.rs`
  - `LTP MEMORY` 保留必要资源证据；极长的 allocator bucket 明细 `LTP ALLOC` 默认关闭，仅 `LTP_ALLOC_DIAG=1` 编译时输出，避免全量评测日志膨胀影响 runthrough。
  - `prepare_ltp_case_run_dir()` 默认给 case 独立 `/tmp/ltp-work/<case>-run`，隔离跨 case 残留；这会让少数依赖 CWD 附带资源的 case 真实失败，报告中不得算 PASS。

## 关键验证证据

### Build / static gates

- `git diff --check`：通过。
- `python3 scripts/test_g010_real_kernel_semantics.py`：36 tests OK。
- `python3 scripts/test_g011_empty_shells.py`：9 tests OK。
- `python3 scripts/test_g012_syscall_review_hotspots.py`：13 tests OK。
- `make A=examples/shell ARCH=loongarch64 SMP=1 LA_MEM=1G build`：通过。
- `make A=examples/shell ARCH=riscv64 SMP=1 defconfig build`：通过。
  - 注：在 LA build 之后直接 `make A=examples/shell ARCH=riscv64 SMP=1 build` 会被 Makefile 拒绝，原因是 `.axconfig.toml` 仍为 LA 配置；重新 `defconfig` 后 RV build 通过。
- `df -h / /root`：最终 build 前为 `/dev/vda2 59G, 34G used, 24G avail, 59%`。

### LA targeted retention / self-signal

- `/tmp/g010-la-ltp-borrow-dispatch2-20260611T184552.log` / `.summary`
  - Command: `OSCOMP_TEST_GROUPS=ltp LTP_CASES='fork13' LTP_CASE_TIMEOUT_SECS=15 timeout 420 make run-la ...`
  - `RUN_EXIT=0`，panic/trap 0；`fork13` 在 musl/glibc 均真实 timeout fail；cleanup 后 `user_process_retained=0 user_task_ext_live=0 exited_task_queue=0`。
- `/tmp/g010-la-ltp-targeted-retention-20260611T184731.log` / `.summary`
  - Command: `OSCOMP_TEST_GROUPS=ltp LTP_CASES='fork13,epoll-ltp,abs01' LTP_CASE_TIMEOUT_SECS=20 timeout 900 make run-la ...`
  - `RUN_EXIT=0`，PASS 2，FAIL 4，timeout 4，panic/trap 0；6 个 libc/case 组合 cleanup 后 retained 全为 0。
- `/tmp/g010-la-ltp-waitpid01-selfsignal-drop-runeval-20260611T202430.log` / `.summary`
  - Command: `OSCOMP_TEST_GROUPS=ltp LTP_CASES='waitpid01' LTP_CASE_TIMEOUT_SECS=30 timeout 600 ./run-eval.sh la ...`
  - `RUN_EXIT=0`，PASS 2，FAIL 0，timeout 0，ENOSYS 0，panic/trap 0。
  - musl cleanup：`user_process_objects=0 user_process_created=45 user_process_dropped=45 user_process_retained=0 user_task_ext_live=0 exited_task_queue=0`。
  - glibc cleanup：`user_process_objects=0 user_process_created=90 user_process_dropped=90 user_process_retained=0 user_task_ext_live=0 exited_task_queue=0`。

### LA full evaluator runthrough

- Raw log：`/tmp/g010-la-blacklist-selfsignal-drop-20260611T202532.log`
- Parser summary：`/tmp/g010-la-blacklist-selfsignal-drop-20260611T202532.summary`
- Host/run result：`RUN_EXIT=0`。
- Full evaluator group evidence：48 个 `#### OS COMP TEST GROUP ... ####` marker，末尾已到：
  - `#### OS COMP TEST GROUP END unixbench-glibc ####`
  - kernel 正常打印 `Shutting down...`
- LTP suite evidence：
  - `ltp-musl`: `all-minus-blacklist skipped=35`，2333 cases，suite summary `957 passed, 1376 failed`。
  - `ltp-glibc`: `all-minus-blacklist skipped=38`，2337 cases，suite summary `966 passed, 1371 failed`。
  - LTP start/end marker：4670 / 4670。
- `scripts/ltp_summary.py` 汇总：
  - PASS LTP CASE: 1923
  - FAIL LTP CASE: 2746
  - Internal TFAIL/TBROK/TCONF: 6466 (`TBROK=509`, `TCONF=2667`, `TFAIL=3290`)
  - timeout matches: 82
  - ENOSYS/not implemented matches: 712
  - panic/trap matches: 0
- 末尾 LTP 资源证据：`zram03 after_cleanup` 为 `free_frames=172187`、`user_process_objects=0`、`user_process_created=27784`、`user_process_dropped=27784`、`user_process_retained=0`、`user_task_ext_live=0`、`exited_task_queue=0`。
- 磁盘：run 后 `df -h / /root` 为 `/dev/vda2 59G, 34G used, 24G avail, 59%`。

### RV full evaluator runthrough

- Raw log：`/tmp/g010-rv-blacklist-clean16-20260612T021029.raw.log`
  - 原始 outer tee 日志为 `/tmp/g010-rv-blacklist-clean16-20260612T021029.log`；由于该命令在结束时把 parser summary 追加回同一个 log，为避免重复解析 summary 文本，本报告以第一个 `# LTP summary:` 之前的 raw-prefix 作为 clean parser 输入。
- Parser summary：`/tmp/g010-rv-blacklist-clean16-20260612T021029.clean.summary`
- Host/run result：`RUN_EXIT=0`，outer exit `0`。
- Full evaluator group evidence：48 个 `#### OS COMP TEST GROUP ... ####` marker / 24 个 group end marker，末尾已到：
  - `#### OS COMP TEST GROUP END unixbench-glibc ####`
  - kernel 正常打印 `Shutting down...`
- LTP suite evidence：
  - `ltp-musl`: `all-minus-blacklist skipped=35`，2333 cases，suite summary `949 passed, 1384 failed`。
  - `ltp-glibc`: `all-minus-blacklist skipped=38`，2337 cases，suite summary `964 passed, 1373 failed`。
  - LTP start/end marker：4670 / 4670。
- `scripts/ltp_summary.py` clean 汇总：
  - PASS LTP CASE: 1913
  - FAIL LTP CASE: 2756
  - Internal TFAIL/TBROK/TCONF: 6438 (`TBROK=510`, `TCONF=2666`, `TFAIL=3262`)
  - timeout matches: 88
  - ENOSYS/not implemented matches: 710
  - panic/trap matches: 0
- 末尾 LTP 资源证据：`zram03 after_cleanup` 为 `free_frames=239045`、`user_process_objects=0`、`user_process_created=20923`、`user_process_dropped=20923`、`user_process_retained=0`、`user_task_ext_live=0`、`exited_task_queue=0`。
- 磁盘：run 后 `df -h / /root` 为 `/dev/vda2 59G, 34G used, 24G avail, 59%`。
- 早期 RV 辅助证据：`/tmp/g010-rv-blacklist-large-direct-20260611T091509.log` / `.summary` 已跑到 ltp-musl 与 ltp-glibc END，parser `panic/trap matches: 0`，但外层 host timeout 退出码为 124，且早于 self-signal drop 修复；只能作为弱历史证据，不作为最终 RV 完整通过证明。

### RV timer-helper regression checks

- `/tmp/g010-rv-ltp-pause-pair-clean16-20260612T020757.raw.log` / `.clean.summary`
  - Command: `OSCOMP_TEST_GROUPS=ltp LTP_CASES='pause02,pause03' LTP_CASE_TIMEOUT_SECS=20 timeout 600 ./run-eval.sh rv ...`
  - `RUN_EXIT=0`，PASS 4，FAIL 0，timeout 0，ENOSYS 0，panic/trap 0。
- `/tmp/g010-rv-ltp-pause-window-clean16-20260612T020857.raw.log` / `.clean.summary`
  - Command: `OSCOMP_TEST_GROUPS=ltp LTP_CASES='pause02,pause03,pause04,alarm02,alarm03,alarm05,overcommit_memory,pathconf02,abs01' LTP_CASE_TIMEOUT_SECS=20 timeout 900 ./run-eval.sh rv ...`
  - `RUN_EXIT=0`，PASS 12，FAIL 6，Internal `TFAIL=12`，timeout 0，ENOSYS 0，panic/trap 0。
  - 失败来自 `overcommit_memory` / `pathconf02` / `pause04` 等真实语义缺口或测试期望差异，未隐藏为 PASS。

### 静态 hardcoding / fake-pass guard

- `git diff --check`：通过。
- `python3 scripts/test_g010_real_kernel_semantics.py`：36 tests OK。
- `python3 scripts/test_g011_empty_shells.py`：9 tests OK。
- `python3 scripts/test_g012_syscall_review_hotspots.py`：13 tests OK。
- `git diff --unified=0 -- api examples kernel ulib vendor | rg '^\+.*(panic!|unreachable!|todo!|unwrap\(|expect\(|LTP_CASE|TPASS|PASS LTP|FAIL LTP|blacklist|set_current_dir|ltp-work|hardcode|stub|TODO|FIXME)'`：仅命中已有 musl syscall stub patch 名称与 user task 初始上下文 invariant `expect`；未发现新增 LTP case/output/PASS 硬编码。

## 仍未解决/必须诚实保留的问题

- 大量 LTP case 仍然 FAIL/TCONF/TBROK/TFAIL/ENOSYS/timeout；这些不是 PASS，也未被 promotion。
- 后续 benchmark 组仍有真实失败/timeout，例如 iperf/netperf socket option/IPv6 能力不足、lmbench/cyclictest/iozone/unixbench 的 60s timeout、iozone 的 fsync/read 语义缺口等。
- `prepare_ltp_case_run_dir()` 的 per-case scratch CWD 提升隔离性，但会让 `gen*` 等依赖当前目录资源的 case 真实失败；后续若要提升分数，应改成“基线目录清理/恢复”而不是按 case 名做特殊处理。
- 代码审查技能要求的独立 `code-reviewer` / `architect` 双 lane 当前无法合规启动：可见 subagent 工具没有 `agent_type` 参数。因此本轮只能报告静态 guard 与自查结果，不能伪称 independent review APPROVE。

## 用户可见语义影响

- fallible allocation 路径现在可以返回 `ENOMEM` / `StorageFull` 等错误，而不是 kernel panic。
- 大/高对齐内核分配直接用 page allocator，释放路径对称回收。
- 常规文件在超过物理 prefix 后使用 sparse backing；读、写、mmap/fallocate 通过真实逻辑大小与 sparse extent 合成可见内容。
- timed wait 被提前唤醒时会取消对应 timer event；零超时 timed wait 立即 timeout。
- eval watchdog 激活时，阻塞 socket、timer wait、signal wait 会周期性检查超时/信号，使 case 能诚实 timeout 退出。
- pidfd/timer helper 不再永久保活已退出进程；目标已释放时按真实错误/已退出状态处理。
