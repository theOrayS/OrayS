# self-check.md 合规审查报告（2026-06-20）

## 结论

**审查结论：REQUEST CHANGES / 严格合规门禁不通过。**

本轮只做审查，不修复代码。按 `self-check.md` 的禁止行为逐条重建审查标准后，未发现生产 syscall 路径中直接以具体 stable LTP case 名（如 `open01`/`fcntl14`）分支或伪造 `TPASS` 的确证；但发现多处足以构成 **self-check 高风险或已确认违规候选** 的实现：固定 synthetic `/dev`/`/proc`/`/sys` 路径与内容、裸用户指针边界、POSIX ABI 错误返回、权限/元数据/进程语义简化、runner 准备失败可缺少评分可见失败记录等。

严格标准下，这些问题不能以“目前某些测例能过”作为合规理由；后续修复必须保留真实能力并暴露真实失败，不能转为 fake fail/fake pass。

## 审查纪律与独立轮次

- **只读审查**：未修改实现代码。
- **不信任既往审查**：没有把旧报告结论当证据；所有条目均回到当前源码复核。
- **不使用既有辅助审查脚本**：没有运行仓库内 `scripts/check_g*.py` / `scripts/test_g*.py` 作为结论依据。本轮重新编写了 3 个临时 `/tmp/selfcheck_audit_20260620_*.py` 脚本，SHA256 记录在 `static-scan-summary.json`。
- **三轮以上独立审查**：
  1. 主线第 1 轮：从 `self-check.md` 建立禁止行为清单，解析 runner/stable case，并运行新写静态扫描。
  2. 并行独立轮：4 个审查子 agent（按要求以 `gpt-5.5` + `xhigh` 请求启动）分别覆盖 runner/evaluator、POSIX/syscall/kernel、ulib/libc/API、adversarial coverage。
  3. 主线第 2 轮：高信号扫描，逐项回到当前源码验证子审查发现。
  4. 主线第 3 轮：对 false positive 与非问题做反证，生成 1000 个 stable case 的逐项名称扫描表。

## 覆盖与证据产物

- 当前 live `LTP_STABLE_CASES`：**1000** 个，重复数 **0**。
- `stable-case-name-scan.csv`：1000 行逐项记录每个 stable case 名称在允许列表外的出现情况。
  - 允许列表外总出现：8。
  - 生产逻辑出现：0。
  - 8 个均为注释或普通词碰撞（如 `stress`/`ftest03`/`ftest07` 注释、`dirty` 普通词）。
- `static-scan-summary.json`：本轮新写脚本的扫描摘要和脚本 SHA256。
- 第 1 轮扫描：384 个源码/脚本/配置文件；初始命中 HIGH 158、MEDIUM 325、LOW 928（含大量需人工排除的宽泛命中）。
- 第 2 轮高信号扫描：436 个命中，集中在 runner 控制面、固定 synthetic FS/proc/dev、裸用户指针、固定元数据、测试 marker 输出。

## self-check 条款映射

本报告使用下列简称：

- **SC1.1**：对测试程序名称或二进制特征特殊判断。
- **SC1.2**：对特定 syscall 参数、输入、目录结构硬编码。
- **SC1.3**：仅针对高分测试集合局部实现，忽略通用语义。
- **SC1.4**：利用固定路径、固定时间、固定顺序、环境差异投机。
- **SC1.5**：为通过测试破坏 Linux syscall 兼容语义或基本安全边界。
- **SC1.6**：非通用方式绕过真实进程、内存、文件系统、同步机制。
- **SC1.7**：其他以猜测测例、适配测例、硬编码测例为目标的投机实现。
- **SC2.1-SC2.6**：牺牲兼容、硬编码结果、跳过权限/资源/错误处理、隐藏崩溃或数据错误、固定测例优化、不正确/不通用/不可解释优化。

## 确认发现

