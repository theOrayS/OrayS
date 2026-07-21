---
title: "PR draft: OS competition final-2026"
date_started: 2026-07-20
date_completed: null
status: draft
pr: null
branch: "dev/oscomp-final-2026"
authors: ["OpenAI Codex (AI-assisted; human owner pending)"]
reviewers: []
base_commit: "d9891d0254b5663436ef53893c105138fc7f009f"
head_commit: "1b5aa307"
capability_domains: ["process", "memory", "filesystem", "procfs", "time", "network", "dynamic-linking", "toolchain", "scheduler", "riscv64", "loongarch64"]
---

# 1. 背景与目标

## 背景

final-2026 由 CAgent 与 BuildStorm 两题组成，仅在 glibc guest 中计分，并分别在
RISC-V64/LoongArch64 上执行。现有仓库 canonical official runner 面向旧 24-group
初赛协议，不含 final-2026 case。任务合同要求从不可回退基线开始，先得到真实未修改
分数，再按通用 RED -> GREEN 持续提高得分，直至满分完整回归或外部阻塞。

## 目标

- 建立 fail-closed、可复算、双架构对等的 final-2026 本地证据入口。
- 真实运行 CAgent 10 项与 BuildStorm toolchain/minibuild/full compile，保留首次失败。
- 只通过规范 Linux/POSIX/ABI/内核修复和可解释性能优化提高得分。

## 非目标

- 不修改外部测例、judge、镜像或评分器，不冒充官方 artifact。
- 不硬编码 prompt/答案/版本/CPU/时间/分数，不由 host 代跑 guest 工作。
- 不修改其他 worktree、main/stabilize、依赖/工具链或无关代码；不 push/PR。

## 验收标准

- [ ] 两架构 CAgent 10/10 真实通过并记录时间与 reference judge 明细；参考权重的实际
      脚本化满分为 199.1，名义题目总分 200 的差异单独保留，不修改外部公式。
- [ ] 两架构 BuildStorm toolchain/minibuild/full compile 成功，真实 elapsed_s 可复算。
- [ ] 每项修复具有通用 red/green、双架构验证、clean quick/baseline 与独立 review。
- [ ] 同一 clean 候选上 full-all 与全部适用 final 门禁明确 PASS，无隐藏非 PASS。

# 2. 基线

| 时间 | 命令 | 架构/目标 | 退出码 | 结果 | 备注/证据 |
|---|---|---|---:|---|---|
| 2026-07-20 | `git status --short --branch`; `git rev-parse HEAD`; `git rev-parse stabilize/post-integration-gates-20260716` | Git | 0 | PASS | `dev/oscomp-final-2026`，clean，三个值均为 `d9891d02...` |
| 2026-07-20 | `git -C /root/testsuits-for-oskernel-final-2026 status --short --branch`; `rev-parse HEAD` | 外部协议 | 0 | PASS | clean `final-2026@15e0355b...`；全程只读 |
| 2026-07-20 | `python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py --list` | 现有 suite | 0 | PASS | 59 case；仅旧 24-group official，无 final-2026 case |
| 2026-07-20 | QEMU/qemu-img/Rust/Python/Make/Git 版本探针 | host | 0 | PASS | RV/LA QEMU 与 qemu-img 9.2.4；pinned Rust 1.89 nightly；Python 3.11.15 |
| 2026-07-20 | `lscpu`; `free -h`; `df -h /root /tmp` | BuildStorm host | 0 | BLOCKED | 2 物理核、3.6 GiB RAM、无 swap、约 7.2 GiB 可用；不满足 8c/8G 官方合同 |
| 2026-07-20 | `sha256sum /root/sdcard-{rv,la}.img`; `debugfs -R 'ls -l /'` | 既有镜像 | 0 | BLOCKED | 哈希为旧初赛固定值；4 GiB ext4 根仅 `/musl`、`/glibc`，不是 final Debian/toolchain rootfs |
| 2026-07-20 | `python3 ... test/run_suite.py --profile quick --output-dir test/output/oscomp-final-14a8d8da-quick-1` | clean `14a8d8da` | 0 | PASS | planned/executed/completed=45/45/45，45 PASS，355.787892 s；`summary.json` SHA-256 `9a5d3ac1...8b8f` |

未修改 d989 的 final-2026 分数矩阵（不能用旧镜像或历史叙述推断 PASS）：

| 架构 | CAgent | BuildStorm toolchain/minibuild/compile | 分数 |
|---|---|---|---|
| RISC-V64 | BLOCKED：缺官方 final rootfs | BLOCKED：缺 rootfs 且 host 不足 8c/8G | 未执行，不计分 |
| LoongArch64 | BLOCKED：缺官方 final rootfs | BLOCKED：缺 rootfs 且 host 不足 8c/8G | 未执行，不计分 |

官方参考命令为 guest 内 `scripts/cagent_testcode.sh` 和 `scripts/buildstorm_testcode.sh`，
并分别由 `judge_cagent-glibc.py`、`judge_buildstorm-glibc.py` 复算。仓库侧尚无对应 final
launcher；当前唯一官方 image 来源是外部 README 指向的比赛方 Baidu 链接，本轮不下载
或使用来源不明镜像。

# 3. 设计与决策

## 方案

新增独立于旧 official 的 final-2026 manifest/runner/parser：固定外部协议 SHA，验证镜像
provenance，使用只读 backing + unique overlay，以 8c/8G 启动两架构；严格解析 CAgent
10 个身份和 BuildStorm 结构化结果，保留原始 stdout/stderr、child rc、timeout 与分数。

## 备选方案

- 直接把现有 4 GiB 初赛镜像当 final：拒绝，内容与官方 README 不符。
- 在 host 运行脚本后喂给 judge：拒绝，这不验证 OrayS guest。
- 只运行 reference judge 而不做结构完整性验证：拒绝，重复/缺失记录可能被覆盖或漏报。

## 关键决策

| 决策 | 理由 | 风险 | 回滚方式 |
|---|---|---|---|
| final 与旧 official profile 分离 | 协议和镜像完全不同 | 新增入口维护成本 | 普通 revert 新逻辑提交 |
| 缺镜像/资源显式 BLOCKED | 防止 fake PASS/错误分数 | 暂无官方 baseline | 完成全部不依赖项，资源到位后恢复 |
| correctness 后再优化 BuildStorm | 时间分依赖成功构建 | 早期得分增长慢 | 每个根因/优化独立提交 |

# 4. 开发与调试记录

## 2026-07-20 — Checkpoint 1：合同、协议与基础设施基线

- 修改：创建本活动计划与开发日志；尚未修改生产代码、runner、parser 或测试业务逻辑。
- 观察：仓库/外部 SHA 均精确；QEMU 版本满足既有固定要求，但 final 镜像不存在，host
  CPU/内存/磁盘也不满足 BuildStorm 官方执行条件。现有 suite 不发现 final case。
- 问题：无法诚实获得未修改 d989 的 final 分数；旧镜像只能证明旧 24-group 环境存在。
- 根因：比赛方 final rootfs 是外部大体积 artifact，未置于 `/root`；当前执行环境仅 2c/3.6 GiB。
- 解决：把四个架构/题目组合显式标为 BLOCKED，同时继续实现 fail-closed runner、parser、
  静态/单元测试、双架构构建和通用 guest 回归，不放宽结果语义。
- 对应文件/提交：本计划、本日志；提交待创建。
- 下一步：审计旧镜像 payload 与 current runner/guest 启动路径，先写 final parser/runner RED tests。

## 2026-07-20 — Checkpoint 2：final 结果解析器 RED -> GREEN

- test-only RED：提交 `36d95801ec8fca043944ba5b03d107a2a8035ce0` 新增 24 个 CAgent/
  BuildStorm 正负 fixture，并将 suite 固定 inventory 从 59 case 扩到 60；未提供解析实现。
- RED 命令：`python3 -I -S -B -X pycache_prefix=/dev/null test/run_unittest_suite.py
  test/unit/test_final_2026_result_validation.py`，clean `36d95801`，退出码 2，0.42 s；因
  `ModuleNotFoundError: parse_final_2026_results` 失败。证据
  `test/output/oscomp-final-36d95801-red-1/focused.log`，SHA-256
  `15d1361ed7d522e560f9757717e2f1ced63fa75adcd330a01e89dda4a5a08b13`。
- GREEN 实现：提交 `e303653be8c4941b157d1e3a9a87587fa01ba77f` 新增
  `test/evaluation/parse_final_2026_results.py`。它固定外部协议 commit，要求精确 group
  lifecycle、CAgent 10 个唯一 identity、BuildStorm toolchain/minibuild/compile 结构，
  并拒绝缺失、重复、未知、畸形、stderr 协议记录、致命尾迹、非 8 核、架构不符和成功但
  artifact 小于 500000 bytes 的矛盾证据。
- GREEN 命令：同一 focused harness，clean `e303653b`，退出码 0，24/24 PASS，0.46 s；
  证据 `test/output/oscomp-final-e303653b-green-1/focused.log`，SHA-256
  `9c654c807d39ee95a170efb3cc1cb2ba85c720e6b0fd613affd7aff99ac214bb`。
- 完整性：`test/checks/check_test_asset_integrity.py` 在 clean `e303653b` 退出码 0，
  `PASS (0 findings)`，0.25 s；证据
  `test/output/oscomp-final-e303653b-parser-checks-1/asset-integrity.log`，SHA-256
  `c76cd12c2fcf287c072dc06a1f0bd2c106784fef61713a68ac4e3fa3cd78ecac`。
- 外部公式对算：把相同合成协议输出分别输入只读 reference judge 与新 parser；CAgent
  10 项快路径均为 199.1，BuildStorm `elapsed_s=600`、baseline 400 均为 120.0，退出码 0。
  CAgent reference 权重加 10% bonus 后最大值确为 199.1；不把名义 200 硬改进解析器。
- 本 checkpoint 只建立结果真实性边界，没有运行 guest，也没有产生任何 RV/LA final 分数。
- 下一步：先写 runner contract/profile mutation RED tests，再接入四个架构/题目 case 与
  fail-closed adapter；正式镜像仍缺失，因此 adapter 的预期基线是 `BLOCKED`，不是 PASS。

## 2026-07-20 — Checkpoint 3：runner 合同与真实 CAgent 帧名

- runner RED：提交 `393a09898fa6bd96feabd094b1cff2469c28fe58` 把 focused suite 扩至
  31 项，先覆盖 final 合同字段、PASS/FAIL/ERROR 映射、非零 child rc、封闭环境继承和
  禁止 official scouting 控制。clean RED 退出码 1，4 FAIL/2 ERROR；证据
  `test/output/oscomp-final-393a0989-runner-red-1/focused.log`，SHA-256
  `43d38515d60d2e2bc94e7500e4cff79c0075eaa63ff1d276fdee8b6d85542fa8`。
- runner GREEN：提交 `e0880debad226929639d63906a5140caa811e0cc` 接入 `final_2026`
  structured result contract，只继承五个 provenance 环境变量，并保留“结构 PASS 但进程
  非零”作为 `INFRA_ERROR`。focused 31/31 PASS，SHA-256 `d85888c6...a251`；旧
  `test_suite_runner.py` 135/135 PASS，SHA-256 `e969a6b5...219`；evaluation runner/parser
  integrity 24/24 PASS，SHA-256 `2c8f171b...3466`。
- 协议复核发现外部 `Makefile` 会把 CAgent 镜像内帧名从 `cagent` 改写为
  `cagent-glibc`。提交 `2b7af950` 先将 32 项 fixture 绑定真实帧名；clean RED 退出码 1，
  3 FAIL/26 ERROR，证明旧实现无法消费真实 CAgent 输出。证据
  `test/output/oscomp-final-e0880deb-label-red-1/focused.log`，SHA-256
  `f74eb2dec20fa23e9ec00114a9b555d9a37f88fc8aa104a891e17e1a0258cb83`。
