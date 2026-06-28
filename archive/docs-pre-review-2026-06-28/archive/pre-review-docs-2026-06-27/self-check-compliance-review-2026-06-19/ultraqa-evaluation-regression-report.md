# UltraQA evaluation regression report - 2026-06-19

## Goal and success criteria
- Goal: 对 self-check 合规修复后的当前树执行 UltraQA，确认没有明显评测能力退化，并且不以伪成功、隐藏失败或误导输出作为通过依据。
- Stop condition: 基线 guard/单测/构建 smoke、对抗 harness、runner dry-run、RV targeted runtime smoke 均有新鲜证据；发现的问题已修复或列入残余风险；UltraQA 状态和临时产物完成清理。
- Safety bounds applied: 不运行无界 full LTP；所有长命令使用 `timeout`；不修改 testsuite/evaluator；不清理无关脏工作树；不把 TCONF/TBROK/TFAIL/ENOSYS/timeout/panic/trap 隐藏成 PASS。

## Scenario matrix
| ID | User/attacker model | Scenario | Command/harness | Expected signal | Actual result | Status | Evidence | Cleanup |
|----|---------------------|----------|-----------------|-----------------|---------------|--------|----------|---------|
| UQA-001 | Normal maintainer | Stable LTP list and self-check guard baseline | stable-list extractor; `scripts/check_g002..g013`; unittest | 1000 unique stable cases; guards/tests pass | 1000/1000 unique; guards PASS; `test_g0*.py` initially exposed G010 fixture drift, then PASS after fix | PASS after fix | `stable_count 1000`, final guards PASS; `Ran 144 tests ... OK` | no temp repo changes |
| UQA-002 | Normal evaluator | Remote runner and non-LTP/LTP ordering dry-run | `make -n all`; stable/blacklist variants; runner scan | Default stable; RV/LA build paths visible; no fake PASS marker | dry-run contains stable default, blacklist mode, RV and LA build paths | PASS | `make_n_variants.summary` | no cleanup needed |
| UQA-003 | Malformed/misleading log attacker | Success text plus FAIL/TFAIL/TBROK/TCONF/ENOSYS | temp LTP log + `scripts/ltp_summary.py --json` | failure markers preserved; no false all-pass | rerun parsed `fail_count=1`, `enosys=1`, `TFAIL/TBROK/TCONF=1` | PASS | `adversarial_ltp_summary_rerun.log/json` | temp fixture removed |
| UQA-004 | Prompt injection attacker | Prompt text tries to skip self-check and print PASS | temp injection fixture + runtime static scan | Injection ignored; forbidden runtime patterns absent | forbidden pattern count 0 in runtime/evaluator source scope | PASS | `prompt_injection_static_scan.txt` | temp fixture removed |
| UQA-005 | Hung/misleading output | Command prints SUCCESS then hangs | `timeout 2s bash -c 'printf SUCCESS; sleep 5'` | exit 124, not accepted as pass | exit 124; `PASS` never printed | PASS | `hung_timeout.log` | no child left |
| UQA-006 | Stale/cancel/resume state | OMX state consistency | `omx state read/list-active/get-status`, final clear planned | During run only UltraQA should be active; final state clear | `list-active` showed only ultraqa, but `omx status/get-status` exposed stale ultragoal/ultrawork/skill-active from prior workflow | WATCH -> cleanup required | `active_state_during_ultraqa.json`; state status commands | final clear below |
| UQA-007 | Dirty worktree owner | Preserve unrelated dirty work | `git status --porcelain` before/after harness | No unrelated edits overwritten | temp harness did not alter worktree; final intentional edits are this report + G010 test fix | PASS | `git-status.before`, `git-status.after-adversarial` | no unrelated cleanup |
| UQA-008 | Flaky/retry guard | Rerun key guards/tests | G005/G013/check + G013 unittest twice | repeated pass | both rounds PASS; G013 8 tests OK both rounds | PASS | `flake_rerun.log` | none |
| UQA-009 | Build/typecheck smoke | Compile changed syscall/user-copy/pthread surfaces | cargo checks + gcc syntax | exit 0 within timeout | `arceos_posix_api`, `axlibc`, `pthread.c` all exit 0; only existing warnings | PASS | `build_smoke.log` | target/build artifacts only |
| UQA-010 | Evaluator build/runtime smoke | Submission-style RV build and targeted LTP runtime | isolated RV `make test_build`; `./run-eval.sh rv` 6 cases | artifact built; summary clean | RV kernel artifact 1.8M; RV musl+glibc 12/12 PASS, 0 fail/timeout/ENOSYS/internal failures | PASS | `rv_test_build_smoke.log`; `rv_targeted_ltp.summary.txt/json` | /tmp artifacts removed after report |

