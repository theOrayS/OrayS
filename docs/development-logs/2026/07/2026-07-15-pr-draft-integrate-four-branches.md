---
title: "PR draft: integrate four OrayS branches"
date_started: 2026-07-15
date_completed: 2026-07-15
status: blocked
pr: null
branch: "integration/four-prs-20260715"
authors:
  - "Codex primary agent (AI-assisted integration operator)"
reviewers:
  - "Codex independent read-only reviewer subagent (AI; not a human reviewer)"
base_commit: "921171ac1ef5c85ab5a7cd1882dd40e1471b79f0"
head_commit: "74f55223c3831e3f5cca45578c064ea45193fbff"
capability_domains:
  - linux-abi-boundary
  - file-object-event-core
  - canonical-test-infrastructure
  - dual-architecture-semantic-evidence
  - repository-governance
---

# 1. 背景与目标

## 背景

四个已完成来源分支分别承载 Linux 边界、文件对象/事件核心、统一 fail-closed 测试套件和 PR3 双架构 semantic evidence。任务要求在新鲜 `origin/main` 上保留四条历史、按语义解决交叉冲突、安装长期治理，并且只有最终同一 HEAD 的双架构完整门禁和独立审查都明确通过后才能推广。

## 目标

- 按 PR1、PR2、统一套件、PR3 的顺序创建四个显式 merge commit。
- 保留 Linux/POSIX ABI、errno、资源生命周期和两架构行为，同时把测试收敛到唯一 `test/` 架构。
- 安装 workflow starter，保留临时 PR1 策略中的项目事实，并建立持续计划/日志/决策/参考目录。
- 在最终候选 HEAD 上运行 canonical quick、baseline、RV/LA official 与 full，并由独立 reviewer 复核。
- 仅在全部条件明确 PASS 且 `origin/main` 未漂移时安全推广；否则保持 Draft 并准确报告 non-pass。

## 非目标

- 不 squash、rebase、force-push 或改写来源历史。
- 不改官方 backing image，不提交构建产物或大体积日志。
- 不用 testcase/path/input 特化、假成功、吞退出码、扩 blacklist、弱化 parser 或断言换取绿色。
- 不做与冲突解决和真实门禁缺陷无关的依赖、工具链、ABI 或大范围格式化变更。

## 验收标准

- [x] 四个来源 tip 都以独立 no-ff merge parent 保留且 ancestry 可验证。
- [x] 最终测试所有权唯一落在 `test/`，59 个 case 可显式列出。
- [x] workflow governance 独立提交且文档完整。
- [x] 最终候选 HEAD 的 quick、baseline、RV official、LA official、full 均已真实执行并保留证据。
- [ ] 上述五个 canonical gate 均明确 PASS（RV、LA、full 当前为 `INFRA_ERROR`）。
- [x] 独立只读 reviewer 的 blocker/major finding 清零。
- [x] 最终远端 freshness 与可恢复的 integration 分支普通推送完成。
- [ ] `main` 安全推广完成（official/full 门禁 non-pass，按合同保持阻断）。

# 2. 基线

| 时间（UTC） | 命令/检查 | 架构/目标 | 退出码 | 结果 | 备注/证据 |
|---|---|---|---:|---|---|
| 2026-07-15T10:28:10Z | target status + `git fetch origin --prune` | 仓库/远端 | 0 | PASS | clean `921171ac...`; `origin/main` 同值 |
| 2026-07-15T10:38:04Z | PR1 focused boundary checks/builds | host/RV64/LA64 | 0 | PASS | guard 0 findings; mutations 15/15; 双架构 shell build |
| 2026-07-15T10:38:04Z | `cargo fmt --all -- --check` | workspace | 1 | FAIL | 四个既有格式漂移，未隐藏 |
| 2026-07-15T10:38:04Z | `make unittest_no_fail_fast` | workspace | 2 | FAIL | axfs `test_devfs_ramfs()` -> `NotFound` |
| 2026-07-15T10:56:31Z | PR2 focused + workspace reproducer | host/RV64/LA64 | 0 | PASS | event registry feature-unification 缺陷已修复并复现闭环 |
| 2026-07-15T11:14:15Z | canonical `quick` at `126e21a4` | common | 1 | FAIL | 40/40 completed; 38 PASS, RR aging guard/unit 2 FAIL |
| 2026-07-15T11:58:16Z | PR3 focused guards/units/build/runtime probes | host/RV64/LA64 | 0 | PASS/PARTIAL | LA exact QEMU 9.2.4 runtime PASS；RV QEMU 6.2.0 仅兼容探测，不是 required evidence |

已有失败与环境约束不会因后续重试被删除。最终 verdict 只能来自治理提交及所有真实修复后的同一 clean HEAD。

## 完整来源/ref inventory

| 来源 | 原始仓库 | mirror snapshot | origin | 选中分支 | 完整 tip | ref snapshot |
|---|---|---|---|---|---|---|
| PR1 | `/root/OrayS-pr1` | `inputs/pr1.git` | `git@github.com:theOrayS/OrayS.git` | `refs/heads/refactor/pr1-linux-boundary` | `70b57e38b42bf09407e405b2fc30ee413dca2404` | 84 行；SHA-256 `2e7bbb970b108a31ee021ffda566da20a6db268e9ba98740b5d6b166fdece93b` |
| PR2 | `/root/OrayS-pr2` | `inputs/pr2.git` | `git@github.com:theOrayS/OrayS.git` | `refs/heads/feat/pr2-file-object-event-core` | `7b16e14709469f4d67ed268eb8433159801e9124` | 84 行；SHA-256 `a73f5a13e24ca7a7d5b1999c140a24a82407ca34aeb80eadc60c38a4a2ef28f9` |
| unified suite | `/root/OrayS-test-suite` | `inputs/test-suite.git` | `git@github.com:theOrayS/OrayS.git` | `refs/heads/test/unified-local-test-suite` | `0c2a3cffca2fa7d276ea8d0ec3524df8fc0669ba` | 84 行；SHA-256 `73d64e4e86bd48c3e2446205d9170835f61d2ed9e497cb9df665ff0995206a2e` |
| PR3 | `/root/OrayS-pr3` | `inputs/pr3.git` | `git@github.com:theOrayS/OrayS.git` | `refs/heads/ci/pr3-competition-semantic-evidence` | `7562ea69770501769fcf5c163a0e95343ffd2e2b` | 84 行；SHA-256 `c53bf39cdc905a0258e5d2ae708a40452b82c41ddbad2380664347bed2c3457f` |

四份完整 84 行 namespace snapshot 保存在任务归档 `preflight/*-refs.txt`；上表记录其不可混淆的行数和 digest，并完整给出实际参与 merge 的 source/ref/full hash。四个选中 tip 在各自 named branch 上都与预期一致，未导入 stash 或未提交改动。

# 3. 设计与决策

## 方案

每个来源先以 `git merge --no-ff --no-commit` 进入 index，按文件和行为解析冲突，运行定向检查后再创建命名 merge commit。统一套件先确定唯一 manifest/runner，PR3 的 collector/render/parser/runtime smoke 作为其 adapter 接入。治理 starter 只在四个 merge 完成后从外部 staging 逐文件安装。

## 备选方案

- 拒绝 squash/cherry-pick 聚合：会丢失来源边界和父提交证据。
- 拒绝 `-X ours/theirs` 或整文件覆盖：会静默丢失另一侧语义。
- 拒绝保留 `scripts/` 与 `test/` 双份测试业务逻辑：会产生 verdict 漂移。
- 拒绝把已有失败记成“既有所以可忽略”：最终合同要求真实修复或准确 BLOCKED/FAILED。

## 关键决策

| 决策 | 理由 | 风险 | 回滚方式 |
|---|---|---|---|
| PR1 边界保留 typed user memory/backend，`UserProcess` 首阶段仍在 shell | 保持依赖方向和 ABI 行为，避免大爆炸迁移 | 临时 compatibility re-export 边界需持续审计 | revert PR1 merge 或从 backup tag 重建 |
| PR2 event registry 固定使用 `SpinNoIrq`，公共 ordered-pair helper 保持 caller mutex 类型 | 关闭 workspace feature-unification 下无 current task 的 abort，同时保持 pipe API | 内部锁语义必须由 mutation + workspace reproducer 约束 | revert narrow reconciliation commit/merge |
| PR3 收敛进 canonical suite，删除 alternate `scripts/` test copies | 一个 manifest、一个结果合同、一个 clean-tree provenance 边界 | 注册数量和路径 inventory 必须同步维护 | revert PR3 merge并重新做语义 port |
| workflow starter 独立提交 | 让治理变更与来源代码可分别审查/回滚 | 暂时需要 dirty worktree，canonical runner会正确拒绝 | revert governance commit |
| promotion 需要 exact QEMU、双 official、full、独立 review 和远端 freshness | 防止局部/旧证据被外推成完成 | 可能因外部镜像 contract 阻塞 | 保持 Draft，不更新 main |

# 4. 开发与调试记录

> 以下 checkpoint 从任务开始即持续写入的外部 journal 原样迁入；时间均为 UTC。失败、被否决方案和非 canonical 探测均保留，未用后续结果覆盖。

## 2026-07-15T10:28:10Z - preflight accepted

- HEAD: `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0` (`main`).
- Target identity: `/root/oskernel2026-orays`; `origin` fetch/push URL is
  `git@github.com:theOrayS/OrayS.git`, matching the OrayS project.
