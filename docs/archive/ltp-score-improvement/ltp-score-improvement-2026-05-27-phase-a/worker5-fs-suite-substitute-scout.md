# Worker 5 task-12 fs-suite substitute scout（report-only）

- 日期：2026-05-27
- Team：`ltp-stable413-to-stab-d9f99e59`
- Worker：`worker-5`
- 任务：task-12 `Worker 5 fs-suite substitute scout`
- 约束：只写报告；未运行 QEMU/evaluator；未修改 `.omx/ultragoal`；未修改 `examples/shell/src/cmd.rs::LTP_STABLE_CASES`。

## 输入与基线

### live stable

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` 现场解析结果：

- total：413
- unique：413
- duplicate：0

### 四路 sdcard inventory

使用 inventory：

- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-rv-musl-ltp-bin-cases.txt`：2820
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-rv-glibc-ltp-bin-cases.txt`：2840
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-la-musl-ltp-bin-cases.txt`：2820
- `docs/ltp-score-improvement-2026-05-21-phase-c/raw/sdcard-la-glibc-ltp-bin-cases.txt`：2840

四路交集：2820；四路交集且不在 live stable：2407。

重要 caveat：这些 inventory 多数记录的是 LTP 二进制/脚本名，不总是 `runtest/*` alias。例如 `fs_perms01`-`fs_perms06` alias 不在 inventory 中，但它们共用的 `fs_perms` 二进制四路存在；`openfile01` 对应 `openfile`，`writetest01` 对应 `writetest`，`fs_inod01` 对应 `fs_inod`，`rwtest01`/`rwtest02`/`iogen01` 对应 `rwtest`/`iogen`/`doio`。

### LTP source

本报告使用只读临时 sparse clone：`/tmp/worker5-ltp-src`，upstream short SHA `96f5559`。读取范围：`runtest/fs`、`runtest/fs_perms_simple`、`testcases/kernel/fs`、`testcases/kernel/io/writetest`。

## 推荐 leader-run scout batch（第一轮）

只推荐先跑最窄、低 I/O 压力、非 mount/bind/填盘的组合；任何 `TCONF`/`TFAIL`/`TBROK`/timeout/marker truncation 都不得推广：

```text
stream02,fs_perms04,fs_perms05,fs_perms06,fs_perms01,fs_perms02,fs_perms03,openfile01,writetest01,fs_inod01
```

执行策略：RV first；RV clean 后再 LA；最终仍需 RV+LA × musl+glibc parser-clean gate。`inode02`、`ftest06`、`rwtest*`、`iogen01` 暂不放首批，因为已有失败史或明显 stress/长时 I/O 特征。

## 15-30 ranked cases / blockers

