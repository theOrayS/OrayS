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
head_commit: "d8e1055d70df2e1aac56e86523fb17e43ff49470"
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

## 2026-07-21 — Checkpoint 20：hardlink canonical unlink/rename 生命周期

- 在 Checkpoint 19 让 hardlink 元数据跨 fork 可见后继续审计其一般生命周期，确认别名只是
  `path -> canonical physical path` 映射：删除 canonical 名会直接删除唯一物理 backing，所有
  剩余别名随即 `ENOENT`；普通 rename 虽移动物理文件，却不更新别名 target 和链接计数。这是
  Linux hardlink inode 生命周期缺陷，不是 BuildStorm 路径或文件名特例。
- 用 fresh 128 MiB ext4、未修改的两架构静态 BusyBox 做行为 RED：创建源文件与硬链接别名，
  删除源文件后由父 shell 读取别名。RV/LA 均打印 `cat: ... No such file or directory`、
  `HARDLINK_CANONICAL_UNLINK fail` 和 guest group FAIL 1；控制台 SHA-256 分别为
  `90e4aa659d9e8bcf0c340d05ab246e55c071c3085b84e9325abdf53f1ee6e0e9` 与
  `3797dfad6622e0789450bc18f1eafee381753da3a7ba3abfb0d587163fb98563`。第一次 RV 误用
  `-smp 2` 启动按 `max-cpu-num=1` 编译的内核，在探针前触发真实 `cpumask` assertion panic；
  该日志 SHA-256 为 `b0bf6d9d...d890`，明确归类为探针未执行的启动配置错误，未计作 hardlink
  RED 或 PASS，也没有隐藏。
- test-only `74b1a4e1` 增加两条通用 mutation 与 guard：`move_path_metadata` 必须迁移 hardlink
  canonical target/count，`unlinkat` 删除 canonical 时必须选择剩余别名、执行真实 rename 并
  迁移元数据；actual/manifest/canonical identity 从 11 同步到 13。生产修复前 guard 精确
  报 8 个 finding，unit 13 项有 3 个预期失败。首次 GREEN 时发现 promotion 变异器仍保留
  guard 关键字，导致该 mutation 错误通过；在生产提交前将其修正为语法有效的
  `None::<String>` 并 amend test-only 提交，随后 guard 与 13/13 mutation 全部 PASS。
- 生产 `ef3d6ef9` 在删除 canonical 且仍有别名时，不复制数据、不吞错，而是以底层
  `axfs::api::rename` 将物理目录项提升为字典序第一个剩余别名；成功后统一迁移 inode/mode/
  owner/xattr/time/sparse 与 hardlink canonical/count 元数据。普通 rename 和已有三步
  `RENAME_EXCHANGE` 也通过同一 helper 迁移整组映射。失败的真实 rename 原样返回 errno；没有
  新增 syscall、架构分支、依赖、`unsafe`、Rust/Cargo/BuildStorm 名称或测例路径特化。
- 精确 BuildStorm adapter 环境下 RV/LA 内核构建均退出 0；compliance guard、13/13 mutation、
  `cargo fmt --all -- --check` 与 `git diff --check` 全部 PASS。扩展后的同一 guest 探针同时执行
  canonical rename 与 canonical unlink：两架构都打印 `HARDLINK_CANONICAL_RENAME pass`、
  `HARDLINK_CANONICAL_UNLINK pass` 与单一 group PASS。GREEN 控制台 SHA-256 为
  RV `24ad560d7e668f528fb5fd28d8e056ad616afbb8ece601644d01358faff85bd3`、LA
  `f80f3fa04915c8afc2e49ad038c23061223a1601e797e363f2fc7d0c032bd090`。运行前后临时 raw 镜像
  SHA-256 保持 RV `834510d94631343f9216eb63c5aae3ba6faa2bda5d5a1f21374d2eccf3556683`、
  LA `05e83809ec449b42ac5bbe9be77729b2cb289f389bb2a035fa4579db3aa07d5b`；这里的 group PASS
  仅证明通用最小行为，不是正式 BuildStorm 得分。ignored 证据目录为
  `test/output/20260721T-hardlink-canonical-lifecycle/`；53 MiB sparse 临时镜像目录
  `/tmp/orays-hardlink-canonical.9UTes2` 已按精确路径移入回收站，可恢复且未触碰官方镜像。
- clean `ef3d6ef9` canonical quick 47/47 PASS、零非 PASS bucket，338.054299 s，summary
  SHA-256 `5926edf738e21f6bd1f1904fd88ccb5259f8ba38d248ffc43012272f3119b1c1`；RV/LA canonical
  runtime 各 1/1 PASS，98.335347/94.643545 s，summary SHA-256 分别为
  `0c74d04452537d1b96c76ab5822e6c132827bbbcce041c762742e89de614562b` 与
  `b02b8d6a3cb84edb8589a23963ca7cb6436a11277a331d876b03189f2131a9b4`。正式 final rootfs
  和合格 8c/8G host 仍缺失，官方结果继续为 BLOCKED/未执行。

## 2026-07-21 — Checkpoint 21：阻塞 `flock` 竞争语义

- 继续按 BuildStorm/Cargo 的一般并发文件访问形态审计锁实现，确认 `sys_flock` 通过持有
  fd-table 锁的 `FdTable::flock` 执行；底层遇到任何冲突都立即返回 `EAGAIN`，没有区分是否
  携带 `LOCK_NB`。这既违反未指定 `LOCK_NB` 时应等待的 Linux 可见语义，也意味着若直接在
  旧临界区内增加等待会阻塞同进程的其他 fd 操作。
- test-only `e487eef8` 增加路径无关的父子进程竞争合同：父进程独立打开文件并持有
  `LOCK_EX`，child 关闭继承 fd 后再次独立 open，发布 ready byte 后执行阻塞 `LOCK_EX`；父进程
  延迟 50 ms 后用 `wait4(WNOHANG)` 必须观察到 child 尚未退出，解锁后 child 才能取得/释放锁并
  正常退出。测试只在 no_std semantic smoke 中增加带不变量说明的 syscall wrapper，没有在
  生产代码中加入测试名或路径。focused semantic unit 75/75、资产完整性 0 finding、format/diff
  检查均 PASS。
- clean test-only RED 在 RV/LA canonical runtime 都实际执行 1/1 并得到 FAIL，而非 parser 或
  基础设施错误；guest 均在既有 marker 全部通过后明确输出 `USER_FAIL flock_blocking` 与
  `HARNESS_FAIL ... guest_nonzero_exit`，没有 panic、trap、ENOSYS 或 timeout。RV/LA summary
  SHA-256 分别为 `a42b33515aff6b8eb386affd7a864ddd04053e6156b037a03c2b8dc8a64812e5`、
  `d2ed8758980f5ce43c403b094c27ddbbe4efdd53ef39e5b5e3bf9a0528319c2b`，耗时
  98.767828/95.070846 s；固化 RED console SHA-256 为
  `0eccf71bb790edfaf3c240e61f7b54d929fe671bc90fbc15d7d0866b1e4e0781`、
  `eeca6d9db8e28c769d464daa762d34bcfa2e89f91877413c7c430dc3c9dc49e8`，保存在 ignored
  `test/output/20260721T-flock-blocking-red/`。
- 生产 `7597bd9c` 先在 fd-table 锁内验证并 clone `FileEntry`，随后释放该锁再执行 flock；
  `LOCK_SH`/`LOCK_EX` 冲突且未带 `LOCK_NB` 时 cooperative yield 后重试，带 `LOCK_NB` 仍原样
  返回 `EAGAIN`，exit-group、watchdog 或未屏蔽 signal 则返回 `EINTR`。`LOCK_UN` 与非法 flag
  的原有行为保持不变；没有新增依赖、`unsafe`、ABI/架构分支或测试路径特化。精确 BuildStorm
  adapter 环境下 RV/LA 内核构建均退出 0。
- clean `7597bd9c` RV/LA canonical runtime 均为 1/1 PASS，guest 明确输出
  `ASSERT flock_blocking PASS`；耗时 97.159781/94.093020 s，summary SHA-256 分别为
  `7f855807cb338062f3d19281eeff285608129cb39bdc21ee7a6bf36675cd72ca`、
  `81b73a3c69d7e4ea9aebf7bc9e9913ed2fa712afc85293909f4f41d4ef9e77ae`，guest console
  SHA-256 分别为 `67dc986948db23cf052560f837a69d31d892957a5d2f0a7124742067c3e29b70`、
  `e21bc1d54e189cfa9ec3e740dc6f38e902ca7fb1c0f4efbf8d9cf5feb5009aa1`。同一 clean commit
  canonical quick planned/executed/completed 47/47/47、PASS 47、其余 bucket 全 0，耗时
  339.530198 s，summary SHA-256
  `bf40c2abe93de8d7f609274a506f3f929243f038f37bdc214826c0e1f733df35`。这些是通用内核语义
  证据，不等于正式 BuildStorm 分数；官方 rootfs 与合格 8c/8G host 缺口保持不变。

## 2026-07-21 — Checkpoint 22：hardlink source 的 replacement rename

- 继续审计 `renameat2_paths` 时确认：source 若为 metadata-backed hardlink alias，destination
  只要已存在就无条件返回 `EEXIST`，即使 flags 为普通 replacement rename；此外物理 source
  覆盖 hardlink destination 时，直接底层 rename 会使 destination 的其他 alias 错误转指新
  inode。这违反 Linux rename 与 hardlink 身份语义，也会阻断构建工具常用的临时产物发布。
- test-only `fdbdb383` 使用三个通用 `/tmp` 文件：写入 source、创建 hardlink alias、写入不同
  内容的既有 target，再执行 `renameat2(alias, target, 0)`；要求 target/source 都读取源内容、
  旧 alias 返回 `ENOENT`、清理成功。raw syscall wrapper 只存在于 no_std smoke 并记录指针
  生命周期。首次编译因误用不存在的 `fd_write` 失败，生产未修改且 LA 未启动；改用已有通用
  SYS_WRITE wrapper 后 RV/LA 测试源均编译。semantic unit 75/75、资产 0 finding 均 PASS。
- clean test-only RV/LA canonical runtime 均实际执行 1/1 后 FAIL，guest 明确输出
  `USER_FAIL hardlink_rename_replace` 与 harness nonzero，且无 panic/trap/ENOSYS/timeout。耗时
  97.570168/94.536128 s，summary SHA-256 分别为
  `9f7f3c739d8424373c854ee4c14bb2092a9628dbd107b748f61a992e25a685b0`、
  `db80cc8520a4e7453c4da580e0cdc8c192699c941c54f1b7fabf9e57e3c42bea`；固化 RED console
  SHA-256 为 `c1a1f1719dbf9d327d9fb6144ba276ff09a583cf4e9e3e1d7c77e365ff7d5886`、
  `bf553e3f5b88081aecd08a5f6c711738da4cc102b442a69759143db2e5e9ea47`，位于 ignored
  `test/output/20260721T-hardlink-rename-replace-red/`。
- 生产 `d8e1055d` 在任何变更前完成 same-inode、exchange、目录/文件类型、sticky、跨挂载和
  只读挂载校验。普通 rename 若需移除 recorded target：hardlink alias 只减少链接；canonical
  且有剩余 alias 时真实 rename backing 完成 promotion；普通文件才调用真实 remove。随后迁移
  source alias，底层错误全部以 `?` 返回。统一 `clear_unlinked_path_metadata` 同时清理 mode、
  inode、special/rdev、owner、symlink、hardlink、flags、xattr、times 与 sparse state，替换了
  unlink 的不完整重复清理。无新增依赖、unsafe、架构或测试路径分支；移除 4 个已失去调用者的
  私有 helper，使双架构 warning 数保持既有 175。
- focused compliance 0 finding，合规+semantic 88/88、资产 0 finding，RV/LA smoke kernel 构建
  均退出 0。clean `d8e1055d` RV/LA canonical runtime 均 1/1 PASS 并明确输出
  `ASSERT hardlink_rename_replace PASS`；耗时 97.235054/94.538765 s，summary SHA-256 分别为
  `1dcfc81d04b8ef4ad00cf0b763871ed3687e0b8a2143e94a67e1200945879ab7`、
  `6ce456c53187fe45f31bb2272514b99d7c8e91f7b6e9f6f359ae6abf884ad45d`，console SHA-256 为
  `78b3b3108b72d6130ab02281b87a992b1e28a1de91c33934e12d9f994fe6b1c5`、
  `62eaa09d1da66f4ec3db4b1021076395c6350ccbafd2640606be89e3d4ff3dd9`。同一 clean commit
  quick planned/executed/completed 47/47/47、PASS 47、其余全 0，339.902215 s，summary
  SHA-256 `34564cd0b9fe2d7c18a2e9e6593cb516021c82f8cb8d7b0c1a5301dd3ae71d7c`。这些仍是
  通用语义证据，不是正式 final 分数。

## 2026-07-21 — Checkpoint 23：`/proc/<pid>/statm` 真实进程内存统计

- 对最小 host Cargo/rustc 构建执行只读 `strace` 后确认，rustc 会打开
  `/proc/self/statm`，再以空路径和 `AT_EMPTY_PATH` 对已打开 fd 执行 `statx`，读取七个
  无符号页计数字段。OrayS 已支持同形态 `statx`，但 `/proc/self/statm` 不存在并返回
  `ENOENT`；这是正式 rootfs 到位前仍可确定的通用 BuildStorm correctness 阻塞，而不是按
  测例名或工具名特化。host 探针位于临时目录
  `/tmp/orays-buildstorm-syscall-audit.MBQSBz/`，只用于发现，不作为 guest PASS 证据。
- test-only `ed5a0773` 在 RV/LA no_std smoke 中打开 `/proc/self/statm`，对 fd 执行
  `statx(AT_EMPTY_PATH)`，验证普通文件类型、恰好七个可解析字段及 size/resident/shared/
  text/data 的基本关系。focused semantic/资产检查通过。clean test-only canonical RED
  在 RV/LA 均实际执行 1/1 并得到 FAIL，guest 精确输出 `USER_FAIL proc_statm_open`，无
  panic/trap/ENOSYS/timeout；耗时 96.415339/92.116888 s，summary SHA-256 分别为
  `788901a4d3e178f653e6ea6f17d8d38c2dfa486c4a56cf6554ac2f7298b0a102`、
  `063f11d4b9b9a473ca13e6944ce8a1f583ed99e3c01269445611766c9aa4b9ed`。固化 RED console
  SHA-256 为 `b2bb090d0560c6841dc20dee4139905df45b094f6004b300f89e5127ba3a6842`、
  `8856943b4c329623cf7219ed7b503de7d6992791992c31391bdb08d3c83e0dd5`，位于 ignored
  `test/output/20260721T-proc-statm-red/`。
- 生产 `91a5cb98` 为 `/proc/self/statm` 与 live `/proc/<pid>/statm` 生成 Linux 七字段快照：
  size/text/data 从合并后的 VMA、brk 与用户栈区间计算，resident/shared 逐页查询目标进程
  当前页表，library/dirty 保持 Linux 已废弃字段的零值；同一 synthetic memory-file fd
  继续复用通用 `statx(AT_EMPTY_PATH)`。只读打开、目录/写打开 errno 与 proc pid 目录枚举
  均按既有 synthetic procfs 合同接入。实现没有新增依赖、`unsafe`、架构或测试路径分支。
- 曾尝试在 `axmm` 增加稀疏页表 walker 以避免逐 VMA 页查询：第一次 RV 编译得到 callback
  entry 类型推断 `E0282`，第二次改用 `PageTable::query` 后得到高阶生命周期约束不满足；两次
  均未进入 LA 或运行阶段，也未保留在提交中。最终回退为只查询已合并的可见 VMA 页，避免
  扩大 MM/HAL API 与 unsafe 边界；这些编译失败不计 PASS。
- focused semantic、asset、kernel-state 检查均 0 finding，相关单元测试 152/152 PASS。
  脏树 RV/LA 只作诊断，均明确通过新 marker 及完整 harness，但不计 canonical evidence。
  clean `91a5cb98` canonical RV/LA runtime 均 1/1 PASS，耗时 97.918160/94.675474 s；summary
  SHA-256 为 `67ede158f2b5e4e4dda529db0223521adcae623f6de53b3a9b5fc4d58ee372d7`、
  `23cc2a53eb2681898333b86a09f25d7ffcf3bc26b70579eb08184e47a861ab37`，console SHA-256 为
  `5748a72685d34ece2aa35f2c437496d19be090321dea2197caaaf0cf89802168`、
  `430af21a0d5a059ec038ad8eca63b42f8f9118fa73d8b726e85110909ed0e30e`。两份 console 均含
  `ASSERT proc_statm PASS`、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，且无隐藏失败 token。
  同一 clean commit canonical quick 为 47/47 PASS、其余 bucket 全 0，336.203409 s，summary
  SHA-256 `4fb056980cdce4f1bdad2ccee8b9087cb16d484d4d555738d2e8d20483d770d9`。这些仍不是正式
  BuildStorm 分数；官方 rootfs 与合格 8c/8G host 缺口保持不变。