- Safety: `git status --porcelain=v2 --branch` showed no worktree or index entries;
  only the clean branch header was present. No original source repository was changed.
- Freshness: `git fetch origin --prune` exited 0. Fresh `origin/main` is
  `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`.
- Source mirrors and selected refs:
  - PR1: `inputs/pr1.git`, `refs/heads/refactor/pr1-linux-boundary`,
    `70b57e38b42bf09407e405b2fc30ee413dca2404`.
  - PR2: `inputs/pr2.git`, `refs/heads/feat/pr2-file-object-event-core`,
    `7b16e14709469f4d67ed268eb8433159801e9124`.
  - unified suite: `inputs/test-suite.git`,
    `refs/heads/test/unified-local-test-suite`,
    `0c2a3cffca2fa7d276ea8d0ec3524df8fc0669ba`.
  - PR3: `inputs/pr3.git`,
    `refs/heads/ci/pr3-competition-semantic-evidence`,
    `7562ea69770501769fcf5c163a0e95343ffd2e2b`.
- Complete source inventories are retained in `preflight/*-refs.txt`. Each selected
  branch exists at exactly the expected tip; no extra named-branch commits were found.
- Official backing images exist as regular files at the canonical paths:
  - RV: `/root/sdcard-rv.img`, SHA-256
    `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99`.
  - LA: `/root/sdcard-la.img`, SHA-256
    `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50`.
- Workflow input: `.codex/input/orays-workflow-starter.zip` exists and was listed
  without extraction. It contains root `AGENTS.md`, a PR template, and development-log
  README/template files.
- Rationale: all stop-before-write preflight conditions pass, so proceed with reversible
  backup/tag/bundle creation and an integration branch from fresh `origin/main`.
- AI/model/Goal-mode use: Codex Goal mode is active with the user-supplied durable
  objective. The primary Codex agent performed this preflight; no subagent or human
  review has yet been used. Exact serving build metadata is not exposed to the agent.
- Remaining risks: semantic conflicts across PR1/PR2, convergence of PR3 with the unified
  runner, two-architecture runtime duration/tool availability, and final remote freshness.

## 2026-07-15T10:29:13Z - rollback checkpoint and integration branch

- HEAD: `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`.
- Created annotated local tag `backup/pre-four-prs-20260715` at the initial
  `origin/main`; peeled tag target was independently checked as `921171ac...`.
- Created complete pre-integration bundle
  `orays-pre-integration-921171ac.bundle` (117,308,363 bytes), SHA-256
  `104e4cba9c782af6717910f7ea35e26f9f2a2bdcdf369157a9183d6f5f3b76d5`.
  `git bundle verify` exited 0 and reported complete history.
- Created and switched to `integration/four-prs-20260715` directly from fresh
  `origin/main`; it tracks `origin/main`. `main` itself remains unchanged.
- Conflicts/failures: none. All commands exited 0.
- Rationale: the tag, complete bundle, and isolated branch provide reversible recovery
  before any source history is merged.
- AI/model/Goal-mode use: primary Codex agent; Goal mode remains active; no subagent.
- Remaining risks: source semantic reconciliation and all validation/review/promotion
  gates remain outstanding.

## 2026-07-15T10:38:04Z - PR1 staged and validated

- Pre-commit HEAD: `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`; MERGE_HEAD:
  `70b57e38b42bf09407e405b2fc30ee413dca2404`.
- Action: `git merge --no-ff --no-commit
  source-pr1/refactor/pr1-linux-boundary` exited 0 with no conflicts. The index tree
  `e263aaf23302013c48fe1fa65b4c5a2bf7692b28` exactly equals the selected PR1 source
  tree, proving no unjournaled resolution changed the source content.
- Semantic result: preserved the new `orays-linux-abi` and `orays-linux` crates,
  typed user memory/backend boundary, syscall metadata, compatibility facade, and
  dependency direction `arceos-shell -> orays-linux -> orays-linux-abi`.
- Passing checks:
  - PR1 static guard: 0 findings; mutation suite: 15/15.
  - `orays-linux-abi` host tests and `orays-linux` 9/9 host tests.
  - Both crates: locked/offline host, RV64, and LA64 check and clippy, all exit 0.
  - locked/offline metadata and shell dependency-tree checks, exit 0.
  - targeted boundary-crate formatting and staged diff check, exit 0.
  - official-feature `user/shell` RV64 and LA64 builds, including link/objcopy,
    both exit 0.
  - workspace tests excluding `axfs`, exit 0.
- Visible pre-existing failures retained:
  - `cargo fmt --all -- --check` exit 1 on the four source-documented files
    (`pipe.rs`, axfs `dev.rs`/`root.rs`, and `wait_queue.rs`).
  - `make unittest_no_fail_fast` exit 2 because axfs `test_fatfs` reports
    `test_devfs_ramfs() failed: NotFound`.
  - G012 mutation tests exit 1 at 25/26 on the source-documented
    `test_detects_empty_central_user_trace`; its static guard passed.
  These are not called PASS and remain final-gate work.
- Test side effect: host tests regenerated `api/arceos_posix_api/src/ctypes_gen.rs`
  with packed epoll layout. It was restored by an exact four-line patch; final
  unstaged diff is empty and index/source trees again match.
- Unsafe/dependency notes: PR1 adds no unsafe block in either boundary crate and no
  registry dependency/version/checksum change; Cargo.lock adds only the intended path
  packages/edges.
- AI/model/Goal-mode use: primary Codex agent; no subagent. No conflict-resolution
  heuristic was used.
- Remaining risks: PR2 must be reconciled against PR1; known baseline failures cannot
  satisfy the final canonical profile contract until honestly resolved.

## 2026-07-15T10:56:31Z - PR2 staged and validated

- Pre-commit HEAD: `aa9072df32e4ced0edc70009ad456d62810ef2f3`; MERGE_HEAD:
  `7b16e14709469f4d67ed268eb8433159801e9124`.
- Action: `git merge --no-ff --no-commit
  source-pr2/feat/pr2-file-object-event-core` exited 0 without textual conflicts.
  The staged tree is `5d6ce751c3392eea993dc6734c6d23574002e3d4`; it intentionally differs from
  the PR2 source tree `d365f3e016d07666013c3a3ae902ecb2d97532d7` because the integration preserves
  PR1 and contains the two documented semantic reconciliations below.
- PR1/PR2 boundary reconciliation: PR2 legitimately adds one typed user-read and
  one typed user-write call site in `fd_table.rs`. The PR1 guard's exact inventories
  were updated from the live merged tree (not weakened), and a new mutation test
  proves removal of the added legacy-caller check is rejected. PR1 static check
  passes with 0 findings and its mutation suite passes 16/16.
- Feature-unification defect found and fixed: isolated `cargo test -p axfile` passed,
  but the first locked/offline workspace run enabled `axsync/multitask` through other
  workspace members and aborted in `EventSource::drop` with `current task is
  uninitialized`. The feature-sensitive alias had turned the event registry's short,
  non-blocking internal lock into a scheduler mutex. The registry now explicitly uses
  `axsync::spin::SpinNoIrq`, while the public ordered-pair helper retains the caller's
  `axsync::Mutex` type. A private address-order primitive keeps canonical acquisition
  order testable without changing that public API. The guard and a mutation test bind
  the feature-invariant registry-lock requirement.
- The first RV64 production build after the broad prototype correctly exposed an
  `E0308` public-helper type mismatch in `fd_pipe.rs`. The broad prototype was not
  retained; the final narrow implementation above preserves the pipe mutex type.
  Rebuilt RV64 and LA64 production kernels both exited 0, including objcopy/wrapping.
- Passing checks on the final staged content:
  - `make pr2-check`: static 0 findings, mutations 24/24, axfile 25/25.
  - Explicit `axsync/multitask` axfile regression run: 25/25.
  - Locked/offline workspace tests excluding `axfs`: exit 0; this is the exact run
    that previously aborted, so the regression is reproduced and closed.
  - PR1 static and mutation guards: 0 findings and 16/16.
  - Targeted axfile formatting, staged/unstaged diff checks, and whitespace check:
    exit 0; there are no unmerged or unstaged paths.
  - Official-feature `make kernel-rv` and `make kernel-la`: both exit 0.
- Visible pre-existing failures retained: `make unittest_no_fail_fast` still exits 2
  only at the existing axfs `test_devfs_ramfs() failed: NotFound`; the G012 mutation
  baseline remains 25/26 as recorded in the PR1 checkpoint. Neither is called PASS.
- Test side effect: the host workspace run again regenerated the checked-in epoll
  layout. The exact four-line change was restored; `ctypes_gen.rs` has no diff.
- Unsafe/dependency notes: no unsafe block was added, no dependency/version/checksum
  was added, and the lock fix uses existing unconditional `axsync` re-exports.
- AI/model/Goal-mode use: primary Codex agent; no subagent. Semantic choices were made
  from the production target failure plus the full workspace reproducer, not from
  testcase-name or path hardcoding.
- Remaining risks: commit ancestry must be checked; the unified suite and PR3 still
  require semantic convergence, and all known baseline/final-profile failures remain
  open until the canonical runner proves otherwise.

PR2 was then committed as `acc6b604eb8132bec8a26900aeb8869fea5feebc`, an
explicit merge whose parents are PR1 integration commit `aa9072df...` and exact PR2
source `7b16e147...`. Both PR1 and PR2 source tips pass `merge-base --is-ancestor`.