### CRITICAL-1：非标准 `/dev/foo/bar` 被硬编码为 ZeroDev

- **位置**：`kernel/fs/axfs/src/mounts.rs:6-18`
- **证据**：`devfs()` 除标准 `null`/`zero`/`urandom` 外创建 `foo` 目录，并把 `bar` 绑定到 `ZeroDev`。
- **对应条款**：SC1.2、SC1.4、SC1.7、SC2.2、SC2.5。
- **判断**：`/dev/foo/bar` 不是 Linux 标准设备节点。当前源码没有说明它是通用内核功能或真实设备抽象；严格看属于固定目录结构/固定结果硬编码。
- **影响面**：devfs 枚举、设备节点 open/read/stat、隐藏路径探测。

### CRITICAL-2：内核 procfs `/proc/self/stat` 固定 PID/comm/state

- **位置**：`kernel/fs/axfs/src/mounts.rs:111-117`, `kernel/fs/axfs/src/mounts.rs:151-152`
- **证据**：`proc_self_stat()` 返回固定字符串 `1 (arceos) R ...`。
- **对应条款**：SC1.1、SC1.2、SC1.5、SC1.6、SC2.2、SC2.5。
- **判断**：这是进程可见语义，固定 PID 和进程名会与 `getpid`/fork/exec/thread 状态不一致。
- **边界说明**：`examples/shell/src/uspace/synthetic_fs.rs:372-416` 为 shell uspace 主路径实现了动态 `/proc/<pid>/stat`，因此不能把 shell 路径等同为固定；但底层 `axfs` procfs 仍保留固定假象，属于需要清理或隔离的合规风险。

### HIGH-1：`/proc/sys/*` 与 `/sys/*` 多处固定能力文本无统一真实后端

- **位置**：`kernel/fs/axfs/src/mounts.rs:45-109`, `kernel/fs/axfs/src/mounts.rs:187-214`
- **证据**：固定写入 `sem`、`pid_max`、`shmmax`、`overcommit_memory`、transparent hugepage、clocksource 等内容。
- **对应条款**：SC1.2、SC1.3、SC1.4、SC1.6、SC2.2、SC2.5。
- **判断**：这些文件会影响用户态能力探测。部分 shell uspace 路径已有可读写状态或 SysV 后端联动（如 `examples/shell/src/uspace/synthetic_fs.rs:1113-1351`、`sysv_*`），但底层 kernel mount 仍有固定文本。

### HIGH-2：`api/arceos_posix_api` 用户指针边界主要只做 null 检查

- **位置**：`api/arceos_posix_api/src/utils.rs:8-15`, `44-56`, `75-111`
- **证据**：`CStr::from_ptr`、`read_unaligned`、`write_unaligned`、`from_raw_parts(_mut)` 在 null 以外没有验证用户地址空间、跨页、权限。
- **对应条款**：SC1.5、SC1.6、SC2.3、SC2.4。
- **判断**：隐藏坏指针、只读页写入、跨页 unmapped 等条件下可能 trap 或越权访问。
- **边界说明**：`examples/shell/src/uspace/user_memory.rs:51-179` 有独立 `fault_in_user_range`/`can_access_range`/`populate_range` 机制，shell 主 syscall 路径不能按该旧 API 层直接定罪；但通用 POSIX API 层仍是高风险。

### HIGH-3：pthread mutex 直接把用户地址 cast 为内核引用

- **位置**：`api/arceos_posix_api/src/imp/pthread/mutex.rs:56-58`, `168-198`
- **证据**：`Ok(unsafe { &*mutex.cast::<Self>() })` 后直接执行 atomic lock/unlock。
- **对应条款**：SC1.5、SC1.6、SC2.3、SC2.4。
- **判断**：仅 null 检查不足以保证对齐、映射、生命周期、process-shared/robust 等语义。

### HIGH-4：`open(..., mode)` 创建权限参数未传播