## 2026-07-21 — Checkpoint 24：Cargo hardlink working-directory publish RED -> GREEN

- host-only Cargo/rustc syscall trace（只用于发现通用行为，不作为 guest 或得分证据）记录
  56 次 `linkat`：rustc 主要把 `target/debug/deps/*.o` 链接进
  `target/debug/incremental/*-working/`，再 rename 整个 working 目录发布 session。旧实现的
  hardlink alias 只存在于路径 metadata：`getdents64` 不枚举它，目录 rename 也只移动目录
  自身 metadata，不移动后代或 hardlink canonical 引用。因此 Cargo 即使 `linkat` 成功，
  后续也可能看不到对象，或在发布目录后失去 alias。
- test-only `ee37e940eefa8875a5f4d6744c4a93de17fc8539` 固化通用合同：创建 source 与
  working 目录，在目录内 `linkat` 一个 `.o` alias，要求 `getdents64` 枚举 alias、两名同
  inode 且 `st_nlink=2`；rename 整个 working 目录后只允许新路径读取，最后 unlink alias
  后 source `st_nlink=1`。第一次测试编译因动态 slice 索引引入 freestanding bounds-panic
  符号而链接失败；改用 checked offset 与逐字节 `.get()` 后两架构构建成功。该编译失败未
  启动 guest、未计 PASS，也没有修改生产代码迎合链接器。
- clean test-only RV/LA canonical runtime 均真实执行 1/1 后 FAIL，guest 精确输出
  `USER_FAIL cargo_link_publish`，harness 非零，且无 panic/trap/ENOSYS/timeout。耗时
  111.912367/96.236053 s，summary SHA-256 为
  `b500a22581960b5ba350e2208e6774d0386b7cd391b43a8a382f4bf8dd92e16f`、
  `458896618a1ebf9348c3d3b4cb2460e9a9ab645c68ac4d22cb44b7bb08db45cf`；RED console
  SHA-256 为 `e3257133f8a88d521c3719346016d320ebee4846db03801e7ec555c2494e132e`、
  `ce08bd90bc1850858284fe79e9d87aeba79833994bdc88fb1b858b7f2c5becd1`，固化于 ignored
  `test/output/20260721T-cargo-link-publish-red/`。
- 生产 `35d1b2705e5f8f66671de3bbcd28020394ccd791` 增加通用路径子树迁移：普通目录 rename
  和 `RENAME_EXCHANGE` 同步移动 mode/inode/special/owner/symlink/hardlink/xattr/time/sparse
  等 metadata；hardlink map 的 key、旧子树 canonical value 与 link-count key 同步改写。
  `getdents64` 只额外枚举非 canonical alias 并标记 `DT_REG`，避免重复真实物理 canonical
  目录项。实现无新增依赖、`unsafe`、架构或测试/Cargo 路径分支。
- focused competition/assets/kernel-state/POSIX-state/stat-metadata guards 均 0 finding，相关
  unittest 171/171 PASS，RV/LA production build 均退出 0，warning 保持既有 175。脏树
  RV/LA 运行只用于诊断且均通过完整 marker 链，不计 canonical evidence。
- clean `35d1b270` RV/LA canonical runtime 均 1/1 PASS，明确输出
  `ASSERT cargo_link_publish PASS`、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，无 fatal token。
  耗时 99.980872/95.891415 s，summary SHA-256 为
  `3804738007f59d6799aeec1f7118d3ea0692d3c6faefc043e41df2522f32a94e`、
  `65110d37e6011fe9e1bc2f218772f117b29367ee96bdd2f863fbacacfd4d742e`；console SHA-256 为
  `09144062ded1c8e96815c23c2aa03d2fef14cf815893dd0c2104bac71667b0a8`、
  `b5f9cf09328a2e5dbe7c58a1408fa2704e6150cc751c1f76e3c0e906286c9204`。证据位于
  `test/output/20260721T053957Z-evidence-runtime-rv-3394720/` 与
  `test/output/20260721T054213Z-evidence-runtime-la-3396326/`。
- 同一 clean commit canonical quick 为 planned/executed/completed/pass=47/47/47/47，全部
  非 PASS bucket 为零，336.304565 s；summary SHA-256
  `ea2595cf4c76b6986a144ddce3aa0dc907b76683acc0cb5c176687b2aeed945e`，证据位于
  `test/output/20260721T054440Z-quick-none-3397825/`。这证明通用 Cargo 式目录发布语义，
  不是正式 toolchain/minibuild/full BuildStorm 执行或得分；final rootfs 与合格 8c/8G host
  缺口保持不变。

## 2026-07-21 — Checkpoint 25：虚拟非空目标目录的 replacement rename RED -> GREEN

- 延续同一 host Cargo/rustc syscall trace 审计普通目录发布。`35d1b270` 后 hardlink/symlink
  alias 会参与 `getdents64`，但这些目录项仍只存在于共享路径 metadata，底层 VFS 认为目录
  为空。普通 `rename(source_dir, target_dir)` 因而能错误替换逻辑非空的 target，并把仍存留的
  alias metadata 与新目录混合；Linux 对该形态必须返回 `ENOTEMPTY`/`EEXIST`。该检查只约束
  replacement rename，`RENAME_EXCHANGE` 仍允许交换两个非空目录。
- test-only `d904eac64ddffc359782a1445154e2a62faca77c` 固化通用合同：target 的唯一子项是
  metadata-backed hardlink `retained.o`，source 是空目录；普通 rename 必须返回
  `-ENOTEMPTY`，target 仍枚举并可读取子项，source 仍存在，最终清理必须成功。clean test-only
  RV/LA canonical runtime 均实际执行 1/1 后 FAIL，guest 精确输出
  `USER_FAIL rename_virtual_nonempty`、harness 非零和 `SHUTDOWN`，且无
  panic/trap/ENOSYS/timeout。耗时 126.272334/95.943972 s，summary SHA-256 为
  `2cacfd7be496429fe2b73ccf2ab8b5b8d07245d44b9437114d00c8c0fa913330`、
  `13712eaab9e96487ab63e819bf6bf8e6dafb08e67cdcbf36cb179b8e5bf195e4`；RED console
  SHA-256 为 `ce30cd009a837d9fcb4fdd9d9c18f7605b4e774035f4e515c60fc5cc713de2e0`、
  `ce1bc0dfecae91b879abec9280f58b4c9755627db4ed76a158075a14c1efbc63`，保存在 ignored
  `test/output/20260721T-rename-virtual-nonempty-red/`。
- `65b3691524a96dbdfddbf6708be2420fceba16fd` 增加只在失败时输出 syscall 返回值和状态位的
  测试诊断，不改变 PASS 合同。生产 `1b990b8f1959c1e61a68417f26964590c1be235b` 增加通用
  `path_has_virtual_dirents` 查询，在 ordinary rename 的类型检查后、任何 namespace 变更前对
  逻辑非空目标返回 `ENOTEMPTY`；exchange 已在该分支前单独完成，因此语义不受影响。实现无
  新增依赖、`unsafe`、架构或 Cargo/测例路径分支。
- focused semantic/POSIX-state/stat-metadata unittest 97/97 PASS，competition evidence、
  test-asset、POSIX-state 与 stat-metadata 四项 guard 均 0 finding，`cargo fmt --check` 和
  `git diff --check` 通过；RV/LA 生产构建及脏树诊断 guest 均通过完整 marker 链。首次 LA
  脏树运行仍使用旧产物而失败，显式重建后通过且失败诊断未触发，未把脏树结果计为 canonical
  evidence。期间三次错误 unittest module 名、一次不存在的 make target、一次缺少 QEMU env、
  一次错误顶层 guard 路径以及一次缺失 `jq` 均保留为命令/环境错误，不计 PASS。
- clean `1b990b8f` RV/LA canonical runtime 均 1/1 PASS，runner commit 前后一致且工作树
  clean，明确输出 `ASSERT rename_virtual_nonempty PASS`、`USER_PASS`、`HARNESS_PASS`、
  `SHUTDOWN`，无 fatal token。耗时 98.821608/95.015367 s，summary SHA-256 为
  `f2bf3c03803c18b4caa70a13d59cf483928a829e71c16a45dd5f4fcf7b0aba81`、
  `021bda50b0eb3bc136016cf12961763e85a16c261794027edd4ae6ee5d132007`；inner evidence
  SHA-256 为 `cc16fe2e92a755c82a5413057603f64c759c4bc693f3de41c6d21d08b2d9019f`、
  `acd25923183cad8001a04548d5473a81ce15af6321a31513f51894ef7b99010e`；raw console
  SHA-256 为 `96be20e879185533662f2d32215b9c6334f78644625490e24db36cc500c85005`、
  `0ed8b708ddd3785e18d56721f67e9a8c92ef4cf0f8fcceeeaf2ccdc1b4253720`。证据位于
  `test/output/20260721T061345Z-evidence-runtime-rv-3415342/` 与
  `test/output/20260721T061531Z-evidence-runtime-la-3416901/`。
- 同一 clean commit canonical quick 为 planned/executed/completed/pass=47/47/47/47，所有
  非 PASS bucket 为零，335.435847 s；summary SHA-256
  `57ed2a381f231c6fddc5038e4b66d2e6a3147aab0ad3dd314f4fe985ef54bcdf`，证据位于
  `test/output/20260721T061757Z-quick-none-3418526/`。这仍只是通用 guest 语义和仓库门禁，
  不是正式 BuildStorm 执行或得分；final rootfs 与合格 8c/8G host 缺口保持不变。

## 2026-07-21 — Checkpoint 26：Unix `SOCK_SEQPACKET` 记录语义 RED -> GREEN

- 对 host 上真实最小 Cargo/rustc 构建做只读 syscall trace 后，确认其两次创建
  `socketpair(AF_UNIX, SOCK_SEQPACKET|SOCK_CLOEXEC, 0, ...)`：父进程关闭不使用的 peer，
  再以 `recvfrom(fd, ..., flags=0, NULL, NULL)` 等待子进程 exec/exit 后的 EOF。该 trace 仅用于
  选择通用 Linux ABI 合同，不是 guest 或正式 BuildStorm 执行；trace 中大量 `readlink(EINVAL)`、
  `openat/newfstatat(ENOENT)`、`futex(EAGAIN)` 是正常探测/竞争结果，没有当作待“修复”失败。
  源码审计确认本地 `socketpair` 只接受 stream/dgram，`SOCK_SEQPACKET` 返回
  `ESOCKTNOSUPPORT`；即使直接放行类型，现有单一字节环也会把记录错误拼接，且
  `sys_recvfrom_bridge` 会在识别 local socket 前走 Inet-only `socket_entry` 并返回 `ENOTSOCK`。
- test-only `f19fc9f6e8cea195fdf4a36ce2810f3df7015524` 固化路径无关的 Linux 合同：请求
  `AF_UNIX + SOCK_SEQPACKET|SOCK_CLOEXEC|SOCK_NONBLOCK`，核验 `F_GETFD/F_GETFL/SO_TYPE`；
  连续写入 `ABC`、`DEFG`，先用 2-byte buffer 读取 `AB`，要求首记录余下的 `C` 被丢弃，下一次
  必须得到完整 `DEFG`，从而排除“映射成 stream”假修复；空队列返回 `EAGAIN`，peer close 后
  返回 EOF。clean RV/LA canonical runtime 均实际执行 1/1 后 FAIL，明确输出
  `USER_FAIL unix_seqpacket`、harness 非零与 `SHUTDOWN`，无 panic/trap/ENOSYS/timeout。
  耗时 98.436434/96.429793 s；outer summary SHA-256 为
  `2b21ed70dd058fa7a93cbbe013d44281e618e39c3acab4b0c6c16d6855c59871`、
  `76d55a1208f4b90c1101a7d11760a8c95beb512474c77c44711e868c9ebf6e6f`，原始 console
  SHA-256 为 `6288b9f3315cfbf0463ac6e2e533d2acf5d2efa9eca43c064c173e1432793566`、
  `333e6a6c2f62a86cc81038646d29341ba1f37ae35ff1f76e81a93751a924211c`。outer 证据保存在
  `test/output/20260721T063748Z-evidence-runtime-rv-3429808/` 与
  `test/output/20260721T064003Z-evidence-runtime-la-3431406/`；build evidence 目录后续按 runner
  合同被新运行覆盖，因此不把旧路径冒充为仍保留的 raw artifact。
- 生产 `89740c70ab107fe1ddbde9a8cf9a8f9aaa223c10` 为 AF_UNIX dgram/seqpacket 增加每方向
  64 KiB、最多 1024 records 的有界队列；每次写入先完整分配再原子入队，超长返回 `EMSGSIZE`，
  满队列按 socket 状态阻塞或返回 `EAGAIN`，read/poll/wait/peer-close 唤醒均区分 packet 与
  stream。短读弹出整条记录并只 copy 最小长度；stream 环形缓冲不变。本地 `recvfrom` 支持
  flags 0/`MSG_DONTWAIT` 并在不支持 peer-address 状态时于消费记录前 fail-closed；未知 flags
  不被静默吞掉。实现没有新增依赖、`unsafe`、架构分支、Cargo/rustc/BuildStorm 名称或路径判断。
- focused semantic/competition/POSIX/stat/socket/boundary 共 157/157 unittest PASS，六项 guard
  均 0 finding，格式/diff 与 RV/LA smoke kernel build 退出 0，warning 保持既有 175。第一次
  clean `89740c70` canonical quick 如实为 45 PASS / 2 FAIL、零 timeout/crash/infra：唯一根因是
  新 local recv copyout 使 `validate_user_write`、`write_user_bytes` legacy inventory 各增加 1；
  summary SHA-256 `c2294ba0c302c897a0171bf3511501ca4c82ee3902d5699132ebe00c414ae2db`，
  证据在 `test/output/20260721T065548Z-quick-none-3439575/`。没有增加允许库存或用后续 PASS
  覆盖该记录。
- `8c23d29af659a83e01632c44ae2b971b62d75490` 增加 typed
  `with_writable_user_slice(UserSlice<u8, Write>)`，让 local recv 在消费记录前由集中边界验证完整
  输出范围，成功后只 copy 实际字节；raw/legacy inventory 恢复原值。最终 clean canonical quick
  为 47/47 PASS、所有非 PASS bucket 为零、336.397217 s，summary SHA-256
  `35159eb6f9e53b894bd7116f05f8cc066bc51efa01aafa0f82259f36e0d88b76`，证据在
  `test/output/20260721T070732Z-quick-none-3447445/`。