- 提交 `969628cfb681171f626136144cda86eaa8d75f1c` 将语义 group 与生命周期 label
  分开，并把 CAgent 固定为 `cagent-glibc`、BuildStorm 固定为 `buildstorm`。clean GREEN
  32/32 PASS，0.67 s；证据 `test/output/oscomp-final-2b7af950-label-green-1/focused.log`，
  SHA-256 `21a03663ccffe2f80cede52ffa073f0a99e161b8806fc71ba396c3e34d6f0f14`。
- 本 checkpoint 仍只验证真实协议边界，没有 guest 得分。下一步是 canonical 四 case 与
  provenance/overlay adapter 的测试先行实现。

## 2026-07-20 — Checkpoint 4：只读 adapter 与四格 canonical profile

- adapter RED：提交 `50276425e4af431291bd5cb47ed6576e729a6f3d` 先定义七项合同测试；
  clean `87405990` 因 `ModuleNotFoundError: run_final_2026_evaluation` 退出码 2。证据
  `test/output/oscomp-final-87405990-adapter-red-1/focused.log`，SHA-256
  `27860c1e4d1b4f6e4c67923e1b665f04f85dcb06e4d267645be9c28747d12505`。
- adapter GREEN：提交 `623c6028bc215970d3d74a6f8ac99d8dfe3ee2ae` 新增只读 image manifest、
  精确 SHA/文件名/架构/题目绑定、旧初赛 hash 拒绝、外部协议 clean exact-SHA 检查、unique
  qcow2 overlay、进程组 timeout、backing 前后 hash、CAgent 1c/1G 与 BuildStorm 8c/8G 启动。
  BuildStorm 在 host 上还要求至少 8 个物理/在线 CPU、9 GiB 可用内存和 4 GiB 输出空间。
  focused 7/7 PASS，证据 SHA-256 `7fb9cae1914768f27c99674bbcf01a07cd6b02f95c751b65eb2ce827bafc1188`；
  asset integrity PASS，SHA-256 `71aa1fa788f23e54da75d7fcfed24506c46db9af62cdac20f25fe6bc91589d66`。
  缺 image 的 adapter 探针退出码 125，stderr SHA-256
  `fd8873ed999bdb5a53feeacbd865976c9c59bd7f6f2f25eca7c2d2b1fffbff31`。
- profile RED：提交 `3221963fa200baede61f32842a2e38a8eb65e13c` 先固定四个 case、顺序、
  group-specific image env、命令、资源、timeout 与 `full` 组成；clean RED 为 1 FAIL/2 ERROR，
  证据 SHA-256 `959f0b2436f1e602ea2b4b00345ce3f59924951a82630f33732b3b0729e9db63`。
- profile GREEN：提交 `67ba351db6c14121f924be3f84a76461205ca916` 注册
  `RV/LA x CAgent/BuildStorm` 四 case 和独立 `final-2026` profile。focused 35/35 PASS，
  SHA-256 `7cd322846308549e58001f4ed4714e8297f4f784eca01c283c562880567f8f57`；既有 runner
  regression 135/135 PASS，SHA-256 `9ff414e1ecf21ff8cdca41b5d80f89570aab07ba32ac2ed94bbb6e2ce541a590`；
  asset integrity 再次 `PASS (0 findings)`。
- clean canonical 缺镜像矩阵：`final-2026 --arch all` 精确计划四项，退出码 2；四项均
  `INFRA_ERROR`、`executed=false`，planned/executed/completed/pass/infra 为 4/0/4/0/4。
  `summary.json` SHA-256 `accb4f497861f341d07113eddbda4c81a60b9c6dacdd5f201961ebc01596eadc`。
  没有 guest 执行或得分；该结果验证缺 artifact 不会产生 PASS。
- `official` 仍只运行旧 24-group；根合同要求 `full` 包含全部注册适用测试，因此 `full`
  同时纳入 final 四 case，而不是在缺 final artifact 时错误通过。
- 下一步：用双架构 semantic smoke 证明 `/proc/uptime` 的真实缺失，再以 HAL 单调时间实现
  Linux 可见的动态 uptime；这属于通用 procfs 修复，不依赖或特化 final 测例。

## 2026-07-20 — Checkpoint 5：动态 `/proc/uptime` 与设备 access mode RED -> GREEN

- 修改前语义基线：clean `2fab26fe` 的既有双架构 semantic smoke 均在
  `USER_FAIL tee_device_mode` 终止。RV/LA raw log SHA-256 分别为
  `972d2f07...a770`、`2ececc30...153`。QEMU 进程退出 0 不能覆盖 guest fatal marker，故
  该基线按协议为非 PASS。
- test-only RED：提交 `1ad30429caf6441952b030f89785151935b41748` 在 freestanding guest
  中通过真实 `openat/read/close/nanosleep` 两次读取 `/proc/uptime`，要求恰有两个两位小数
  字段且 uptime 严格前进；manifest 同步要求有序 `ASSERT proc_uptime PASS`。clean RED 的
  RV/LA 均构建 PASS、runtime `ERROR/fatal_runtime_signal`，首个 marker 分别为
  `proc_uptime_open arch=riscv64/loongarch64`。结果 JSON SHA-256 为
  `b0d0ff62...e49`、`f050086a...e34`，耗时 46.083345/43.753724 s。
- uptime GREEN：提交 `6c989eb9be62cb38e9c97c8412a34afb3e26bbe8` 新增动态 proc snapshot；
  uptime 使用 HAL 单调时间，idle 字段累计所有 CPU idle task 的真实 scheduler runtime，
  没有填零、复制 uptime 或固定文本，也没有新增 production `unsafe`。RV/LA 均出现
  `ASSERT proc_uptime PASS`，但完整 case 仍诚实保留为 `ERROR`，因为下一真实失败仍是
  `tee_device_mode`。结果 JSON SHA-256 为 `95ed8314...5f5`、`7c1298e6...cc14`。
- tee 根因：`/dev/null` 的 `FdEntry` 与 `/dev/zero` 不同，打开时丢弃
  `O_RDONLY/O_WRONLY/O_RDWR`，使 read/write、tee 的方向检查和 `F_GETFL/F_SETFL` 无法共享
  真实 open-file access mode。提交 `a3b3d548c03b43ca2a07f30a1cc6f4dcd6a77620` 保存该状态，
  并在 fork 复制、I/O、fcntl、tee snapshot 与 metadata 分支一致传播；未按测例或路径输出
  特化 errno。