## 2026-07-15T11:09:18Z - canonical test suite staged and focused checks passed

- Pre-commit HEAD: `acc6b604eb8132bec8a26900aeb8869fea5feebc`; MERGE_HEAD:
  `0c2a3cffca2fa7d276ea8d0ec3524df8fc0669ba`.
- Action: `git merge --no-ff --no-commit
  source-test-suite/test/unified-local-test-suite` produced one textual conflict,
  in `Makefile`. Resolution retains PR2's `pr2-check` prerequisite on both unittest
  targets and adds the suite's isolated `test-list`, `test-checks`, `test-unit`,
  `test-quick`, and `test-baseline` entries. No other text conflict occurred.
- Canonical ownership reconciliation: the PR1/PR2 guard and mutation files were moved,
  not copied, from `scripts/` into semantic paths under `test/checks/` and `test/unit/`.
  Their commands, fixed inventories, manifest IDs, exact method counts, migration map,
  and documentation totals were updated. `scripts/` retains no second copy. The
  compatibility `pr2-check` Make target invokes the isolated canonical implementations
  plus the axfile Rust tests.
- Canonical inventory now contains 50 cases: 18 checks, 22 unit suites with 532 exact
  methods, 8 baseline cases, and 2 architecture-specific official cases. `quick` plans
  40 and `baseline` plans 48. Both the runner's fixed anti-injection inventory and the
  test-asset guard's independent fixed path inventory were extended; neither dynamic
  discovery nor unknown-case acceptance was introduced.
- Passing focused validation on staged content:
  - canonical `--list`: exit 0, 50 registered cases, including each merged guard once;
  - the same absolute runner invocation from `/tmp`: exit 0 with the same plan;
  - Linux boundary check and mutation suite: PASS and 16/16 exact-bound methods;
  - file-object/event check and mutation suite: PASS and 24/24 exact-bound methods;
  - runner lifecycle suite: 133/133 exact-bound methods;
  - test-asset integrity check and suite: PASS and 36/36;
  - evaluator/parser integrity 23/23, official parser 106/106, LTP summarizer 20/20,
    and failure reporter 8/8;
  - all three shell entry points pass `bash -n`; staged whitespace and unmerged-path
    checks pass, with no unstaged paths.
- The first runner-unit attempt failed 14/133 because the newly registered four IDs
  were intentionally rejected by the still-source-fixed runner inventory. After adding
  the same four IDs and pinned 16/24 method counts to that independent inventory, the
  complete 133-test rerun passed. The failed attempt remains recorded here.
- Staged tree: `26914d7e73eed61b493ac6562bd0e564844728e7`; suite source tree:
  `d136d3f9bad0dd77799c84ad7e4f2dd05cb61bab`. The difference is the documented
  PR1/PR2 preservation, Makefile resolution, canonical registrations, and current-count
  documentation.
- Failures/risks not normalized: clean-tree `quick` has not yet run because the runner
  correctly refuses a merge-in-progress worktree. The suite source documentation also
  records historical production baseline failures and a duplicate BusyBox identity in
  its then-current official image snapshot; these are risk evidence, not a current
  verdict, and must be re-evaluated after the clean merge commit.
- Unsafe/dependency notes: test/framework paths only; no production unsafe or dependency
  change is introduced by this merge-stage reconciliation.
- AI/model/Goal-mode use: primary Codex agent; no subagent. Fixed inventories were
  consciously extended from the four reviewed semantic implementations rather than
  relaxed to accept arbitrary files.

The suite was committed as `126e21a402dc773b1057fcb83f204d11b62d3a4b`, an
explicit merge whose parents are PR2 integration commit `acc6b604...` and exact suite
source `0c2a3cff...`. All first three source tips pass `merge-base --is-ancestor`.

## 2026-07-15T11:14:15Z - first clean canonical quick completed non-passing

- Command: `python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py
  --profile quick --output-dir
  test/output/integration-suite-126e21a4-quick-1`.
- Attribution: clean, stable runner commit
  `126e21a402dc773b1057fcb83f204d11b62d3a4b`; provenance stability is true.
- Result: exit 1 / `FAIL`, duration 259.665 s. Planned 40, executed 40,
  completed 40, PASS 38, FAIL 2, TIMEOUT 0, CRASH 0, INFRA_ERROR 0,
  NOT_RUN 0, unknown 0. Summary:
  `test/output/integration-suite-126e21a4-quick-1/summary.json`.
- The two visible failures are `check.kernel_state_backed_semantics` and its
  exact-bound 36-method unit suite's current-tree positive assertion. The check reports
  ten absent RR skipped-task aging contracts in `vendor/axsched/src/round_robin.rs`:
  `skipped_rounds`, scheduling class/effective priority/key, normal/realtime class
  constants, skipped/reset methods, and select/skip update calls. All 39 other unit
  methods in that suite context executed; one failed because the real-tree guard failed.
- This exactly matches the unified-suite source's documented unchanged production
  blocker. It is not called PASS and is not erased by later attempts. PR1, PR2, runner,
  parser, result-integrity, and every other quick item explicitly passed.
- The Git worktree remained clean after the run. RV backing-image SHA-256 was rechecked
  as `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99`;
  no official adapter or overlay was launched in this profile.
- Rationale: continue the independent PR3 and governance merge work first, then repair
  the preserved production blocker against its semantic tests before final gates.

## 2026-07-15T11:58:16Z - PR3 merge resolved and pre-commit runtime probes completed

- Pre-commit HEAD: `126e21a402dc773b1057fcb83f204d11b62d3a4b`; MERGE_HEAD:
  exact PR3 source `7562ea69770501769fcf5c163a0e95343ffd2e2b`.
- Action: `git merge --no-ff --no-commit
  source-pr3/ci/pr3-competition-semantic-evidence` produced conflicts in README,
  the root evaluator wrapper, two retired `scripts/` LTP files, the scheduler and
  syscall guards/units, and the evaluation failure reporter. All are resolved in the
  index; conflict-marker, unmerged-path, and staged whitespace checks exit 0.
- Canonical ownership resolution: `test/` remains the only test/evidence owner. The
  PR3 collector, renderer, protocol parser, schema, manifest, QEMU setup, guard, and
  four unit suites live under `test/evidence`, `test/checks`, and `test/unit`.
  The source branch's alternate `scripts/` test copies are absent. Root `run-eval.sh`
  remains a thin `exec` wrapper. CI and compatibility Make targets call only explicit
  canonical profiles.
- The canonical inventory is now 59 cases: 19 static checks, 26 unit suites with 681
  exact-bound methods, four specialized evidence adapters, eight baseline cases, and
  two official cases. `quick` plans 45; `evidence-required` plans four; `baseline`
  plans 57. `--list` exits 0 both from the repository and through an absolute runner
  path from `/tmp`.
- Honest integration defects found and fixed before commit:
  - six source-branch `.log` fixtures were ignored and therefore absent from the PR3
    commit; equivalent tracked `.txt` fixtures now make the suite self-contained;
  - generated evidence output directories were initially validated as pre-existing
    input files; commands now use safe runtime-relative output paths;
  - isolated `-I -S` execution initially could not import sibling evidence modules;
    both entry points now bind their resolved module directory explicitly;
  - source-era campaign IDs conflicted with canonical semantic naming and were removed
    from live paths/output; retired-path assertions construct the legacy names without
    reintroducing them into canonical content;
  - the merged scheduler suite has 41, not 36, methods because five PR3 RR/vendor
    regression tests are retained and exactly bound;
  - a decorated Linux-only process test violated the canonical AST identity contract;
    its platform skip is now inside the method, leaving all 75 method identities fixed.
- Passing pre-commit checks on final indexed content:
  - competition workflow guard and mutations: 0 findings and 33/33;
  - semantic evidence pipeline: 75/75; byte protocol: 27/27; QEMU setup: 9/9;
  - scheduler semantics: 0 findings and 41/41; syscall boundary: 0 findings and 26/26;
  - test assets: 0 findings and 36/36; suite runner lifecycle: 133/133;
  - evaluator supervision/parser: 0 findings and 23/23; failure reporter 8/8;
    aggregate compliance: 0 findings and 7/7;
  - generated schema exact check and all relevant shell syntax checks exit 0.
- Real build/runtime evidence, without promotion overclaim:
  - `make pr3-smoke-kernel-rv` and `make pr3-smoke-kernel-la` both exit 0 and produce
    fresh kernels from the merged tree.
  - LA raw smoke, supervised by the PR3 process-group supervisor and run on exact
    QEMU 9.2.4, exits 0 with one ordered set of all write/getpid/uname/user/harness/
    shutdown markers and no fatal marker.
  - RV raw smoke also exits 0 with the complete ordered marker protocol, but the host
    binary is QEMU 6.2.0. This is only a pre-commit compatibility probe and is explicitly
    **not** accepted as the required PR3 RV evidence. A verified dual-target QEMU 9.2.4
    installation is still required for the canonical post-commit gate.
- The source archive `/root/OrayS-pr3/build/qemu-source/qemu-9.2.4.tar.xz` is locally
  available; the canonical setup script will independently enforce the pinned size,
  SHA-256, target list, configure profile, stamp, and installed binary hashes.