| Rank | Case | sdcard availability | Source-level expectation | Existing local/prior evidence | Scout decision |
| ---: | --- | --- | --- | --- | --- |
| 1 | `stream02` | alias/binary 四路存在；不在 stable | `runtest/fs` 直接运行 `stream02`；源码 `SAFE_MKNOD(... S_IFIFO ...)` 后用 `fopen()` 的 `r+`/`w+`/`a+` 打开 FIFO 节点。 | 本地 `sys_mknodat`/FIFO path 有实现；worker-4 刚修 FIFO `O_NONBLOCK|O_WRONLY` 相关守卫。旧 RV 记录为 `FAIL 1` 且有 internal `TFAIL=1`，所以只能作为“修复后重探”。 | **首批 scout**，但若仍 TFAIL 立即移除。 |
| 2 | `fs_perms04` | alias 未单列；`fs_perms` 二进制四路存在；不在 stable | `runtest/fs_perms_simple`：mode `002`，uid/gid 切换后测试 write 权限。 | 本地 chmod/chown/setuid/setgid/权限元数据路径存在；不含长时 I/O。风险在 uid/gid + shell exec/权限模型。 | **首批 scout**，write-only 权限比 exec 路径更窄。 |
| 3 | `fs_perms05` | 同上 | mode `020`，测试 group write。 | 同上；覆盖 group ownership 权限。 | **首批 scout**。 |
| 4 | `fs_perms06` | 同上 | mode `200`，测试 owner write。 | 同上；最接近普通 owner write 语义。 | **首批 scout**。 |
| 5 | `fs_perms01` | alias 未单列；`fs_perms` 二进制四路存在；不在 stable | mode `005`，uid/gid 切换后测试 execute；源码会创建含 shebang 的文件并走 `execl()`/`execlp()`。 | 比 write 组多了 exec/shebang 解析风险；本地 exec path 存在但 shebang + permission 组合需确认。 | **首批 scout**，排在 write 组之后。 |
| 6 | `fs_perms02` | 同上 | mode `050`，group execute。 | exec/shebang + group 权限风险。 | **首批 scout**。 |
| 7 | `fs_perms03` | 同上 | mode `500`，owner execute。 | exec/shebang + owner 权限风险。 | **首批 scout**。 |
| 8 | `openfile01` | alias 未单列；`openfile` 二进制四路存在；不在 stable | `runtest/fs`：`openfile -f10 -t10`，10 个线程并发 open 10 个文件。 | 本地 clone/futex/pthread 路径存在，但多线程 fd/open 并发可能暴露锁/FD table 竞争。 | **首批 scout**，低文件数但注意 hang。 |
| 9 | `writetest01` | alias 未单列；`writetest` 二进制四路存在；不在 stable | `runtest/fs` 直接运行 `writetest`；源码做顺序写/校验型 I/O。 | 旧 `target-fs8-rv-001` 记录 glibc/musl `FAIL -1`，像 harness/二进制启动类失败；源码本身压力中等。 | **首批末位 scout**；若仍 `FAIL -1`，归为 harness/binary issue。 |
| 10 | `fs_inod01` | alias 未单列；`fs_inod` 脚本四路存在；不在 stable | `fs_inod $TMPDIR 10 10 10`；脚本快速创建/删除目录和文件。 | 旧 `target-fs8-rv-001` 记录 `FAIL -1`，可能是脚本解释器/路径问题；I/O 数量有限但 fork/background wait 需确认。 | **首批末位 scout**；仅作为替补，不先推广。 |
| 11 | `inode02` | alias/binary 四路存在；不在 stable | 多进程 mkdir/stat/open 压力，默认 child=5，目录树深度/广度较大。 | 旧记录不稳定：RV 曾 `FAIL 137`，后 RV clean；LA glibc 曾 `FAIL 137`、LA musl PASS。 | **第二轮 reserve**；首批排除。 |
| 12 | `ftest06` | alias/binary 四路存在；不在 stable | 与已稳定 `ftest02` 近似，但使用 `lseek64`；默认 child=5、iterations=50，目录操作+随机文件操作。 | 旧 RV 多次 `FAIL 4`；已有 `ftest01`-`05`、`ftest07`、`ftest08` 稳定，唯独 `ftest06` 缺口像语义/64 位偏移路径问题。 | **diagnostic only**；不要放首批。 |
| 13 | `rwtest05` | alias 未单列；`rwtest`/`iogen`/`doio` 四路存在；不在 stable | `rwtest -i 50 -T 64b 500b:...`，比 60s/120s 组短，但仍是 doio wrapper。 | doio/rwtest 使用 shell、df、锁、随机/多进程 I/O；容易 timeout/noise。 | **reserve only**；若 leader 必须扩批，再单独跑。 |
| 14 | `rwtest01` | alias 未单列；`rwtest`/`doio` 四路存在；不在 stable | 60s sync I/O，按 `$TMPDIR/rw-sync-$$` 造文件。 | 明显长时 stress，可能拖慢或触发 runtime noise。 | **defer**。 |
| 15 | `rwtest02` | alias 未单列；`rwtest`/`doio` 四路存在；不在 stable | 60s buffered I/O。 | 同 `rwtest01`，只是 buffered。 | **defer**。 |
| 16 | `iogen01` | alias 未单列；`rwtest`/`iogen`/`doio` 四路存在；不在 stable | 120s read/write + `-Da -Dv -n 2`，两文件、doio 多进程。 | 长时、多进程、随机 I/O；不符合“低风险补分首批”。 | **defer**。 |
| 17 | `fs_fill` | binary 四路存在；不在 stable | `runtest/fs` 中为填满文件系统类测试。 | 填盘/容量边界测试容易污染后续 case 和日志。 | **reject for this batch**。 |
| 18 | `fs_di` | binary 四路存在；不在 stable | `runtest/fs` 数据完整性类测试。 | 偏 stress/data-integrity；不是轻量 substitute。 | **reject for this batch**。 |
| 19 | `ftest09` | 未在当前 upstream `runtest/fs` / source tree 定位到 case；四路 inventory 也未见。 | 当前 LTP 源只显示 `ftest01`-`ftest08`。 | 无可执行 case/source 锚点。 | **reject**：不要把不存在/不可定位 case 放入 leader run。 |