- 最终 clean GREEN：相同 semantic evidence case 在 RV/LA 均得到 build + runtime 2/2 PASS，
  `required_nonpass=0`，并完整出现 uptime、splice、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`。
  RV 结果/raw SHA-256 为 `cffd647b...dfb9`/`a5e6d354...e845`，44.581134 s；LA 为
  `ea7a98c4...cc14`/`654cd5c6...90fc`，42.522110 s。证据分别位于
  `test/output/oscomp-final-a3b3d548-tee-green-{rv,la}-1/`。
- 定向 host 检查：semantic evidence 75/75、competition evidence 33/33、competition
  workflow guard 与 test asset integrity 均 PASS；RV/LA smoke kernel 均实际编译成功。
- 本 checkpoint 仍不是 final 官方得分：官方 rootfs 缺失，BuildStorm host 资源不达标。
  下一步继续做不依赖该 artifact 的 8-vCPU SMP/动态 ELF/rootfs 行为审计与通用回归。

## 2026-07-20 — Checkpoint 6：8-vCPU task affinity RED -> GREEN

- 协议审计：final BuildStorm 脚本用 BusyBox `nproc` 记录并要求 8 核；BusyBox 通过
  `sched_getaffinity(0, sizeof(mask), mask)` 统计 bit。旧生产实现却无条件清空用户 buffer、
  只写 `[1]`，`sched_setaffinity` 也只检查 bit0 后返回成功，未修改任务调度 mask。
- SMP 能力隔离探针：clean `c6aa2cc0` 分别以 `KERNEL_SMP=8` 构建 RV/LA smoke kernel，
  两架构均识别 `CPU number: max = 8, platform = 8, use = 8`，启动并初始化全部次级 CPU，
  旧 semantic protocol 完整 PASS。该结果只证明 8-vCPU 内核能力，不冒充官方得分。
- test-only RED：提交 `eea04c5741be37a0ffe78167fab04e991bcfa5ce` 将 canonical semantic
  smoke 默认设为 8 vCPU，raw syscall 要求初始 `0xff` mask、绑到 CPU 7 后读回 `0x80`、
  恢复后读回 `0xff`，并验证 syscall 只写返回的一个 unsigned-long，不覆盖尾随 sentinel。
  host semantic 75/75、competition 33/33、workflow guard 与 asset integrity 均 PASS。
- clean RED：RV/LA 均实际启动 8 CPU、smoke build PASS，但 runtime 在
  `USER_FAIL sched_affinity_mask` 处 fail-closed 为 `ERROR/fatal_runtime_signal`；结果 JSON
  SHA-256 为 `0934032a...f57d`、`124b687d...25c2`，raw log SHA-256 为
  `639ee9b1...004b`、`8cb9f5f2...b17e`。证据位于
  `test/output/oscomp-final-eea04c57-affinity-red-{rv,la}-1/`。
- GREEN 实现：提交 `5a98b24a7f3f1fdcf7b9069c59ffc252a7fc4654` 暴露 runtime online
  scheduler mask；`sched_getaffinity` 返回目标任务真实 cpumask、使用 Linux word-aligned
  宽度且只写该宽度；`sched_setaffinity` 解析完整 word、与在线 CPU 相交、拒绝空集合，
  并更新真实 task cpumask。当前任务不在新集合时走既有 migration path，没有固定 `nproc`
  输出、CPU 数或无条件成功。
- clean GREEN：RV/LA 均为 build + runtime 2/2 PASS、`required_nonpass=0`，完整出现
  `ASSERT sched_affinity PASS`、后续 uptime/splice/uname、`USER_PASS`、`HARNESS_PASS` 和
  `SHUTDOWN`。结果 JSON SHA-256 为 `920df48b...cca0`、`c4e02e7c...38d7`；runtime raw
  SHA-256 为 `ccbe1a2d...8a6f`、`26c4d004...e910`，证据位于
  `test/output/oscomp-final-5a98b24a-affinity-green-{rv,la}-1/`。
- 仍无官方 final 得分：正式 rootfs 未取得且 host 不满足 BuildStorm 资源合同。下一步继续
  动态 ELF/interpreter/rootfs 行为审计与通用 guest 回归。

## 2026-07-20 — Checkpoint 7：CAgent 系统 applet 的真实 PATH 入口

- 只读审计外部 `cagent_testcode.sh`、`agent_lite` 与 `simple_llm_server` 源码确认：runner
  用受限 `PATH={wrapper-dir}:.` 启动 shell；评分 prompt 实际调用 `date`、`df`、`nproc`、
  `awk`、`find`、`grep`、`ls`、`mkdir`、`printf`、`touch`、`wc` 等真实程序。两架构官方
  BusyBox config 都启用 `DF` 与 `NPROC`，但当时仓库的 PATH wrapper inventory 漏掉这两项。
- test-only RED `8ba1966e` 将 `df`、`nproc` 纳入 runner/parser integrity 合同；GREEN
  `e007b2c6` 只为官方 BusyBox 已提供的这两个 applet 创建真实 wrapper，不伪造输出、不按
  prompt 返回答案。`ss` 在官方 BusyBox config 中不存在，因此没有用 `netstat` 冒名或添加
  特化实现。`dd520379` 将新增测试方法数固定为 25，`b0001898` 同步已审查的 affinity
  copy-boundary inventory。
- clean `b0001898` canonical quick 为 47/47 PASS，runner provenance stable，累计 case
  duration 333.251882 s；summary SHA-256
  `830c0f5df539316cf462b745241eaa6c658293b1997c9ee0d7bd88a219c10861`，位于
  `test/output/20260720T200022Z-quick-none-3026875/summary.json`。
- 同一 clean commit 的 RV/LA canonical evidence-runtime 均为 1/1 PASS，duration
  159.082378/166.668069 s；summary SHA-256 为 `8ca5d2d9...ee7d`、`80e216a4...2fd`，
  位于 `test/output/20260720T200609Z-evidence-runtime-rv-3031794/` 与
  `test/output/20260720T200853Z-evidence-runtime-la-3035500/`。
- 这些结果证明 wrapper 和既有 guest 回归未退化；缺正式 rootfs 时不能断言 CAgent identity
  通过或产生分数。

## 2026-07-20 — Checkpoint 8：CAgent 并发网络与 popen 核心链路

- test-only `265984ab74415702f013adb1913b021b85a254c1` 新增通用并发 TCP/fork 门禁：
  127.0.0.1 listener 在 accept 前 fork 八个 client，每个发送不同 payload、要求精确 echo，
  父进程检查唯一性并按 exact pid `wait4` 零退出。内核看不到 evaluator 路径、程序名或
  协议答案。clean RV/LA canonical 均 1/1 PASS，duration 94.420480/89.103212 s；summary
  SHA-256 为 `9845dac9...1839`、`6403cef3...e2de`，证据在
  `test/output/20260720T202229Z-evidence-runtime-rv-3044065/` 与
  `test/output/20260720T202408Z-evidence-runtime-la-3045485/`。因此它是已有 GREEN 覆盖，
  没有生产修复或分数 delta。
- test-only `bc25c26e64bd73d70f0d91f15556ac040a67d45a` 增加第二个经严格 ELF64/ET_EXEC/
  PT_LOAD/无 PT_INTERP 校验的 freestanding helper。主 smoke 建 pipe 后 fork，子进程用
  `dup3` 重定向 stdout 并 `execve` 该独立静态 ELF；父进程精确读取 payload、确认 EOF、
  关闭 fd，再按 exact pid `wait4` 且要求 status 0。这直接覆盖 CAgent `popen/pclose` 的
  通用内核组成，不调用 libc wrapper，也不硬编码 CAgent 命令或答案。
- helper 和主 smoke 的 RV/LA 内核均实际编译成功。host semantic evidence 75/75、competition
  workflow guard 与 test asset integrity 均 PASS。两次直接 raw 启动分别因缺 supervisor
  标志和未解析 QEMU 路径以 rc=2 拒绝，公开 runner 又因测试先行脏树以 rc=2 拒绝；这些均
  没有启动 guest，也没有计作 PASS。提交后再从 clean tree 运行 canonical profile。
- clean `bc25c26e` canonical RV/LA evidence-runtime 均 1/1 PASS，duration
  95.157992/90.525652 s，runner provenance stable；summary SHA-256 为
  `1416563d...a2f`、`8e5d15e6...7af`，证据分别位于
  `test/output/20260720T203548Z-evidence-runtime-rv-3051061/` 与
  `test/output/20260720T203731Z-evidence-runtime-la-3052449/`。guest raw log 唯一有序出现
  `ASSERT pipe_fork_exec PASS`、后续 TCP/uname、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，
  SHA-256 为 `72eb6593...5cef`、`fea6baa2...2140`，没有 fatal pattern。
- 两架构均在新增测试前已有正确生产语义，因此本 checkpoint 是风险收敛而不是分数提升。
  正式 CAgent 仍因 final rootfs 缺失保持 BLOCKED/未执行。

## 2026-07-20 — Checkpoint 9：BuildStorm `clone3` 版本化 ABI RED -> GREEN

- BuildStorm 调用链审计：在 host 上对最小 `cargo new`/`cargo build` 做只读行为探针；该探针
  不是 guest 或官方得分证据。`cargo-build.strace` 记录 30 次 `clone3`，独立 flag trace
  显示 Rust worker thread 使用 88-byte `struct clone_args`、显式 stack base/size、TLS、
  parent/child TID；glibc `posix_spawn` 使用 `CLONE_VM|CLONE_VFORK` 和显式 stack。三份探针
  SHA-256 分别为 `559c9a1f...c68d`、`cd0cd948...cf6`、`836d5947...0792`，保存在 host
  临时目录 `/tmp/orays-buildstorm-strace.msL2f6/`，不提交为仓库证据。
- 根因：OrayS dispatcher 没有 syscall 435 路由，所有 `clone3` 调用返回 `ENOSYS`；glibc
  可能退回 legacy `clone`，但这不是版本化 ABI 支持，并增加构建链兼容和性能风险。
- test-only RED：提交 `cbf9e7e5180e97931cb6ed53a3bfc8502666dc10` 增加通用 direct
  syscall 门禁，覆盖 null pointer `EFAULT`、size 56 `EINVAL`、size 88 普通 process clone/
  exact `wait4`、size 96 零扩展接受及非零扩展 `E2BIG`。clean RV/LA canonical runtime
  都在 `USER_FAIL clone3_process` 后 fail closed；summary SHA-256 分别为
  `20f2c779...51f79`、`873fa8c3...4fce`，证据位于
  `test/output/20260720T205051Z-evidence-runtime-rv-3058650/` 与
  `test/output/20260720T205245Z-evidence-runtime-la-3060155/`。
- GREEN 实现：提交 `96bab578b93419e2a05695bd0e6a7fb98de426e2` 新增 syscall 435
  dispatcher 和 `copy_struct_from_user` 风格的版本化参数读取：最小版本 64 bytes、当前
  88 bytes、未知尾部必须全零；验证 flag/exit-signal/set-tid/stack/cgroup 组合，将合法基础
  process/vfork/thread 字段映射到既有通用 `sys_clone`。PIDFD、指定 TID、
  `CLONE_CLEAR_SIGHAND` 与 `CLONE_INTO_CGROUP` 所需机制尚未实现，故显式返回 `ENOSYS`，
  没有静默忽略、无条件成功或新增 production `unsafe`。提交 `b8338015` 再把新增 boundary
  mutation 数精确绑定为 17；第一次 clean quick 在执行任何 case 前因 manifest 仍写 16
  而拒绝，属于 fail-closed 基础设施前置失败，不记录为测试 PASS。
- clean canonical quick：`b8338015` 上 47/47 PASS，planned/executed/completed 均为 47，
  summary SHA-256 `4be3f61f9d9aa808cd0e4055da323f5d8a2bb95459ed73f264ada7f6468e9edd`，
  位于 `test/output/20260720T210549Z-quick-none-3066483/summary.json`。
- clean 双架构 GREEN：RV/LA `evidence-runtime` 均 1/1 PASS，runner commit 前后一致且
  worktree clean；raw guest protocol 恰好有序出现 `ASSERT clone3_process PASS`、
  `USER_PASS`、`HARNESS_PASS status=0` 和 `SHUTDOWN`。RV summary/raw SHA-256 为
  `2ab0cf0c...2578`/`058b04df...7a66`，95.052575 s；LA 为
  `9cf6ce19...00be`/`59c0f239...0536`，89.697459 s。证据位于
  `test/output/20260720T211134Z-evidence-runtime-rv-3071295/`、
  `test/output/20260720T211344Z-evidence-runtime-la-3072874/` 和 ignored
  `build/pr3-evidence/{rv64,la64}/`。
- 该里程碑证明可泛化的双架构 `clone3` 基础 ABI，不证明正式 Rust toolchain/minibuild 或
  full BuildStorm 已运行/得分。正式 rootfs 与合格 8c/8G host 仍缺失，下一步继续审计
  Cargo/rustc 的 thread/vfork、futex、mmap/madvise、statx 与文件系统热路径。

## 2026-07-20 — Checkpoint 10：Cargo/glibc `clone3` worker-thread 形态

- test-only 提交 `5304967567251c828020c28ce433a4712e09a138` 增加 Cargo/glibc 实际使用的
  88-byte `clone3` worker-thread 形态：`CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|
  CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID`，
  使用显式 64 KiB child stack、TLS、共享 parent/child TID 与双 pipe 握手。child 在新栈上
  以 raw assembly/syscall 比较架构 TLS register、输出独立 marker 并退出；parent 验证
  parent-TID 写入、child-TID 清零、共享 fd 与 marker 的真实 `write(2)` 结果。新增 `unsafe`
  仅在 freestanding 测试中，两个架构都记录了栈所有权、指针/描述符有效期和 child 不返回
  Rust 的不变量；生产代码没有新增 `unsafe` 或语义改动。
- 首次 clean RV canonical 运行（`53049675`）退出码 1、1 FAIL，summary SHA-256
  `5ae442b32f0e4695cee6cedac4019f5f7d91ddebd427f7ad8c9c0a40efadeddb`，证据位于
  `test/output/20260720T213933Z-evidence-runtime-rv-3083242/`。提交 `a97c44f8` 保存跨 syscall
  的 marker pointer/length 后第二次 clean RV 仍退出码 1，summary SHA-256
  `06d1744f6e414b35f47f02f20641482c60d91d6b7adf8d3ed0eacc469b13fbe1`，证据位于
  `test/output/20260720T214502Z-evidence-runtime-rv-3086798/`。两次 guest 都已通过 parent 侧
  `clone3_thread` 断言，但 classifier 未收到 child marker；这些 FAIL 保留为测试缺陷证据，
  不归类为内核回归或 PASS。
- 根因与修复：child 把 TLS 比较字节写到 `sp[0]`，随后又把 release pipe 的 `G` 读取到
  同一字节，导致它按测试逻辑跳过 marker。非 canonical dirty 诊断通过 pipe 回传观察到
  marker 结果为 `-1`，据此定位覆盖而非内核 errno。提交
  `17b1e2ca8e0ad0111302bc8d850d679832b2f7d6` 将持久 TLS 状态、pipe payload、fd、marker
  pointer/length 与 `write(2)` 结果放入互不重叠的 child-stack slot，并保留 EBADF/EFAULT/
  other 诊断；没有放宽 marker 或 classifier。
- clean canonical GREEN：RV/LA `evidence-runtime` 均为 1/1 PASS、runner commit 前后一致且
  worktree clean。RV summary/raw SHA-256 为
  `b95fe31402139e81abfe610426360360915675dca61a5b58f6606121c6dfeb5c`/
  `e376af1c34fecb5a624db839220ff5f94c4e1bd8d2128ec476a5da5c45c1e7ee`，94.647573 s；LA 为
  `61f12ccc3e8675952cf3775424bac903c009c042e174d10ec1832eaaf83c373c`/
  `3a5762a0b6f6f344525b00c011f1c0cd37b9279eb7381657477e268fd076e4fe`，90.220875 s。原始
  protocol 都有序出现 `clone3_process`、`clone3_thread_child`、`clone3_thread`、
  `USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，无 panic/trap/timeout/fatal marker。证据位于
  `test/output/20260720T215704Z-evidence-runtime-rv-3093991/` 与
  `test/output/20260720T215845Z-evidence-runtime-la-3095432/`。
- 同一 clean commit 的 canonical quick 为 47/47 PASS，planned/executed/completed 均为 47，
  summary SHA-256 `46247d0a679ca8484253f730f33f6e3a983c036887847c867f6bdb8a627a5225`，证据位于
  `test/output/20260720T220020Z-quick-none-3096791/`。外部协议仓库仍 clean 且精确位于
  `15e0355b...a1b`。
- 官方 artifact 获取审计仅使用比赛 README 所给的 Baidu 来源及其第一方接口；公开 share
  可列出 `2026OSImage-Pub/sdcard-rv-pub.img.gz`（1,360,145,887 bytes，fs_id
  `922498973463434`）和 `sdcard-la-pub.img.gz`（1,346,224,600 bytes，fs_id
  `194573041106573`），但未登录下载接口只返回加密/混淆字段，没有可审计的直接下载 URL。
  未使用第三方 downloader、账户或凭据，也未把文件名/大小当作 image provenance。正式
  镜像仍未取得；BuildStorm host 资源也仍不满足 8c/8G 合同，因此没有官方得分。
- 下一步：用相同测试先行方法覆盖 glibc `posix_spawn` 的
  `CLONE_VM|CLONE_VFORK` + explicit-stack 形态；只有真实失败才修改生产实现。

## 2026-07-20 — Checkpoint 11：glibc `posix_spawn` 的 clone3 vfork/exec 形态