- AI/model/Goal-mode use: primary Codex agent; Goal mode remains active; no subagent
  has been used. Independent subagent review remains reserved for the final clean-tree
  gate required by the goal.
- Remaining risks: commit/ancestry verification, exact dual-target QEMU 9.2.4 build,
  clean canonical quick/evidence/baseline/official runs, known axfs/format/clippy and
  official-image identity risks, workflow-starter governance installation, independent
  final review, remote freshness, push, and promotion all remain outstanding.

## 2026-07-15T12:06:32Z - PR3 committed and workflow governance installation started

- PR3 was committed as `03269960bb440e45f6e97999c20532cb3977c9be` with
  parents `126e21a402dc773b1057fcb83f204d11b62d3a4b` and exact PR3 source
  `7562ea69770501769fcf5c163a0e95343ffd2e2b`. All four selected source tips
  pass `merge-base --is-ancestor` against the integration branch.
- The workflow ZIP was extracted only into the archive staging directory
  `workflow-starter-stage/orays-workflow-starter`; it was not unpacked over the
  repository. Its root `AGENTS.md` replaced the temporary PR1 policy through an
  explicit file patch, was then read completely from the repository, and has the
  same SHA-256 as the staged source:
  `b2fb55e8dd790168e95044a4b503b1f149b9088f802dd30c7b3744e57864bf93`.
- The starter PR template and development-log README/template were added through
  explicit patches and independently hash-compared to their staged sources.
- Long-term policy is now active for the remainder of the integration: `test/`
  remains the sole test owner; non-pass states remain visible; final evidence must
  be clean-HEAD attributed; and an independent final reviewer is required.
- No production source, test verdict, dependency, toolchain, or official image was
  changed at this checkpoint. The worktree is intentionally dirty only with the
  uncommitted governance installation until its dedicated commit is made.
- AI/model/Goal-mode use: primary Codex agent; Goal mode remains active; no subagent
  has yet been used. The final independent reviewer remains outstanding.
- Remaining risks: governance diff/commit, the known scheduler/axfs/format/clippy
  baseline defects, exact QEMU 9.2.4 dual-target evidence, both official image runs,
  full gate, independent review, remote freshness, and safe promotion.

## 2026-07-15T12:18:35Z - governance reconciliation and focused validation completed

- Installed the starter PR template and development-log README/template without
  overwriting any prior repository log. Added concise README files for active/completed
  plans, decisions, and references, plus an active integration plan and this repository
  development log. The temporary PR1 policy's dependency direction, ABI/errno
  invariants, `UserProcess` ownership, and first-milestone non-goals are preserved in
  those records; they no longer override the long-term root policy.
- Normalized the starter `AGENTS.md` example's single trailing space after first
  verifying exact staged-source identity. This semantic no-op is required so
  `git diff --check` passes; no policy text was weakened.
- Reconciled the image-path contract across Makefile, canonical manifest/runner,
  official adapter, documentation, guard, and existing exact-bound unit suites.
  Architecture-specific image variables remain highest priority,
  `ORAYS_WORKSPACE_ROOT`/`TESTSUITE_DIR` are supported overrides, and the final
  fallback is `sdcard-{rv,la}.img` in the repository parent. Missing files remain
  explicit infrastructure errors; backing images and parser semantics were untouched.
- A first rerun of the 23-method evaluator integrity suite failed two subtests. The
  adapter selected the correct parent files but passed a lexically equivalent
  `repo/../sdcard-*.img` path; the new test incorrectly required a pre-canonicalized
  string. The assertion now resolves the captured path before comparison. This changes
  only the test oracle, not path selection or missing-file policy. The complete rerun
  passed 23/23.
- Passing focused validation on the final governance worktree:
  - `git diff --check`, adapter `bash -n`, canonical `--list`, and the evaluation
    static guard all exit 0; list still registers exactly 59 cases;
  - evaluator integrity unit suite: 23/23;
  - test asset guard and unit suite: 0 findings and 36/36;
  - suite runner lifecycle unit suite: 133/133 in 200.025 s.
- No production syscall/ABI behavior, dependency, toolchain, `Cargo.lock`, official
  image, parser status mapping, blacklist, or `unsafe` block changed in this
  governance reconciliation.
- AI/model/Goal-mode use: primary Codex agent; Goal mode remains active; no subagent
  or human review has yet occurred.
- Remaining risks: dedicated governance commit, clean canonical quick and baseline
  repair loop, exact dual-target QEMU 9.2.4, RV/LA official/full runs, independent
  reviewer, remote freshness, and safe promotion.

## 2026-07-15T12:30:49Z - first post-governance canonical quick completed non-passing

- Clean/stable commit: `764211c5c221d7c64d57a658eac05fe7c5cee38c`; start/final
  revision and cleanliness match; provenance stability is true.
- Command: `python3 -I -S -B -X pycache_prefix=/dev/null test/run_suite.py
  --profile quick --output-dir test/output/integration-764211c5-quick-1`.
- Result: exit 1 / `FAIL`, duration 515.911347 s; planned/executed/completed
  45/45/45; PASS 42, FAIL 2, TIMEOUT 1; no crash, infrastructure error,
  NOT_RUN, or unknown status.
- `check.file_object_event_core` and the 24-method unit suite's current-tree
  assertion fail because Test CI no longer runs `make unittest_no_fail_fast`;
  quick alone does not execute the axfile Rust semantics wired by `pr2-check`.
- `unit.suite_runner` timed out at 302.122305 s under its 300-second contract
  while sharing a 2-core host with `ninja -j2` QEMU compilation. This remains
  TIMEOUT; a non-concurrent rerun is required.
- Summary: `test/output/integration-764211c5-quick-1/summary.json`.

## 2026-07-15T12:31:57Z - required CI file-object regression coverage restored

- Narrow repair: `.github/workflows/test.yml` keeps the canonical quick step and
  adds a required `make unittest_no_fail_fast` step. This restores execution of
  the PR2 `pr2-check` static/mutation/axfile Rust tests plus existing workspace
  units instead of weakening the guard or treating Python mutation coverage as a
  substitute for executable Rust semantics.
- Focused validation on the dirty repair worktree:
  - file-object/event guard: PASS with 0 findings;
  - exact-bound file-object/event unit suite: 24/24;
  - PR3 competition workflow guard: PASS with 0 findings;
  - exact-bound PR3 workflow unit suite: 33/33;
  - `git diff --check`: exit 0.
- No production source, test count, parser, status mapping, skip/blacklist,
  dependency, toolchain, lockfile, image, or unsafe block changed.
- The QEMU setup build remains an ignored-output background process; the next
  canonical quick will wait until that CPU-intensive build ends.
- Remaining risks: clean quick rerun, baseline repair loop,
  exact QEMU evidence, official/full gates, independent review, and promotion.

## 2026-07-15T12:34:57Z - CI repair committed and exact dual-target QEMU verified

- CI repair commit: `761da910e8573e0f428846ce15c1fa863759d219`
  (`ci: restore file-object Rust regression coverage`); the worktree was clean
  immediately after commit.
- The supervised source build completed with exit 0, followed by an independent
  `bash test/evidence/setup_qemu.sh --verify-only
  /tmp/orays-pr3-qemu-9.2.4 build/qemu-source` exit 0.
- Both installed binaries report exactly `QEMU emulator version 9.2.4`:
  - RISC-V64 SHA-256:
    `00bf7520524a45d38508fe65a5b8f476b2db0c693a4d9e89547e2a9e38178878`;
  - LoongArch64 SHA-256:
    `2b1fead12bd7c7116fa10db04b5bcd8da4c2ac4f64b8c0f9a4ca436457cb6353`.
- The installation stamp records the fixed source archive SHA-256
  `f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a`,
  size 134782772 bytes, target list
  `riscv64-softmmu,loongarch64-softmmu`, and the fail-closed configure profile.
- This proves the required emulator toolchain is available; it is not itself a
  semantic-evidence or official-suite PASS. Those runs remain pending.

## 2026-07-15T12:41:34Z - clean repaired-candidate quick passed

- Candidate: clean/stable commit
  `05b123266fe3695bc660c2cd281a56d2ac44ccea`; start/final revisions match,
  both dirty flags are false, and provenance stability is true.
- Canonical command used profile `quick` and the new output directory
  `test/output/integration-05b12326-quick-1`.
- Result: exit 0 / `PASS`, duration 279.598694 s;
  planned/executed/completed 45/45/45, PASS 45, every non-pass bucket 0.
- This closes only the earlier CI-coverage and resource-contention quick findings;
  it is not baseline, official, full, review, or promotion evidence.

## 2026-07-15T12:57:59Z - first clean baseline completed non-passing

- Candidate and provenance: the same clean/stable `05b123266fe...`; canonical
  output directory `test/output/integration-05b12326-baseline-1`.
- Result: exit 2 / `INFRA_ERROR`, duration 890.07075 s;
  planned/executed/completed 57/56/57; PASS 50, FAIL 6, INFRA_ERROR 1;
  TIMEOUT/CRASH/NOT_RUN/unknown all 0.