- 同一最终 clean commit 的 RV/LA canonical runtime 各 1/1 PASS，明确输出
  `ASSERT unix_seqpacket PASS`、`USER_PASS`、`HARNESS_PASS`、`SHUTDOWN`，无 fatal token；
  耗时 98.754050/95.747174 s。outer summary SHA-256 为
  `ad1f5536d77cee146d79e68c7499b80e15ddc253daf4d1f8aaabb2046dfefb5c`、
  `af8f094a678be1fc3a0f19d60d37054d60ae2da7bd2e2e8625eca3d6fcd5a18b`；inner evidence 为
  `13a44983536ef09619b21b7bc73625b4ff96789fd53ac30f253607620f27ac04`、
  `e7018efcc2b5b98038f4cb73587904734925c302a454be02b760c95cb25fa34b`；console 为
  `93aba8cab8a0446ba459986821ccb2a968261373e1f4a516aa4ffa9100505ad4`、
  `73dffd684d221f8b115c667e31f867d4e55962ba47e2d4993072aaa45b953526`。证据位于
  `test/output/20260721T071332Z-evidence-runtime-rv-3452330/` 与
  `test/output/20260721T071529Z-evidence-runtime-la-3453868/`。这证明通用 socket 合同，不等于
  正式 rustc/Cargo 或 BuildStorm 得分；final rootfs 与合格 8c/8G host 仍缺失，官方状态继续
  `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 27：rename-stable 文件对象锁身份 RED -> GREEN

- host 上对最小 Cargo/rustc 构建的 syscall trace 继续只读分类：常见失败主要是正常探测，
  包括 `readlink(EINVAL)` 1512 次、`openat(ENOENT)` 633 次、`newfstatat(ENOENT)` 628 次与
  `futex(EAGAIN)` 303 次，没有发现意外 `ENOSYS`。trace 共见 `flock` 22 次；独立 Linux
  行为探针确认：旧 fd 持有 `LOCK_EX` 后 rename，按新名称独立 open 的 `LOCK_EX|LOCK_NB`
  必须返回 `EAGAIN`，旧 fd 解锁后才能取得锁。
- 源码审计定位到 `record_lock_key`、`file_lease_key`、`flock_key` 都使用
  `path_inode(Some(file.path))`。路径在 rename 后变化，因此旧 fd 和新路径 open 会进入不同
  锁桶；同路径删除重建又可能错误复用旧桶。`rseq` 同期仍保持 `ENOSYS`：当前没有完整迁移/
  抢占/signal abort 语义，拒绝用无效果注册伪造成功。
- test-only `0c97a5f56cc931ff6e122abd3980940ba2b6a914` 增加通用 rename/open/flock 合同。
  clean canonical RED 在 RV/LA 均实际计划并执行 1/1，分别明确
  `USER_FAIL flock_rename_identity`，无 timeout/crash/infra。RV outer summary SHA-256
  `92e3c4892ea2293f5ef11500804e7cde5c07c0e417d62e33ed8fb3d6faf9ed8b`、console
  `42b34c2d29734da28d3e1e5855aeff1c5556f35b53a33515c95a3795bdfca5c4`，证据在
  `test/output/20260721T073147Z-evidence-runtime-rv-3461400/`；LA 对应为
  `e81c777729e69d3e8de898eb2ca662e2e5610a21e5b298ccd034b1861dccfd42`、
  `3a665f23fb02dfee2c21867ac31efe4f4836265c9a78d9f57b0aec0a053e140d`，证据在
  `test/output/20260721T073352Z-evidence-runtime-la-3462926/`。不会用后续 GREEN 覆盖这些记录。
- `9ca98b0de1907414a1a94fb8e003409482fbc30e` 让低层 `axfs::fops::File` 在 open 时保存
  resolved `VfsNodeRef` 的 data-pointer identity；持有的 `Arc` 保证该值在 File/clone 存活期
  不被复用。API 只把它作为 opaque、非 on-disk inode 的内核对象标识暴露，POSIX/OFD
  record lock、file lease 与 flock 三类共享状态统一使用该 key。实现没有新增依赖、`unsafe`、
  ABI、架构分支或测试/Cargo/BuildStorm 路径判断；rename 保持同一对象，unlink 后同路径新建
  则得到不同对象。
- 静态 guard 同时验证低层 identity 的初始化/clone/API 传播以及三类 key 禁止回退到 pathname；
  mutation 从 15 增至 16。第一次在 clean 生产提交启动 canonical GREEN 时，runner 在执行
  case 前 fail closed（exit 2）：manifest/pinned count 仍为 15，而源码发现 16 个方法。该
  infrastructure ERROR 未算作测试 PASS；`4e2e63dad03abc65cc451b095fd069f5d2d91f4c` 将
  manifest、runner pin 与 migration map 同步为 16，随后绑定 harness 明确
  `planned=started=executed=stopped=16`，runner regression 135/135、资产检查 0 findings。
- focused 验证中 POSIX guard 0 findings、mutation 16/16、semantic 75/75、competition 33/33，
  `cargo fmt --all -- --check` 与 `git diff --check` 通过；RV/LA semantic kernel build 均 exit 0，
  warning 保持既有 175。两条早期误用命令也保留：错误 module 名的 `python3 -m unittest
  test.test_semantic_evidence test.test_competition_compliance` 为 exit 1/`ModuleNotFoundError`，
  不存在的 `python3 test/check_competition_compliance.py` 为 exit 2，二者均未计为测试执行。
- 最终 clean `4e2e63da` canonical GREEN：RV/LA runtime 各 1/1 PASS，耗时
  98.796166/95.574923 s，明确出现 `ASSERT flock_rename_identity PASS`、`USER_PASS` 与
  `HARNESS_PASS`，无 USER_FAIL/panic/trap/ENOSYS/timeout。outer summary SHA-256 为
  `7548545bfe38b97970f37d3b67e7dc3d62a0ca51a6bef88fc71bf45d8dc9df49`、
  `048fdffd33ae265d4b4606d486a07fd465824d7cdcd015d9baabb4fcad84ce3d`；console 为
  `ebffafd80580838b6e9ba8ee046146aee7514f97e7e1eaebb2b935939748785b`、
  `549f9b7af1fc95d21e8a804dd7714e549842a472d7c99ee958bf8f112941ff0e`。证据位于
  `test/output/20260721T074529Z-evidence-runtime-rv-3470432/` 与
  `test/output/20260721T074724Z-evidence-runtime-la-3471957/`。clean quick 又为 47/47 PASS、
  所有非 PASS bucket 为零、336.239994 s，summary SHA-256
  `8c81ac2c9d4bb8738ba58b483c249c9ca4b02038bcf3d46a54d84767b4a9d6f7`，证据在
  `test/output/20260721T074924Z-quick-none-3473384/`。
- 上述仅证明通用 Linux 文件对象锁语义，不是正式 CAgent/BuildStorm 执行或分数。final
  rootfs 与合格 8c/8G host 缺口不变，官方四格继续为 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 28：raised RLIMIT_STACK 真实容量 RED -> GREEN

- 继续只读分类同一最小 Cargo/rustc host trace：共见 `prlimit64` 38 次；rustc 先读取默认
  8 MiB 栈限制，再请求 `{cur=64 MiB,max=infinity}`。同期 `readlink(EINVAL)`、路径
  `ENOENT` 和 `futex(EAGAIN)` 仍主要是正常探测。两个候选没有被强行制造 RED：现有
  `move_path_metadata` 会把 rename 前 inode override 迁到新路径，未发现可证明的 `statx`
  identity 缺陷；`MmapFileBacking::same_backing` 的 pathname 比较只影响内部 exec shared-map
  cache，尚无足够外部错误合同，因此均未改生产代码。
- 源码根因是 `sys_prlimit64` 会接受上述新限制，但 `program_loader` 永久只映射
  `USER_STACK_SIZE=8 MiB`。该映射虽由 `map_alloc(..., populate=false)` 懒分配、无需预占全部
  frame，却没有在 limit 提高后提供相应虚拟地址容量；工具链可得到成功 syscall 后再因合法
  栈使用 SIGSEGV。
- test-only `f21e419cb6f461cc321e508bd28afbb1c84ce2be` 增加通用 fork-child 合同：child
  `prlimit64(RLIMIT_STACK, 64 MiB)` 成功后，以 volatile 访问逐页触碰 12 MiB 栈帧；parent
  必须观察 status 0，否则输出 `USER_FAIL rlimit_stack_growth`。clean canonical RED 在 RV/LA
  均计划并执行 1/1 后 FAIL，无 timeout/crash/infra；RV outer summary/inner evidence/console
  SHA-256 分别为 `59a5ac243e827c65650b2ffe390ef21a77bdbad0e7ea3c88dde7d506bd557555`、
  `12c0b5bf98f7f5f7de4e271f79104acbe4ef20fcab4500504e6b7550c5b065f7`、
  `d48b277bf543465440bbb0b4fb717433691895f9448283e20f27d2f49d347cb3`，证据在
  `test/output/20260721T081438Z-evidence-runtime-rv-3488241/`；LA 对应为
  `646d4fc071bfb0deee78af49b8232af45b83762c86176547a82a9fbaecd373fc`、
  `f18b070a8f2334ad6a0d75356ce5ae13cd72e17ac3da7f038c95590aec6975e8`、
  `8892c9bc1d9e209fafcb425c9d83be97c655c4ceecc357d647356964a6b04eb9`，证据在
  `test/output/20260721T081709Z-evidence-runtime-la-3490017/`。首次失败不会被 GREEN 覆盖。
- `1ce2919036d24eb21332576203bcc7282b3f97d7` 把共享 `USER_STACK_SIZE` 扩为 64 MiB；
  loader、mmap 上界、SysV SHM 避让与 `/proc` 合成视图继续使用同一边界。loader 仍以
  `populate=false` 映射，因此未触碰页不占物理 frame。实现没有新增依赖、`unsafe`、ABI、
  架构分支或 Cargo/rustc/BuildStorm 名称与路径判断；evidence manifest 补入实际的
  `linux_abi.rs`/`program_loader.rs` provenance。该修复满足 trace 中 64 MiB 需求，但当前仍是
  固定容量，不宣称实现任意大小的 rlimit 驱动动态栈增长。
- focused 验证中 rlimit guard 0 findings、对应 unittest 13/13、semantic evidence 75/75、
  suite-runner 135/135、format/diff check 均通过；RV/LA semantic kernel build 均 exit 0，
  44.54/42.35 s，warning 保持既有 175。开发过程中的失败也如实保留：不存在的
  `test/unit/test_test_manifest.py` 导致 harness error，不存在的 `make pr3-smoke-build-rv/la`
  target 导致 make error，二者均未计为验证；首次加入 production provenance 时字典序错误，
  semantic 单测出现 2 个 setup error，修正排序后重跑为 75/75 PASS。
- 最终 clean `1ce29190` canonical GREEN：RV/LA runtime 各 1/1 PASS，耗时
  99.772205/96.869639 s，明确出现 `ASSERT rlimit_stack_growth PASS`、`USER_PASS` 与
  `HARNESS_PASS`，无 USER_FAIL/panic/trap/ENOSYS/timeout。RV outer summary/inner/console
  SHA-256 为 `5585c7992e6d3ef60325cc337b00186dd071713a27b9af32b0c1cf57cf03a1e4`、
  `aefd17a95aa4c4811f6f50ddf0fa2a0e2d046811513ae0face91bbbe7bdf934e`、
  `0f80294a9577aa863dfb053434bee0d6141977e89a7b0f283eff880bda115f35`，证据在
  `test/output/20260721T082115Z-evidence-runtime-rv-3492851/`；LA 对应为
  `c46a8f0866c9f0a15770caf6c7a8d3540b87b2d5da7131d4d5b006d322aaf551`、
  `3a6753889b008800d0b45ffdb7a3a48914a14b86a71fe1e4375fac58961dc19f`、
  `8d5ee66c7399e147b1a06676fad895c434b104eab7f4c3355e50e8ac4f3ab86b`，证据在
  `test/output/20260721T082321Z-evidence-runtime-la-3494419/`。
- 同一 clean commit 的 canonical quick 为 47/47 PASS，所有非 PASS bucket 为零、
  338.801596 s，summary SHA-256
  `aea46eb4b6ffd53f7f253840392d422c1cba588a707a8c59a61f7e50a89fedfc`，证据在
  `test/output/20260721T082610Z-quick-none-3496138/`。以上仍不是正式 CAgent/BuildStorm
  执行或分数；final rootfs 与合格 8c/8G host 缺口不变，官方四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 29：pipe FIONBIO 真实状态 RED -> GREEN

- 对同一最小 Cargo/rustc host trace 继续只读分类，确认 12 次成功的 pipe
  `ioctl(FIONBIO, [0|1])`：父进程把 compiler child 的 stdout/stderr pipe 切换为非阻塞，
  部分读取阶段再清除。OrayS 的 `sys_ioctl` 没有 `FIONBIO` 分支，未知 ioctl 走 `ENOTTY`；
  既有 fd table 已有共享 file-status 的 `F_GETFL/F_SETFL` 与 pipe `O_NONBLOCK/EAGAIN` 语义，
  因此缺口是 ioctl 到真实状态的通用桥接，而不是 Cargo 专用路径。
- test-only `bf053edc5a9dcbe218dd05cefa0a3acb7a0bd4ed` 创建 pipe，先用 FIONBIO 非零值开启，
  要求 `F_GETFL` 可观察 `O_NONBLOCK` 且空 pipe read 为 `EAGAIN`，再用零值关闭并要求 flag
  消失；这排除了无条件返回 0。clean canonical RED 在 RV/LA 均计划并执行 1/1 后 FAIL，
  明确 `USER_FAIL pipe_fionbio`，无 timeout/crash/infra。RV outer/inner/console SHA-256 为
  `8a242a52d7bfaa646efd74ee496bbb6fdc97ba4ad154de4757ebd66bceaa4962`、
  `5e08f7f2df5eb2306120e1e895da0ea860efe0fefa7c065f972826295140ce65`、
  `404c5fb0137fc134eb1a8cbda111544c0c7ee6afd4fbe71d66eb9a150b4707c7`，证据在
  `test/output/20260721T084021Z-evidence-runtime-rv-3504712/`；LA 对应为
  `db78901a6b5d655e758132b7a15c122ba87bc9dd7733f3d76214f5cf9c3e84fd`、
  `eb151f8fb2acfee92dd1d7da199d428391e0e442e68bdf6264878fa4d95032f2`、
  `c7af1898c34e3a4c82acad8694081fcd8e98828d1ccb95a9e721d3571e3198fa`，证据在
  `test/output/20260721T084216Z-evidence-runtime-la-3506212/`。首次失败不会被 GREEN 覆盖。
- `cd6c871142e646ab78589d370363f271804e8a11` 读取用户 C `int`，在 fd-table 锁内复用
  `F_GETFL/F_SETFL`，非零设置、零清除 `O_NONBLOCK`，使支持状态 flag 的一般 fd 类型共享
  同一合同；没有新增依赖、`unsafe`、架构分支、程序名或路径判断。首次 GREEN RV/LA
  runtime 各 1/1 PASS，但随后 clean quick 如实得到 45/47 FAIL：只有 Linux-boundary
  check/unit 因 legacy `read_user_value` 调用库存 91 -> 92 失败，337.727674 s，summary
  SHA-256 `edbd70f7c43f4836c745d06ac94d5bc94ad516dea917bd7952b0d468a3cba081`，证据在
  `test/output/20260721T085227Z-quick-none-3512860/`，没有修改 guard 数字掩盖回归。
- `d5b606c718528de8ef44e1756313bc1832b5beda` 在 user-memory boundary 内抽出接收
  `UserPtr<T, Read>` 的 value-copy helper，旧整数地址入口只作兼容包装，FIONBIO 使用 typed
  pointer；既有 unsafe body 未复制也未新增。Linux boundary guard 0 finding、17/17 mutation、
  format/diff check 与 RV/LA semantic kernel build（45.30/42.35 s）均 PASS，warning 保持既有
  175。`427ea83574300cfab56d58787c5b9dabc7010b6e` 随后把 `user_memory.rs` 加入 RV/LA
  smoke build/runtime provenance；JSON、75/75 semantic unittest 和 asset guard 0 finding PASS。
- 最终 clean `427ea835` canonical quick 为 47/47 PASS，所有非 PASS bucket 为零，
  337.848743 s，summary SHA-256
  `0fa479fc8d61f7a132abfc8b972225adb64278492f394254aa7e86a21ba8a3c2`，证据在
  `test/output/20260721T091341Z-quick-none-3528649/`。同一提交 RV/LA runtime 各 1/1 PASS，
  99.648451/96.189316 s，均明确 `ASSERT pipe_fionbio PASS`、`USER_PASS`、`HARNESS_PASS`，
  无 USER_FAIL/panic/trap/ENOSYS/timeout。RV outer/inner/console SHA-256 为
  `1f01d91c091e31a379a4e1096119682713ce1b8235b69a351414fa2ec3bd7288`、
  `ece99ac6ec1aeee9c97cd52a9e196b2113f82c63e06efe392cf0ab014839678b`、
  `c791920c839a5f4d2aefb22dda99ca9e053016170e1e85e58e2afc0330745194`，证据在
  `test/output/20260721T091924Z-evidence-runtime-rv-3533571/`；LA 对应为
  `232a472693b678763bf99f7962c0f05d20a2bb92cd11a953e528551b88abd608`、
  `2f277caa3d1e96057c4d080714b40c5e4b08632b579e138248a9f47cd500ae70`、
  `22a6b96cd38d141f842944fb6cbb70f6626fe36be73de6bc943179ef700dc1c5`，证据在
  `test/output/20260721T092123Z-evidence-runtime-la-3535080/`。inner build/runtime provenance
  均明确包含 `user_memory.rs`。以上是通用 Linux fd 状态语义，不是正式 CAgent/BuildStorm
  执行或分数；final rootfs 与合格 8c/8G host 缺口不变，官方四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 30：per-thread comm / prctl name 隔离 RED -> GREEN

- 对同一最小 Cargo/rustc host trace 继续只读分类，确认 65 次
  `prctl(PR_SET_NAME, ...)`，用于 Rust worker thread 命名。源码审计发现 OrayS 把名称保存在
  `UserProcess.prctl_name`：任一线程设置名称都会覆盖整个线程组，`PR_GET_NAME` 与
  `/proc/<pid>/task/<tid>/comm` 也都读取该进程级值。这是 Linux 可见的通用线程隔离缺陷，
  不是 Cargo/BuildStorm 程序名或路径特例。
- test-only `2e7becaaf90164236358c45dc8f4c43d380a68c6` 先给 parent 设置
  `orays-parent`，再让现有 Cargo-shape clone3 child 通过真实 syscall 设置不同名称；child
  完成并 join 后，parent 的 `PR_GET_NAME` 必须仍返回精确 16-byte parent buffer。clean
  canonical RED 在 RV/LA 均计划并执行 1/1 后 FAIL，明确
  `USER_FAIL prctl_thread_name`，无 timeout/crash/infra。RV outer/inner/console SHA-256 为
  `b786a500727de6a14dcf6f78a9da4f73ae44c69b82f6a42ec2777479290f47fb`、
  `e55ebd2c152e2e72fef59209d43f2699144352c2ddb03d0d28e7d0c22dbf89e3`、
  `99295e485b660943be12557e78369594c24d45a30776a250a8231e386caf0f7b`，证据在
  `test/output/20260721T093343Z-evidence-runtime-rv-3540559/`；LA 对应为
  `b509b442fe94c6acb3b2ad644e87d8458e71aa6dfa1c1a88d9d644541077118e`、
  `9da706ee397187ebaf39c892bad846d7bb5ae52485411c34a09506260f0a8706`、
  `8b3dddd9d46ae333167afc6ed250614275c0d2c5cd97418af428e93e386b1168`，证据在
  `test/output/20260721T093558Z-evidence-runtime-la-3542162/`。LA harness 在 USER_FAIL 后仍
  打印 `HARNESS_PASS status=0`，但 fatal-pattern parser 正确保持 outer 1/1 FAIL，未吞失败。
- `eedc4df3ea0ceb8c317b46cec0854608abc39658` 在 `UserTaskExt` 增加由 mutex 保护的
  per-thread `comm`；初始任务使用既有兼容名称，fork-like 子进程与 CLONE_THREAD 子线程均
  继承调用线程名称，`PR_SET_NAME/PR_GET_NAME` 只操作当前 task。leader 设置时同步
  `UserProcess` 回退值；procfs 的 process `comm` 读取 leader，task `comm` 读取指定 TID。
  没有新增 syscall、依赖、`unsafe`、ABI/架构分支或程序名/路径判断。状态语义、Linux
  boundary、asset guard 均 0 finding，semantic unit 75/75 PASS；RV/LA smoke kernel build
  44.25/42.19 s、退出 0，warning 保持既有 175。一次不存在的 guard 路径命令退出 2，随后
  从 manifest 解析正确 `check_linux_boundary.py` 并 PASS；一次 unsupervised raw QEMU 与一次
  dirty-tree supervised canonical 分别被 Makefile/runner fail closed 拒绝，均未计为验证。
- `17d30a67ae76dd3d4a4286f24bc1d8387fd775b5` 把 `system_info.rs` 与
  `task_context.rs` 加入 RV/LA smoke build/runtime 四组 provenance；JSON、competition/asset
  guard 与 semantic 75/75 unit 均 PASS。最终 clean canonical RV/LA runtime 各 1/1 PASS，
  99.440612/96.494603 s，inner 各 3/3 PASS，明确出现 `ASSERT prctl_thread_name PASS`、
  `USER_PASS`、`HARNESS_PASS`，无 USER_FAIL/panic/trap/ENOSYS/timeout，且 build/runtime
  provenance 均包含两个新增文件。RV outer/inner/console SHA-256 为
  `9d115d25c1909da3c49ccddbf56d84e3f2ede4c2771f96603c1f5d3717daa342`、
  `402130952055d7aefd9288b01eca89f51eefc25e8069f74ca68c2c84e748505e`、
  `54ce19ceb6bff6bd209f7278e56b4cce58ce673e1e3be8257f7d9966d5ee01cf`，证据在
  `test/output/20260721T094947Z-evidence-runtime-rv-3550866/`；LA 对应为
  `96193a37b56b175ceeefd9caffeb0b2e3974885454608efc07d25f9096e8d54b`、
  `336de8130a133db59cb253923d6f025ce18e72aa08e3c97e4941d53bfc27e392`、
  `c05ff40b528e5abd8a9b9f82a76879e0feca4bf3add347bbe622cfd6fc2100d4`，证据在
  `test/output/20260721T095140Z-evidence-runtime-la-3552399/`。
- 同一 clean commit 的 canonical quick 为 47/47 PASS，所有非 PASS bucket 为零，
  337.127419 s，summary SHA-256
  `c79da98aa796ee4a154f173dc04edabb722b159ce4133371b4e5f055fe82d250`，证据在
  `test/output/20260721T095323Z-quick-none-3553730/`。以上只证明通用 Linux thread-comm
  语义与仓内双架构门禁，不是正式 CAgent/BuildStorm 执行或分数；final rootfs 与合格
  8c/8G host 缺口不变，官方四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 31：sigaltstack fork/vfork/exec lifecycle RED -> GREEN

- 对同一最小 Cargo/rustc host trace 继续只读分类，确认 227 次 `sigaltstack`，包含安装、查询与
  禁用。源码审计发现 `UserTaskExt::new` 总把备用信号栈初始化为 `SS_DISABLE`，因此普通 fork
  与 `CLONE_VM|CLONE_VFORK` 错误丢失调用线程设置；成功 `execve` 又未清除旧 task 状态。主机
  `man-pages 5.10-1ubuntu1` 的 `sigaltstack(2)` 明确规定成功 exec 移除备用栈、fork 复制设置，
  且 clone 只在带 `CLONE_VM` 而不带 `CLONE_VFORK` 时清除。这是通用 Linux task lifecycle
  缺陷，不是 Cargo/BuildStorm 名称、路径或输入特例。
- test-only `5d1e2581106954ba32a50daec42ffe7e4b3a24ad` 安装真实 64 KiB `stack_t`，让普通
  clone3 child 通过 syscall 132 查询自身状态，让既有 `CLONE_VM|CLONE_VFORK` child 在 exec
  前以双架构原始汇编核对继承值，并让独立 exec ELF 查询 `SS_DISABLE`、空地址和零大小。
  `f28dc367587ac85c0aebe058ae0cb52be0a66505` 将 child 专用退出码按 Linux `wait4` 的
  `code << 8` 编码分流到 `USER_FAIL sigaltstack_fork_exec`。最终 clean canonical RED 在 RV/LA
  均计划并执行 1/1 后 FAIL，无 timeout/crash/infra：RV outer/inner/console SHA-256 为
  `61d72560df4fa73321b2540a7a90bfbeb6e50346ff88f5d321c27d204c308717`、
  `bb616ec46cddf60cc148ab9f6c10bd19c6947df912dc20bcc4f77972087f9def`、
  `727b9d67c611b99fac560ebaf08ad392256631d9e7924085b0b6fc0532b28911`，证据在
  `test/output/20260721T101825Z-evidence-runtime-rv-3568697/`；LA 对应为
  `dfc0c474abfe35a17262f75e23d214f5ca5a9eb241f50842d5ec74815f8ad677`、
  `c70daf5685efd1efda77f5cfe1d7b3684e8697f0fb7ec1d07fc5dd2dc6a81671`、
  `9f0256b319e4343a64ded8db6818299d3e7b323ea7b07913840d47f032210dce`，证据在
  `test/output/20260721T102028Z-evidence-runtime-la-3570238/`，耗时 99.811806/96.981920 s。
- 测试标签经历两次诚实诊断：首次 `5d1e2581` 的 child failure 被既有
  `USER_FAIL clone3_process` 宽标签捕获；随后第一次状态分流错误比较裸 `45`，而 OrayS 正确
  输出 POSIX wait status `45 << 8`，故 RV/LA 仍显示宽标签。两轮均保留为 FAIL，不计作专用
  RED 或 PASS；修正后的 `f28dc367` 才取得上述专用双架构 RED。另有一次错误 unittest
  discover 路径得到 `Ran 0 tests`，随即用正确 `test/unit` 目录重跑 75/75 PASS，零测试不计验证。
- `915cafd11cc12abc00229ef5b8da22988aeaf7ae` 在新 fork-like `UserTaskExt` 发布前复制调用
  task 的 `(sp, flags, size)`；普通 `CLONE_VM` thread 继续使用构造器的禁用状态；成功 exec
  换像后清零备用栈、旧 signal frame、pending sigreturn 与 syscall restart frame。无新增
  syscall、依赖、`unsafe`、ABI、架构分支或测试/程序名/路径判断。kernel-state、Linux
  boundary、syscall-boundary、competition 与 asset 五项 guard 均 0 finding，semantic unit
  75/75 PASS；RV/LA smoke build 退出 0，warning 保持既有 175。
- 最终 clean canonical RV/LA runtime 各 1/1 PASS，100.833471/97.283654 s，inner 各 3/3
  PASS，明确出现 `ASSERT sigaltstack_fork_exec PASS`、`USER_PASS`、`HARNESS_PASS`，无
  USER_FAIL/panic/trap/ENOSYS/timeout；smoke build/runtime provenance 均包含
  `process_lifecycle.rs` 与 `task_context.rs`。RV outer/inner/console SHA-256 为
  `9b3de296479f4f96e98c0217c67444d871c274d30cfb2a029c065a91e283560a`、
  `48b8734690807b7160f1e6815dc7084ced73b88c4fa68c41d53e87f34e85d333`、
  `db996d28cabf44d9ce8f8e5b2e841a86b6f6b2427d8a5cae164af06bbfeac135`，证据在
  `test/output/20260721T102644Z-evidence-runtime-rv-3574152/`；LA 对应为
  `cc53248c5979d577cb9d43711a76cd34b869d16fa91836e33888306be2ca320b`、
  `467e477696679706f79ceec31b3f6af2aab0577c20aaf3b718a2ea20cdbd6f01`、
  `aabf192becd17fd78c97430b68fc8ed42a4f5a7d42480201ff1bc506514ccd61`，证据在
  `test/output/20260721T102829Z-evidence-runtime-la-3575536/`。同 commit canonical quick 为
  47/47 PASS、零非 PASS，337.937485 s，summary SHA-256
  `2d8e1ce826d84a64aa38c2a34e074299fdff666380f7c4bb3c2ae6845e3532e9`，证据在
  `test/output/20260721T103033Z-quick-none-3577022/`。以上不是正式 CAgent/BuildStorm 执行或
  分数；final rootfs 与合格 8c/8G host 缺口不变，官方四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 32：sigaltstack SA_ONSTACK 实际投递 RED -> GREEN

- 对同一最小 Cargo/rustc host trace 的 227 次 `sigaltstack` 继续做源码侧审计，发现
  `inject_pending_signal` 虽保存 `SA_ONSTACK`，RV/LA 却都始终从被中断的普通 `tf.sp` 分配
  signal frame；`current_sigaltstack` 又把“存在任意 signal frame”误报为 `SS_ONSTACK`。
  主机 `man-pages 5.10-1ubuntu1` 的 `sigaltstack(2)` 明确要求带 `SA_ONSTACK` 的 handler 在
  已登记且启用的备用栈上执行，并规定处于备用栈时查询 `SS_ONSTACK`、修改返回 `EPERM`。
  这是一般 Linux signal ABI 与内存安全边界缺陷，不是 Cargo/BuildStorm 名称、路径或输入特例。
- test-only `5ab91a011b1ee13519044cb74ea659014b34d5a7` 使用原始 syscall 129 `kill`、
  134 `rt_sigaction`、真实 24-byte kernel sigaction 和普通 `extern "C"` handler。测试复用既有
  64 KiB 备用栈，要求 handler 的局部变量地址真实落入该范围，且 handler 内
  `sigaltstack(NULL, &old)` 同时报告 `SS_ONSTACK`；atomic 状态必须精确为 7。
  clean canonical RED 在 RV/LA 均计划并执行 1/1 后 FAIL，零 timeout/crash/infra，guest
  明确 `USER_FAIL sigaltstack_onstack` 与 `HARNESS_FAIL guest_nonzero_exit`。RV outer/console
  SHA-256 为 `38998cfe02caa12984ea94bf82ce7386a7e03a5c279afec447cb239fb5e52142`/
  `a55e4e492713bea462170e1eac7b925f50a60a2bfc33bbd0c6ffad1c36aeb28a`，耗时
  102.129392 s，证据在 `test/output/20260721T104554Z-evidence-runtime-rv-3585658/`；LA
  对应为 `c003ccb1b308555797d63210e2cd214dea9a27d0a53884183a30e37618473210`/
  `116280cf27ff12691494688b908f95aa5a7f53bbbdf0226fe8e2f060df4c5445`，耗时
  97.547648 s，证据在 `test/output/20260721T104812Z-evidence-runtime-la-3588467/`。
- focused semantic-evidence 回归第一次得到 205 PASS、3 FAIL：新增 marker 已进入 manifest，
  synthetic complete-log fixture 还未同步，三项均保留为 FAIL，不以旧 fixture 冒充通过。
  test-only `066c4ade` 只向 pass fixture 加入该实际必需 marker，随后同一相关集合 208/208
  PASS。另一次误用已不存在的旧静态脚本名在执行检查前退出 2，一次 unittest 会话丢失最终
  exit status，二者都没有计作验证结果。
- `3658457072028d2d508418f0ee8f5a04874fdf8e` 在 `signal_abi.rs` 增加两架构共享的 checked
  alternate-stack range/frame-layout helper：只有 action 请求 `SA_ONSTACK`、备用栈启用且
  当前 frame 尚不在该范围时，才从备用栈顶按 ABI 对齐分配；所有范围、转换与减法均检查
  overflow/underflow。RV/LA signal ucontext 现在写入实际 interrupted `uc_stack`，查询状态按
  当前 signal-frame 地址派生 `SS_ONSTACK`，修改备用栈仅在 handler 真实位于该栈时返回
  `EPERM`。无新增 syscall、依赖、`unsafe`、ABI、架构分流或测试/程序名/路径判断。
- 实施中首轮普通 RV build 暴露两处 `ss_size` 的 `u64`/`usize` 类型错误并退出非零；改为
  fallible conversion 后，RV/LA 普通 build 均退出 0，各保持既有 11 条 warning。kernel-state、
  Linux boundary、syscall-boundary、competition、asset、compliance 六项 guard 均 0 finding，
  相关单元回归 208/208 PASS，`git diff --check` PASS。
- 最终 clean canonical RV/LA runtime 各 1/1 PASS，100.381632/97.712269 s，inner 各 3/3
  PASS，明确出现 `ASSERT sigaltstack_onstack PASS`、`ASSERT sigaltstack_fork_exec PASS`、
  `USER_PASS`、`HARNESS_PASS`，无 USER_FAIL/panic/trap/ENOSYS/timeout。RV outer/inner/console
  SHA-256 为 `7c7c35ad325b1a10317b147fb13c27e38599749dd92179d5cadf4644fd1bf9bd`、
  `8b702960a6eecaa564bdddb9209399a08ac498b2ebf894af5bc2334779d23bec`、
  `17494c54d9c4dbb76308a64876da79002b73b16fee08c5af670680eb93471222`，证据在
  `test/output/20260721T110510Z-evidence-runtime-rv-3600053/`；LA 对应为
  `22f9d39a70e41981942048e74942e27861d5fb2bc48db8da0cb000d066fe5a7f`、
  `be0c311a97852249eb3cb67b6b5048f529ca1febec1116e935a168e6efe22c37`、
  `c2dbbb5258d259ea657e7974d7b7f0649435bf74345568ca94d655a131d10fb2`，证据在
  `test/output/20260721T110658Z-evidence-runtime-la-3601449/`。同 commit canonical quick 为
  47/47 PASS、零非 PASS，337.675069 s，summary SHA-256
  `75f0aa300bf5fcb30ce51533b965238a94fbdd6dc76f6fdc2772126cd3b927dd`，证据在
  `test/output/20260721T110859Z-quick-none-3602841/`。
- 本 checkpoint 只证明通用 Linux `SA_ONSTACK` 投递与仓内双架构门禁；`SS_AUTODISARM` 的
  handler 期间临时禁用、context restore 与嵌套 signal 仍需单独审计。final rootfs 与合格
  8c/8G host 缺口不变，正式 CAgent/BuildStorm 四格继续 `BLOCKED`/未执行，不能据此宣称分数。

## 2026-07-21 — Checkpoint 33：private 文件映射 demand paging RED -> GREEN

- 继续审计 `/tmp/orays-buildstorm-syscall-audit.MBQSBz/` 中真实 host Cargo/rustc trace。
  动态 loader 对 `librustc_driver-3b5a0434428b5a33.so` 先保留 125,760,708 bytes 地址空间，
  随后以 `MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE` 映射 123,663,556-byte read-only 段和多个
  execute/write 段；这些调用没有 `MAP_POPULATE` 或锁定。源码审计确认旧 `sys_mmap` 对所有
  file-backed mapping 先 `map_alloc(..., populate=true)`，再从 fd 同步循环读取完整 `len`。
  这会把 loader 应按需处理的百 MiB 文件 I/O 与物理页分配乘以并发 compiler 数，是一般
  demand-paging 缺口，不是对 BuildStorm 路径、crate 或文件名的特例。
- test-only `0f37d27e3222d386c1a74bbd1a183fd6cc4a298e` 建立 16 MiB 稀疏普通文件，在首、末页
  各写一个 sentinel 后以普通 `MAP_PRIVATE` 映射。合同要求首次 `mincore` 精确为 0 resident，
  访问首尾后精确为 2 resident，private 写不能改变 fd backing。clean canonical RED 在 RV/LA
  都实际计划/执行 1/1 后 FAIL，零 timeout/crash/infra，guest 明确
  `USER_FAIL mmap_private_lazy` 与 `HARNESS_FAIL guest_nonzero_exit`。RV outer/inner/console
  SHA-256 为 `b4c6744500d51513b17fadf1782e372454fb819fff41ef3a9055b3633533a2b2`、
  `74dec81f992752f4a7eb381824c7b06f9e5adb0caa325053ff455e8d46ffb6fb`、
  `6b06ce1fad672aecfce01b1e1dc997f6942cde5fe7855efa2568fee373c0dfb6`，耗时
  99.069460 s，证据在 `test/output/20260721T113413Z-evidence-runtime-rv-3614636/`；LA 对应为
  `face1bd715099d5b9f63f65c7404e8fb9263dbb505a6be5994906bac9f58bcfe`、
  `57371632cc3d750a94da4903e95052f5a8054f5f545a4c55414415d92ee47bed`、
  `aca364ffbd76bf2c94adc7ef2d4c0ead4111ddc60c9a56d06436e3587cae7283`，耗时
  93.553982 s，证据在 `test/output/20260721T113622Z-evidence-runtime-la-3616288/`。
- 提交前语义审计发现 lazy PTE 不能让合法 VMA 上的 `msync` 错误返回 `ENOMEM`，且
  `MADV_DONTNEED` 必须丢弃 private 修改后重新读取 backing。test-only
  `35a65b75e8343a9a80aa2db2f6d7b4eac0916c80` 在同一 marker 内增加 `MS_SYNC` no-prefetch 与
  discard/mincore/reload 合同；RV/LA 均先成功编译。实施中一次直接对独立 smoke 文件运行
  `rustfmt` 造成无关格式变化，已用 `apply_patch` 完整恢复并只保留目标插入；最终以
  `cargo fmt --all` 检查，未把无关格式 diff 带入提交。
- `c2258c56a9ad49182a43f8cbe444f3f8ab334f34` 为未锁定、未请求有效 `MAP_POPULATE` 的普通
  private 文件 VMA 保存 backing/offset/valid length，初始不分配 PTE；首访时在地址空间锁外
  每次只读取一个 4 KiB 页，取得锁后重验 VMA、offset、长度与 backing identity 才安装数据。
  shared、locked 与显式 populate 映射保留 eager 行为；`MAP_NONBLOCK|MAP_POPULATE` 不强制
  预取。权限拒绝仍走 `SIGSEGV`，越过 backing 有效末尾或真实 I/O 失败才走 `SIGBUS`。
  user copy、`MADV_POPULATE_*`、`mlock` 统一走同一 fault-in；`msync` 按 VMA 而不是 PTE 验证；
  lazy private `MADV_DONTNEED` 撤销 resident frame，下一访问从 backing 重载；`mremap` 移动时
  只复制已驻留 private 页，避免把 absent 文件页变成零页。无新增 `unsafe`、依赖、syscall、
  ABI、架构分流或测试/程序名/路径判断。
- RV/LA 普通 build 最终均退出 0，44.48/42.33 s，保持既有 175 条 warning；增强 smoke 的
  非 canonical RV/LA 诊断均完整 `USER_PASS/HARNESS_PASS`。competition、memory-policy、
  Linux-boundary、syscall-boundary、kernel-state、compliance、no-fake-success、asset 八项 guard
  均 0 finding；semantic unit 75/75，另外五组相关 unit 123/123，`cargo fmt --all -- --check`
  与 `git diff --check` 均 PASS。生产 diff 仅四个通用 uspace 文件。
- 最终 clean canonical RV/LA runtime 各 1/1 PASS，102.108468/99.612675 s，inner 各 3/3
  PASS，明确出现 `ASSERT mmap_private_lazy PASS`、`USER_PASS`、`HARNESS_PASS`，无
  USER_FAIL/HARNESS_FAIL/panic/trap/ENOSYS/TIMEOUT/TFAIL/TBROK/TCONF。RV outer/inner/console
  SHA-256 为 `9c400619f038c7fb62bd7760ded610fa1fc8c495674d2e679430adcc1b3d3960`、
  `8456dae9f3450cc5b82c16d8cfaa0fe8fffba77e4e391e102f3288d357792a7e`、
  `a974cb2d2f20ea212d5f3c760cad5912c40fb06e73d5a925f76f3ca79d82b177`，证据在
  `test/output/20260721T115903Z-evidence-runtime-rv-3626803/`；LA 对应为
  `83cf7881acceac89e6d2581255776df66c6325aaf2cc282828f43b18e1a3485c`、
  `6e57479b2bcc851fcd60bc36fc331d08dfbea6d8fc4466ad5b58c0d62d7bdc83`、
  `430fbe02ffe49e2e19f27886442dbca793eb3eeb29c27883b0185102c9748583`，证据在
  `test/output/20260721T120123Z-evidence-runtime-la-3628459/`。同 commit canonical quick 为
  47/47 PASS、零非 PASS，338.509931 s，summary SHA-256
  `7e4de07070bbfc8c8a45093a73ae2e0973fa41e91e5d3a0f3e843262b5866031`，证据在
  `test/output/20260721T120324Z-quick-none-3629870/`；三组 provenance 均 clean/stable。
- 本 checkpoint 证明通用 private 文件 demand paging 与仓内双架构门禁，不是正式
  CAgent/BuildStorm 执行或 score delta。当前 backing 长度仍在建图时快照；映射后的
  truncate/grow、扩展 `mremap` 与更多 invalidate 组合列为后续审计项。final rootfs 与合格
  8c/8G host 缺口不变，正式四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 34：private futex 跨 COW 的 mm 身份 RED -> GREEN

- 继续审计 `/tmp/orays-buildstorm-syscall-audit.MBQSBz/` 的真实 host Cargo/rustc trace：
  `futex` 是最高频同步 syscall，glibc/Rust worker 大量使用 `FUTEX_PRIVATE_FLAG`。源码审计
  发现旧 table 对 private/shared 一律以当前物理地址键控，完全忽略 private flag。fork 后
  private page 处于 COW：waiter 读值并以旧 frame 排队，sibling 首次 store 解析到新 frame，
  随后的 `FUTEX_WAKE_PRIVATE` 因 key 不同返回 0。这是地址空间语义缺口，不依赖 Cargo 名称、
  crate、路径、输出或执行顺序。
- test-only `53656508e8746d4fab2ff793122cf523b7b60f82` 在 fork 前分配/初始化 private anonymous
  futex word，child 再以 Cargo/glibc exact clone3 thread flags 创建 waiter。双 pipe 只用于
  确认线程已到达 bounded wait 前沿；sibling 在 waiter 排队后 store 触发 COW，再要求 private
  wake 返回 1、wait 返回 0、clear-child-tid join 清零且所有 fd/映射回收。测试使用 250 ms
  有界 timeout，两个架构均实际编译；第一次 RV 编译因测试把 `b'G'` 误传给 slice 参数而
  E0308/exit 2，随后仅修正测试类型，未修改生产或期望。
- 修复前 dirty-tree supervised 诊断在 RV/LA 均依次通过 `clone3_futex_join`，随后明确输出
  `USER_FAIL private_futex_cow`、`HARNESS_FAIL guest_nonzero_exit` 并正常关机；无 timeout、
  panic 或 trap。RV/LA log SHA-256 为
  `fd47a3a06d592bd5c4d6234df626ba56ef9b555c3d335c67c9bb129612bec39c`、
  `0a5601b9821c4cfef348180cf3531035c43d7e0b808dc414b3ab335df632add9`。这些只作为定向 RED
  诊断，不冒充 canonical clean-tree 结果。
- `0adfc1758b0825cb7e2432855713b69a96df75a8` 引入 typed `FutexKey`：private variant 使用
  `Arc<AddrSpace>` 的共享对象身份加 `uaddr`，所以 CLONE_VM 线程共享、fork child 隔离且
  private COW remap 不改变 key；shared variant 保留 backing physical address，使不同进程的
  MAP_SHARED futex 仍能 rendezvous。wait/wake/bitset/requeue 均从 syscall flag 传播同一类型。
  signal 与 exit-group 中断在两类 state 中按精确 `AxTaskRef` 唤醒，不提前清除错误队列；
  kernel-generated clear-child-tid/robust wake 保持 shared identity，与现有非-private glibc join
  匹配。无新增 production `unsafe`、依赖、syscall、ABI、架构或程序名/路径分支。
- dirty-tree GREEN 在 RV/LA 均完整出现 `ASSERT private_futex_cow PASS`、后续 vfork/flock/
  rename/socket/TCP marker、`USER_PASS` 与 `HARNESS_PASS`。GREEN log SHA-256 为
  `38e7ca1de74a24762fe2e25f276b3e7279ca4dc19bb75c27bb5c3bbb1b4f841a`、
  `7276685620c39af1ff61813946006f50eeb919411ded06f6e4e0955791de8e1f`。三项定向静态 guard
  均 0 finding，相关 mutation/unit 为 41/41、10/10、26/26，semantic evidence 75/75，
  RV/LA smoke kernel 均编译 exit 0。
- clean `0adfc175` canonical RV/LA evidence-runtime 各 1/1 PASS，零 FAIL/TIMEOUT/CRASH/
  INFRA，101.900932/99.235038 s，runner commit 前后一致且 dirty=false。RV summary/inner/
  console SHA-256 为 `2b50ff7f...6f7f`/`65e692cf...d89c`/`c40a3f12...379`，证据在
  `test/output/20260721T123657Z-evidence-runtime-rv-3645502/`；LA 对应为
  `bee067c7...638`/`c5ba5ec6...14e0`/`f20e0464...053`，证据在
  `test/output/20260721T123857Z-evidence-runtime-la-3647029/`。同一 clean commit quick
  47/47 PASS，零非 PASS，339.926120 s，summary SHA-256 `65fb21f7...45b1`，证据在
  `test/output/20260721T124043Z-quick-none-3648442/`。
- 本 checkpoint 证明 private/shared futex key 的通用 Linux 可见身份和仓内双架构回归，
  不证明正式 Rust toolchain、minibuild 或 full BuildStorm 已运行/得分。比较值检查到真正
  排队之间的原子性、requeue 与 signal/timeout 竞争仍列为后续审计；final rootfs 与合格
  8c/8G host 缺口不变，正式四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 35：`FUTEX_WAIT_BITSET` 的 `SA_RESTART` RED -> GREEN

- 真实 host Cargo/rustc strace 显示 `FUTEX_WAIT_BITSET_PRIVATE` 被 signal 中断后进入
  `restart_syscall`。Linux `signal(7)` 也把 `FUTEX_WAIT`/`FUTEX_WAIT_BITSET` 列为
  `SA_RESTART` 可重启接口，`FUTEX_WAIT_BITSET(2const)` 明确其 timeout 为 absolute，
  `restart_syscall(2)` 则说明 relative futex wait 必须扣除 signal handler 已消耗的时间。
  OrayS 旧 `restartable_blocking_syscall` 只识别 `wait4`/`waitid`，因此真实 futex wait 会在
  handler 返回后以 `EINTR` 结束。这是通用 syscall restart 语义缺口，不依赖 Cargo 名称、
  路径、输出或执行顺序。
- test-only `607b0379a96e200928bdc66d54bcfa4fb7b8409e` 在对齐的共享匿名页上创建 futex word，
  child 安装真实 `SIGUSR1|SA_RESTART` handler 后，以 2 秒 absolute `CLOCK_MONOTONIC`
  deadline 发起 `FUTEX_WAIT_BITSET`；parent 等待就绪、发送真实 signal、观察 handler 状态、
  确认 child 重新进入 wait，再 store 并执行真实 shared `FUTEX_WAKE`。PASS 同时要求 wake=1、
  wait=0、handler 已运行、共享 publication 可见、child 正常回收及 fd/mapping 全部清理。
  修复前 RV/LA dirty-tree supervised 诊断均先通过此前 marker，随后明确输出
  `USER_FAIL futex_sa_restart`、`HARNESS_FAIL guest_nonzero_exit` 并正常关机，无 timeout、
  panic 或 trap；RED log SHA-256 分别为
  `293e9115ca311f2ff07e09e2a04425806b27383a2186f1e5c1b0d9075d118d8f`、
  `0b21118f4a8dbaa2ebd294ee9d9fd8074ea113241a470556acfae53ef45278ad`。QEMU/make shell 因
  guest poweroff 返回 0，但 guest/harness marker 正确判为 FAIL，未冒充 canonical PASS。
- `991f5856dcd73783e7d3d6eac0d7774ada3e2d7b` 只在 masked futex command 为
  `FUTEX_WAIT_BITSET` 时允许现有 signal-frame replay。BITSET timeout 是 absolute，因此
  重放原用户参数仍指向同一 deadline；空 timeout 也不会产生时间延长。普通 `FUTEX_WAIT`
  的 timeout 是 relative，直接重放会把 signal 前已等待时间重新加一遍，所以本 checkpoint
  明确保持其 `EINTR`，待实现保存 deadline/remaining-time、覆盖重入并处理取消/exec/exit 的
  restart block。无新增 production `unsafe`、依赖、syscall、ABI、架构或程序名/路径特化。
- dirty-tree GREEN 在 RV/LA 均出现 `ASSERT futex_sa_restart PASS`、后续 marker、
  `USER_PASS` 与 `HARNESS_PASS`；log SHA-256 为
  `4f46f2dabd00e3514d46fe4439e3a4d2bcfbc6b3e165003715345ef5a065a963`、
  `11e475d25f596b0936aefeba55b4f892cd28bd8d5cfc64839168dd829e8d30f5`。三项定向 guard
  均 0 finding，相关 mutation/unit 为 41/41、10/10、26/26，semantic evidence 75/75，
  RV/LA smoke kernel 均编译 exit 0。
- clean `991f5856` canonical RV evidence-runtime 为 1/1 PASS、零非 PASS，102.600339 s；
  summary/inner/console SHA-256 为 `e4ea411b...f59`/`fdf49006...34b`/`75edaba6...5b3`，
  证据在 `test/output/20260721T130428Z-evidence-runtime-rv-3660723/`。LA 同样 1/1 PASS、
  零非 PASS，99.351574 s；对应 SHA-256 为
  `7adad038...83e`/`ceac5329...a51`/`660ead25...2f1`，证据在
  `test/output/20260721T130625Z-evidence-runtime-la-3662194/`。两者 runner commit 前后一致、
  dirty=false，inner required 3/3 PASS。同 commit canonical quick 47/47 PASS、零非 PASS，
  340.749107 s，summary SHA-256 `fb99ebde5b24afdde44b005669a0e6240b59e759a4a7d0241f177d523a0c4709`，
  证据在 `test/output/20260721T130836Z-quick-none-3663611/`。
- 本 checkpoint 证明 absolute BITSET wait 的通用 restart 语义和仓内双架构回归，不证明
  relative futex timeout、正式 Rust toolchain、minibuild 或 full BuildStorm 已运行/得分。
  final rootfs 与合格 8c/8G host 缺口不变，正式四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 36：relative `FUTEX_WAIT` remaining-time RED -> GREEN

- Linux `signal(7)` 将 `FUTEX_WAIT` 列为 `SA_RESTART` 可重启接口，`restart_syscall(2)` 说明
  relative futex timeout 必须扣除 signal handler 已消耗的时间。test-only
  `4eef0eb84f759b28a230b859c55843601d263b9d` 因此没有只检查“最终重启”：child 在共享匿名
  futex 上以 1.5 秒 relative timeout 阻塞，parent 约 0.75 秒发送真实
  `SIGUSR1|SA_RESTART` 且绝不 wake；PASS 要求 handler 确实运行、syscall 最终返回
  `ETIMEDOUT`，并且总耗时落在 1.25--2.0 秒，既拒绝旧实现约 0.75 秒的 `EINTR`，也拒绝
  重新等待完整 1.5 秒而延长到约 2.25 秒。测试回收 child、关闭 fd 并解除 mapping，没有
  根据程序名、路径、输出或执行顺序改变 production 行为。
- 首次 supervised 命令遗漏 `PR3_QEMU_RV_BIN`，runner 在启动 QEMU 前 fail closed、exit 2；
  这是命令配置错误、探针未执行，不计为 guest RED 或 PASS。补齐显式
  `/usr/local/bin/qemu-system-*` 后，修复前 RV/LA 均先通过既有 `futex_sa_restart`，随后
  明确 `USER_FAIL futex_relative_restart`、`HARNESS_FAIL guest_nonzero_exit` 并正常关机，
  无 timeout、panic 或 trap。RED log SHA-256 分别为
  `1275351c93e19d27742df1b221eef1399b2af762b549e582bc1ad9d1e556355a`、
  `504662ff25bad158780ba155376092ea3a4c67b1ed98db5643889274cef5058e`；QEMU/make shell 的
  poweroff exit 0 不覆盖 guest/harness FAIL 分类。
- `fb88957293e463603f490084d109d1910523db7f` 增加 per-task relative futex restart 状态：以
  `{uaddr, futex_op, val, timeout pointer}` 精确绑定原 syscall，以内核单调时间保存 deadline，
  并且只在匹配 wait 实际返回 `EINTR` 后 armed。replay 使用 `deadline - now`，不改写用户
  timespec，也不重新使用完整相对时长；untimed `FUTEX_WAIT` 可直接重启，BITSET absolute
  路径保持不变。handler 内 syscall 不能消费或替换外层状态，`rt_sigreturn` 保留状态直至
  replay；普通完成/错误、非 `SA_RESTART`/default/ignored action、成功 exec 与 task drop
  都清理状态。无新增 production `unsafe`、依赖、syscall、ABI、架构分支或程序名/路径特化。
- dirty-tree GREEN 的 RV/LA log SHA-256 分别为
  `001f94725cd50b3e1844c7c508d0635dc20abe0d9bb4126bc8746032941fe91a`、
  `63a0a2bbcee8a0cd896cd35e1d64192468cd5d765166e5d409cfd7f92cf8c424`，均明确出现新 marker、
  后续 marker、`USER_PASS` 与 `HARNESS_PASS`。三项 guard 均 0 finding；相关 unit 为
  41/41、10/10、26/26，semantic evidence 75/75；RV/LA smoke build 均 exit 0，warning
  保持基线 175。
- clean `fb889572` canonical RV evidence-runtime 为 1/1 PASS、零非 PASS，103.863282 s；
  summary/inner/console SHA-256 为 `3b00ff4a...5d94`/`29e48742...2021`/`7bd5954d...bfc0`，
  证据在 `test/output/20260721T133138Z-evidence-runtime-rv-3675779/`。LA 同样 1/1 PASS、
  零非 PASS，101.100630 s；对应 SHA-256 为
  `6c7b38c4...6277`/`552b8614...941c`/`2386cf6c...a1f`，证据在
  `test/output/20260721T133330Z-evidence-runtime-la-3677222/`。两者 runner commit 前后均为
  `fb889572`、dirty=false，inner required 3/3 PASS。同 commit canonical quick 47/47 PASS、
  零非 PASS，339.867055 s，summary SHA-256
  `4f11ab17b420b968f45c038f7f847831861d0768fdb49b73122791d9e266088a`，证据在
  `test/output/20260721T133529Z-quick-none-3678600/`。
- 本 checkpoint 只证明通用 relative futex restart deadline 与仓内双架构回归；没有运行或
  推导正式 Rust toolchain、minibuild 或 full BuildStorm 分数。futex compare/enqueue、
  requeue 和更复杂 signal/timeout 竞争仍需继续审计；final rootfs 与合格 8c/8G host 缺口
  不变，正式四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 37：普通 `FUTEX_REQUEUE` 返回值 RED -> GREEN

- Linux 当前 `b95f03f04d475aa6719d15a636ddf32222d55657` 的
  `kernel/futex/requeue.c` 把 wake 和 requeue 都计入 `task_count`，并返回该总数；内核 locking
  文档也表述为 requeued 或 woken 的任务数。一个独立 host Linux 原始 syscall 探针用
  `wake_count=0/requeue_count=1` 得到 requeue 返回 1、target wake 返回 1、child 正常退出。
  `FUTEX_REQUEUE(2const)` man-pages 页面却写成只返回 woken 数；这里明确记录文档冲突，并以
  当前 Linux 源码和实际 ABI 行为作为兼容依据，没有把互相矛盾的来源拼成虚假共识。
- test-only `a720bc6e6d69fa359ff134427cd0be0e726ee622` 增加通用
  `futex_requeue_count` 合同：共享匿名页上的 child 先发布 ready，再对 source 做有界 wait；
  parent 调用普通 requeue 将一个 waiter 移到 target，要求 syscall 返回 1，再由 target wake
  返回 1 并回收 child、fd 与 mapping。失败清理仍会 wake target，因此不会把超时当作预期
  RED。修复前 RV/LA 都先通过既有 futex restart marker，随后明确
  `USER_FAIL futex_requeue_count`、`HARNESS_FAIL guest_nonzero_exit` 并正常关机，无 panic、
  trap 或 timeout；RED log SHA-256 分别为
  `2008d1814d6c37a2fdb70a7441dafc93060fc8da0f02f7aa4e2134d46b9c4829`、
  `307b9e3d7775f878adcf815e4a7516198185eb13302b241c9f27a19ebfc085f6`。
- `f6b3a14dcc72c807830ece071daec206d4388e6d` 将普通 requeue 返回值从 `woken`
  改为 `woken.saturating_add(requeued)`；没有改变队列选择、wake/requeue 数量或 errno。
  首次 focused 检查暴露仓内旧静态 guard 正在错误要求“只返回 woken”，因此 boundary guard
  明确 FAIL、全组 151/152，而不是绕过检查。修正 guard 后的首版 mutation 又因只在整个
  `sys_futex` 搜索重复 token、被 CMP arm 意外满足而 1/101 FAIL；最终 guard 分别截取普通与
  CMP match arm，能捕获仅丢弃普通 requeue count 的变异。最终三项 guard 均 0 finding，
  focused unit 152/152、boundary/semantic 101/101 PASS；RV/LA 普通 build 均 exit 0，warning
  保持 175。无新增 production `unsafe`、依赖、syscall、ABI、架构分支或程序名/路径特化。
- dirty-tree GREEN log SHA-256 分别为
  `579c355c658a72247cb914353b09133cbe3fb74b3d830026ab306a90e5fdecea`、
  `c2f0190ffeb16d69f38df3ae0c63f63a4cc8ea6097f08763cec85731e20eea24`，两架构均明确出现
  新 marker、`USER_PASS`、`HARNESS_PASS` 与正常关机。clean `f6b3a14d` canonical RV
  evidence-runtime 为 1/1 PASS、零非 PASS，103.923602 s；summary/inner/console SHA-256 为
  `accaa276ae38f5b8d95e41aa563e6e0b5311ea4b758cc7ac8d91e120821d22df`/
  `ae44023e62b069bf3596a0752a23607fb486ffbcb75d132e2c9a525eb6cdd14f`/
  `ec72ae793a862995d6cd35bab07d9652e015edefe3e8ef5f19827fab57ab8583`，证据在
  `test/output/20260721T135548Z-evidence-runtime-rv-3691396/`。LA 同样 1/1 PASS、零非
  PASS，101.316525 s；对应 SHA-256 为
  `ad200c91864dfb0c0f564845f7f48b06651290fc90e67e05f8373e712806641c`/
  `dc07ee366a8667dcc76d39d77411fd89d575e22a099ef6448e709493b06fe530`/
  `a2aca2259b5cb95795f856228d2c239917519c2846b0f4334cd2e8d78f08406c`，证据在
  `test/output/20260721T135738Z-evidence-runtime-la-3692800/`。两份 inner evidence 都是
  required 3/3 PASS、`revision=f6b3a14d...`、`dirty=false`。同 commit canonical quick
  47/47 PASS、零非 PASS，340.940448 s，summary SHA-256
  `2a43540144c7bee813fe44d0b4b51042c1fc193365796717822761e60f7f8010`，证据在
  `test/output/20260721T135944Z-quick-none-3694217/`。
- 首次 `cargo fmt --all -- --check` 诚实 exit 1，只指出 Checkpoint 36 相邻的
  `futex.rs`/`signal_abi.rs` 两处换行漂移；纯机械提交
  `dc46877307f7ec540362d24ca49e0d9cfacb3a0a` 修复后该命令和 `git diff --check` 均通过。
  `jq` 不在 host 上，证据读取命令曾 exit 127，随后用 Python/`rg` 只读解析完成 provenance
  核验；这两项工具结果都没有冒充 suite PASS。
- 本 checkpoint 只证明普通 requeue 的 Linux 可见计数和仓内双架构行为；没有运行或推导
  正式 Rust toolchain、minibuild 或 full BuildStorm 分数。CMP requeue 的比较原子性、
  same-address 行为和更复杂 signal/timeout 竞争仍需继续审计；final rootfs 与合格 8c/8G
  host 缺口不变，正式四格继续 `BLOCKED`/未执行。

## 2026-07-21 — Checkpoint 38：同地址 `FUTEX_REQUEUE` RED -> GREEN

- 独立 host Linux 原始 syscall 探针让一个 child 等待共享 futex，再以
  `wake_count=0/requeue_count=1`、`uaddr2=uaddr` 调用普通 requeue，结果为 requeue 返回 1、
  随后同地址 wake 返回 1、child 正常退出。Linux 当前
  `b95f03f04d475aa6719d15a636ddf32222d55657` 的 `requeue_futex()` 同样明确：源/目标落在同一
  hash bucket 时无需移动链表，但仍更新 waiter key；外层循环在此之前已递增 `task_count`。
  OrayS 的 `notify_and_requeue_where()` 同对象分支调用 `operate(..., None)`，旧 closure 只有
  destination 存在才进入 requeue 计数，因此 waiter 留在原队列却错误返回 0。
- test-only `e4b693c83312ad3902b47f334410fb1b7da2b48f` 增加
  `futex_requeue_same_addr`：共享匿名 futex 上的 child 以 2 秒有界 timeout 等待，parent 在
  ready 后 settle，再要求同地址 requeue 返回 1、随后 wake 返回 1 并回收 child、fd 和
  mapping。修复前 RV/LA 都先明确通过 `futex_requeue_count`，随后得到
  `USER_FAIL futex_requeue_same_addr`、`HARNESS_FAIL guest_nonzero_exit` 与正常关机，无 panic、
  trap 或 timeout；RED log SHA-256 分别为
  `57a5eff34ec66fda6db0fdc5488f875a81ccfbe61b0b8f4fab6c0f1ee5290b9a`、
  `c58b1fc2cafab72b63ac068a8d6a806dad8cefcbf5f5df1775dfb5568675683a`。
- `673cb888f9df41393d78f303df2e9e81e1e45e62` 为同 queue 路径增加通用 in-place
  requeue：在既有 queue lock 下过滤已 wake waiter、去除重复节点、按 predicate 选择不超过
  `requeue_count` 个 waiter、调用 `on_requeue`，再按原相对顺序放回 source；它只返回受影响
  数，不把 requeue 伪装成 wake。静态 guard 同步约束计数、hook、保留与同对象调用链，并用
  mutation 将 increment 改为 0 验证。guard 首轮因旧 helper parser 不识别泛型/impl 方法而
  明确 FAIL，补强泛型 helper 解析并对 impl 路径做显式 token 约束后通过。另一次 focused
  聚合命令误写 `check_no_fake_pass.py` 且向单实现 runner 传入多个文件，exit 2、相关测试未
  执行；改正为 `check_no_fake_success.py` 和逐文件 runner 后，三项 guard 均 0 finding，
  41/41 + 10/10 + 27/27 + 75/75，共 153/153 focused unit PASS。RV/LA 普通 build 均 exit 0，
  warning 保持 175；无新增 production `unsafe`、依赖、syscall、ABI、架构或测例特化。
- dirty-tree GREEN log SHA-256 分别为
  `2217c5eced7bd858d7d7edf10d869350f2537fd6514a53fad11fb13ab6c241a3`、
  `f2e7e7e9078fee5c194325b687f11ac360c6de156be98b9e3535e144e85ce2e6`，两架构均明确出现
  新 marker、`USER_PASS`、`HARNESS_PASS` 和正常关机。实现提交后的首次 clean canonical
  在执行任何 case 前以 exit 2 fail closed：新增 mutation 实际 27 项，manifest/runner pin
  仍是 26；这不是 runtime RED，也未计 PASS。`0c6069af7ab00a6e07099316298ec21dab73af75`
  只把两处 versioned pin 更新为 27，并通过 27/27 mutation、135/135 runner regression、
  suite list 和 asset guard。
- clean `0c6069af` canonical RV evidence-runtime 为 1/1 PASS、零非 PASS，102.815919 s；
  summary/inner/console SHA-256 为
  `2e532b23bd222dac77d2e6b9756233469c5eb69b7afca2d41f1c17ec56af3e17`/
  `a9d7b127af72cc0f567e19fe5091360387c8f53caff281b2dd8f3b4c1bea42e5`/
  `8e03540c2a511f75b10f06b71a2f4810f9c648bbe97d0de19b38558c77df080c`，证据在
  `test/output/20260721T142720Z-evidence-runtime-rv-3709993/`。LA 同样 1/1 PASS、零非
  PASS，100.931243 s；对应 SHA-256 为
  `e5af561aa438f40f5573066c3aeda6e90053ae037b8b2c12df0407f93c114ac8`/
  `b254c0cee175bcc57cba868d883cfb82a45fb721b66512bd7ab50a0ab600aac0`/
  `61cac7f76ffb9474d8cc493507e7262d2d43c0dfa609396b4bb85266a287d69f`，证据在
  `test/output/20260721T142908Z-evidence-runtime-la-3711479/`。两份 outer/inner evidence
  都绑定 `0c6069af...`、前后 `dirty=false`、provenance stable，inner required 3/3 PASS。
  同 commit canonical quick 47/47 PASS、零非 PASS，339.058820 s，summary SHA-256
  `8c6adfb9dd4346635d1f114eaecb5027e2c18072c942702d24d35601b7cbe129`，证据在
  `test/output/20260721T143056Z-quick-none-3712846/`。
- 本 checkpoint 只证明普通 futex 的同地址单 waiter 计数/保留/后续 wake 和仓内双架构回归；
  没有运行或推导正式 Rust toolchain、minibuild 或 full BuildStorm 分数。多 waiter、CMP
  requeue 原子比较和 signal/timeout 交错仍需继续审计；final rootfs 与合格 8c/8G host 缺口
  不变，正式四格继续 `BLOCKED`/未执行。

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex / GPT-5 系列（精确子版本未知） | 合同读取、协议审计、基础设施盘点、parser/adapter/profile 与 Linux procfs/device/scheduler/进程/线程 comm/信号备用栈/网络/文件系统/动态 loader/`clone3`/`prctl`/futex/`flock`/rename/rlimit/FIONBIO/栈容量、进程内存统计及 Cargo hardlink 目录发布语义设计、编码、测试及日志 | 本任务计划、开发日志、final parser/adapter、suite runner/manifest、final suite discovery/timeout/POSIX shell/framing、semantic smoke、`/proc/uptime`/`statm`/task comm、设备 fd 状态、CPU affinity、BusyBox PATH wrapper、并发 TCP/fork、fork/exec/pipe、版本化 `clone3` ABI、worker-thread 名称隔离、vfork/exec、sigaltstack fork/vfork/exec lifecycle、futex join、`MADV_DONTNEED` page discard/COW 测试、ext4/ramfs volatile overlay、打开对象生命周期、lower ext4 symlink no-follow/readlink、标准 root SONAME 查找、fork 共享路径元数据、hardlink canonical/rename/目录 publish/虚拟非空目录生命周期、Unix seqpacket 记录边界、pipe FIONBIO 状态、阻塞 `flock` 竞争、rename-stable VFS 对象锁身份与 lazy 用户栈容量 | 拒绝旧镜像冒充 final、host 代跑、缺资源计 PASS、伪造 `ss` 或 prompt 答案；严格拒绝重复/缺失/资格冲突；以真实 idle runtime、task cpumask、VMA/PTE 统计、BusyBox applet、普通 POSIX shell、独立 exec ELF、标准动态 loader/SONAME root、通用 clone/futex/sigaltstack ABI、per-task mutex/atomic 状态、VMA/PTE/frame discard、路径无关 copy-up/whiteout、namespace/打开对象分离、VFS link metadata、共享 filesystem namespace、真实 backing promotion/replacement rename、目录子树 metadata 迁移、逻辑 dirent 非空检查、packet queue、打开 VFS 对象身份、typed copy-in 与逐页栈触碰替代固定值、路径哈希或程序特化；保留测试、manifest preflight、错误探针、错误 SMP 启动 panic、测试 wrapper/页表 walker/动态 slice 编译错误、statm/阻塞锁/rename/hardlink publish/虚拟目录 rename/seqpacket/rename-lock/stack-growth/FIONBIO/prctl-name/sigaltstack 首次 guest FAIL、错误 wait-status/模块/target/path/env 命令以及边界库存/注册计数/manifest 排序的首次 FAIL/ERROR，未实现 clone3 扩展与 rseq 显式 `ENOSYS` | Git/外部 SHA、image hash/payload、mutation fixture、runner regression、资产检查、外部 judge 对算、clean 缺镜像矩阵、host syscall 发现、双架构 semantic QEMU、真实 ext4 动态 ELF/交互写入/符号链接、跨进程及 canonical/replacement/published-directory hardlink、虚拟非空目录 replacement rename、Unix seqpacket、pipe FIONBIO 状态、per-thread prctl name、sigaltstack fork/vfork/exec lifecycle、阻塞 flock、rename 后独立 open 的 axfs 对象合同与 raised-stack 逐页容量合同；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 32 `SA_ONSTACK` 源码审计、行为合同、双架构 RED/GREEN、实现与记录 | `semantic_smoke.rs`、suite manifest/fixture、`signal_abi.rs`、计划与本日志 | 采用真实 handler 局部地址与 handler 内 `SS_ONSTACK` 查询；保留 fixture、错误命令、丢失状态会话和首次编译 FAIL；未实现或宣称 `SS_AUTODISARM` | 六项 guard、208/208 focused unit、RV/LA 普通 build、clean canonical RV/LA runtime 与 quick；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 33 host trace/source 审计、lazy private file mmap 合同、双架构 RED/GREEN、实现与记录 | `semantic_smoke.rs`、`fd_table.rs`、`memory_map.rs`、`mod.rs`、`user_memory.rs`、计划与本日志 | 采用稀疏普通文件、`mincore`、真实 sentinel、`msync` 与 discard/reload；拒绝文件名/路径特化、整文件假缓存或空成功；完整撤销一次误格式化；保留映射后 truncate/grow 为已知后续项 | 八项 guard、198/198 focused unit、RV/LA 普通 build、clean canonical RV/LA runtime 与 quick；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 34 host trace/source 审计、private futex COW 合同、双架构 RED/GREEN、实现与记录 | `semantic_smoke.rs`、semantic manifest/fixture、`futex.rs`、计划与本日志 | 采用 pre-fork private page、exact clone3 worker、真实 COW store 与 bounded futex wait/wake；拒绝物理地址 private key、固定成功或程序/路径特化；保留一次测试类型编译错误并不把 dirty RED/GREEN 冒充 canonical | 三项 guard、77/77 focused mutation/unit、semantic 75/75、RV/LA build、clean canonical RV/LA runtime 与 quick；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 35 host trace/man-pages/source 审计、absolute futex restart 合同、双架构 RED/GREEN、实现与记录 | `semantic_smoke.rs`、semantic manifest/fixture、`signal_abi.rs`、计划与本日志 | 采用真实 signal handler、absolute monotonic deadline、重入 wait 与真实 shared wake；拒绝直接重放 relative timeout、固定成功或程序/路径特化；dirty RED/GREEN 不冒充 canonical | 三项 guard、77/77 focused mutation/unit、semantic 75/75、RV/LA build、clean canonical RV/LA runtime 与 quick；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 36 relative futex restart deadline 审计、合同、双架构 RED/GREEN、实现与记录 | `semantic_smoke.rs`、semantic manifest/fixture、`task_context.rs`、`futex.rs`、`signal_abi.rs`、计划与本日志 | 采用真实 handler 用时、原 deadline 内 `ETIMEDOUT`、per-task 精确键/单调 deadline/armed 生命周期；拒绝完整相对时长重放、用户 timespec 改写、固定成功或特化；保留漏传 QEMU 路径的 pre-QEMU exit 2 | 三项 guard、77/77 focused mutation/unit、semantic 75/75、双架构 build、dirty RED/GREEN 与 clean canonical RV/LA runtime/quick；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 37 Linux/host futex requeue 返回值审计、合同、双架构 RED/GREEN、guard 修正与记录 | `semantic_smoke.rs`、semantic manifest/fixture、`futex.rs`、boundary guard/unit、计划与本日志 | 采用真实 source->target waiter 转移、返回计数、target wake/reap；明确保留 man-pages 与当前源码/运行时冲突、旧 guard 的错误要求和首版 mutation 漏检；拒绝固定成功或程序/路径特化 | host 原始 syscall、三项 guard、152/152 focused unit、semantic 75/75、双架构 build、dirty RED/GREEN、clean canonical RV/LA runtime/quick 与 provenance；最终仍需人工复核 | 待人工负责人确认 |
| OpenAI Codex / GPT-5 系列（精确子版本未知） | Checkpoint 38 host/Linux 同地址 futex requeue 审计、合同、双架构 RED/GREEN、通用 WaitQueue 修复与记录 | `semantic_smoke.rs`、semantic manifest/fixture、`wait_queue.rs`、boundary guard/unit、suite manifest/runner pin、计划与本日志 | 采用原队列持锁去重/保序/计数与 metadata hook；保留 guard parser 首次 FAIL、错误 focused 命令与 canonical 26/27 preflight ERROR；拒绝 wake 代替 requeue、固定成功或特化 | host 原始 syscall、Linux exact source、三项 guard、153/153 focused unit、双架构 build、dirty RED/GREEN、27/27+135/135 注册回归、clean canonical RV/LA runtime/quick 与 provenance；最终仍需人工复核 | 待人工负责人确认 |

交互摘要或记录位置：本日志只记录决定、命令、结果与证据，不提交完整对话、凭据或隐私。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| oscomp/testsuits-for-oskernel | `final-2026@15e0355bbee0373de4048002448cee37dbb7ca1b` | 只读协议、脚本、judge、BusyBox config 与 CAgent 构建说明；不修改、不复制测例进生产逻辑 | 仓库各文件原许可证，后续复制前逐项核对 | parser 独立重述结构/权重/阈值并增加 fail-closed 完整性约束；不复制测例实现，不改外部内容 | 本日志、活动计划、`test/evaluation/parse_final_2026_results.py` |
| QEMU | `v9.2.4` | 只读核验 LoongArch virt machine low/high memory map，确定 8 GiB guest 的 highram 可用窗口 | GPL-2.0-or-later；未复制代码 | adapter 只使用公开 machine contract 计算 `0x1f0000000` highram size | 本日志、`test/evaluation/run_final_2026_evaluation.py` |
| Linux kernel | `b95f03f04d475aa6719d15a636ddf32222d55657` | 只读核验 `include/uapi/linux/sched.h` 的 `clone_args` 版本尺寸、`copy_struct_from_user` 零扩展规则及 `kernel/fork.c` 的参数验证/stack base 语义 | GPL-2.0-only WITH Linux-syscall-note（UAPI）；GPL-2.0-only（kernel） | 独立 Rust 实现，仅重述公开 ABI/验证语义，没有复制 Linux 源码 | `user/shell/src/uspace/process_lifecycle.rs`、本日志 |
| Linux man-pages | Ubuntu `manpages-dev 5.10-1ubuntu1`，`sigaltstack(2)` | 只读核验成功 exec 清除、fork 复制、`CLONE_VM`/`CLONE_VFORK` 例外，以及 `SA_ONSTACK`/`SS_ONSTACK`/`EPERM` 语义 | man-pages 页面内许可；未复制实现代码 | 独立 Rust 状态转换、frame layout 和原始 syscall 测试，仅重述公开 Linux 语义 | `user/shell/src/uspace/task_context.rs`、`process_lifecycle.rs`、`signal_abi.rs`、本日志 |
| Linux man-pages | man-pages 6.18，`signal(7)`、`FUTEX_WAIT_BITSET(2const)`、`restart_syscall(2)` | 只读核验 futex 的 `SA_RESTART`、BITSET absolute timeout 与 relative wait 剩余时间调整语义 | man-pages 页面内许可；未复制实现代码 | 独立 Rust syscall 分类与原始 syscall 测试，仅重述公开 Linux 语义 | `user/shell/src/uspace/signal_abi.rs`、本日志 |
| Linux kernel + kernel docs | `b95f03f04d475aa6719d15a636ddf32222d55657`，`kernel/futex/requeue.c`；v6.12 locking 文档 | 只读核验普通 requeue 返回 requeued 或 woken 的任务总数，以及同 bucket 不移动链表但仍更新 key/计数；并与 host 原始 syscall 交叉验证 | GPL-2.0-only（kernel）；文档原许可；未复制实现代码 | 独立 Rust 计数/同队列保留修正、行为测试与 scoped mutation guard；未复制 Linux 算法 | `user/shell/src/uspace/futex.rs`、`kernel/task/axtask/src/wait_queue.rs`、本日志 |
| Linux man-pages | man-pages 6.18，`FUTEX_REQUEUE(2const)` | 只读发现其“只返回 woken”描述与当前 Linux 源码/host ABI 冲突，并将冲突保留为审计记录 | man-pages 页面内许可；未复制实现代码 | 不采用冲突描述，不改外部文档；以可复现 host 探针和当前源码限定兼容结论 | 本日志 |

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
| `74b1a4e1` | hardlink canonical lifecycle test-only RED | 1 / 1 | FAIL，guard 8 finding；unit 13 项中 3 FAIL；RV/LA canonical unlink guest 均 `ENOENT` | RED console SHA-256 `90e4aa65...e0e9` / `3797dfad...8563`；一次错误 `-smp 2` panic 单列为探针未执行 |
| `ef3d6ef9` | hardlink canonical lifecycle focused + exact-env RV/LA builds | 0 | PASS，guard + 13/13 mutation + fmt/diff + 双架构 build | RV/LA guest rename/unlink 均 PASS；raw image hash 运行前后不变；不等于正式 BuildStorm 分数 |
| `ef3d6ef9` | canonical quick | 0 | PASS，47/47，零非 PASS | 338.054299 s；summary SHA-256 `5926edf7...b1c1`；`20260721T032632Z...` |
| `ef3d6ef9` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS | 98.335347 / 94.643545 s；summary SHA-256 `0c74d044...562b` / `b02b8d6a...a9b4`；`20260721T033228Z...`、`20260721T033419Z...` |
| `e487eef8` | RV/LA blocking flock test-only RED | 1 / 1 | FAIL / FAIL，均真实执行 1/1；guest 明确 `USER_FAIL flock_blocking` | 98.767828 / 95.070846 s；summary SHA-256 `a42b3351...12e5` / `d2ed8758...9c2b`；RED console `0eccf71b...0781` / `eeca6d9d...49e8` |
| `7597bd9c` | blocking flock focused + exact-env RV/LA builds | 0 | PASS，semantic 75/75、asset 0 finding、fmt/diff、双架构 build | fd-table 临界区外等待；保留 `LOCK_NB=EAGAIN` 与中断 `EINTR`；无新增 unsafe/依赖/特化 |
| `7597bd9c` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT flock_blocking PASS` | 97.159781 / 94.093020 s；summary SHA-256 `7f855807...72ca` / `81b73a3c...77ae`；`20260721T035236Z...`、`20260721T035423Z...` |
| `7597bd9c` | canonical quick | 0 | PASS，47/47，零非 PASS | 339.530198 s；summary SHA-256 `bf40c2ab...df35`；`20260721T035606Z...` |
| `fdbdb383` | RV/LA hardlink replacement rename test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL hardlink_rename_replace` | 97.570168 / 94.536128 s；summary SHA-256 `9f7f3c73...85b0` / `db80cc85...2bea`；console `c1a1f171...5886` / `bf553e3f...ea47` |
| `d8e1055d` | hardlink replacement focused + RV/LA builds | 0 | PASS，compliance/semantic 88/88、资产 0 finding、fmt/diff、双架构 build | alias/canonical/普通目标统一处理；无新增 unsafe/依赖/特化，warning 保持 175 |
| `d8e1055d` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确新 marker PASS | 97.235054 / 94.538765 s；summary SHA-256 `1dcfc81d...9ab7` / `6ce456c5...d45d`；`20260721T042025Z...`、`20260721T042218Z...` |
| `d8e1055d` | canonical quick | 0 | PASS，47/47，零非 PASS | 339.902215 s；summary SHA-256 `34564cd0...1d7c`；`20260721T042409Z...` |
| `ed5a0773` | RV/LA `/proc/self/statm` test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL proc_statm_open` | 96.415339 / 92.116888 s；summary SHA-256 `788901a4...a102` / `063f11d4...b9ed`；RED console `b2bb090d...a6842` / `8856943b...dd5` |
| `91a5cb98` | focused checks + related units | 0 | PASS，三项 guard 0 finding、152/152 unittest | 页表 walker 两次编译失败另记，不计 PASS；生产差异仅两个 synthetic procfs/fd 文件 |
| `91a5cb98` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT proc_statm PASS` | 97.918160 / 94.675474 s；summary SHA-256 `67ede158...72d7` / `23cc2a53...1ab37`；console `5748a726...2168` / `430af21a...30e` |
| `91a5cb98` | canonical quick | 0 | PASS，47/47，零非 PASS | 336.203409 s；summary SHA-256 `4fb05698...70d9`；`20260721T050944Z...` |
| `ee37e940` | RV/LA Cargo hardlink directory publish test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL cargo_link_publish` | 111.912367 / 96.236053 s；summary SHA-256 `b500a225...16f` / `45889661...45cf`；console `e3257133...132e` / `ce08bd90...ecd1` |
| `35d1b270` | focused checks + related units + RV/LA builds | 0 | PASS，五项 guard 0 finding、171/171 unittest、双架构 build | readdir alias + 目录 subtree metadata/canonical 迁移；无新增 unsafe/依赖/特化，warning 保持 175 |
| `35d1b270` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT cargo_link_publish PASS` | 99.980872 / 95.891415 s；summary SHA-256 `38047380...94e` / `65110d37...742e`；console `09144062...b0a8` / `b5f9cf09...9204` |
| `35d1b270` | canonical quick | 0 | PASS，47/47，零非 PASS | 336.304565 s；summary SHA-256 `ea2595cf...945e`；`20260721T054440Z...` |
| `d904eac6` | RV/LA virtual-nonempty replacement rename test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL rename_virtual_nonempty` | 126.272334 / 95.943972 s；summary SHA-256 `2cacfd7b...330` / `13712eaa...54e4`；console `ce30cd00...de2e0` / `ce1bc0df...bc63` |
| `1b990b8f` | focused checks + RV/LA builds | 0 | PASS，四项 guard 0 finding、97/97 unittest、双架构 build | ordinary rename 检查逻辑 dirent；exchange 保持；无新增 unsafe/依赖/特化 |
| `1b990b8f` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确新 marker PASS | 98.821608 / 95.015367 s；summary SHA-256 `f2bf3c03...ba81` / `021bda50...007`；console `96be20e8...005` / `0ed8b708...720` |
| `1b990b8f` | canonical quick | 0 | PASS，47/47，零非 PASS | 335.435847 s；summary SHA-256 `57ed2a38...bcdf`；`20260721T061757Z...` |
| `f19fc9f6` | RV/LA Unix seqpacket test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL unix_seqpacket` | 98.436434 / 96.429793 s；summary SHA-256 `2b21ed70...871` / `76d55a12...e6f`；console `6288b9f3...3566` / `333e6a6c...211c` |
| `89740c70` | focused checks + RV/LA builds | 0 | PASS，六项 guard 0 finding、157/157 unittest、双架构 build | 有界原子 record queue + local recvfrom；无新增 unsafe/依赖/特化，warning 保持 175 |
| `89740c70` | 首次 canonical quick | 1 | FAIL，45/47；仅 Linux-boundary check/unit 因 legacy copyout 库存增长失败 | summary SHA-256 `c2294ba0...2db`；`20260721T065548Z...`，完整保留 |
| `8c23d29a` | typed local recv boundary + canonical quick | 0 | PASS，47/47，零非 PASS | 336.397217 s；summary SHA-256 `35159eb6...b76`；`20260721T070732Z...` |
| `8c23d29a` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT unix_seqpacket PASS` | 98.754050 / 95.747174 s；summary SHA-256 `ad1f5536...b5c` / `af8f094a...a18b`；console `93aba8ca...ad4` / `73dffd68...526` |
| `0c97a5f5` | RV/LA rename-stable flock test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL flock_rename_identity` | 99.146718 / 95.296556 s；summary SHA-256 `92e3c489...9ed8b` / `e81c7777...fd42`；console `42b34c2d...ca5c4` / `3a665f23...140d` |
| `9ca98b0d` | focused checks + RV/LA builds | 0 | PASS，POSIX guard 0 finding、mutation 16/16、semantic 75/75、competition 33/33、双架构 build | resolved VFS node identity；无新增 unsafe/依赖/ABI/架构/特化，warning 保持 175 |
| `9ca98b0d` | 首次 canonical GREEN preflight | 2 | ERROR，0 case executed；POSIX mutation 实际 16、manifest/pin 仍为 15 | fail-closed infrastructure error；未计 PASS，随后由 `4e2e63da` 注册修复 |
| `4e2e63da` | POSIX binding + runner regression + asset/list | 0 | PASS，16/16 + 135/135，资产 0 finding，完整 profile list 成功 | mutation 注册实际/manifest/canonical 计数统一为 16 |
| `4e2e63da` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT flock_rename_identity PASS` | 98.796166 / 95.574923 s；summary SHA-256 `7548545b...df49` / `048fdffd...ce3d`；console `ebffafd8...785b` / `549f9b7a...ff0e` |
| `4e2e63da` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 336.239994 s；summary SHA-256 `8c81ac2c...d6f7`；`20260721T074924Z-quick-none-3473384/` |
| `f21e419c` | RV/LA raised-stack test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL rlimit_stack_growth` | 97.388520 / 91.816650 s；summary SHA-256 `59a5ac24...75555` / `646d4fc0...73fc`；console `d48b277b...47cb3` / `8892c9bc...04eb9` |
| `1ce29190` | focused rlimit/manifest/runner + RV/LA builds | 0 | PASS，rlimit guard 0 finding、13/13 + 75/75 + 135/135、双架构 build | 64 MiB lazy VMA；无新增 unsafe/依赖/ABI/架构/特化；命令名误用与 manifest 排序 ERROR 单列保留 |
| `1ce29190` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT rlimit_stack_growth PASS` | 99.772205 / 96.869639 s；summary SHA-256 `5585c799...a1e4` / `c46a8f08...f551`；console `0f80294a...1f35` / `8d5ee66c...b86b` |
| `1ce29190` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 338.801596 s；summary SHA-256 `aea46eb4...edfc`；`20260721T082610Z-quick-none-3496138/` |
| `bf053edc` | RV/LA pipe FIONBIO test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL pipe_fionbio` | 97.988846 / 92.456968 s；summary SHA-256 `8a242a52...962` / `db78901a...4fd`；console `404c5fb0...c7` / `c7af1898...fa` |
| `cd6c8711` | FIONBIO focused + RV/LA builds | 0 | PASS，file-object guard 0 finding、event 33/33、semantic 75/75、双架构 build | 复用真实 F_GETFL/F_SETFL 状态；无新增 unsafe/依赖/特化；warning 保持 175 |
| `cd6c8711` | 首次 canonical quick | 1 | FAIL，45/47；仅 Linux-boundary check/unit 因 legacy copy-in 库存增长失败 | 337.727674 s；summary SHA-256 `edbd70f7...081`；`20260721T085227Z...`，完整保留 |
| `d5b606c7` | typed FIONBIO copy-in focused + RV/LA builds | 0 | PASS，boundary guard 0 finding、17/17 mutation、fmt/diff、双架构 build | `UserPtr<i32, Read>`；未新增 unsafe；45.30/42.35 s |
| `427ea835` | evidence provenance + focused validation | 0 | PASS，JSON、semantic 75/75、asset guard 0 finding | RV/LA smoke build/runtime 均绑定 `user_memory.rs` |
| `427ea835` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 337.848743 s；summary SHA-256 `0fa479fc...a3c2`；`20260721T091341Z...` |
| `427ea835` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT pipe_fionbio PASS` | 99.648451 / 96.189316 s；summary SHA-256 `1f01d91c...7288` / `232a4726...608`；console `c791920c...194` / `22a6b96c...c5` |
| `2e7becaa` | RV/LA per-thread prctl name test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL prctl_thread_name` | 100.332579 / 96.768469 s；summary SHA-256 `b786a500...f47fb` / `b509b442...118e`；console `99295e48...f7b` / `8b3dddd9...1168` |
| `eedc4df3` | per-thread comm focused + RV/LA builds | 0 | PASS，state/boundary/asset guard 0 finding、semantic 75/75、双架构 build | per-task mutex + fork/clone inheritance + procfs task lookup；无新增 unsafe/依赖/特化；warning 保持 175 |
| `17d30a67` | evidence provenance + focused validation | 0 | PASS，JSON、competition/asset guard、semantic 75/75 | RV/LA smoke build/runtime 均绑定 `system_info.rs` 与 `task_context.rs` |
| `17d30a67` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT prctl_thread_name PASS` | 99.440612 / 96.494603 s；summary SHA-256 `9d115d25...a342` / `96193a37...d54b`；console `54ce19ce...01cf` / `c05ff40b...00d4` |
| `17d30a67` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 337.127419 s；summary SHA-256 `c79da98a...d250`；`20260721T095323Z...` |
| `f28dc367` | RV/LA sigaltstack lifecycle test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1；guest 明确 `USER_FAIL sigaltstack_fork_exec` | 99.811806 / 96.981920 s；summary SHA-256 `61d72560...8717` / `dfc0c474...d677`；console `727b9d67...911` / `9f0256b3...dce` |
| `915cafd1` | focused guards + semantic unit + RV/LA builds | 0 | PASS，五项 guard 0 finding、75/75 unittest、双架构 build | fork/vfork inherit + exec reset；无新增 unsafe/依赖/ABI/架构/特化；warning 保持 175 |
| `915cafd1` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT sigaltstack_fork_exec PASS` | 100.833471 / 97.283654 s；summary SHA-256 `9b3de296...560a` / `cc53248c...320b`；console `db996d28...c135` / `aabf192b...d61` |
| `915cafd1` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 337.937485 s；summary SHA-256 `2d8e1ce8...32e9`；`20260721T103033Z...` |
| `5ab91a01` | RV/LA SA_ONSTACK test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1、零 timeout/crash/infra；guest 明确 `USER_FAIL sigaltstack_onstack` | 102.129392 / 97.547648 s；summary SHA-256 `38998cfe...2142` / `c003ccb1...3210`；console `a55e4e49...b28a` / `116280cf...5445` |
| `066c4ade` / `36584570` | SA_ONSTACK focused guards/unit + RV/LA builds | 0 | PASS，六项 guard 0 finding、相关 unit 208/208、双架构普通 build | fixture 首轮 205/208 FAIL 与 RV 首轮类型编译 FAIL 均保留；修复为 fallible conversion；无新增 unsafe/依赖/特化 |
| `36584570` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT sigaltstack_onstack PASS` | 100.381632 / 97.712269 s；summary SHA-256 `7c7c35ad...9bd` / `22f9d39a...a7f`；console `17494c54...122` / `c2dbbb52...fb2` |
| `36584570` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 337.675069 s；summary SHA-256 `75f0aa30...7dd`；`20260721T110859Z...` |
| `0f37d27e` | RV/LA lazy private file mmap test-only RED | 1 / 1 | FAIL / FAIL，均实际执行 1/1、零 timeout/crash/infra；guest 明确 `USER_FAIL mmap_private_lazy` | 99.069460 / 93.553982 s；summary SHA-256 `b4c67445...a2b2` / `face1bd7...bcfe`；console `6b06ce1f...fb6` / `aca364ff...7283` |
| `35a65b75` / `c2258c56` | lazy mapping lifecycle focused guards/unit + RV/LA builds | 0 | PASS，八项 guard 0 finding、相关 unit 198/198、双架构普通 build | `msync` no-prefetch + private discard/reload；无新增 unsafe/依赖/ABI/架构/特化，warning 保持 175 |
| `c2258c56` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS；guest 明确 `ASSERT mmap_private_lazy PASS` | 102.108468 / 99.612675 s；summary SHA-256 `9c400619...960` / `83cf7881...85c`；console `a974cb2d...177` / `430fbe02...583` |
| `c2258c56` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 338.509931 s；summary SHA-256 `7e4de070...031`；`20260721T120324Z...` |
| `53656508` | RV/LA private-futex COW dirty supervised RED | QEMU 0 / 0，guest nonzero | FAIL / FAIL；均先过 clear-child-tid join，再明确 `USER_FAIL private_futex_cow`，无 timeout/panic/trap | RED log SHA-256 `fd47a3a0...c39c` / `0a5601b9...add9`；仅定向诊断，不作为 canonical PASS |
| `0adfc175` | private-futex focused guards/unit + RV/LA smoke builds | 0 | PASS，三项 guard 0 finding、77/77 mutation/unit、semantic 75/75、双架构 build | private mm+uaddr / shared paddr；信号精确 task wake；无新增 production unsafe/依赖/ABI/特化 |
| `0adfc175` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance clean/stable；guest 明确 `ASSERT private_futex_cow PASS` | 101.900932 / 99.235038 s；summary SHA-256 `2b50ff7f...6f7f` / `bee067c7...638`；console `c40a3f12...379` / `f20e0464...053` |
| `0adfc175` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance clean/stable | 339.926120 s；summary SHA-256 `65fb21f7...45b1`；`20260721T124043Z-quick-none-3648442/` |
| `607b0379` | RV/LA futex `SA_RESTART` dirty supervised RED | QEMU 0 / 0，guest nonzero | FAIL / FAIL；均明确 `USER_FAIL futex_sa_restart`，无 timeout/panic/trap | RED log SHA-256 `293e9115...18d8f` / `0b21118f...ad83`；仅定向诊断，不作为 canonical PASS |
| `991f5856` | futex-restart focused guards/unit + RV/LA smoke builds | 0 | PASS，三项 guard 0 finding、77/77 mutation/unit、semantic 75/75、双架构 build | 仅重启 absolute/no-timeout BITSET wait；无新增 production unsafe/依赖/ABI/特化；dirty GREEN log `4f46f2da...a963` / `11e475d2...30f5` |
| `991f5856` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance clean/stable；guest 明确 `ASSERT futex_sa_restart PASS` | 102.600339 / 99.351574 s；summary SHA-256 `e4ea411b...f59` / `7adad038...83e`；console `75edaba6...5b3` / `660ead25...2f1` |
| `991f5856` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance clean/stable | 340.749107 s；summary SHA-256 `fb99ebde...709`；`20260721T130836Z-quick-none-3663611/` |
| `4eef0eb8` | RV/LA relative futex restart dirty supervised RED | QEMU 0 / 0，guest nonzero | FAIL / FAIL；均先过 absolute restart，再明确 `USER_FAIL futex_relative_restart`，无 timeout/panic/trap | RED log SHA-256 `1275351c...5355a` / `504662ff...58e`；首次漏传 QEMU 路径 exit 2 为 pre-QEMU ERROR，未计测试结果 |
| `fb889572` | relative restart focused guards/unit + RV/LA smoke builds | 0 | PASS，三项 guard 0 finding、77/77 focused mutation/unit、semantic 75/75、双架构 build | per-task exact key + monotonic deadline + armed lifecycle；dirty GREEN log `001f9472...91a` / `63a0a2bb...424`；无新增 unsafe/依赖/ABI/特化 |
| `fb889572` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，provenance clean/stable；guest 明确 `ASSERT futex_relative_restart PASS` | 103.863282 / 101.100630 s；summary SHA-256 `3b00ff4a...5d94` / `6c7b38c4...6277`；console `7bd5954d...bfc0` / `2386cf6c...a1f` |
| `fb889572` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance clean/stable | 339.867055 s；summary SHA-256 `4f11ab17...88a`；`20260721T133529Z-quick-none-3678600/` |
| `a720bc6e` | RV/LA plain requeue count dirty supervised RED | QEMU 0 / 0，guest nonzero | FAIL / FAIL；均先过 futex restart marker，再明确 `USER_FAIL futex_requeue_count`，无 timeout/panic/trap | RED log SHA-256 `2008d181...c4829` / `307b9e3d...58f6`；仅定向诊断，不作为 canonical PASS |
| `f6b3a14d` | requeue-count focused guards/unit + RV/LA smoke builds | 0 | PASS，三项 guard 0 finding、152/152 focused unit、boundary/semantic 101/101、双架构 build | old guard 首次 151/152 FAIL、首版 mutation 1/101 FAIL 均保留；dirty GREEN `579c355c...ecea` / `c2f0190f...e24` |
| `f6b3a14d` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，inner provenance clean/stable；guest 明确 `ASSERT futex_requeue_count PASS` | 103.923602 / 101.316525 s；summary SHA-256 `accaa276...22df` / `ad200c91...641c`；console `ec72ae79...83` / `a2aca225...06c` |
| `f6b3a14d` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance stable | 340.940448 s；summary SHA-256 `2a435401...010`；`20260721T135944Z-quick-none-3694217/` |
| `dc468773` | rustfmt/diff mechanical closure | 0 | PASS，`cargo fmt --all -- --check` 与 `git diff --check` | 首次 fmt exit 1 的两处差异保留；仅 3 insertions/4 deletions 的换行变化，无语义改动 |
| `e4b693c8` | RV/LA same-address futex requeue dirty supervised RED | QEMU 0 / 0，guest nonzero | FAIL / FAIL；均先过普通 requeue count，再明确 `USER_FAIL futex_requeue_same_addr`，无 timeout/panic/trap | RED log SHA-256 `57a5eff3...90b9a` / `c58b1fc2...5683a`；仅定向诊断，不作为 canonical PASS |
| `673cb888` | same-address focused guards/unit + RV/LA smoke builds | 0 | PASS，三项 guard 0 finding、153/153 focused unit、双架构 build | guard parser 首次 FAIL 与错误 focused 命令 exit 2 均保留；dirty GREEN `2217c5ec...41a3` / `f2e7e7e9...ce2e6` |
| `673cb888` | 首次 clean canonical preflight | 2 | ERROR，0 case executed；mutation 实际 27、manifest/runner pin 仍为 26 | fail-closed infrastructure error；未计 PASS，随后由 `0c6069af` 注册修复 |
| `0c6069af` | mutation identity/runner/list/asset registration | 0 | PASS，27/27 + 135/135、suite list、asset guard 0 finding | 两处 versioned count 从 26 收紧到 27 |
| `0c6069af` | RV/LA canonical evidence-runtime | 0 / 0 | PASS，1/1 + 1/1，零非 PASS，outer/inner provenance clean/stable；guest 明确 `ASSERT futex_requeue_same_addr PASS` | 102.815919 / 100.931243 s；summary SHA-256 `2e532b23...3e17` / `e5af561a...4ac8`；console `8e03540c...080c` / `61cac7f7...d69f` |
| `0c6069af` | canonical quick | 0 | PASS，47/47，零非 PASS，provenance clean/stable | 339.058820 s；summary SHA-256 `8c6adfb9...e129`；`20260721T143056Z-quick-none-3712846/` |

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
不能从后续候选或历史记录倒推 baseline。relative `FUTEX_WAIT` 的 remaining-time restart
已由 `4eef0eb8` -> `fb889572`、普通 requeue 返回计数已由 `a720bc6e` -> `f6b3a14d`、
同地址 requeue 已由 `e4b693c8` -> `673cb888`/`0c6069af` 形成双架构 RED/GREEN；仓内
继续审计 CMP requeue 原子比较、signal/timeout 与比较-排队竞争，并以通用可复现行为合同
限定每项结论，不把当前 checkpoint 外推为完整并发压力或正式 toolchain 通过。

## 回滚方式

后续每个逻辑提交使用普通 revert；不 reset --hard、破坏性 rebase 或覆盖历史。

# 10. 最终摘要

任务进行中；尚无 final-2026 得分，未宣称 PASS、Ready 或 merge-ready。