- host-only 行为探针（不作为 guest 或得分证据）显示 glibc `posix_spawn` 使用 88-byte
  `clone_args`、`CLONE_VM|CLONE_VFORK`、`exit_signal=SIGCHLD`、显式 stack base/size；
  `/tmp/orays-buildstorm-strace.msL2f6/clone-flags.strace` 为 10,267 bytes，SHA-256
  `836d594776ff656483125c4b64873da592d99c6cff98a2e85b2cdbf389640792`。
- test-only 提交 `b52c8494da91d7a9dc1b647f5ae058db54ef5a88` 增加相同 88-byte ABI
  形态与独立 64 KiB 对齐栈。child 在架构专用 raw assembly 中关闭/重定向 fd、发布共享
  pre-exec stage，并 `execve` 独立 freestanding helper；失败路径精确 `_exit(44)`。parent
  要求 clone3 返回时 stage 已可见，以证明 vfork 父进程确实挂起至 exec/exit，再验证精确
  helper payload、EOF 和 `wait4` status 0。新增 `unsafe` 只在测试中，记录了栈、指针、fd
  有效期和 stack switch 后不返回 Rust 的不变量；没有生产语义改动。
- focused semantic/competition 单元测试 108/108 PASS，workflow guard 与 asset integrity
  均为 `PASS (0 findings)`；RV/LA smoke kernel 均实际编译成功。脏树受监督 raw 诊断仅用于
  预检，未记为 canonical PASS。
- clean canonical RV/LA `evidence-runtime` 均 1/1 PASS，runner provenance stable；RV
  summary/raw SHA-256 为
  `5efe6998bce98a30a5d2903bd742bb07f7b4034f165aa9b30451401a9580ab80`/
  `1e1667089090845458ce81c7e04ad7c43d4aa8d4052b1e7c129c57883d66e5b8`，95.566718 s；LA 为
  `d41e4cc783995816032fbf2b5228190d0d2b7f32cffdb2e2298126f8d8bd53e2`/
  `9aef3bca75493a8bc6900cae58635ce4d053c7ed71e41031714473535e63f32f`，90.549200 s。证据在
  `test/output/20260720T221605Z-evidence-runtime-rv-3105825/` 与
  `test/output/20260720T221753Z-evidence-runtime-la-3107307/`。同一 clean commit 的 quick
  47/47 PASS，332.263537 s，summary SHA-256
  `9e44f8b8b47bb15a6fffbefc5e688bd79af9931b0cbd25d45aac1c5c15af2fe0`，证据在
  `test/output/20260720T221934Z-quick-none-3108693/`。
- 新门禁首次即双架构 GREEN，因此没有生产修复或可归因官方分数 delta；正式 BuildStorm
  仍因 final rootfs 与 host 资源缺口保持 BLOCKED/未执行。

## 2026-07-20 — Checkpoint 12：glibc clear-child-tid futex join 形态

- 对 host Cargo minibuild 的只读 strace 显示 worker 常用
  `FUTEX_WAIT_BITSET_PRIVATE`，glibc thread join/clear-child-tid 使用非 private 的
  `FUTEX_WAIT_BITSET|FUTEX_CLOCK_REALTIME`、expected tid、null timeout 与
  `FUTEX_BITSET_MATCH_ANY`。探针 `/tmp/orays-futex-probe.w7MRYj/cargo-futex.strace` 为
  124,525 bytes，SHA-256
  `0f1aacf737ed732205d8f5d944e26948a8afab9b4080fc28ed0c77025004895f`；它不作为 guest 或
  得分证据。
- test-only 提交 `debab681ac6c14a5f4dc7a94a17c852e3b1b2c6a` 在既有 exact Cargo
  clone3 thread 上无条件执行 syscall 98 的相同 join 形态。返回 0 必须伴随 acquire-load
  观察到 clear-child-tid 为零；Linux 允许 child 先清零导致 `EAGAIN`，测试仅在同一 acquire
  load 已证明为零时接受该竞态，不吞其他 errno、不退回自旋假装 futex 成功。另将前一
  clone3 实现中 rustfmt 指出的一个条件表达式改为等价单行，无生产语义变化。
- focused semantic/competition 单元测试 108/108 PASS，workflow guard、asset integrity、
  `cargo fmt --all -- --check` 和 `git diff --check` 均通过，RV/LA smoke kernel 均实际构建。
  脏树受监督 QEMU 诊断分别输出新 marker 与完整收尾链，但只作为预检，不作为 canonical
  PASS；日志 SHA-256 为 `b08ab55e...85ad`、`8711fc0a...ab74`。
- clean canonical RV/LA `evidence-runtime` 均 1/1 PASS，runner commit 前后一致、worktree
  clean、provenance stable。RV summary/raw SHA-256 为
  `bb11225c1e36137922eaf042fef4ad37eb9e6fe033da7f6b1563343934d7a5f1`/
  `6da6706751f198f92e20da05288c3722b5943c45c51a4c6b94dc1b80aa1f4af8`，95.337348 s；LA 为
  `3cb8bf4ab817b28a67d4243700eae884438ad8f49e347ef21ff65386c03593b6`/
  `8d735a2d6c3404a85c4108ccd771b6f1fc078043e4fa7f2413902168b40bc28b`，89.760112 s。证据在
  `test/output/20260720T223539Z-evidence-runtime-rv-3144714/` 与
  `test/output/20260720T223726Z-evidence-runtime-la-3146103/`；raw protocol 有序包含 process、
  thread child、thread、futex join、vfork exec、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，
  无 fatal pattern。
- 同一 clean commit 的 canonical quick 为 planned/executed/completed=47/47/47、47 PASS、
  零 FAIL/TIMEOUT/INFRA，333.832446 s；summary SHA-256
  `6ec871e8b2ccca472627783e4be21a675f6d4b44afdd868029d2a10d09680d6e`，证据在
  `test/output/20260720T223911Z-quick-none-3147425/`。外部协议仓库再次确认 clean 且精确位于
  `15e0355b...a1b`。
- 现有 futex 实现通过该精确双架构门禁，未观察到可归因生产 RED，故没有修改内核或宣称
  分数提升。下一步继续从真实 Cargo syscall/path 轨迹审计 mmap/madvise、statx 与文件系统。

## 2026-07-20 — Checkpoint 13：`MADV_DONTNEED` 真实 page discard RED -> GREEN

- host-only Cargo/rustc minibuild 的 `strace -ff`（不作为 guest 或官方得分证据）记录了
  234 次 `statx`、120 次 `MADV_DONTNEED` 和 6 次 `MADV_FREE`；其中 61 次 DONTNEED 命中
  `PROT_READ|PROT_WRITE`、`MAP_PRIVATE|MAP_ANONYMOUS|MAP_NORESERVE` 的 allocator 映射。
  这证明该行为是通用工具链热路径，不是根据 final 测例名或输出选出的特化分支。
- 旧生产实现对 DONTNEED 范围调用 `zero_user_range`，分配 64 KiB 临时 Vec 并逐字节写零，
  同时保留所有 resident frame。test-only 提交
  `bab5372285c4365a02523b0653d524885a0f1558` 增加 8 MiB x 16 轮匿名映射 workload：每页
  写入后 direct syscall DONTNEED，检查全范围读零、再次 fault/write 和 live mapping；耗时只作
  诊断，不作为 PASS 阈值。clean RV/LA 基线均 1/1 PASS，syscall 时间分别为
  164.084300/117.335220 ms，证明语义已正确但实现存在可测成本。
- 生产提交 `e311d98d2c2a6636a8cf574614dd1c93b2f9a3ec` 在 axmm 中实现通用 discard：先验证整个
  page-aligned range 和全部 VMA/backend，预留完整临时 frame bookkeeping 后才改 PTE；随后按
  area 批量 unmap，在 frame 归还前一次性 flush 所有受影响 TLB，并正确区分 metadata-owned
  frame、private COW PTE 与 shared metadata，保留 VMA 供下一次缺页映射全新零页。巨页、
  file-backed、locked 和不支持的 backend 不被静默接受；MADV_REMOVE 仍使用原清零路径。
- 测试扩展为 fork/COW：child 对继承页 DONTNEED 后必须读零并写入独立值，parent 必须保留
  原 0x5a 数据。新增 `unsafe` 仅限 freestanding 测试中的 volatile 映射访问，每处均记录
  映射和生命周期不变量；生产实现没有新增 `unsafe`。
- clean canonical GREEN：RV summary/raw/semantic JSON SHA-256 分别为
  `fcec0cdde2ebd233fdc1f83723097927b1f92ec8720b986742654e9e00da469d`、
  `efa0a4c1c4090def73aa58d678b7d5adfe3ea37e87fbdaca52fc1e098730d522`、
  `ea6ca6d3bbb200b692d110a2cd07a7814d4ef1db232f7ea47b9cd839d9863408`，96.903628 s；
  DONTNEED syscall 为 69.850300 ms，较 test-only clean 基线下降 57.4%。LA 三个哈希为
  `97b0057b709c345f9a31b61bd208ca4d87a96e6e32b49a2364829a530706a02a`、
  `ec9e22711977eeacc6ecbc53b92413a8f4e7969b9a2deff9b5acc48dbad62328`、
  `d9e09e041c9c6d69f5b6bfeda8aaca147c9c890116cc499d535409005202a043`，100.967435 s；
  syscall 为 42.626040 ms，下降 63.7%。证据在
  `test/output/20260720T231930Z-evidence-runtime-rv-3169626/` 与
  `test/output/20260720T232127Z-evidence-runtime-la-3171123/`。
- 同一 clean commit 的 canonical quick 为 planned/executed/completed/pass=47/47/47/47，
  所有非 PASS bucket 为零，333.973154 s；summary SHA-256
  `df44b8aef5450eedb579f296f37a9a6c169050c6319566beca7d0f5b3a4498a7`，证据在
  `test/output/20260720T232333Z-quick-none-3172617/`。syscall boundary 26/26、file-object
  core 33/33、两个架构规定的 smoke kernel build、competition evidence、format 和 diff
  检查也通过。
- 一次直接 `cargo check -p axmm --target riscv64gc-unknown-none-elf` 因绕过 Makefile 生成的
  platform crate 而退出 101；一次 `python -I -m unittest test...` 因 isolated mode 移除
  cwd 而无法导入模块；一次把 QEMU 管道接到 `rg` 的 PTY 探针被 job control 暂停。三者均为
  无效命令上下文/采样方式，已用仓库规定入口重跑并通过，不将它们隐藏或归为代码 PASS。
- 该里程碑给出仓库内通用语义 workload 的真实双架构性能改进，但没有正式 final rootfs，
  host 也不满足 8c/8G，因此不能推导或宣称官方 BuildStorm elapsed/分数。下一步继续审计
  `statx` 与 filesystem hot path，并保留正式官方结果为 BLOCKED/未执行。

## 2026-07-21 — Checkpoint 14：只读 ext4 root 的通用 volatile overlay RED -> GREEN

- host-only Cargo trace（只用于发现通用调用形态）含 234 次 `statx`，均为现有实现已经接受的
  `STATX_ALL`、`AT_EMPTY_PATH`、`AT_SYMLINK_NOFOLLOW` 组合；没有为得分强行制造 statx RED。
  继续只读审计外部 final BuildStorm 脚本后确认：脚本会清理并重建 `/work/tgoskits/target`，
  写 `/work/.build.rc` 与 `/work/buildstorm.build.out`，但
  `kernel/fs/axfs/src/fs/ext4fs.rs` 的 create/write/truncate/remove/rename 全部显式返回
  `ReadOnlyFilesystem`。qcow2 只保护 host backing，不能让 guest 内只读驱动产生可写语义；
  这是任意 lower 目录写入都会触发的通用 correctness 阻塞，不是 `/work` 特例。