- **位置**：`api/arceos_posix_api/src/imp/fs.rs:218-260`, `kernel/fs/axfs/src/fops.rs:60-95`
- **证据**：`flags_to_options(flags, _mode)` 参数名带 `_`，`OpenOptions` 默认 `_mode: 0o666`，没有从 syscall mode 写入创建权限。
- **对应条款**：SC1.5、SC2.1、SC2.3、SC2.5。
- **判断**：`mode`/umask/权限位是 Linux/POSIX 可见语义；忽略会让 `open`/`creat`/`stat`/权限测试得到非调用者请求的结果。

### HIGH-5：stat/权限/rename 语义过度合成或非原子

- **位置**：
  - `api/arceos_posix_api/src/imp/fs.rs:45-90`：uid/gid 固定 0，inode 由路径 hash，时间戳默认。
  - `kernel/fs/axfs/src/root.rs:470-531`：删除/chdir 权限主要看 owner bits。
  - `kernel/fs/axfs/src/root.rs:539-562`：`rename` 先删除目标再 rename。
- **对应条款**：SC1.2、SC1.5、SC2.1、SC2.2、SC2.3、SC2.4、SC2.5。
- **判断**：这些不是 case-name hack，但会破坏通用 Linux 语义；特别是 rename 失败时目标可能已被删除。

### HIGH-6：`getpid` 在通用 POSIX API 层使用 task id 或固定 1

- **位置**：`api/arceos_posix_api/src/imp/task.rs:19-38`
- **证据**：注释说明无 multi-process object；实现使用 `axtask::current().id()`，fallback `unwrap_or(1)`/`Ok(1)`。
- **对应条款**：SC1.5、SC1.6、SC2.1、SC2.5。
- **判断**：pthread-style task id 与 Linux process id 不是同一语义；多线程下容易与 `gettid`/`/proc`/wait 语义冲突。

### HIGH-7：`getcwd` 错误路径可把负 errno cast 成非 NULL 指针

- **位置**：`api/arceos_posix_api/src/imp/fs.rs:371-385`, `ulib/axlibc/src/fs.rs:54-58`, `api/arceos_posix_api/src/utils.rs:138-142`
- **证据**：`sys_getcwd` 通过 `syscall_body!` 把 `Err(e)` 转成负整数；libc `getcwd()` 直接返回指针，没有 `e()`/errno 映射；`buf == NULL` 返回 `Ok(NULL)`。
- **对应条款**：SC2.3、SC2.4、SC2.6。
- **判断**：size 不足等错误可能返回形如 `0xffff...` 的非 NULL 指针；NULL buffer 也没有实现 GNU 分配语义或显式失败。

### HIGH-8：`dup3` 忽略非法 flags 并忽略 `F_SETFD` 失败

- **位置**：`ulib/axlibc/src/fd_ops.rs:29-44`
- **证据**：除 `old_fd == new_fd` 外，未检查 `flags & !O_CLOEXEC`；设置 `FD_CLOEXEC` 的 `sys_fcntl` 返回值被忽略。
- **对应条款**：SC1.2、SC2.1、SC2.3。
- **判断**：非法参数组合应 `EINVAL`，不能假成功。

### HIGH-9：`fflush(NULL)` 可空指针解引用；`puts` 吞掉换行写失败

- **位置**：`ulib/axlibc/c/stdio.c:53-56`, `122-124`, `148-162`
- **证据**：`fflush(FILE *f)` 直接调用 `__fflush(f)`，随后 `__write_buffer` 访问 `f->buffer_len`；`puts` 不检查第二次 `write("\n")` 返回值。
- **对应条款**：SC2.3、SC2.4、SC2.6。
- **判断**：合法 POSIX 用法和错误路径可能崩溃或报告假成功。

### HIGH-10：LTP suite 准备/选择失败可能缺少评分可见失败记录