- Explicit non-pass cases:
  - `evidence.host`: the static competition guard emitted its valid one-line PASS,
    but its manifest incorrectly also required a unittest count;
  - `evidence.aggregate`: downstream non-pass because host evidence was incomplete;
  - `baseline.cargo_format`: four pre-recorded rustfmt drifts;
  - `baseline.workspace_unit_tests`: axfs FAT integration test still addressed the
    deliberately removed fake `/dev/foo/bar` node and returned `NotFound`;
  - `baseline.clippy_default`: x86 host lint attempted the target-only shell and
    failed on 42 architecture-specific fields/imports;
  - `baseline.clippy_riscv64`: child exit 0, but the runner mistook clippy source
    display `5408 ~ ... timeout ...` for runtime timeout evidence;
  - `baseline.clippy_loongarch64`: INFRA_ERROR because host clang 14 lacked the
    required LoongArch frontend target.
- Positive RV64/LA64 semantic evidence and kernel/submission builds in this run do
  not override the seven non-pass cases.

## 2026-07-15T13:22:08Z - baseline repair batch focused validation completed

- Narrow contract repairs:
  - removed `min_tests` only from the static competition guard classifier; its
    unittest companion retains `min_tests: 1`, with existing exact-bound tests;
  - accepted clippy's `~` source-display marker in the bounded exit-code parser and
    added the observed line to an existing parser test without weakening real
    timeout/failure detection;
  - excluded `arceos-shell` only from default host workspace clippy; both explicit
    RV64 and LA64 clippy commands continue to lint it;
  - applied rustfmt only to the four reported drift areas;
  - replaced the stale axfs test's deleted fake `/dev/foo/bar` dependency with the
    real `/dev/zero` node and a mount-normalization path that does not require fake
    capabilities. Production `RootDirectory` behavior is unchanged.
- Rejected approach: an initial production-side attempt canonicalized every path
  before mount selection. Focused tests showed that this changes existing `..`
  traversal across mount boundaries, so the production change was fully removed;
  no form of that semantic broadening is present in the reviewed diff.
- Tool capability work was kept outside the repository. clang 14 and Ubuntu clang
  15 both lacked a LoongArch backend. clang 21.1.8 was then installed on the host;
  `-print-targets` lists `loongarch64`, and an empty C translation unit with
  `--target=loongarch64-unknown-none -fsyntax-only` exits 0. The temporary PATH
  shim resolves to `/usr/bin/clang-21`, SHA-256
  `82481792aef943c1750ae5fd71e5a5737212741337debd0fe5d28bd82dd018e9`;
  the system default compiler and repository toolchain files were not changed.
- Focused validation on the dirty repair worktree, all exit 0 unless explicitly
  described otherwise:
  - suite-runner unit suite 133/133; competition workflow unit suite 33/33;
    semantic-evidence unit suite 75/75;
  - `make unittest_no_fail_fast`, including axfs FAT `/dev/zero` behavior and
    executable axfile regressions;
  - `cargo fmt --all -- --check`;
  - `make clippy`, `make clippy ARCH=riscv64`, and clang-21-backed
    `make clippy ARCH=loongarch64`;
  - current-manifest host 8/8, RV64 3/3, and LA64 3/3 semantic-evidence cases,
    including both exact-QEMU ABI smokes; aggregate 14/14 and report rendering;
  - competition semantic-evidence guard and test-asset integrity guard, both
    PASS with 0 findings.
- The first aggregate attempt after changing the manifest exited 2 because the
  existing RV64/LA64 results had the old manifest identity. That fail-closed stale
  result was retained; both architecture shards were regenerated before aggregate
  subsequently passed. None of these dirty-worktree checks is called a canonical
  profile verdict.
- No dependency, `Cargo.lock`, repository toolchain, image, blacklist, status
  mapping, production unsafe, syscall/ABI/errno behavior, or test method count was
  changed. The next step is a scoped commit followed by fresh clean quick/baseline.

## 2026-07-15T13:25:29Z - first baseline repair committed

- Commit `1c0e3ba0396fcd9d8dde2ef6bb1cfc34e32647f5`
  (`test: repair baseline evidence contracts`) contains the reviewed narrow repair,
  four bounded format corrections, this plan, and this log.
- Cached/committed whitespace checks passed; the post-commit worktree was clean.
  No push, main promotion, dependency, lockfile, repository toolchain, image, generated
  evidence, blacklist, unsafe, ABI, or errno change occurred.

## 2026-07-15T13:31:17Z - clean repair-candidate quick passed

- Candidate: clean/stable `1c0e3ba0396fcd9d8dde2ef6bb1cfc34e32647f5`;
  start/final commit match, both dirty flags are false, and provenance stability is true.
- Canonical quick exited 0 / `PASS` in 280.783039 s with
  planned/executed/completed 45/45/45, PASS 45, and every non-pass bucket 0.
- Summary: `test/output/integration-1c0e3ba0-quick-1/summary.json`.
  This closes quick only; it is not baseline, official, full, review, or promotion proof.

## 2026-07-15T13:43:51Z - clean repair-candidate baseline retained one failure

- Candidate and provenance are the same clean/stable `1c0e3ba0...`; canonical baseline
  exited 1 / `FAIL` in 716.422999 s with planned/executed/completed 57/57/57,
  PASS 56, FAIL 1, and all other buckets 0.
- All seven non-pass cases from `05b12326` now pass. The sole remaining case is
  `baseline.workspace_unit_tests`; its child exited 0, but the strict cargo contract
  returned `cargo test output contains explicit non-pass evidence`.
- The first concrete match was the raw `panicked` report from the successful Rust
  `metadata_rejects_more_than_six_arguments - should panic ... ok` lifecycle identity.
  That is still a real baseline FAIL; a zero child exit did not override the parser.
- Summary: `test/output/integration-1c0e3ba0-baseline-1/summary.json`.

## 2026-07-15T14:09:32Z - layered cargo-contract repair focused validation

- Reclassifying the retained raw baseline logs after each bounded fix exposed all
  layers hidden behind the same generic result string; no old verdict was rewritten:
  1. the successful `should_panic` case emitted its expected raw panic report;
  2. Makefile command echo contained the option `--no-fail-fast`;
  3. the PR2 prerequisite emitted an exact successful identity-bound Python unittest
     block on stderr;
  4. block 49 contained a real ignored axns doctest with an outdated two-argument macro
     example. In-memory replay changing only that lifecycle to executed `ok` then
     classified the retained log as 55 blocks / 73 tests PASS.
- The cargo parser now records, without altering raw logs, only:
  - exact identity-bound unittest blocks whose dots, planned/started/executed/stopped,
    `Ran N tests`, separator, and terminal `OK` all agree and are positive;
  - a bounded panic report whose thread identity exactly equals one successful
    `- should panic ... ok` lifecycle and whose body contains no unknown/non-pass,
    failure, timeout, crash, or extra panic marker;
  - the exact command-option token `--no-fail-fast` during failure-word scanning.
  Every unmatched line remains available to the original fail-closed classifiers.
- Existing method
  `test_cargo_test_accepts_trusted_build_diagnostics_on_stderr` now includes positive
  fixtures plus mutations for count mismatch, inserted TCONF/TBROK/TFAIL/ENOSYS,
  timeout, unknown status, ordinary failure, and extra panic before/after the bounded
  report. All mutations remain nonzero; no method-count or manifest-count change was
  used.
- The axns Linux/macOS examples now use the real three-argument macro form and are no
  longer marked `ignore`; `cargo test -p axns --doc -- --nocapture` exits 0 with
  2 passed, 0 failed, 0 ignored.
- Final focused evidence on the exact dirty worktree:
  - the affected parser unit method: 1/1, including all subcases;
  - complete suite-runner identity-bound regression:
    planned/started/executed/stopped 133/133/133/133, `Ran 133 tests`, exit 0,
    193.694 s;
  - `git diff --check`: exit 0.
- These are focused dirty-worktree results, not a canonical baseline PASS. The repair
  still requires a scoped commit followed by fresh clean quick and baseline runs.

## 2026-07-15T14:12:22Z - cargo evidence contract repaired and clean gates passed

- Commit `0c6c2f58afad9a83b3be74da78a0e539e0a43fe3` contains the reviewed
  identity-bound cargo/unittest/expected-panic repair and the executable axns doctest.
- Clean/stable quick passed 45/45 in 289.904906 s; clean/stable baseline passed 57/57
  in 771.339618 s. Their summary SHA-256 values are respectively
  `69b0079310482c75ccea773bb381c7d7ff96a6c78fffcbdc2feb322381b16012` and
  `544d35e388217950a73099b4886e88a1da1733b9dbc2c68f34f140d0601b333f`.
- The first official RV launch failed honestly before guest boot because the original
  exact QEMU 9.2.4 smoke build lacked the required `user` netdev. A second external
  QEMU 9.2.4 prefix was built from the same pinned source archive with system libslirp;
  both target binaries, `qemu-img`, version and backend inventory were verified without
  changing repository source or the fixed PR3-smoke profile.
- The next RV run completed all 24 guest groups and normal shutdown but remained
  `INFRA_ERROR`: strict parsing first exposed normal ANSI controls. Immutable-log replay
  then drove a generic fail-closed repair; old summaries were not rewritten.

## 2026-07-15T16:30:57Z - structured official output repair committed

- Commit `7eaf3c1c1e164115de4c41cbf0f2f569d621e875` recognizes only bounded ANSI
  clear/style sequences while retaining printable failure payload and rejecting
  malformed controls. Contextual scanning distinguishes benign diagnostics from real
  TFAIL/TBROK/TCONF/ENOSYS/nonzero/timeout/panic/trap evidence.