- test-only 提交 `273b5de9d6b5e7a58555f2aeee6814117124f354` 新增独立 axfs 合同，覆盖 lower
  不可变、existing-file copy-up、upper create/write/permission、merged readdir、whiteout
  delete、覆盖 rename、递归目录 rename、`O_TRUNC` 等价 truncate、文件 `parent=None` 和
  rename 类型/非空目录错误。clean RED 命令 `cargo test -p axfs --test test_overlayfs` 退出
  101，错误是缺少 `axfs::overlayfs`，与预期缺实现一致。首次 GREEN 暴露测试期望把 offset 6
  写入 `lower-source` 错算为 `lowerUPPERe`；修正为实际的 `lower-UPPERe` 后保留该测试缺陷，
  没有改生产实现迎合错误断言。
- 生产提交 `5972629e49f1fdd2cafe2dd7cceaf8c564f2e14b` 为所有 ext4+ramfs 根挂载增加通用
  immutable lower + volatile ramfs upper：读取按 upper 优先并回落 lower；首次 mutation
  分块 copy-up 并保留权限；零 truncate 不先复制旧内容；删除/移动 lower 节点建立内存
  whiteout；目录枚举合并去重；rename 在操作前验证文件/目录类型及目标目录非空规则；失败的
  copy-up 删除残留 upper 文件。whiteout 查询按路径祖先做 `BTreeSet` 对数查找，父目录删除
  会折叠后代项，避免 `rm -rf` 后线性扫描累积成二次复杂度。生产没有新增 `unsafe`，也没有
  测例名、二进制名、`/work`、Cargo 或 BuildStorm 路径分支。
- host 单元与编译：`cargo test -p axfs --test test_overlayfs` 1/1 PASS，`cargo test -p axfs`
  的 fatfs/overlayfs 全部 PASS；`cargo fmt --all` 与 `git diff --check` PASS；正式
  `make kernel-rv`、`make kernel-la` 均退出 0，证明 overlay 进入两个架构的普通 ext4 root
  配置。`cargo clippy -p axfs --all-targets -- -D warnings` 首次被已有
  `kernel/arch/axhal/src/mem.rs` 两条 `uninlined_format_args` 阻断；即使加 `--no-deps`，axfs
  既有 fatfs/root/api/fops 的 16 条同类/复杂度 lint 仍使命令失败。没有把这些既有 FAIL
  改写成本 diff 的 PASS，也没有顺手扩大修改范围。
- 旧 RV ext4 镜像上的受监督交互 QEMU 清晰验证：`mkdir /overlay-probe`、创建/读取新文件、
  覆盖 lower `/musl/busybox_cmd.txt`、删除后 lookup 为 `NotFound`、同路径重建、删除 upper
  目录均成功；启动明确打印 `detected root filesystem: Ext4`。退出前后 backing SHA-256
  都是 `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99`，运行 qcow2
  只有 196 KiB 元数据。LA 也在真实 ext4 启动上清晰验证新目录/文件创建读取和 whiteout；
  批量串口输入有一次回显粘连，因此不把那段模糊 lower copy-up 回显单独计为 PASS。LA backing
  仍为 `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50`，qcow2 同为
  196 KiB。
- clean `5972629e` canonical quick 为 47/47 PASS，所有非 PASS bucket 为零，335.261429 s；
  summary SHA-256 `481c9d2d9384e349daafdfc902ca9787149eab23426dbdb6d6654d6ab7ef0783`，证据在
  `test/output/20260720T235632Z-quick-none-3191854/`。clean RV/LA `evidence-runtime` 各
  1/1 PASS，99.188914/103.377835 s；summary SHA-256 分别为
  `9df6bb73ac1712d28291b39518c11fcd627f74a240c2873458c5ca9799c4b838` 与
  `a0f9759e0566cd6fe3cbaf48f79944fbabb2e8c21ed951c036fb7601bae64926`，证据在
  `test/output/20260721T000224Z-evidence-runtime-rv-3196598/` 与
  `test/output/20260721T000418Z-evidence-runtime-la-3198047/`。
- 这些证据消除了已确认的“根盘普遍 EROFS”阻塞，但不等于 final BuildStorm toolchain、
  minibuild 或 full compile 已运行。正式 final rootfs 仍缺失，host 仍不满足 8c/8G，官方
  elapsed/分数继续为 BLOCKED/未执行；下一步审计 open/unlink/rename 文件句柄生命周期、动态
  toolchain 和其余 filesystem 热路径。

## 2026-07-21 — Checkpoint 15：overlay 打开对象跨 rename/unlink 的 POSIX 生命周期

- 在 overlay 合同继续审计中确认旧实现把每个 `OverlayNode` 只绑定到 namespace 路径：文件
  被 rename 或 unlink 后，既有打开句柄下一次 mutation 会重新解析旧路径而返回 `NotFound`；
  覆盖 rename 还必须保证旧目标句柄继续引用被替换对象，而不是随新目录项改指源对象。另外，
  `rename("/missing", "/missing")` 在验证源存在前直接成功，违反缺失源应返回 `ENOENT` 的语义。
- test-only 提交 `8fdcf4bc` 增加同一路径多句柄 copy-up 一致性、源句柄跨 rename 及后续 unlink
  继续读写、打开 lower 文件 unlink 后形成独立可写对象、父目录 rename 后已打开嵌套目录仍能
  相对 lookup，以及缺失源同路径 rename 返回 `NotFound`。首次 RED 精确失败于缺失源同路径
  rename 被错误接受；旧路径重解析实现也无法满足其后的句柄断言。test-only 提交 `1f7c14ef`
  再增加覆盖 rename 的目标对象身份合同，RED 精确表现为旧目标句柄写入返回 `NotFound`。
- 生产提交 `0f985e28eeff5bc6f5229c41b5df8e263cb81912` 将 namespace 路径与打开对象绑定分离：
  `NodeLocation` 保存当前路径、实际 backing、upper 状态和是否仍链接，按路径的弱引用 cache
  只规范化仍活跃的 namespace 对象，不延长已关闭句柄寿命；过期弱引用按几何阈值清理。
  copy-up 完整成功后才更新活跃绑定，避免并发读取半复制文件；rename 先验证源，再分离旧
  目标对象、递归迁移源及活跃后代路径；unlink 对仍打开的 lower 普通文件先 copy-up，再删除
  namespace/建立 whiteout，保留 detached `Arc` 读写。所有 namespace mutation 继续由同一
  mutex 串行化。实现没有新增 `unsafe`，也没有 `/work`、Cargo、BuildStorm、测例名或固定路径
  分支。
- 定向 GREEN：`cargo test -p axfs --test test_overlayfs` 1/1 PASS，`cargo test -p axfs`
  的 host axfs suite 全部 PASS；`cargo fmt --all`、`git diff --check`、`make kernel-rv`、
  `make kernel-la` 均退出 0。`cargo clippy -p axfs --all-targets --no-deps` 退出 0，但仍明确报告
  axfs 既有 16 条 warning 和既有测试 warning；新 overlay 文件没有 warning。此前带
  `-D warnings` 的严格 clippy 仍受 axhal/axfs 既有 lint 阻断，未改写为 PASS。
- clean `0f985e28` canonical quick 为 planned/executed/completed/pass=47/47/47/47，所有
  非 PASS bucket 为零，332.446714 s；summary SHA-256
  `cb551c1621282f87b9cac8b4fe5d6303bfe636111358046b0bd51a9ee00e32c7`，证据在
  `test/output/20260721T001748Z-quick-none-3204497/`。clean RV/LA `evidence-runtime` 各
  1/1 PASS，分别为 98.234252/94.747532 s；summary SHA-256 为
  `c885a4bc9f0534563690a65464cf07563303b17aceb468f406a24a61054725ae` 与
  `0ed964dbf0534d8903267dc57f71da515d646452c91108730dad026ce976e81b`，证据在
  `test/output/20260721T002333Z-evidence-runtime-rv-3209172/` 与
  `test/output/20260721T002518Z-evidence-runtime-la-3210560/`。三次 runner 前后均保持 clean
  且绑定同一生产提交；外部协议 checkout 也再次确认为 clean `15e0355b...a1b`。
- 该 checkpoint 修复的是通用 POSIX 打开对象身份，不等于正式 Cargo toolchain 已运行，也
  不推导 BuildStorm 分数。final rootfs 和合格 8c/8G host 仍缺失，官方结果继续保持
  BLOCKED/未执行；下一步从 `statfs`、动态 ELF/toolchain 和通用 filesystem capability 中寻找
  可由源码与行为合同证明的下一处真实阻塞。

## 2026-07-21 — Checkpoint 16：lower ext4 符号链接与 typed `readlinkat` copyout

- 对 host 上真实 Cargo 最小构建做只读 syscall trace，确认常见访问形态包含
  `openat/newfstatat/statx/readlink/madvise/clone3/futex`；该 trace 仅用于选择通用审计目标，
  不是 guest 或官方 BuildStorm PASS。进一步源码审计确认 `statfs` 已由 user ABI 层按挂载类型
  提供，不是 ext4 VFS 缺口；真正缺陷是 ext4-view 虽能读取 link inode 与 target，OrayS VFS
  没有 no-follow metadata/readlink 接口，导致 lower link 被 `lstat/statx(AT_SYMLINK_NOFOLLOW)`
  当作 target，`readlinkat` 则返回 `EINVAL`。旧初赛镜像的有限 payload 没有可用 final toolchain
  link 样本，因此没有据此推断 final 行为。
- test-only 提交 `18a305d6` 为通用 mock lower link 增加 overlay 合同：普通 attr 是 target file，
  no-follow attr 是 `SymLink`，target 为相对路径。RED 为真实编译失败 `E0407/E0599`，证明
  `VfsNodeOps` 尚无 `get_link_attr/read_link`，不是人为失败断言。
- 生产提交 `639d8034` 在 VFS 增加默认 fail-closed 的 no-follow metadata/readlink；ext4 file/dir
  node 均委托真实 symlink inode（link 指向目录时普通 metadata 会形成 dir node，因此两类节点
  都必须实现）；overlay 委托当前 backing。axfs API 与 POSIX lstat 边界暴露该能力，user
  `readlinkat`、`newfstatat/statx/faccessat/fchownat/utimensat` 的 no-follow 查询不再错误跟随最终
  link，文件类型映射也区分 block device 与 symlink。没有新增依赖、`unsafe`、固定路径或
  BuildStorm/Cargo 特化；symlink 创建仍保留既有 user-process 模型，本 checkpoint 不宣称已把
  创建/变更语义统一迁入 overlay。
- 定向 GREEN：`cargo test -p axfs --test test_overlayfs` 1/1 PASS，`cargo test -p axfs` 全部
  PASS，`cargo check -p arceos_posix_api`、`check_stat_metadata_semantics.py`、格式和
  `git diff --check` 均 PASS，正式 `make kernel-rv` 与 `make kernel-la` 均退出 0。
- 为避免把旧镜像当 final，另创建并最终删除 `/tmp/orays-symlink-probe` 下的两个 fresh 64 MiB
  ext4 临时镜像，只从旧镜像提取各架构静态 BusyBox，并建立真实 ext4
  `/toolchain -> musl/busybox`。RV/LA guest 均得到 `readlink /toolchain` 输出
  `musl/busybox`、exit 0；no-follow `stat` 为 `symbolic link:12:777`，follow `stat -L` 为
  regular file（RV 1387560 bytes，LA 2065912 bytes），`test -L` exit 0；对普通文件 readlink
  exit 1。执行 `/toolchain` 已解析并进入 BusyBox，但因 argv0 为 `toolchain` 而按 applet 规则
  exit 127，故只作为 exec resolution 证据，不记为程序成功。临时目录按精确路径清理；两个
  原始 backing 的 SHA-256 复核仍为本日志镜像表中的固定值。
- clean `639d8034` 首次 canonical quick 诚实失败：planned/executed/completed=47/47/47，
  45 PASS / 2 FAIL，只有 `check.linux_boundary` 与其 unittest 失败；原因为新增真实 lower
  `readlinkat` copyout 让 legacy `write_user_bytes` 库存从 47 增到 48。证据保留在
  `test/output/20260721T010426Z-quick-none-3234530/`，334.346206 s，summary SHA-256
  `8e238639c721e3ce414e4baff002ac202b002d609c80db86832c742af0c23411`；没有更新库存为 48
  或用后续 PASS 覆盖它。