## 明确不建议首批的邻近项

- `chmod06`、`chmod07`、`fchmod02`、`fchmod06`：四路存在且不在 stable，但旧 RV 记录持续 `FAIL 2` + internal `TCONF=1`，不适合作为“低风险补分”。
- `creat04`、`creat06`、`creat07`：旧 RV 有 `FAIL 1/2` 或 unknown；不比 fs_perms/openfile 更低风险。
- `chown04`：旧 RV `FAIL 2` + `TCONF=1`；uid/gid/chown 负例不清洁。
- `readlinkat02`：虽然四路存在且曾 RV/LA 部分 clean，但最新 leader/task-10 已标记 LA-musl `TFAIL` blocked，不能混入替补批。
- `statfs02`/`statvfs02`/`truncate02`/`mkdir05`/`mkdirat01`/`unlinkat01`/`symlink04`/`symlinkat01`：已在 live stable，不能作为新增候选。

## 建议 gate 规则

1. 首轮只跑上面的 10-case inline batch；任何非 clean wrapper、internal `TFAIL/TBROK/TCONF`、timeout、panic、marker truncation 都停止扩大。
2. 若 `stream02` clean，可回看 FIFO 修复是否应再 scout FIFO/open 相关小批；否则继续把 FIFO 类移出 stable460 计划。
3. 若 `fs_perms04`-`06` clean，再看 `fs_perms01`-`03` 的 exec/shebang 是否 clean；不要反向用 exec clean 推断 write clean。
4. `openfile01` 若 hang 或 futex/thread 相关异常，应移交 process/futex lane；不要与 stress I/O 混跑。
5. `writetest01`/`fs_inod01` 若仍 `FAIL -1`，优先怀疑 script/binary/harness path，而不是直接做 syscall 语义修改。
6. `inode02`/`ftest06` 只作为第二轮诊断：二者都有历史 failure，不应作为填补 stable460 的先手。

## Verification

- PASS：mailbox flow delivered task-12 messages；task-12 task file shows owner/claim `worker-5` token `d717dccf-6970-4bb9-a56b-5400b833855d`。
- PASS：live stable parser：`413 total / 413 unique / 0 duplicates`。
- PASS：sdcard inventory scan：`rv-musl=2820`、`rv-glibc=2840`、`la-musl=2820`、`la-glibc=2840`、`four_way_common=2820`、`four_way_nonstable=2407`。
- PASS：LTP source sparse clone：`git -C /tmp/worker5-ltp-src rev-parse --short HEAD` -> `96f5559`。
- PASS：source/runtest anchors inspected：`runtest/fs`、`runtest/fs_perms_simple`、`testcases/kernel/fs/{stream,ftest,inode,openfile,fs_perms,fs_inod,doio}`、`testcases/kernel/io/writetest`。
- NOT RUN：QEMU/evaluator intentionally not run per task.
- NOT CHANGED：`.omx/ultragoal` and `examples/shell/src/cmd.rs::LTP_STABLE_CASES` intentionally untouched.
