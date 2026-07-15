---
title: "PR draft: integrate four OrayS branches"
date_started: 2026-07-15
date_completed: null
status: draft
pr: null
branch: "integration/four-prs-20260715"
authors:
  - "Codex primary agent (AI-assisted integration operator)"
reviewers: []
base_commit: "921171ac1ef5c85ab5a7cd1882dd40e1471b79f0"
head_commit: "03269960bb440e45f6e97999c20532cb3977c9be"
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
- [ ] workflow governance 独立提交且文档完整。
- [ ] 最终候选 HEAD 的 quick、baseline、RV official、LA official、full 均明确 PASS。
- [ ] 独立只读 reviewer 的 blocker/major finding 清零。
- [ ] 最终远端 freshness、安全推广和推送完成。

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

+## 2026-07-15T12:18:35Z - governance reconciliation and focused validation completed

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

# 5. AI 使用披露

| 工具/模型 | 使用场景 | 影响范围 | 人工修改与取舍 | 验证方法 | 负责人 |
|---|---|---|---|---|---|
| OpenAI Codex（精确 serving build 未向 agent 暴露），Goal mode | preflight、Git 历史集成、冲突分析、窄修复、测试/证据审计、治理文档 | 四个 merge 的 reconciliation、PR2 lock fix、canonical test registrations、PR3 adapter 修复、本计划与日志 | 拒绝全局 ours/theirs、fake PASS、弱化 parser、动态 inventory 和错误版本 QEMU 的 required overclaim；所有选择以 diff、编译、mutation、runtime marker 和 runner summary 复核 | 当前由 Codex primary agent 执行；最终人类负责人和独立 reviewer 尚未完成 |

交互摘要保存在本任务的 Codex Goal 会话与外部 journal；未提交完整对话、凭据或无关隐私数据。AI 未替代最终人工理解：负责人仍需独立解释 ABI 边界、event registry 锁不变量、suite provenance 和 official parser 合同。最终独立 reviewer 将另行记录，当前不得虚构。

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

当前测试结果（最终候选门禁尚未开始；下表不会把定向或旧提交证据写成最终 PASS）：

| Run ID | 命令 | 架构/目标 | 退出码 | 结果 | 耗时 | 原始证据 |
|---|---|---|---:|---|---:|---|
| `integration-suite-126e21a4-quick-1` | canonical quick | common / suite merge commit | 1 | FAIL | 259.665 s | `test/output/integration-suite-126e21a4-quick-1/summary.json` |
| pre-commit focused PR3 suites | 33+75+27+9+41+26+36+133+23+8+7 methods/checks | host | 0 | PASS | 分项 | 外部 journal checkpoint 2026-07-15T11:58:16Z |
| pre-commit raw LA smoke | supervised exact QEMU 9.2.4 | LA64 | 0 | PASS | 分项 | 外部 ignored build evidence |
| pre-commit raw RV smoke | supervised QEMU 6.2.0 | RV64 | 0 | BLOCKED | 分项 | markers complete，但版本不满足 required contract |
| pending | canonical quick on final candidate | common |  | BLOCKED |  | 治理提交后执行 |
| pending | canonical baseline | RV64 + LA64 |  | BLOCKED |  | exact QEMU 9.2.4 准备后执行 |
| pending | canonical official RV | RISC-V64 |  | BLOCKED |  | 尚未执行 |
| pending | canonical official LA | LoongArch64 |  | BLOCKED |  | 尚未执行 |
| pending | canonical full all | RV64 + LA64 |  | BLOCKED |  | 尚未执行 |

结果状态只使用 `PASS`、`FAIL`、`ERROR`、`TIMEOUT`、`BLOCKED`、`SKIPPED`。上表将在每次 clean-HEAD canonical run 后追加；首次失败不会被删除。

# 8. 最终审查

- [ ] `git diff --check` 通过。
- [ ] 无测例特化、假成功或吞退出码。
- [ ] 无凭据、无机器相关无关绝对路径、无大体积生成物。
- [ ] Linux/ABI/errno/并发/资源回收已检查。
- [ ] RISC-V64 与 LoongArch64 完整门禁通过。
- [x] AI 和当前外部来源披露已记录；最终验证/审查后仍需更新。
- [ ] 独立 reviewer 的 blocker/major finding 已清零。
- [ ] 人类负责人确认能够不依赖 AI 解释和调试本集成。

审查人及结论：尚未进行，不得写为通过。

# 9. 已知限制、后续工作与回滚

## 已知限制

- suite merge commit 上 clean quick 的 RR skipped-task aging 检查真实失败。
- 历史 axfs、rustfmt、clippy 基线失败需要在最终 HEAD 重新诊断并诚实处理。
- 当前官方 image plan 的 BusyBox duplicate identity 可能使 official gate 成为外部输入 blocker。
- RV required semantic evidence 尚缺 exact QEMU 9.2.4 的 canonical post-commit run。
- 尚无独立 reviewer 或人类可解释性确认。

## 后续工作

完成治理提交；构建/校验双 target QEMU；按 quick、baseline、RV official、LA official、full 顺序运行并检查 summary；只修复真实且在任务范围内的缺陷；独立审查；必要时从头重跑；最后重新 fetch 并决定推广或 BLOCKED/FAILED。

## 回滚方式

初始基线 annotated tag：`backup/pre-four-prs-20260715`。完整 bundle：任务归档中的 `orays-pre-integration-921171ac.bundle`，SHA-256 `104e4cba9c782af6717910f7ea35e26f9f2a2bdcdf369157a9183d6f5f3b76d5`。在推广前 `main` 保持原值；回滚使用新的安全分支或 revert，不使用 destructive reset/clean/force push。

# 10. 最终摘要

当前状态为 Draft：四个来源 merge 已完成且 ancestry 明确，starter 长期策略已激活，治理提交正在形成。最终 canonical 门禁、独立审查、远端 freshness 和安全推广仍未完成，因此本日志不宣称 ready 或 merged。
