## 变更摘要

<!-- 说明问题、目标、实现方式及用户/内核可见影响。 -->

## 范围

- 能力域：
- 非目标：
- 关联 issue/计划/ADR：
- 开发日志：`docs/development-logs/...`

## 设计与兼容性

- Linux/POSIX ABI、syscall、errno 或结构布局变化：
- RISC-V64 / LoongArch64 差异：
- 并发、资源回收或安全不变量：
- 回滚方式：

## 测试证据

- HEAD：
- Run ID：
- RISC-V64 镜像 SHA-256：
- LoongArch64 镜像 SHA-256：

| 命令 | 架构/目标 | 退出码 | 结果 | 证据 |
|---|---|---:|---|---|
| `python3 test/run_suite.py --profile quick` | 通用 | | | |
| `python3 test/run_suite.py --profile baseline` | RV64 + LA64 | | | |
| `python3 test/run_suite.py --profile official --arch rv` | RISC-V64 | | | |
| `python3 test/run_suite.py --profile official --arch la` | LoongArch64 | | | |
| `python3 test/run_suite.py --profile full --arch all` | 全部 | | | |

## AI 与外部来源

- AI 工具/模型、使用场景、影响范围及人工验证：
- 外部代码/设计来源、版本或 commit、许可证及改写情况：

## 最终检查

- [ ] 开发日志从开发开始持续更新，而非事后补写。
- [ ] 未使用测例名称、固定路径、特殊输入或评测环境进行特化。
- [ ] 未通过弱化测试、扩大 blacklist、吞退出码或伪造成功获得绿色结果。
- [ ] `git diff --check` 通过，diff 无无关格式化、生成物、凭据或机器相关绝对路径。
- [ ] 新增/修复行为有成功、错误、边界及适用的并发/资源回收测试。
- [ ] RISC-V64 与 LoongArch64 的完整官方评测均在当前 HEAD 明确通过。
- [ ] AI 使用与外部参考已经如实披露。
- [ ] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 负责人能够在无 AI 辅助下解释并现场定位本 PR 的关键代码。