- 修复提交 `a4d25d56` 将四类 readlink target 统一成 checked `UserSlice<u8, Write>` copyout，
  长度/地址溢出在 typed 边界处理，并将 legacy helper 的全树库存由原基线 47 收紧至 45。
  `check_linux_boundary.py` 与 17 个 boundary unittest PASS，两架构正常内核构建退出 0。
  同一 clean commit 的 canonical quick 为 47/47 PASS、所有非 PASS bucket 为零，
  330.628906 s；summary SHA-256
  `922424884dd944c8e19350888ee04d94fad9485217af4cdbea4ba5276a386e47`，证据在
  `test/output/20260721T011516Z-quick-none-3241642/`。clean RV/LA evidence-runtime 各
  1/1 PASS，98.810174/95.626819 s；summary SHA-256 为
  `0d211db02864744520b6d974d677836fde57db77c5d86570975389c24a38b4ae` 与
  `68b5e0fe7b3b359f21cbf86e72f4691af47f7716354af91568306273acff5efb`，证据在
  `test/output/20260721T012057Z-evidence-runtime-rv-3246309/` 与
  `test/output/20260721T012248Z-evidence-runtime-la-3247728/`；三次 runner 均记录
  `runner_provenance_stable=true`、前后 clean 且绑定 `a4d25d56`。外部协议 checkout 再次确认
  clean `15e0355b...a1b`。
- 以上是通用 filesystem/dynamic-rootfs correctness 修复，不是正式 final BuildStorm 运行或
  分数 delta。final rootfs 与合格 8c/8G host 仍缺失，官方结果继续 BLOCKED/未执行；下一步
  继续从动态 loader/toolchain 与文件系统能力中寻找可以由真实通用合同闭环的阻塞。

## 2026-07-21 — Checkpoint 17：final suite 发现、Debian shell 与 BuildStorm 真实时间预算

- 只读审计固定外部协议提交的 `scripts/buildstorm_testcode.sh` 与仓库 autorunner 后确认若干
  在正式 rootfs 到位后仍会阻止执行的基础设施缺陷：autorunner 只扫描 `/musl`、`/glibc`，
  不发现 manifest 固定的 `/scripts/buildstorm_testcode.sh`；默认 guest group timeout 为 60 s，
  远短于脚本内部真实 `timeout 14400 cargo ...`；即便额外发现 `/scripts`，旧 staging 仍把
  Debian `/bin/sh` 当作 BusyBox multicall binary，并把 judge-visible label 改成
  `buildstorm-scripts`。外部脚本还有一个 START、两个互斥控制流 END，旧的“源码中恰好一个
  END”检查会在执行前拒绝合法脚本。这些都是通用 runner/rootfs/protocol 缺陷，不是 syscall
  测例特化，也没有据此推导 guest PASS 或分数。
- test-only 提交 `154c4bad` 首先要求 adapter 从已经 provenance 绑定的 guest script 父目录
  派生额外 suite dir，并为 BuildStorm 提供不低于脚本 14400 s 的预算；首次 RED 为 adapter
  7 PASS / 1 ERROR（缺 `OSCOMP_EXTRA_TESTSUITE_DIRS`）及 integrity guard 三个明确 finding。
  `7cf8ab03`、`05d4e21c`、`9d93ab27` 继续固定 Debian POSIX shell/可选 BusyBox provider、配置
  suite 保留原 judge group label、以及多分支 END frame 的合同；每轮当前树测试均真实 RED，
  没有把检查删除或改成无条件成功。
- 生产提交 `bae025dbb594752dc5429fefa129d112a10bef5a` 让 final adapter 在闭合编译环境中仅为
  BuildStorm 注入 `/scripts` 与 18000 s ceiling；autorunner 对额外绝对目录做 fail-closed
  规范化验证，并为 BuildStorm 设置 18000 s nominal timeout。通用 shell 选择把普通 POSIX
  `/bin/sh` 与 BusyBox applet provider 分离：Debian 脚本不再做 BusyBox utility rewrite，
  BusyBox legacy suite 仍保留原 wrapper/staging；缺 shell 形成 runner-owned FAIL frame，不再
  `exit(0)`。配置 suite 使用原 group label，legacy `/musl`/`/glibc` 仍保留 libc 后缀；framing
  删除全部静态 START/END 分支，但仍要求两种 marker 均存在，唯一 runner frame 和 PASS/FAIL
  继续绑定真实 child status。没有修改外部脚本/judge/image，没有新增依赖或 `unsafe`。
- focused GREEN：格式、evaluation integrity 29/29、final adapter 8/8、test asset integrity
  0 finding、`git diff --check` 均 PASS；完全使用 adapter 编译环境
  `OSCOMP_TEST_GROUPS=buildstorm OSCOMP_EXTRA_TESTSUITE_DIRS=/scripts
  OSCOMP_GROUP_TIMEOUT_CEILING_SECS=18000` 的 RV/LA 正常内核构建均退出 0。一次组合命令把资产
  检查名误写为不存在的 `check_final_2026_assets.py`，在构建前 exit 2；随后从 manifest 解析并
  运行正确 `check_test_asset_integrity.py` 得到 PASS，该命令错误不记为测试 PASS。
- 首次 clean canonical quick 在执行 case 前 fail-closed 为 infrastructure ERROR：
  `unit.compliance_regressions` 实际 7 项但 manifest 错写 8。根因是 test-only `154c4bad` 的机械
  计数修改命中相邻 case；`b5616bbd6aa20c87350535ef1d8d2571d26d5222` 将 compliance、
  integrity、adapter 三组实际/manifest/canonical identity 分别统一为 7/29/8。资产检查与
  suite-runner/test-asset 171 项 focused mutation 全 PASS 后，clean `b5616bbd` quick 才得到
  planned/executed/completed/pass=47/47/47/47、所有非 PASS bucket 为零，343.386464 s；summary
  SHA-256 `b3a127db8700dc1d63ac87f8426833231bc963930c019f9db6aa14abe9a628a0`，证据在
  `test/output/20260721T015831Z-quick-none-3264886/`。
- 同一 clean 提交的 RV/LA canonical evidence-runtime 各 1/1 PASS，分别为
  112.250301/98.597706 s；summary SHA-256 为
  `7601e42c871a0c36966a66701b843dd4f4e473cd8f7f8596919233f6ce27d6c3` 与
  `272c77f0ec18f423b0e9f2d0d9a63348b7ee469a65e3b3ce60503e5c5d9a368e`，证据在
  `test/output/20260721T020438Z-evidence-runtime-rv-3271491/` 与
  `test/output/20260721T020651Z-evidence-runtime-la-3273129/`。它们验证 legacy image 路径
  没有回归，不是 final rootfs CAgent/BuildStorm 运行。外部协议 checkout 再次确认 clean
  `15e0355b...a1b`；正式 rootfs 与合格 8c/8G host 仍缺失，因此官方结果继续
  BLOCKED/未执行、无 score delta。

## 2026-07-21 — Checkpoint 18：Debian 标准根动态 glibc SONAME RED -> GREEN

- 先用 fresh 128 MiB ext4 诊断镜像验证 checkpoint 17 的新启动分支。镜像仅从旧只读镜像
  提取各架构真实静态 BusyBox，按 Debian 形态安装 `/bin/busybox`、`/bin/sh`，并将固定外部
  提交中未修改的 `/scripts/buildstorm_testcode.sh` 放入 `/scripts`；没有 Rust toolchain、
  `/work/tgoskits`，因此诚实预期是脚本被执行后明确失败。第一次 RV 探针误用了随后被其他
  build 覆盖、未嵌入 extra-dir 配置的 `kernel-rv`，进入交互 shell 后被 host timeout 以 124
  终止；没有执行脚本，也未计为 RED/PASS。按精确 adapter 环境重建后，RV/LA 都只有一个
  `buildstorm` START/END，真实输出 `BUILDSTORM_TOOLCHAIN fail`、`BUILDSTORM_MINIBUILD fail`、
  `FAIL OFFICIAL TEST GROUP buildstorm : 1`，child status 为 1，没有 PASS。QEMU 自身退出 0
  不能覆盖 guest FAIL。控制台 SHA-256 为 `8f9c4f45...fba`、`e1eacc46...1797`；两个 raw
  backing 复核仍为 `4bbfe34a...2f8`、`c5ce308e...17bb9`。
- 继续从旧只读镜像提取真实 glibc `arithoh`、loader、`libc.so.6`，构造只含标准
  `/bin`、`/lib`/`/lib64`、没有 `/glibc`/`/musl` suite root 的双架构 fresh ext4。相同脚本
  只运行动态 `/bin/arithoh 1` 并把真实 rc 交给 runner。修复前 RV 退出 0，但 LA loader
  报 `libc.so.6: cannot open shared object file`，程序 rc 127、outer group FAIL 127；raw
  backing SHA-256 为 `a236e796...acc5`、`992fab07...5537`。这证明标准布局的 LA64 动态 ELF
  有真实 correctness 缺口，而不是从源码猜测或 final 分数推导。
- 根因是 `effective_exec_root` 对根目录动态 ELF 仅凭 interpreter basename 强制改成旧式
  `/glibc`/`/musl` root；同时 bare SONAME compatibility 候选在 exec root 为 `/` 时反而省略
  标准 `/lib*`。test-only `007ae7aa` 在 broad compliance guard 中要求 rootfs ELF 保留自身
  runtime root，并要求普通/fallible SONAME 查询先记录标准 root；当前树得到三条精确 RED，
  paired unittest 仅 `test_current_tree_passes` 失败，其他 mutation 继续通过。
- 生产提交 `adacda8c` 保留由实际程序路径派生的 `/`，对 root runtime 先加入 `/lib`、
  `/lib64`、`/usr/lib`、`/usr/lib64`，然后才使用既有 `/musl`、`/glibc` fallback；非 root
  suite runtime 的顺序和 compatibility 路径没有删除。没有新增依赖、`unsafe`、架构分支、
  测例名、Rust/Cargo 路径或伪返回值。compliance guard、7 项原 mutation、格式和 diff check
  均 PASS；精确 final adapter 环境的 RV/LA 正常内核构建都退出 0。
- 同一动态镜像复测：LA 从 loader rc 127 转为程序 rc 0/outer PASS 0；RV 修复前后都保持
  rc 0/outer PASS 0。修复后控制台 SHA-256 为 LA `f37c0552...fa8bf`、RV
  `f5789f22...f73c`，raw backing 再次与上述哈希一致。这里的 PASS 只表示该最小动态程序
  行为通过，不是 BuildStorm toolchain/minibuild/compile 或官方得分。
- 提交 `66a453fe` 再增加 root remap 与遗漏标准 SONAME root 两个 mutation，实际/manifest/
  canonical identity 同步为 9；compliance 9/9、test asset 36/36、suite runner 135/135 PASS。
  曾错误地一次向单文件 harness 传两个文件，harness 在执行前 exit 2；随后分别运行并得到
  上述 PASS，该命令错误没有被隐去。控制台证据位于 ignored
  `test/output/20260721T022616Z-buildstorm-rootfs-probe/`；临时镜像与提取目录已按精确路径
  移入回收站。正式 final rootfs 仍缺失且 host 仍不足 8c/8G，官方成绩保持
  BLOCKED/未执行。

## 2026-07-21 — Checkpoint 19：fork 共享路径元数据与跨进程 hardlink

- 沿 host-only Cargo trace 的 `linkat`/rename 文件生命周期做只读审计时发现，用户态层的
  `linkat` 没有向底层 ext4/ramfs 创建硬链接，而是将别名记录在 `UserProcess` 的路径元数据
  表中；`fork_with_fd_sharing` 又把 hardlink、symlink、mode、owner、xattr、time 等 11 张表
  深拷贝进子进程。结果是子进程可以从 `linkat` 得到成功，但父进程仍看不到别名，违反共享
  文件系统 namespace 语义，也会直接阻断 Cargo/rustc 一类跨进程文件操作。