- **位置**：`examples/shell/src/cmd.rs:2226-2236`, `examples/shell/src/cmd.rs:2525-2528`
- **证据**：`run_ltp_suite()` 在 `OS COMP TEST GROUP START` 之前执行 busybox wrapper 准备、helper bin 准备、`selected_ltp_cases`；失败时调用方只打印 `autorun: ltp suite failed: ...`。
- **对应条款**：SC2.3，以及“不隐藏 TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap”的仓库红线。
- **判断**：不是 fake PASS，但 LTP setup/selection failure 可能不进入稳定 case/group fail matrix。

### MEDIUM-1：畸形 inline `LTP_CASES` 可能静默回退到 `core`

- **位置**：`examples/shell/src/cmd.rs:1348-1363`
- **证据**：`file:` 空列表会报错；但 inline spec 经 `split_ltp_case_list(spec)` 解析为空后落到 `Ok(("core", LTP_CORE_CASES))`。
- **对应条款**：SC1.3、SC1.4、SC2.3。
- **判断**：测例选择入口应显式失败，不应从畸形非空输入退回更小固定集合。

### MEDIUM-2：official group 过滤解析失败静默视作全部组

- **位置**：`examples/shell/src/cmd.rs:1377-1391`
- **证据**：`/test_groups.txt`/`OSCOMP_TEST_GROUPS` 解析失败或空列表时返回 `None`。
- **对应条款**：SC1.4、SC2.3。
- **判断**：这不会跳过测试，反而可能跑全部；但错误输入没有显式暴露，属于 runner 控制面弱点。

### MEDIUM-3：`sys_nanosleep` 注释承认缺少信号唤醒语义

- **位置**：`api/arceos_posix_api/src/imp/time.rs:89-120`
- **证据**：注释 `TODO: should be woken by signals, and set errno`；实现 sleep 后仅按实际睡眠不足返回 `EINTR`。
- **对应条款**：SC2.1、SC2.3。
- **判断**：真实 POSIX 语义缺口，可能造成信号/timeout 隐藏行为。

### MEDIUM-4：TCP `sendto` 对目标地址参数不校验

- **位置**：`api/arceos_posix_api/src/imp/net.rs:706-714`
- **证据**：TCP socket 分支在 `from_sockaddr(socket_addr, addrlen)` 前直接 `return socket.send(buf)`。
- **对应条款**：SC2.1、SC2.3。
- **判断**：TCP `sendto` 的非空 dest addr、坏指针、长度错误等 Linux 兼容细节可能被吞掉。

### MEDIUM-5：stdin readiness 固定为 readable

- **位置**：`api/arceos_posix_api/src/imp/stdio.rs:146-150`
- **证据**：`poll()` 返回 `readable: true`。
- **对应条款**：SC2.1、SC2.5。
- **判断**：无输入时 select/poll 可能误报 ready，绕过阻塞/timeout 语义。

### MEDIUM-6：`accept4` 忽略 `fcntl` 结果

- **位置**：`ulib/axlibc/c/socket.c:9-24`
- **证据**：`SOCK_CLOEXEC`/`SOCK_NONBLOCK` 分支调用 `fcntl` 后不检查返回值。
- **对应条款**：SC2.3。
- **判断**：如果设置失败仍返回 fd，会假成功。

## 高风险但本轮不直接定性为违规的项

1. **固定 stable LTP 集合与远程默认 stable**
   - 位置：`examples/shell/src/cmd.rs:50-619`, `Makefile:84-98`。
   - 风险：表面符合 SC1.3/SC1.4 的“固定高分集合”形态。
   - 本轮定性：由于 runner 明确输出 `ltp case list` manifest，且 blacklist/sweep promotion 由 `scripts/ltp_summary.py:411-418` 阻断，本轮不判定为隐藏 PASS；但所有 promotion 仍必须禁止把 blacklist/sweep/TCONF/timeout 当 stable 通过。