- The generic producer owns exactly one group frame and emits PASS only after child
  status 0. Nonzero/timeout/launch/preparation/malformed-frame outcomes remain failing;
  specialized LTP, BusyBox and libctest producers were not weakened. No testcase,
  group, input or host-path-specific success rule was added.
- Exact focused checks before commit: official parser 106/106, suite runner 133/133,
  evaluator-integrity mutations 23/23, static guard 0 findings, repository rustfmt,
  diff check, and both architecture kernel builds all passed.

## 2026-07-15T16:36:27Z - final-candidate quick and baseline passed

- On clean/stable `7eaf3c1c...`, quick passed 45/45 in 295.325320 s; summary SHA-256
  `5bac5095f9527a29709d30c44134fb81e1be5b955a43c0a9357f43174410d846`.
- The same candidate baseline passed 57/57 in 826.131206 s; summary SHA-256
  `a9b11b36182e64e2b092df47eda799aef279c9390f0ab5538237d23204af812c`.
- Both runs record matching start/final commit, clean start/final status, and stable
  provenance. Neither result was extrapolated to official/full.

## 2026-07-15T18:17:30Z - final-candidate official RV completed non-passing

- All 24/24 guest groups ran and the guest shut down normally; child status was 0.
  Canonical runner exit 2 / `INFRA_ERROR`, 4956.178130 s, because strict semantic
  parsing returned `ERROR`.
- Exactly two structural errors identify the duplicate BusyBox plan row
  `echo "bbbbbbb" >> test.txt`, once per libc group. Additionally 112 semantic failure
  records preserve forbidden statuses, LTP internals, timeout, panic/trap, libctest and
  an explicit cyclictest-musl group failure.
- Aggregates: LTP musl 981/19/0, LTP glibc 987/13/0, libctest musl 217/0,
  libctest glibc 179/38 with 2 timed-out entries; cyclictest-musl exited 137 after an
  explicit 900 s timeout. Summary SHA-256:
  `5d95f5b8dd3c3e5210e1dd178557486774a9225961f49bbf8225d17fa0d7a5c2`.

## 2026-07-15T19:40:50Z - final-candidate official LA completed non-passing

- All 24/24 guest groups ran and the guest shut down normally; child status was 0.
  Canonical runner exit 2 / `INFRA_ERROR`, 4880.545466 s, again with the same two
  duplicate BusyBox structural errors plus 158 preserved semantic failure records.
- Aggregates: LTP musl 975/25/0, LTP glibc 984/16/0, libctest musl 217/0,
  libctest glibc 179/38 with 2 timed-out entries. Both cyclictest groups returned
  status-bound PASS. Summary SHA-256:
  `551f1e3dd221d3b14cd498aa6b322c9fa0bf6258a5f2805d888f1f88d6e70d9b`.

## 2026-07-15T22:34:19Z - exact full/all gate completed and promotion blocked

- Exact full/all on clean/stable `7eaf3c1c...` ran 59/59 cases in 10194.568365 s:
  57 PASS and 2 `INFRA_ERROR`; exit 2. Start/final commit and dirty inventories match,
  and `runner_provenance_stable` is true.
- Both official cases again completed 24/24 groups and normal shutdown. Full-run RV
  retained 2 structural errors + 117 failures (LTP musl 981/19/0, glibc 985/15/0,
  cyclictest-musl timeout); LA retained 2 structural errors + 156 failures (LTP musl
  975/25/0, glibc 984/16/0, both cyclictest groups PASS).
- Summary SHA-256 is
  `1c8906d0b2e4fafbdbb69e9396b8b655156c633aa8f32b1661bef604bc51202a`.
  The run left no overlay. Both image hashes equal their preflight values, and both
  immutable guest logs end in normal architecture-specific shutdown markers.
- Required promotion conditions are false. `main` must remain unchanged; the safe
  terminal state is `BLOCKED`, not a parser waiver, image mutation, or fake PASS.

## 2026-07-15T22:44:53Z - docs-only closeout quick passed

- Commit `a00243d9ae41e516dfdd766e59271875cf086ebb` changed only this development
  log. Its clean/stable canonical quick passed 45/45 in 307.655291 s; summary SHA-256
  `321cef6207ad01fd8b49ad16c62681b80fb932c9dbf0c5a8bb1980574239eaee`.
- This docs-only quick validates the closeout commit; it does not relabel or replace the
  parent source commit's RV/LA/full non-pass evidence.

## 2026-07-15T22:45Z - independent read-only review found one Major and one Minor

- A separate Codex reviewer subagent inspected `921171ac..a00243d9` read-only and made
  no file, commit or remote change. Result: 0 Blocker, 1 Major, 1 Minor.
- Major: PR3 introduced the freestanding RV64/LA64 semantic-smoke `syscall3`, exported
  `memset`/`memcmp`, and their raw-pointer call sites without the code-level invariants,
  caller responsibilities and test basis required by root `AGENTS.md`. Existing dual
  runtime PASS is test evidence but cannot replace the missing safety contract.
- Minor: the active integration plan still described the pre-`0c6c2f58` cargo-contract
  phase and omitted the completed but non-passing official/full gates.
- No second blocker/major was found: the reviewer confirmed all four source tips remain
  ancestors, one canonical `test/` owner, no conflict residue, no ABI/errno/resource/
  lock regression evidence, fail-closed official producer/parser behavior, immutable
  backing images through disposable overlays, and consistent CI required-lane policy.
- Repair is intentionally documentation/contract-only: explain why this freestanding
  probe cannot use a safe libc wrapper; bind each architecture's syscall registers and
  pointer lifetime/access rules; document `UtsName`, `memset` and `memcmp` validity,
  aliasing and zero-length contracts; add local `// SAFETY:` justifications; and update
  the active plan. No unsafe expression is added, moved or removed, and no syscall,
  ABI, errno, control flow, parser or image changes.
- The prior RV64/LA64 smoke runs establish historical test basis only. This repair must
  be committed, followed by fresh clean dual-architecture smoke and full/all; reviewer
  blocker/major is not considered cleared until that evidence and follow-up review.

## 2026-07-15T23:00Z - reviewer repair committed and fresh dual-architecture smoke passed

- Commit `e115f916ed7cc816ebe6c0504fc3348be4da91ee` adds the missing code-level
  unsafe contracts and updates the integration records. The semantic-smoke executable
  instructions, syscall numbers, register bindings, ABI, errno handling, control flow
  and expected markers are unchanged.
- Pre-commit `make pr3-smoke-kernel-rv` and `make pr3-smoke-kernel-la` both exited 0;
  `rustfmt --edition 2024 --check user/shell/runtime_smoke/semantic_smoke.rs` and
  `git diff --check` also exited 0.
- Fresh clean/stable `evidence-runtime` passed 1/1 on each architecture. RV64 completed
  in 100.517584 s with summary SHA-256
  `6247f2e93a3156069fd891eaa3b49411a12df82f42ec32adeae2c4997aedc30e`;
  LA64 completed in 94.628248 s with summary SHA-256
  `3b3f48c67611376945636f220daae00b728c263bbd2d7e6e6df17e112d0864f7`.
  Both summaries record the same start/final commit, clean start/final status,
  stable provenance, one completed PASS and zero fail/timeout/crash/infra.
- These runs close the requested affected-smoke rerun only. The reviewer Major remains
  open until a fresh full/all result and the same reviewer's follow-up disposition.

## 2026-07-16T02:11Z - post-review full/all completed and remained blocked

- The exact fresh full/all rerun used clean/stable
  `74f55223c3831e3f5cca45578c064ea45193fbff` with the same pinned QEMU 9.2.4,
  clang-21, isolated Python and immutable backing images. It exited 2 / `INFRA_ERROR`
  after 11142.025951 s: planned/executed/completed 59/59/59, 57 PASS and 2 INFRA_ERROR.
  Start/final commit and empty status inventories match; provenance is stable. Summary
  SHA-256 is `540f7d1e543659111e44016a6b1c78fc31f5a29908b8e41f2b204f58c5b90ceb`.
- RV64 completed 24/24 groups, child status 0 and normal shutdown at 4953.506215 s.
  It retained 2 BusyBox duplicate errors and 119 failure records: forbidden-status 70,
  LTP internal 35, timeout 4, panic/trap 4, LTP summary 2, LTP nonzero 2, libctest 1,
  official-group 1. Aggregates were LTP musl 981/19/0, glibc 985/15/0, libctest musl
  217/0, libctest glibc 179/38 plus 2 timed out; cyclictest-musl again timed out at 900 s.
- LA64 completed 24/24 groups, child status 0 and normal shutdown at 5357.281894 s.
  It retained 2 BusyBox duplicate errors and 161 failure records: forbidden-status 95,
  LTP internal 54, timeout 4, panic/trap 2, LTP summary 2, LTP nonzero 2, libctest 1,
  official-group 1. Aggregates were LTP musl 975/25/0, glibc 984/16/0, libctest musl
  217/0, libctest glibc 179/38 plus 2 timed out. Unlike the first full, this fresh run's
  cyclictest-glibc timed out at 900 s; the worse result is retained rather than selecting
  the older PASS.
- Retained RV stdout/stderr SHA-256 are `74dd190b...01cf` / `f47a03c0...db81`;
  LA values are `7e1df41b...26da` / `510e961d...5864`. Both image hashes match
  preflight, both overlays were removed, and the repository remained clean.
