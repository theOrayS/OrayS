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
head_commit: "b83380159ebbe68a065d78823f91229920d7c2d1"
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

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex / GPT-5 系列（精确子版本未知） | 合同读取、协议审计、基础设施盘点、parser/adapter/profile 与 Linux procfs/device/scheduler/进程/网络/`clone3` 语义设计、编码、测试及日志 | 本任务计划、开发日志、final parser/adapter、suite runner/manifest、semantic smoke、`/proc/uptime`、设备 fd 状态、CPU affinity、BusyBox PATH wrapper、并发 TCP/fork、fork/exec/pipe 与版本化 `clone3` ABI | 拒绝旧镜像冒充 final、host 代跑、缺资源计 PASS、伪造 `ss` 或 prompt 答案；严格拒绝重复/缺失/资格冲突；以真实 idle runtime、task cpumask、BusyBox applet、独立 exec ELF 和通用 clone ABI 替代固定值；未实现 clone3 扩展显式 `ENOSYS` | Git/外部 SHA、image hash/payload、mutation fixture、runner regression、资产检查、外部 judge 对算、clean 缺镜像矩阵及双架构 RED/GREEN semantic QEMU；最终仍需人工复核 | 待人工负责人确认 |

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
这不影响继续完成 runner、parser、build 和通用 guest 语义工作。

## 后续工作

按活动计划持续执行，资源到位后必须先在精确 d989 或可追溯等价基线重建真实分数矩阵，
不能从后续候选或历史记录倒推 baseline。

## 回滚方式

后续每个逻辑提交使用普通 revert；不 reset --hard、破坏性 rebase 或覆盖历史。

# 10. 最终摘要

任务进行中；尚无 final-2026 得分，未宣称 PASS、Ready 或 merge-ready。