## Commands run
- `[0]` stable-list extractor — `stable_count 1000`, `unique_count 1000`, `duplicates 0`.
- `[0]` final static guards — G002/G003/G004/G005/G006/G007/G008/G009/G010/G012/G013 all `PASS (0 findings)`.
- `[initial 1 -> fixed 0]` `python3 -m unittest scripts/test_g0*.py` — initially `144 tests, 1 failure` in G010 fixture drift; after fix `Ran 144 tests ... OK`.
- `[0]` `python3 scripts/test_ltp_summary.py` — `Ran 12 tests ... OK`.
- `[0]` `cargo check -p arceos_posix_api --offline --features 'fd fs net pipe select epoll multitask uspace'`.
- `[0]` `cargo check -p axlibc --offline --features 'fd fs net pipe select epoll multitask alloc'`.
- `[0]` `gcc -fsyntax-only ulib/axlibc/c/pthread.c`.
- `[0]` `make -n all`, `make -n all REMOTE_LTP_CASES=stable`, `make -n all REMOTE_LTP_CASES=blacklist`.
- `[2]` direct `make A=examples/shell ARCH=riscv64 build` — rejected as stale `.axconfig.toml` architecture cache; replaced with isolated `test_build` below.
- `[0]` isolated `make test_build ARCH=riscv64 BUS=mmio ... OUT_CONFIG=/tmp/...` — built `/tmp/.../kernel-rv` (1.8M).
- `[0]` RV targeted runtime: `OSCOMP_TEST_GROUPS=ltp LTP_CASES=open01,fcntl14,pipe01,stat01,lstat01,waitpid01 LTP_CASE_TIMEOUT_SECS=30 timeout 900s ./run-eval.sh rv`.
- `[0]` `scripts/ltp_summary.py` on RV targeted log — pass_count=12, fail_count=0, timeouts=0, enosys=0, internal={}, fail_cases=[].

## Failures found
- UQA-001: G010 guard self-test drift. The test replaced obsolete `File::new(...).add_to_fd_table()` text, so it did not inject the intended open-flag regression and could false-green.
  - Root cause: source changed to `add_to_fd_table(open_fd_flags(flags))`, test fixture was stale.
  - User impact: guard regression tests could miss future F_GETFL/open flag drops.
  - Safety impact: self-check audit confidence reduced until fixed.
- UQA-010 setup issue: direct `make build` failed with `ARCH or MYPLAT has been changed` because the shared `.axconfig.toml` cache did not match. This is a harness/setup issue, not product failure; isolated OUT_CONFIG `test_build` passed.
- UQA-006 WATCH: OMX status surfaces had stale `ultragoal`/`ultrawork`/`skill-active` even while `list-active` only reported `ultraqa`. This affects workflow reporting, not kernel evaluation behavior; final cleanup is mandatory.

## Fixes applied
- `scripts/test_g010_real_kernel_semantics.py`
  - Added `replace_once()` so fixture mutations assert their target exists.
  - Updated `test_detects_regular_file_open_flags_dropped` to mutate the current `File::new(file, filename, flags).add_to_fd_table(open_fd_flags(flags))` expression.
  - Regression evidence: `scripts/test_g010_real_kernel_semantics.py` ran 36 tests OK; `python3 -m unittest scripts/test_g0*.py` ran 144 tests OK.

## Evaluation capability conclusion
- Current evidence does **not** show LTP evaluation capability regression in the checked path:
  - stable list remains 1000 unique cases;
  - runner/parser guards pass;
  - remote `make all` dry-run still builds RV/LA with `LTP_CASES="stable"` by default;
  - RV targeted LTP smoke for `open01,fcntl14,pipe01,stat01,lstat01,waitpid01` passed in both musl and glibc with 0 TFAIL/TBROK/TCONF/timeout/ENOSYS/panic.
- There is a known compliance-vs-score residual risk for non-LTP: old suite/script-specific adaptations are now forbidden/removed by self-check policy. That is a legitimate compliance correction, but without full non-LTP QEMU/remote evaluation it cannot be proven score-neutral.

## Cleanup and rollback
- Generated adversarial fixtures under `/tmp/ultraqa-*` were removed immediately by the harness.
- Persistent deliverable intentionally kept: this report.
- Intentional code/test change: `scripts/test_g010_real_kernel_semantics.py`.
- Build smoke temporarily regenerated `api/arceos_posix_api/src/ctypes_gen.rs` with host ABI layout; it was restored to HEAD and not kept.
- Pre-existing dirty worktree entries were not reverted or hidden.
- UltraQA/OMX state cleanup completed: `omx state list-active --json` returned `{"active_modes":[]}` and `omx status` showed no active workflow modes.

## Residual risks
- Full stable LTP 1000-case RV/LA and full non-LTP were not rerun in this bounded UltraQA pass; only a targeted RV runtime smoke was executed.
- Local LA runtime was not run; per repo policy, local LA behavior can differ from official remote address mapping.
- `make all` full RV+LA kernel generation was dry-run plus RV isolated smoke, not a full two-arch build in this pass.
- Current worktree still contains pre-existing unrelated dirty/deleted/untracked files; runtime conclusions apply to the tested tree plus the intentional G010 test fix, not to a clean release branch unless re-run there.
- OMX stale entries observed during the run were cleared through `omx state clear`; final status showed no active workflow modes.