- The requested fresh smoke/full evidence now exists. The review Major is still open
  until the same independent reviewer inspects the repair and evidence; promotion is
  independently blocked by the unchanged duplicate plan and real semantic failures.

## 2026-07-16T02:30:55Z - follow-up review cleared findings; promotion remained blocked

- The same independent read-only Codex reviewer inspected clean current HEAD
  `5e6f2107ca90d6b9c49e62f8f9fdc02754e898f5`, the `e115f916` contract-only
  diff, both fresh architecture smoke summaries, the clean/stable `74f55223` full
  summary and raw non-pass records, and the clean/stable docs-only quick summary. It
  made no file, commit or remote change.
- Final review classification is 0 Blocker, 0 Major and 0 Minor. The original Major is
  closed because the freestanding syscall ABI, pointer/lifetime, aliasing, register,
  errno and call-site responsibilities now satisfy root `AGENTS.md`; the change is 67
  added comment lines and 0 deletions in `semantic_smoke.rs`, with no instruction,
  syscall, ABI, errno, control-flow, marker or parser change. The original plan-status
  Minor is also closed.
- Review finding clearance is deliberately separate from promotion readiness. The
  reviewer confirmed that fresh full remains 57 PASS + 2 INFRA_ERROR and retains RV
  119 / LA 161 failure records, both duplicate-plan errors, RV cyclictest-musl and LA
  cyclictest-glibc 900 s timeouts. No result was hidden, relabeled or replaced by an
  older greener run.
- A final `git fetch --prune origin` exited 0. `origin/main` and local `main` both remain
  the initial `921171ac1ef5c85ab5a7cd1882dd40e1471b79f0`; the remote integration
  branch did not previously exist. A normal non-force push created
  `origin/integration/four-prs-20260715` at clean `5e6f2107...`; remote `main` was not
  moved. The docs-only blocked closeout commit will advance only that integration ref
  after its own canonical quick passes.
- Terminal disposition is therefore `BLOCKED`, not ready/merged/promoted. The review
  gate is closed, but the official/full test gate is false and forbids any `main` push.

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（精确 serving build 未向 agent 暴露），Goal mode | preflight、Git 历史集成、冲突分析、窄修复、测试/证据审计、治理文档、独立只读 review | 四个 merge 的 reconciliation、PR2 lock fix、canonical test registrations、PR3 adapter 修复、安全契约补录、本计划与日志 | 拒绝全局 ours/theirs、fake PASS、弱化 parser、动态 inventory 和错误版本 QEMU 的 required overclaim；所有选择以 diff、编译、mutation、runtime marker、runner summary 与独立 review 复核 | Codex primary agent 实施；独立 Codex reviewer 首轮 0 Blocker/1 Major/1 Minor，follow-up 0/0/0 并关闭两项；人类负责人尚未完成 |

交互摘要保存在本任务的 Codex Goal 会话与外部 journal；未提交完整对话、凭据或无关隐私数据。AI 未替代最终人工理解：负责人仍需独立解释 ABI 边界、event registry 锁不变量、suite provenance、semantic-smoke unsafe 合同和 official parser 合同。独立 reviewer 是 AI subagent，不冒充人类复核。

# 6. 外部参考与许可证

| 来源 | 版本/commit | 借鉴范围 | 许可证 | OrayS 修改 | 记录/文件 |
|---|---|---|---|---|---|
| OrayS PR1 source history | `70b57e38b42bf09407e405b2fc30ee413dca2404` | Linux ABI/service boundary | 仓库既有许可证 | 通过 explicit merge 保留，增加与 PR2/suite 的 inventory reconciliation | 本日志 source inventory；merge `aa9072df...` |
| OrayS PR2 source history | `7b16e14709469f4d67ed268eb8433159801e9124` | file object/readiness/event core | 仓库既有许可证 | 修复 feature-unified registry lock，保留公共 mutex API | merge `acc6b604...` |
| OrayS unified-suite source history | `0c2a3cffca2fa7d276ea8d0ec3524df8fc0669ba` | canonical runner/parser/checks | 仓库既有许可证 | 合并 PR1/PR2 checks，保持 fixed inventory | merge `126e21a4...` |
| OrayS PR3 source history | `7562ea69770501769fcf5c163a0e95343ffd2e2b` | semantic evidence、QEMU smoke、CI | 仓库既有许可证 | port 到 canonical suite；补齐丢失 fixture 和隔离 import | merge `03269960...` |
| QEMU | 9.2.4 source tarball, SHA-256 `f3cc1c4eabfdb288218ac3e33763dbe9e276d8bc890b867a2335d58de2ddd39a` | 仅作为本地/CI runtime 验证工具，不复制其源码进仓库 | QEMU upstream licenses | 按固定 configure target 构建，不修改 source | `test/evidence/setup_qemu.sh`; `docs/pr3-semantic-evidence.md` |

本次集成没有从新的外部项目复制生产实现，也没有新增第三方 dependency/version/checksum。继承的 ArceOS/Linux/POSIX 参考仍受仓库现有 SPDX、版权和文档记录约束。

# 7. 最终验证

镜像信息：

| 架构 | 文件名 | SHA-256 | 来源/版本 |
|---|---|---|---|
| RISC-V64 | `sdcard-rv.img` | `4336475432728e485bc52f54f0b8ef06910e84d7c425fbba49361a4065cccb99` | 用户提供的 canonical official backing image，只读 |
| LoongArch64 | `sdcard-la.img` | `1aa79d03cf41e2a80ae4ed43771101c1e67ec8db41c3c20b77792fe6b1b85b50` | 用户提供的 canonical official backing image，只读 |

首轮五项 canonical gate 的候选 evidence HEAD 为完整
`7eaf3c1c1e164115de4c41cbf0f2f569d621e875`；reviewer contract 修复后的
affected smoke/full evidence HEAD 分别为 `e115f916...` / `74f55223...`。所有 run 均使用
`PYTHONDONTWRITEBYTECODE=1`、`PYTHONPYCACHEPREFIX=/dev/null`、
`PYTHONNOUSERSITE=1`、`LIBCLANG_PATH=/usr/lib/llvm-21/lib`，以及 external
QEMU 9.2.4 / clang-21 shim 的固定 PATH；official/full 另显式设置
`RV_TESTSUITE_IMG=/root/sdcard-rv.img` 与
`LA_TESTSUITE_IMG=/root/sdcard-la.img`。summary 中保留的精确参数为：

```text
/usr/bin/python3 -I -S -B -X pycache_prefix=/dev/null /root/oskernel2026-orays/test/run_suite.py --profile quick --output-dir test/output/integration-7eaf3c1c-quick-1
/usr/bin/python3 -I -S -B -X pycache_prefix=/dev/null /root/oskernel2026-orays/test/run_suite.py --profile baseline --output-dir test/output/integration-7eaf3c1c-baseline-1
/usr/bin/python3 -I -S -B -X pycache_prefix=/dev/null /root/oskernel2026-orays/test/run_suite.py --profile official --arch rv --output-dir test/output/integration-7eaf3c1c-official-rv-1
/usr/bin/python3 -I -S -B -X pycache_prefix=/dev/null /root/oskernel2026-orays/test/run_suite.py --profile official --arch la --output-dir test/output/integration-7eaf3c1c-official-la-1
/usr/bin/python3 -I -S -B -X pycache_prefix=/dev/null /root/oskernel2026-orays/test/run_suite.py --profile full --arch all --output-dir test/output/integration-7eaf3c1c-full-all-1
```

测试结果（保留首次失败；定向或旧提交证据不替代最终 clean-HEAD gate）：

