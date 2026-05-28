# stable250 delivery report

Date: 2026-05-23 (Asia/Shanghai)
Branch: `refactor/moss_kernel_like`
Commit target: local stable250 delivery before continuing remote scorer-zero repair.

## Result

`LTP_STABLE_CASES` is now exactly 250 unique cases. Full local evaluator gates passed on both supported evaluator architectures with the canonical parser:

| Gate | Command | Parser result |
| --- | --- | --- |
| RISC-V stable250 | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=60 ./run-eval.sh rv` then `python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable250-final-rv.log` | `PASS LTP CASE: 500`, `FAIL LTP CASE: 0`, `ltp-musl: 250 passed, 0 failed`, `ltp-glibc: 250 passed, 0 failed`, `timeout: 0`, `ENOSYS: 0`, `panic/trap: 0`; only existing `read02` is `pass_with_tconf` in both libc groups. |
| LoongArch stable250 | `OSCOMP_TEST_GROUPS=ltp LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 ./run-eval.sh la` then `python3 -B scripts/ltp_summary.py docs/ltp-score-improvement-2026-05-22-phase-d/raw/stable250-final-la.log` | `PASS LTP CASE: 500`, `FAIL LTP CASE: 0`, `ltp-musl: 250 passed, 0 failed`, `ltp-glibc: 250 passed, 0 failed`, `timeout: 0`, `ENOSYS: 0`, `panic/trap: 0`; only existing `read02` is `pass_with_tconf` in both libc groups. |

Small parser summaries are kept in:

- `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-final-rv-summary.txt`
- `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-final-la-summary.txt`

Raw QEMU logs are intentionally left ignored under `docs/ltp-score-improvement-2026-05-22-phase-d/raw/` because they are generated evidence.

## Promoted cases

The stable list increased from stable220 to stable250 by adding 30 cases:

```text
setegid02
setfsgid02
setfsuid02
setfsuid03
setfsuid04
setgid02
setregid02
setregid03
setregid04
setresgid02
setresgid03
setresuid02
setresuid03
setreuid02
setreuid03
setreuid04
setuid04
rt_sigsuspend01
signal02
signal03
signal04
sighold02
sched_getaffinity01
sched_setparam01
sched_setparam02
sched_setparam03
mlock03
mlock04
sched_setscheduler03
set_tid_address01
```

All promoted cases have strict PASS on both RISC-V and LoongArch, for both glibc and musl, in the final stable250 full gates. No timeout, panic/trap, ENOSYS, TFAIL, or TBROK evidence was promoted as PASS.

## Implementation changes

- Credential transitions now enforce Linux-like permission rules for unprivileged `set*uid` and `set*gid` calls instead of silently accepting disallowed IDs.
- Saved-ID updates for `setreuid` / `setregid` were adjusted to support the LTP set-ID matrix.
- File-system IDs are tracked separately as `fsuid` / `fsgid`, copied on process clone, shown as the fourth Uid/Gid field in `/proc/<pid>/status`, and used for file permission checks.
- `setfsuid` / `setfsgid` now return the previous fs ID and only change fs IDs when Linux-style authorization permits it.
- `open` and `chdir` now check recorded path permissions with fsuid/fsgid. `faccessat` keeps real-ID behavior unless `AT_EACCESS` requests effective-ID checks. `stat_path` uses `O_PATH` so metadata lookup does not incorrectly require read permission.
- New file ownership records use fsuid/fsgid.

These are real semantics changes required by the promoted LTP cases; no case-name hardcoding or fake PASS path was added.

## Explicitly excluded evidence

- `sched_getscheduler02` remains excluded even though it passed RISC-V and LoongArch glibc in targeted scouting. LoongArch musl still fails because the loader's patched scheduler stub returns raw `-ESRCH` instead of passing through musl `__syscall_ret`; that issue is a separate loader-stub fix and was not promoted.
- Earlier broad scouts included failures and one panic while probing unrelated candidates; those cases were not promoted.

## Build and hygiene validation

Passed after the final credential-boundary correction:

```text
cargo fmt --all -- --check
git diff --check
make all
CARGO_HOME=$PWD/cargo-home CARGO_NET_OFFLINE=true PATH=$PWD/tools/bin:$PATH make all
```

`make all` and the offline `make all` both generated root-level remote-submission outputs `kernel-rv` and `kernel-la`.

Disk monitor / cleanup:

```text
Before cleanup observed during the run: / was 89% used and /root/.codex was about 33G, dominated by /root/.codex/log/codex-tui.log.
Cleanup performed: compacted /root/.codex/log/codex-tui.log in place, preserving the last 32MiB.
After final build checks: / is 32% used with 39G available; /root/.codex is 486M.
```

## User-visible / ABI-visible behavior

Intended visible behavior changes:

- Unprivileged set-ID syscalls can now return `EPERM` where they previously succeeded incorrectly.
- `setfsuid` / `setfsgid` now affect fsuid/fsgid rather than effective uid/gid.
- File permission checks for `open`, `chdir`, and `faccessat(AT_EACCESS)` now depend on the appropriate real/effective/fs IDs.
- `/proc/<pid>/status` Uid/Gid lines now expose fsuid/fsgid as the fourth field.
- The stable LTP autorun group now contains 250 cases.

No struct layout change was made in the guest ABI beyond the existing text output of `/proc/<pid>/status`.

## Follow-up not included in this commit

Remote evaluator LTP scoring still needs a separate fix. The next task is to inspect `LoongArch输出.txt` and `Riscv输出.txt`, compare against the read-only `refactor/moss_kernel_like_remote` behavior, and repair the scorer-visible output path without hiding real LTP failures.