## Evidence excerpts

### Final guard excerpt
```text
--- check_g005_runner_parser.py ---
G005 runner/parser static check: PASS (0 findings)
--- check_g006_synthetic_capabilities.py ---
G006 synthetic capability static check: PASS (0 findings)
--- check_g007_socket_time_mempolicy.py ---
G007 socket/time/mempolicy static check: PASS (0 findings)
--- check_g008_musl_patch_stable.py ---
G008 musl runtime patch retirement static check: PASS (0 findings)
--- check_g009_post_review_semantics.py ---
G009 post-review real-semantics static check: PASS (0 findings)
--- check_g010_real_kernel_semantics.py ---
G010 real-kernel-semantics static check: PASS (0 findings)
--- check_g012_syscall_review_hotspots.py ---
G012 syscall review hotspot guard: PASS (0 findings)
--- check_g013_user_copy_boundary.py ---
G013 user-copy boundary static check: PASS (0 findings)
```

### G010/all guard-test fix evidence
```text
=== all test_g0*.py after fix ===
................................................................................................................................................
----------------------------------------------------------------------
Ran 144 tests in 7.274s

OK
```

### RV targeted LTP summary excerpt
```text
| fcntl14 | rv | glibc | ltp-glibc | PASS | 0 | 9101 | 258814 | 258814 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| fcntl14 | rv | musl | ltp-musl | PASS | 0 | 11445 | 259094 | 258902 | -192 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | rv | glibc | ltp-glibc | PASS | 0 | 1163 | 258814 | 258814 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| lstat01 | rv | musl | ltp-musl | PASS | 0 | 788 | 258902 | 258902 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | rv | glibc | ltp-glibc | PASS | 0 | 1440 | 258902 | 258814 | -88 | 0 | 0 | 0 | 0 | 0 | 0 |
| open01 | rv | musl | ltp-musl | PASS | 0 | 1066 | 259694 | 259094 | -600 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | rv | glibc | ltp-glibc | PASS | 0 | 1359 | 258814 | 258814 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| pipe01 | rv | musl | ltp-musl | PASS | 0 | 978 | 258902 | 258902 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | rv | glibc | ltp-glibc | PASS | 0 | 1532 | 258814 | 258814 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| stat01 | rv | musl | ltp-musl | PASS | 0 | 797 | 258902 | 258902 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| waitpid01 | rv | glibc | ltp-glibc | PASS | 0 | 2191 | 258814 | 258814 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| waitpid01 | rv | musl | ltp-musl | PASS | 0 | 1624 | 258902 | 258902 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## Categories
- pass_clean: 12 (rv:glibc:fcntl14, rv:musl:fcntl14, rv:glibc:lstat01, rv:musl:lstat01, rv:glibc:open01, rv:musl:open01, rv:glibc:pipe01, rv:musl:pipe01, rv:glibc:stat01, rv:musl:stat01, rv:glibc:waitpid01, rv:musl:waitpid01)
- pass_with_tconf: 0
- fail_wrapper: 0
- internal_tfail: 0
- internal_tbrok: 0
- timeout: 0
- enosys: 0
- panic_trap: 0
- unknown: 0

## Groups
### ltp-musl
- PASS: 6
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0

### ltp-glibc
- PASS: 6
- FAIL: 0
- Internal: {}
- timeout: 0
- ENOSYS/not implemented: 0
- panic/trap: 0
```

### RV build artifact excerpt
```text
1387 | pub(super) fn sleep_duration(duration: core::time::Duration) {
     |               ^^^^^^^^^^^^^^

warning: `arceos-shell` (bin "arceos-shell") generated 28 warnings (run `cargo fix --bin "arceos-shell"` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 1m 51s
sh /root/oskernel2026-orays/scripts/rust-objcopy.sh --binary-architecture=riscv64 /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/riscv64/shell_riscv64-qemu-virt.elf --strip-all -O binary /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/riscv64/shell_riscv64-qemu-virt.bin
make[1]: Leaving directory '/root/oskernel2026-orays'
sh /root/oskernel2026-orays/scripts/rust-objcopy.sh -I binary -O elf64-littleriscv --rename-section .data=.text,alloc,load,readonly,code /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/riscv64/shell_riscv64-qemu-virt.bin /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/kernel-rv.wrap.o
rust-lld -flavor gnu -m elf64lriscv -T scripts/make/riscv64-kernel-wrap.lds /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/kernel-rv.wrap.o -o /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/kernel-rv
=== kernel-rv artifact ===
-rwxr-xr-x 1 root root 1.8M Jun 19 10:44 /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/kernel-rv
-rwxr-xr-x 1 root root 1.8M Jun 19 10:44 /tmp/ultraqa-eval-regression-20260619/kernel-smoke-rv/riscv64/shell_riscv64-qemu-virt.bin
=== df after rv test_build ===
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        59G   51G  6.0G  90% /
/dev/vda2        59G   51G  6.0G  90% /
```