| Run ID | 命令 | 架构/目标 | 退出码 | 结果 | 耗时 | 原始证据 |
|---|---|---|---:|---|---:|---|
| `integration-suite-126e21a4-quick-1` | canonical quick | common / suite merge commit | 1 | FAIL | 259.665 s | `test/output/integration-suite-126e21a4-quick-1/summary.json` |
| `integration-764211c5-quick-1` | canonical quick | common / governance commit | 1 | FAIL | 515.911347 s | 42 PASS, 2 FAIL, 1 TIMEOUT; `test/output/integration-764211c5-quick-1/summary.json` |
| `integration-05b12326-quick-1` | canonical quick | common / clean `05b12326` | 0 | PASS | 279.598694 s | 45/45 PASS; stable provenance; `test/output/integration-05b12326-quick-1/summary.json` |
| `integration-05b12326-baseline-1` | canonical baseline | host + RV64 + LA64 / clean `05b12326` | 2 | INFRA_ERROR | 890.07075 s | 50 PASS, 6 FAIL, 1 INFRA_ERROR; `test/output/integration-05b12326-baseline-1/summary.json` |
| `integration-1c0e3ba0-quick-1` | canonical quick | common / clean `1c0e3ba0` | 0 | PASS | 280.783039 s | 45/45 PASS; stable provenance; `test/output/integration-1c0e3ba0-quick-1/summary.json` |
| `integration-1c0e3ba0-baseline-1` | canonical baseline | host + RV64 + LA64 / clean `1c0e3ba0` | 1 | FAIL | 716.422999 s | 56 PASS, 1 FAIL; workspace cargo contract; stable provenance; `test/output/integration-1c0e3ba0-baseline-1/summary.json` |
| `qemu-setup-764211c5` | supervised source build + independent `--verify-only` | RISC-V64 + LoongArch64 toolchain | 0 | PASS | 分项 | exact 9.2.4 versions, fixed source/stamp and binary SHA-256 values |
| pre-commit focused PR3 suites | 33+75+27+9+41+26+36+133+23+8+7 methods/checks | host | 0 | PASS | 分项 | 外部 journal checkpoint 2026-07-15T11:58:16Z |
| pre-commit raw LA smoke | supervised exact QEMU 9.2.4 | LA64 | 0 | PASS | 分项 | 外部 ignored build evidence |
| pre-commit raw RV smoke | supervised QEMU 6.2.0 | RV64 | 0 | BLOCKED | 分项 | markers complete，但版本不满足 required contract |
| dirty-baseline-repair-focused | units + fmt + host/RV64/LA64 clippy + semantic evidence/aggregate | host + RV64 + LA64 | 0 | PASS | 分项 | 定向复验；不是 canonical verdict；`build/pr3-evidence/required/semantic-evidence-v1.json` |
| dirty-cargo-contract-focused | parser positive/mutation fixtures + exact runner regression + axns doctest | host | 0 | PASS | 193.694 s + 分项 | 133/133 runner；axns 2 passed/0 ignored；不是 canonical verdict |
| `integration-0c6c2f58-quick-1` | canonical quick | common / clean `0c6c2f58` | 0 | PASS | 289.904906 s | 45/45；summary SHA-256 `69b00793...6012` |
| `integration-0c6c2f58-baseline-1` | canonical baseline | host + RV64 + LA64 / clean `0c6c2f58` | 0 | PASS | 771.339618 s | 57/57；summary SHA-256 `544d35e3...33f` |
| `integration-0c6c2f58-official-rv-1` | canonical official RV | RISC-V64 | 1 | FAIL | 48.819969 s | exact QEMU lacked `user` netdev；guest 未启动；summary SHA-256 `2bcb26cc...097` |
| `integration-0c6c2f58-official-rv-2` | canonical official RV | RISC-V64 | 2 | INFRA_ERROR | 3962.030039 s | 24/24 groups + normal shutdown；ANSI strict parser blocker；summary SHA-256 `156958cd...c1` |
| `integration-7eaf3c1c-quick-1` | canonical quick | common / clean `7eaf3c1c` | 0 | PASS | 295.325320 s | 45/45；summary SHA-256 `5bac5095...d846` |
| `integration-7eaf3c1c-baseline-1` | canonical baseline | host + RV64 + LA64 / clean `7eaf3c1c` | 0 | PASS | 826.131206 s | 57/57；summary SHA-256 `a9b11b36...812c` |
| `integration-7eaf3c1c-official-rv-1` | canonical official | RISC-V64 / clean `7eaf3c1c` | 2 | INFRA_ERROR | 4956.178130 s | 24/24 + shutdown；2 structural + 112 semantic failures；summary SHA-256 `5d95f5b8...a5c2` |
| `integration-7eaf3c1c-official-la-1` | canonical official | LoongArch64 / clean `7eaf3c1c` | 2 | INFRA_ERROR | 4880.545466 s | 24/24 + shutdown；2 structural + 158 semantic failures；summary SHA-256 `551f1e3d...0d9b` |
| `integration-7eaf3c1c-full-all-1` | canonical full | RV64 + LA64 / clean `7eaf3c1c` | 2 | INFRA_ERROR | 10194.568365 s | 59/59：57 PASS + 2 INFRA_ERROR；summary SHA-256 `1c8906d0...1202a` |
| `integration-a00243d9-docs-quick-1` | canonical quick | docs-only clean `a00243d9` | 0 | PASS | 307.655291 s | 45/45；stable provenance；summary SHA-256 `321cef62...eaee` |
| `integration-e115f916-review-smoke-rv-1` | evidence-runtime | RISC-V64 / clean `e115f916` | 0 | PASS | 100.517584 s | 1/1；stable provenance；summary SHA-256 `6247f2e9...c30e` |
| `integration-e115f916-review-smoke-la-1` | evidence-runtime | LoongArch64 / clean `e115f916` | 0 | PASS | 94.628248 s | 1/1；stable provenance；summary SHA-256 `3b3f48c...64f7` |
| `integration-74f55223-review-full-all-1` | canonical full | RV64 + LA64 / clean `74f55223` | 2 | INFRA_ERROR | 11142.025951 s | 59/59：57 PASS + 2 INFRA_ERROR；summary SHA-256 `540f7d1e...0ceb` |
| `integration-5e6f2107-review-docs-quick-1` | canonical quick | docs-only clean `5e6f2107` | 0 | PASS | 295.118691 s | 45/45；stable provenance；summary SHA-256 `7e9ccf4f...38ad` |

`INFRA_ERROR` 是 canonical runner 的原始 fail-closed 状态；不得重标为 PASS。
首次失败与非 canonical 探测均保留，后续成功不会改写旧 verdict。

# 8. 最终审查

- [x] `git diff --check` 通过。
- [x] 未发现测例特化、假成功或吞退出码。
- [x] 未发现凭据、无关机器绝对路径或被跟踪的大体积生成物。
- [x] Linux/ABI/errno/并发/资源回收已由定向检查和独立 review 覆盖。
- [ ] RISC-V64 与 LoongArch64 完整门禁通过。
- [x] AI 和当前外部来源披露已记录，AI reviewer 未冒充人类。
- [x] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 人类负责人确认能够不依赖 AI 解释和调试本集成。

审查人及结论：独立 Codex read-only reviewer subagent；首轮 0 Blocker、1 Major、
1 Minor。Major 为 semantic-smoke unsafe 契约缺失，Minor 为 active plan 状态陈旧。
窄修复、fresh 双架构 smoke、fresh full 和 docs-only quick 完成后，同一 reviewer
follow-up 给出 0 Blocker、0 Major、0 Minor，明确关闭原两项。该结论只清零 review
finding；它不把 official/full 的 INFRA_ERROR 或真实 failure 变成通过。

# 9. 已知限制、后续工作与回滚

## 已知限制

- 最终 quick 45/45 与 baseline 57/57 已关闭早期 RR、cargo-contract 和 evidence
  基线失败；旧 run 的失败仍保留，不能被后来 PASS 改写。
- 两份 official image 中的 tracked BusyBox plan 都有 55 行但仅 54 个唯一身份：
  `echo "bbbbbbb" >> test.txt` 重复两次。每个 libc group 都因此产生一条
  `busybox-duplicate-case`，这是需要外部计划修正并有意识重新 snapshot 的阻断；
  本集成不得弱化 duplicate 检测或直接修改 trusted backing image。
- 即使外部计划修正，生产语义仍非通过：full RV 保留 117 条 failure record，
  full LA 保留 156 条，涵盖 LTP TFAIL/TBROK/nonzero/timeout/panic-trap、
  libctest-glibc 失败；RV cyclictest-musl 还明确 900 s timeout/exit 137。
- 独立 RV run 的 LTP glibc 为 987/13，full RV 为 985/15；该差异需要在后续
  生产修复和重复运行中诊断，不能挑选更绿色的一次作为完成证据。
- 独立只读 AI reviewer 首轮 finding 已由同一 reviewer follow-up 全部关闭；这不
  消除 official/full promotion blocker。人类负责人也尚未确认可独立解释和调试。
- Fresh full 的 LA cyclictest-glibc 新增 900 s timeout，而首轮同组 PASS；必须保留
  新鲜的较差结果并诊断波动，不能选择旧 PASS 作为当前证据。

## 后续工作

首轮 reviewer 的 unsafe 契约与 active-plan 窄修复、fresh RV64/LA64 smoke、
full/all 和 follow-up disposition 均已完成，review finding 为 0/0/0。后续仍须
修正外部 BusyBox plan 并通过受控 provenance 重新生成两架构 image，
再按 failure record 修复真实内核/libc 语义和 RV cyclictest 超时。只有所有项目明确
PASS 且远端仍新鲜时才能另行推广 main。

## 回滚方式

初始基线 annotated tag：`backup/pre-four-prs-20260715`。完整 bundle：任务归档中的 `orays-pre-integration-921171ac.bundle`，SHA-256 `104e4cba9c782af6717910f7ea35e26f9f2a2bdcdf369157a9183d6f5f3b76d5`。在推广前 `main` 保持原值；回滚使用新的安全分支或 revert，不使用 destructive reset/clean/force push。

# 10. 最终摘要

当前状态为 `blocked`。四个来源以独立 no-ff merge 保留，workflow governance、
CI coverage、cargo evidence contract 和 generic official framing 均已提交并通过
定向复验。clean `7eaf3c1c...` quick 为 45/45 PASS，baseline 为 57/57 PASS；
但 RV/LA official 均为 `INFRA_ERROR`，full/all 为 59/59 completed、57 PASS +
2 INFRA_ERROR。两架构都有不可变 trusted BusyBox plan 重复项和大量真实语义
失败；fresh full 的 RV cyclictest-musl 与 LA cyclictest-glibc 各有 900 s timeout。
首轮独立 review 的 1 Major、1 Minor 已由同一 reviewer follow-up 关闭，最终 review
finding 为 0/0/0；但 fresh full 仍为 57 PASS + 2 INFRA_ERROR。最终 fetch 证明
`origin/main` 未漂移，可恢复的 integration 分支已普通推送；`main` 未移动且不得
推送。本日志不宣称 ready/merged/PROMOTED，只给出精确的 `BLOCKED` 终态。