- 用未修改的静态 BusyBox 为两架构构造 fresh 128 MiB ext4：shell 创建普通源文件，独立
  `/bin/ln` 子进程创建别名，返回父 shell 后再由 `/bin/cat` 读取。RV 与 LA 均真实得到
  `cat: ... No such file or directory`、guest group FAIL 1；最终 RED 控制台 SHA-256 分别为
  `15aa1a5bd0d5f0dbcbf22da7721d76defd186e0dd67f3dbca7928f4d9d2f34de` 与
  `a6e923f4805c01c37aeaacb4ba1d74b084c48af96b28871122cce4a9532e6d0d`。QEMU 进程退出 0
  不覆盖 guest FAIL。第一次 RV 脚本因缺官方 START/END 文本被 fail-closed 拒绝，前两次 LA
  又因旧 BusyBox `sh` 不支持 `set -eu`/`set -u` 在探针前退出；三次都明确记录为未执行探针，
  没有当作语义证据或 PASS。
- test-only `ab2d9880` 扩展 broad compliance guard，要求这 11 类路径元数据使用共享容器且
  fork 只克隆共享引用，并新增两条 mutation；actual/manifest/canonical identity 从 9 同步到
  11。生产修复前 guard 精确报两类 finding，unit 11 项中有 3 项失败：当前树失败与两个面向
  未来结构的 fixture drift，形成预期 RED；其余 mutation 没有被弱化。
- 生产提交 `1b5aa307` 只把上述字段改为 `Arc<Mutex<...>>`，初始进程创建共享对象，fork
  子进程克隆 Arc。没有修改 syscall 返回值、errno、架构配置或测试分支，没有新增依赖、
  `unsafe`、Rust/Cargo/BuildStorm 名称和路径特化。compliance guard、11/11 mutation、
  `cargo fmt --check`、`git diff --check` 均 PASS；精确 BuildStorm adapter 环境下 RV/LA 内核
  构建均退出 0。
- 修复后同一通用行为在 RV/LA 均打印 `HARDLINK_CROSS_PROCESS pass`，外层各自只有一组
  `PASS OFFICIAL TEST GROUP buildstorm : 0`；这里的 PASS 仅表示最小 hardlink 行为通过，不是
  正式 BuildStorm toolchain/minibuild/compile 或分数。GREEN 控制台 SHA-256 为
  `060011ac81bbb1689f27599e0b2a62a1ea0efe777ac9cfa6665f6afce97f0efe` 与
  `08540e98799f632d3dece6a4d9ebeacb702544ab143422f586b4f0eeb3927819`。最终运行前后 raw
  镜像 SHA-256 保持 RV `dc631630a2ab36a4aeb24010367d0df833bd0d2ac14ea5df78d2e518facb25c9`、
  LA `e9eb0001a98a10c5c61402c58eedd3ced1e5d5a3b3e8fef24cfc942b406acfcf`；临时目录已按
  精确路径移入回收站，原始控制台保存在 ignored
  `test/output/20260721T-hardlink-cross-process-red/`。
- clean `1b5aa307` canonical quick 47/47 PASS、零非 PASS bucket，340.987273 s，summary
  SHA-256 `b16fff22c69dd64a8f8f1c52cc9e1d3f942244e436323d8094b4ad94d6b3daa9`；RV/LA
  canonical runtime 各 1/1 PASS，99.409973/95.518897 s，summary SHA-256 分别为
  `1dc25a8e024a3a68f3d8fdfd577977d8734e49d3d267f0c7aa703490e6d718a0` 与
  `eaf1de82da81e75e04333c83e63ab611ea0c6e35efd56880db2b954f2fe08017`。正式 final rootfs
  和合格 8c/8G host 仍缺失，官方结果继续为 BLOCKED/未执行。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex / GPT-5 系列（精确子版本未知） | 合同读取、协议审计、基础设施盘点、parser/adapter/profile 与 Linux procfs/device/scheduler/进程/网络/内存/文件系统/动态 loader/`clone3`/futex 语义设计、编码、测试及日志 | 本任务计划、开发日志、final parser/adapter、suite runner/manifest、final suite discovery/timeout/POSIX shell/framing、semantic smoke、`/proc/uptime`、设备 fd 状态、CPU affinity、BusyBox PATH wrapper、并发 TCP/fork、fork/exec/pipe、版本化 `clone3` ABI、worker-thread、vfork/exec、futex join、`MADV_DONTNEED` page discard/COW 测试、ext4/ramfs volatile overlay、打开对象生命周期、lower ext4 symlink no-follow/readlink、标准 root SONAME 查找与 fork 共享路径元数据 | 拒绝旧镜像冒充 final、host 代跑、缺资源计 PASS、伪造 `ss` 或 prompt 答案；严格拒绝重复/缺失/资格冲突；以真实 idle runtime、task cpumask、BusyBox applet、普通 POSIX shell、独立 exec ELF、标准动态 loader/SONAME root、通用 clone/futex ABI、VMA/PTE/frame discard、路径无关 copy-up/whiteout、namespace/打开对象分离、VFS link metadata 与共享 filesystem namespace 替代固定值；保留测试、manifest preflight、错误探针与边界库存的首次 FAIL/ERROR，未实现 clone3 扩展显式 `ENOSYS` | Git/外部 SHA、image hash/payload、mutation fixture、runner regression、资产检查、外部 judge 对算、clean 缺镜像矩阵、双架构 semantic QEMU、真实 ext4 动态 ELF/交互写入/符号链接、跨进程 hardlink 与 axfs 对象合同；最终仍需人工复核 | 待人工负责人确认 |

交互摘要或记录位置：本日志只记录决定、命令、结果与证据，不提交完整对话、凭据或隐私。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| oscomp/testsuits-for-oskernel | `final-2026@15e0355bbee0373de4048002448cee37dbb7ca1b` | 只读协议、脚本、judge、BusyBox config 与 CAgent 构建说明；不修改、不复制测例进生产逻辑 | 仓库各文件原许可证，后续复制前逐项核对 | parser 独立重述结构/权重/阈值并增加 fail-closed 完整性约束；不复制测例实现，不改外部内容 | 本日志、活动计划、`test/evaluation/parse_final_2026_results.py` |
| QEMU | `v9.2.4` | 只读核验 LoongArch virt machine low/high memory map，确定 8 GiB guest 的 highram 可用窗口 | GPL-2.0-or-later；未复制代码 | adapter 只使用公开 machine contract 计算 `0x1f0000000` highram size | 本日志、`test/evaluation/run_final_2026_evaluation.py` |
| Linux kernel | `b95f03f04d475aa6719d15a636ddf32222d55657` | 只读核验 `include/uapi/linux/sched.h` 的 `clone_args` 版本尺寸、`copy_struct_from_user` 零扩展规则及 `kernel/fork.c` 的参数验证/stack base 语义 | GPL-2.0-only WITH Linux-syscall-note（UAPI）；GPL-2.0-only（kernel） | 独立 Rust 实现，仅重述公开 ABI/验证语义，没有复制 Linux 源码 | `user/shell/src/uspace/process_lifecycle.rs`、本日志 |

# 7. 最终验证

镜像信息：

| 架构 | 文件名 | SHA-256 | 来源/版本 |
|---|---|---|---|
| RISC-V64 | `sdcard-rv.img` | `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` | 旧初赛只读镜像；明确不作为 final 输入 |
| LoongArch64 | `sdcard-la.img` | `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50` | 旧初赛只读镜像；明确不作为 final 输入 |

测试结果将在每个 checkpoint 追加。结果状态只使用：`PASS`、`FAIL`、`ERROR`、`TIMEOUT`、
`BLOCKED`、`SKIPPED`。

当前已执行的 parser/runner 基础验证：