2. **`FAIL LTP CASE {case} : 0` + `Pass!`**
   - 位置：`examples/shell/src/cmd.rs:2298-2306`。
   - 风险：输出形态异常，可能被外部 scorer 误解。
   - 本轮定性：当前代码以真实进程 exit code 为来源，非零和 timeout 仍输出 FAIL/TIMEOUT；`scripts/ltp_summary.py:115-127` 也按 numeric code 分类。本轮不判定为 fake PASS，但要求用官方 scorer 证据持续复核。

3. **`LTP_COLORIZE_OUTPUT=1`**
   - 位置：`examples/shell/src/cmd.rs:1868-1874`。
   - 本轮定性：注释与实现目的为保留官方可解析的 `TPASS/TFAIL/TBROK/TCONF` 呈现，不是隐藏内部结果。

4. **固定 suite 顺序，LTP 最后**
   - 位置：`examples/shell/src/cmd.rs:2156-2180`, `2437-2444`。
   - 风险：SC1.4 固定顺序高风险。
   - 本轮定性：注释解释为避免长 LTP 饿死 non-LTP；不是 case-name fake pass，但必须保持透明并避免用顺序隐藏失败。

5. **`/musl`、`/glibc`、loader 名称分支**
   - 位置：`examples/shell/src/uspace/runtime_paths.rs:37-55`, `182-258`; `examples/shell/src/uspace/program_loader.rs:74-81`, `310-325`。
   - 风险：固定路径/二进制特征分支。
   - 本轮定性：这是 dual-libc runtime 支撑面，不是按测例名返回 syscall 结果；但不应扩大为只服务测评目录的语义分支。

6. **shell uspace synthetic `/proc`**
   - 位置：`examples/shell/src/uspace/synthetic_fs.rs:372-416`, `568-590`, `988-1351`。
   - 本轮定性：shell 主路径的 `/proc/<pid>/stat`、部分 `/proc/sys` 不是简单固定文本，存在进程对象和状态联动；但仍需动态测试验证完整 Linux 行为。

## 未发现的直接违规证据

- 未发现生产 syscall 路径中 `if case == "open01"` / `if name == "fcntl14"` 这类按具体 stable case 名分支。
- 未发现生产 runner 伪造 `TPASS` 或隐藏 `TFAIL/TBROK/TCONF` 的直接代码。
- 未发现 stable case 名称在允许列表外进入生产逻辑；`stable-case-name-scan.csv` 逐项覆盖 1000 个 case，生产逻辑外部出现数为 0。

## 未覆盖与残余风险

- 本轮未运行 QEMU、官方远程 scorer、LTP/libctest 动态测试；报告是源码合规审查，不是分数证明。
- 未审查生成物、二进制反汇编、`target/`、`build/`、旧 `archive/docs-pre-review-2026-06-28/archive/`、`cargo-home/`、大部分第三方 `vendor/`。
- 没有逐条建模 1000 个 LTP case 的完整语义需求；本轮逐项覆盖的是 **case-name special-casing 扫描**，语义审查按 syscall/FS/process/memory/libc 子系统展开。
- 静态扫描存在 false positive；本报告只列已人工回源验证的条目。

## 建议后续修复优先级（本轮不修）

1. 先修合规红线：`/dev/foo/bar`、固定底层 `/proc/self/stat`、LTP setup failure 可见性、`getcwd`/`dup3`/`fflush(NULL)` 假成功或崩溃路径。
2. 再修 POSIX 通用语义：用户指针验证、open mode/umask、rename 原子性、stat 元数据、权限模型、getpid/process identity。
3. 所有修复必须保留能力：不能把原本能工作的路径简单改成硬失败；unsupported 必须诚实返回 errno，不能 fake PASS。
4. 修复后至少运行：新静态 guard、targeted libc/POSIX 单测、`make A=examples/shell ARCH=riscv64`，涉及 evaluator 输出再跑 `scripts/ltp_summary.py` 和官方 runner 等价验证。