| 提交 | 命令 | 退出码 | 结果 | 耗时/证据 |
|---|---|---:|---|---|
| `14a8d8da` | canonical `--profile quick` | 0 | PASS，45/45 | 355.787892 s；`oscomp-final-14a8d8da-quick-1/summary.json` |
| `36d95801` | final parser focused RED | 2 | FAIL（预期缺实现） | 0.42 s；`oscomp-final-36d95801-red-1/focused.log` |
| `e303653b` | final parser focused GREEN | 0 | PASS，24/24 | 0.46 s；`oscomp-final-e303653b-green-1/focused.log` |
| `e303653b` | test asset integrity | 0 | PASS，0 findings | 0.25 s；`oscomp-final-e303653b-parser-checks-1/asset-integrity.log` |
| `393a0989` | final runner focused RED | 1 | FAIL（预期 4 FAIL/2 ERROR） | 0.76 s；`oscomp-final-393a0989-runner-red-1/focused.log` |
| `e0880deb` | final runner focused GREEN | 0 | PASS，31/31 | 0.70 s；`oscomp-final-e0880deb-runner-green-1/focused.log` |
| `e0880deb` | existing suite runner regression | 0 | PASS，135/135 | 255.14 s；`oscomp-final-e0880deb-runner-regression-1/focused.log` |
| `e0880deb` | evaluation runner/parser integrity | 0 | PASS，24/24 | 9.07 s；`oscomp-final-e0880deb-runner-integrity-1/focused.log` |
| `2b7af950` | official CAgent frame RED | 1 | FAIL（预期 3 FAIL/26 ERROR） | 0.70 s；`oscomp-final-e0880deb-label-red-1/focused.log` |
| `969628cf` | official CAgent frame GREEN | 0 | PASS，32/32 | 0.67 s；`oscomp-final-2b7af950-label-green-1/focused.log` |
| `87405990` | final adapter focused RED | 2 | FAIL（预期缺实现） | `oscomp-final-87405990-adapter-red-1/focused.log` |
| `623c6028` | final adapter focused GREEN | 0 | PASS，7/7 | `oscomp-final-50276425-adapter-green-1/focused.log` |
| `623c6028` | test asset integrity + missing-image probe | 0 / 125 | PASS / BLOCKED | `oscomp-final-50276425-adapter-checks-1/` |
| `623c6028` | canonical final profile focused RED | 1 | FAIL（预期 1 FAIL/2 ERROR） | `oscomp-final-623c6028-profile-red-1/focused.log` |
| `67ba351d` | canonical final profile focused GREEN | 0 | PASS，35/35 | 4.071 s；`oscomp-final-3221963f-profile-green-1/focused.log` |
| `67ba351d` | existing suite runner regression | 0 | PASS，135/135 | 228.80 s；`oscomp-final-3221963f-profile-runner-regression-1/focused.log` |
| `67ba351d` | `final-2026 --arch all`（未提供官方 image env） | 2 | BLOCKED，0/4 executed，4 INFRA_ERROR | 1.2 s；`oscomp-final-67ba351d-missing-images-1/summary.json` |
| `1ad30429` | RV/LA semantic `/proc/uptime` test-only RED | 1 / 1 | ERROR / ERROR，真实 open 失败 | 46.083345 / 43.753724 s；`oscomp-final-1ad30429-uptime-red-{rv,la}-1/` |
| `6c989eb9` | RV/LA semantic uptime GREEN、整套继续到 tee | 1 / 1 | uptime marker PASS；整套 ERROR | 44.806307 / 42.428472 s；`oscomp-final-6c989eb9-uptime-green-{rv,la}-1/` |
| `a3b3d548` | RV/LA semantic clean GREEN | 0 / 0 | PASS，build + runtime 各 2/2 | 44.581134 / 42.522110 s；`oscomp-final-a3b3d548-tee-green-{rv,la}-1/` |
| `eea04c57` | RV/LA 8-vCPU affinity test-only RED | 1 / 1 | ERROR / ERROR，真实 mask 仍为 bit0 | 45.058398 / 42.160081 s build；`oscomp-final-eea04c57-affinity-red-{rv,la}-1/` |
| `5a98b24a` | RV/LA 8-vCPU affinity clean GREEN | 0 / 0 | PASS，build + runtime 各 2/2 | 46.463758 / 47.466223 s；`oscomp-final-5a98b24a-affinity-green-{rv,la}-1/` |
| `b0001898` | canonical quick | 0 | PASS，47/47，provenance stable | 333.251882 s case duration；`20260720T200022Z-quick-none-3026875/summary.json` |
| `b0001898` | RV/LA canonical evidence-runtime（BusyBox PATH applet 后） | 0 / 0 | PASS，1/1 + 1/1 | 159.082378 / 166.668069 s；`20260720T200609Z...`、`20260720T200853Z...` |
| `265984ab` | RV/LA concurrent TCP/fork semantic gate | 0 / 0 | PASS，1/1 + 1/1；已有 GREEN | 94.420480 / 89.103212 s；`20260720T202229Z...`、`20260720T202408Z...` |
| `bc25c26e` | RV/LA pipe/fork/execve/wait4 semantic gate | 0 / 0 | PASS，1/1 + 1/1；已有 GREEN | 95.157992 / 90.525652 s；`20260720T203548Z...`、`20260720T203731Z...` |
| `cbf9e7e5` | RV/LA `clone3` test-only RED | 1 / 1 | FAIL / FAIL，真实 syscall 435 为 `ENOSYS` | `20260720T205051Z...`、`20260720T205245Z...` |
| `b8338015` | canonical quick | 0 | PASS，47/47，provenance stable | summary SHA-256 `4be3f61f...e9edd`；`20260720T210549Z...` |
| `b8338015` | RV/LA `clone3` clean GREEN | 0 / 0 | PASS，1/1 + 1/1；marker/provenance 完整 | 95.052575 / 89.697459 s；`20260720T211134Z...`、`20260720T211344Z...` |
| `53049675` | RV Cargo-shape `clone3` thread 首次 clean gate | 1 | FAIL，child marker 缺失；后证实为测试状态覆盖 | 95.835876 s；`20260720T213933Z...` |
| `a97c44f8` | RV Cargo-shape `clone3` thread 第二次 clean gate | 1 | FAIL，测试修复不完整；不归类为内核回归 | 94.569893 s；`20260720T214502Z...` |
| `17b1e2ca` | RV/LA Cargo-shape `clone3` thread clean GREEN | 0 / 0 | PASS，1/1 + 1/1；真实 child marker/TLS/TID/provenance 完整 | 94.647573 / 90.220875 s；`20260720T215704Z...`、`20260720T215845Z...` |
| `17b1e2ca` | canonical quick | 0 | PASS，47/47，provenance stable | summary SHA-256 `46247d0a...225`；`20260720T220020Z...` |
| `b52c8494` | RV/LA glibc-shape clone3 vfork/exec clean gate | 0 / 0 | PASS，1/1 + 1/1；parent suspension/stage/exec/wait/provenance 完整 | 95.566718 / 90.549200 s；`20260720T221605Z...`、`20260720T221753Z...` |
| `b52c8494` | canonical quick | 0 | PASS，47/47，provenance stable | 332.263537 s；summary SHA-256 `9e44f8b8...fe0`；`20260720T221934Z...` |
| `debab681` | RV/LA glibc clear-child-tid futex join clean gate | 0 / 0 | PASS，1/1 + 1/1；精确 op/bitset/TID 与完整 protocol | 95.337348 / 89.760112 s；`20260720T223539Z...`、`20260720T223726Z...` |
| `debab681` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 333.832446 s；summary SHA-256 `6ec871e8...d6e`；`20260720T223911Z...` |
| `bab53722` | RV/LA `MADV_DONTNEED` test-only clean 基线 | 0 / 0 | PASS，1/1 + 1/1；语义正确，逐字节清零基线 | syscall 164.084300 / 117.335220 ms；`20260720T230003Z...`、`20260720T230228Z...` |
| `e311d98d` | RV/LA `MADV_DONTNEED` discard/COW clean GREEN | 0 / 0 | PASS，1/1 + 1/1；归零/refault/fork 隔离/provenance 完整 | syscall 69.850300 / 42.626040 ms，下降 57.4% / 63.7%；`20260720T231930Z...`、`20260720T232127Z...` |
| `e311d98d` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 333.973154 s；summary SHA-256 `df44b8ae...98a7`；`20260720T232333Z...` |
| `273b5de9` | axfs overlay test-only RED | 101 | FAIL，缺少 `axfs::overlayfs`（预期缺实现） | `cargo test -p axfs --test test_overlayfs` |
| `5972629e` | axfs overlay targeted + RV/LA normal kernel build | 0 | PASS，overlay 1/1、axfs 全套、两架构 build | 真实 ext4 交互另核验 backing SHA 不变；clippy 既有 lint FAIL 单列保留 |
| `5972629e` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance stable | 99.188914 / 103.377835 s；summary SHA-256 `9df6bb73...b838` / `a0f9759e...a64926`；`20260721T000224Z...`、`20260721T000418Z...` |
| `5972629e` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 335.261429 s；summary SHA-256 `481c9d2d...0783`；`20260720T235632Z...` |
| `8fdcf4bc` / `1f7c14ef` | overlay open-handle test-only RED | 101 / 101 | FAIL，分别暴露缺失同路径 rename 错误成功、被覆盖目标旧句柄写入 `NotFound` | `cargo test -p axfs --test test_overlayfs` |
| `0f985e28` | overlay handle targeted + RV/LA normal kernel build | 0 | PASS，overlay 1/1、axfs 全套、两架构 build；non-strict clippy 退出 0 且新文件无 warning | host 定向合同；严格 clippy 既有 lint FAIL 单列保留 |
| `0f985e28` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance stable | 98.234252 / 94.747532 s；summary SHA-256 `c885a4bc...25ae` / `0ed964db...e81b`；`20260721T002333Z...`、`20260721T002518Z...` |
| `0f985e28` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 332.446714 s；summary SHA-256 `cb551c16...32c7`；`20260721T001748Z...` |
| `18a305d6` | lower symlink overlay test-only RED | 101 | FAIL，VFS 缺 `get_link_attr/read_link`，编译期 `E0407/E0599` | `cargo test -p axfs --test test_overlayfs` |
| `639d8034` | lower ext4 symlink targeted + RV/LA fresh ext4 probe + normal builds | 0 | PASS，readlink/no-follow/follow/link exec resolution；两架构 build | fresh 临时镜像已删除，原 backing SHA 不变；普通文件 readlink 与 BusyBox applet exit 127 不误记 PASS |
| `639d8034` | 首次 canonical quick | 1 | FAIL，45/47；仅 linux-boundary check/unit 因 legacy copyout 库存增长失败 | 334.346206 s；summary SHA-256 `8e238639...3411`；`20260721T010426Z...`，完整保留 |
| `a4d25d56` | typed readlink copyout boundary + RV/LA normal build | 0 | PASS，boundary check、17 unittest、两架构 build；legacy 库存 47 -> 45 | typed `UserSlice` 地址/长度边界，无新增 unsafe/依赖 |
| `a4d25d56` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 330.628906 s；summary SHA-256 `92242488...e47`；`20260721T011516Z...` |
| `a4d25d56` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance stable | 98.810174 / 95.626819 s；summary SHA-256 `0d211db0...b4ae` / `68b5e0fe...f5efb`；`20260721T012057Z...`、`20260721T012248Z...` |
| `154c4bad` / `7cf8ab03` / `05d4e21c` / `9d93ab27` | final launch test-only RED | 1/非零 | FAIL/ERROR（预期缺 suite dir、预算、POSIX shell、judge label 与多分支 framing） | adapter 首次 7 PASS/1 ERROR；integrity guard 显式 findings，均保留 |
| `bae025db` | final launch focused GREEN + RV/LA exact-env build | 0 | PASS，integrity 29/29、adapter 8/8、asset 0 finding、双架构 build | 18000 s guest ceiling；一次错误资产脚本路径 exit 2 单列保留 |
| `bae025db` | 首次 canonical quick preflight | 2 | ERROR，0 case executed；manifest identity 7/8 不一致 | fail-closed infrastructure error；未覆盖为 PASS |
| `b5616bbd` | test-asset/suite-runner focused | 0 | PASS，171/171 | 修复实际/manifest/canonical 计数为 7/29/8 |
| `b5616bbd` | canonical quick | 0 | PASS，47/47，零非 PASS | 343.386464 s；summary SHA-256 `b3a127db...28a0`；`20260721T015831Z...` |
| `b5616bbd` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS | 112.250301 / 98.597706 s；summary SHA-256 `7601e42c...d6c3` / `272c77f0...368e`；`20260721T020438Z...`、`20260721T020651Z...` |
| `d2a73a5b` | RV/LA minimal Debian-rootfs final launch probe | 0 / 0 QEMU，guest 1 / 1 | FAIL / FAIL（预期缺 rustc/cargo/tgoskits）；脚本真实执行、单一 `buildstorm` frame、无 PASS | console SHA-256 `8f9c4f45...fba` / `e1eacc46...1797`；`20260721T022616Z-buildstorm-rootfs-probe/`；首次错误 RV binary probe timeout 124 单列保留 |
| `007ae7aa` | standard-root dynamic glibc test-only RED | 1 | FAIL，guard 三 finding；LA runtime rc 127、RV rc 0 | LA `libc.so.6` lookup failure；pre-fix console SHA-256 `fd221032...defe` / `897f2267...a7c` |
| `adacda8c` | RV/LA standard-root dynamic glibc GREEN + exact-env normal builds | 0 / 0 | PASS，LA 127 -> 0，RV 0 -> 0；backing hashes unchanged | after console SHA-256 `f37c0552...fa8bf` / `f5789f22...f73c`；非官方最小动态载荷 |
| `66a453fe` | compliance/mutation + asset + suite-runner focused | 0 | PASS，9/9 + 36/36 + 135/135 | 一次错误双文件 harness 调用 exit 2 后分别重跑；错误未计 PASS |
| `ab2d9880` | fork-shared path metadata test-only RED | 1 / 1 | FAIL，guard 两 finding；unit 11 项中 3 FAIL | RV/LA 通用 hardlink guest 均为父进程 `ENOENT`、outer FAIL；三次未执行探针的脚本错误另保留 |
| `1b5aa307` | shared path metadata focused + exact-env RV/LA builds | 0 | PASS，guard + 11/11 mutation + fmt/diff + 双架构 build | 无新增 unsafe/依赖/架构或测例分支；同一通用 hardlink 行为 RV/LA 均 GREEN |
| `1b5aa307` | canonical quick | 0 | PASS，47/47，零非 PASS | 340.987273 s；summary SHA-256 `b16fff22...daa9`；`20260721T030009Z...` |
| `1b5aa307` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS | 99.409973 / 95.518897 s；summary SHA-256 `1dc25a8e...18a0` / `eaf1de82...8017`；`20260721T030608Z...`、`20260721T030758Z...` |

# 8. 最终审查

- [ ] `git diff --check` 通过。
- [ ] 无测例特化、假成功或吞退出码。
- [ ] 无凭据、机器绝对路径泄漏或大体积生成物。
- [ ] Linux/ABI/errno/并发/资源回收已检查。
- [ ] RISC-V64 与 LoongArch64 完整门禁通过。
- [ ] AI 和外部来源披露完整。
- [ ] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 负责人能够不依赖 AI 解释和调试本 PR。

审查人及结论：待完成。

# 9. 已知限制、后续工作与回滚

## 已知限制

官方 final rootfs 与合格 8c/8G host 当前缺失，因此所有 final 分数均为 BLOCKED/未执行；
比赛方公开 share 可列出两个 gzip artifact，但匿名第一方接口未提供可审计下载 URL，未借助
第三方或凭据绕过。该外部缺口不影响继续完成 runner、parser、build 和通用 guest 语义工作。

## 后续工作

按活动计划持续执行，资源到位后必须先在精确 d989 或可追溯等价基线重建真实分数矩阵，
不能从后续候选或历史记录倒推 baseline。

## 回滚方式

后续每个逻辑提交使用普通 revert；不 reset --hard、破坏性 rebase 或覆盖历史。

# 10. 最终摘要

任务进行中；尚无 final-2026 得分，未宣称 PASS、Ready 或 merge-ready。
