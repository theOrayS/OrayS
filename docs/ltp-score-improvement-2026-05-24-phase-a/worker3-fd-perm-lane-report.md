# Worker 3 fd/pipe/open/access/fcntl/fsuid/permission lane report

Date: 2026-05-24  
Task: `task-22`  
Scope: find or repair near-clean fd/pipe/open/access/fcntl/fsuid/permission candidates, prevent credential/fsid/open-permission regressions, and report evidence without editing leader-owned `LTP_STABLE_CASES` or `.omx/ultragoal`.

## Result

No new stable250 -> stable300 promotion candidate is recommended from this lane yet.

The lane has strong regression-guard evidence for the existing stable250 fd/permission set, but the currently available evidence does not prove any additional fd/pipe/open/access/fcntl/fsuid/permission case is clean across RV + LA and musl + glibc. Cases with historical failures are kept blocked below instead of being converted to SKIP/TCONF/PASS.

## Evidence used

| Evidence | What it proves | Notes |
| --- | --- | --- |
| `docs/ltp-score-improvement-2026-05-24-phase-a/stable250-live.cases` | Current live baseline has 250 unique cases. | Used only as baseline filter; no edits made. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-final-rv-summary.txt` | RV stable250 final gate: 500 PASS wrappers, 0 FAIL wrappers, 4 TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap. | Known `read02` TCONF remains transparent; lane cases listed below are clean. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/stable250-final-la-summary.txt` | LA stable250 final gate: 500 PASS wrappers, 0 FAIL wrappers, 4 TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap. | Same transparency rule. |
| `docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker3-current-output-promotion-candidates.txt` | `scripts/ltp_summary.py --promotion-candidates output_rv.md output_la.md` found 62 clean cases and `read02` blocked by TCONF. | These outputs are older/baseline-shaped; every lane clean case from this file is already in `stable250-live.cases`, so this is guardrail evidence, not a new promotion list. |
| `docs/ltp-score-improvement-2026-05-22-phase-d/raw/worker3-wave1-rv-summary-recheck.txt` | Historical near-miss/failure classification for fd/open/pipe/fcntl candidates. | Used to preserve exact blocker signals. |
| `docs/ltp-score-improvement-2026-05-24-phase-a/raw/worker3-fd-perm-rv-targeted.log` + `...-summary.txt` | Fresh RV targeted attempt for 12 lane blockers started but was terminated during first cargo build before QEMU/LTP markers. | Parser summary correctly reports 0 PASS/FAIL/internal markers; this is not promotion evidence. |

## Existing stable250 lane guardrails

These families are already in stable250 and should remain regression guards in aggregate gates:

- Access/permission: `access01`, `access03`, `faccessat01`, `faccessat02`, `faccessat201`, `faccessat202`.
- Open/path permission: `open01`, `open02`, `open03`, `open04`, `openat01`.
- FD lifecycle and duplication: `close01`, `close02`, `dup01`, `dup02`, `dup03`, `dup04`, `dup06`, `dup07`, `dup201`, `dup202`, `dup203`, `dup204`, `dup205`, `dup206`, `dup3_01`, `dup3_02`.
- Fcntl: `fcntl01`, `fcntl02`, `fcntl03`, `fcntl04`, `fcntl08`, `fcntl09`, `fcntl10`, `fcntl16`, `fcntl29`, plus `_64` variants currently in stable250.
- Pipe: `pipe01`, `pipe03`, `pipe04`, `pipe05`, `pipe06`, `pipe09`, `pipe10`, `pipe11`, `pipe14`.
- Credential/fsid/permission-sensitive set: `setfsgid01`, `setfsgid02`, `setfsuid01`, `setfsuid02`, `setfsuid03`, `setfsuid04`, plus stable `set*uid`, `set*gid`, `setgroups`, and related credential cases.

The stable250 final RV/LA summaries show these stable cases as clean in both libc variants, while keeping the unrelated known `read02` TCONF visible at suite level.

## Blocked candidates to keep out of promotion

| Case(s) | Latest usable blocker evidence | Classification | Next step |
| --- | --- | --- | --- |
| `read02` | `worker3-current-output-promotion-candidates.txt`: `la:glibc`, `la:musl`, `rv:glibc`, `rv:musl` each have `TCONF=2`. | `pass_with_tconf`, not clean. | Keep transparent; do not treat wrapper PASS as promotion-clean. |
| `pipe02` | `worker3-wave1-rv-summary-recheck.txt`: RV glibc PASS but RV musl FAIL code 3 with `TFAIL=1`, `TBROK=1`. | Split-libc pipe semantics blocker. | Repair musl/pipe child-signal semantics, then rerun RV+LA targeted. |
| `pipe07` | `worker3-wave1-rv-summary-recheck.txt`: RV glibc and musl FAIL code 2 with `TBROK=1`. | Broken setup/pipe semantics. | Repair before any LA confirmation. |
| `pipe08` | `worker3-wave1-rv-summary-recheck.txt`: RV glibc PASS but RV musl FAIL code 1 with `TFAIL=1`; older matrix also records TFAIL. | Split-libc pipe semantics blocker. | Fresh repair + full matrix required. |
| `pipe11` | Historical RV recheck shows code 137 timeout; final stable250 later proves it was fixed and is already stable. | Already promoted guardrail, not new candidate. | Keep as regression guard; timeout must remain promotion-blocking if it reappears. |
| `pipe12`, `pipe13`, `pipe15`, `pipe2_01`, `pipe2_02`, `pipe2_04` | `candidate-matrix.md` records timeout/TFAIL/TBROK blockers. | Not near-clean. | Needs implementation repair before targeted confirmation. |
| `open05` | `worker3-wave1-rv-summary-recheck.txt`: RV glibc/musl wrapper FAIL code -1. | Missing/runner failure; no clean evidence. | Re-run only after setup/root-cause check. |
| `open06`, `openat02` | RV glibc/musl FAIL with `TBROK` and ENOSYS/semantic signal in historical summary. | Open/openat semantics blocker. | Repair open permission/error semantics; then RV+LA targeted. |
| `open07`-`open14`, `openat03`, `openat04` | `candidate-matrix.md` records TBROK/TFAIL/ENOSYS blockers. | Not promotion-safe. | Needs code repair first. |
| `access02`, `access04` | Older phase docs classify `access02` TFAIL and `access04` TBROK/tmpfs setup risk; no current clean RV+LA proof. | Permission/setup blocker. | Re-run after controlled access fixture repair. |
| `close08`, `close09` | RV recheck has wrapper FAIL code -1 for both libcs. | No clean execution proof. | Re-run after binary/setup cause is understood. |
| `lseek02` | RV recheck has glibc/musl FAIL code 2 with `TBROK=1` and ENOSYS signal. | Lseek/error semantics blocker. | Repair and re-run. |
| `lseek03` | RV recheck has glibc/musl wrapper FAIL code -1. | No clean execution proof. | Re-run after setup/root-cause check. |
| `fcntl05`, `fcntl07`, `fcntl11`-`fcntl23`, `fcntl27`, `fcntl30`, `fcntl31`, `fcntl34`, `fcntl36`-`fcntl39` | `worker3-wave1-rv-summary-recheck.txt` and `candidate-matrix.md` record TFAIL/TBROK/ENOSYS patterns. | Not near-clean. | Needs fcntl feature/errno semantics repair before targeting. |
| `fcntl24`, `fcntl26`, `fcntl32`, `fcntl33`, `fcntl35` | `candidate-matrix.md` records TCONF/code 32 or code 36. | TCONF/incomplete, not clean. | Keep visible; do not hide as SKIP/PASS. |
| `seteuid01`, `setuid03`, `setgroups03`, `setgroups04` | `candidate-matrix.md` records code -1 or TFAIL. | Credential semantics blocker. | Repair credential authorization semantics before promotion. |

## Fresh targeted attempt

Command recorded in `raw/worker3-fd-perm-rv-targeted.status`:

```bash
OSCOMP_TEST_GROUPS=ltp \
LTP_CASES=access02,access04,open05,open06,openat02,pipe02,pipe07,pipe08,close08,close09,lseek02,lseek03 \
LTP_CASE_TIMEOUT_SECS=30 \
RV_TESTSUITE_IMG=/root/oskernel2026-orays/sdcard-rv.img \
./run-eval.sh rv
```

The attempt was terminated during the first cargo build before any LTP case emitted `RUN/PASS/FAIL LTP CASE` markers. After the leader guardrail about shared `/tmp/arceos-sdcard-*.run.qcow2` runner names, this attempted run is marked aborted/untrusted for promotion unless the leader reruns it serially. `scripts/ltp_summary.py` therefore reports zero wrappers and zero internal signals. This is explicitly not counted as PASS, not counted as a timeout result, and not used as promotion evidence.

## Subagent integration

Subagent `019e578e-3d8a-7491-9b79-b368df4f643a` reported many fd/permission cases as clean from baseline-style promotion-candidate evidence. I integrated only the locally verified part: those cases are already present in `stable250-live.cases`, so they are regression guards, not new stable250 -> stable300 candidates. I did not use unverified cited files that are absent from this worktree.

## Recommendation

1. Do not promote any new worker-3 lane case yet.
2. Use existing stable250 lane cases as guardrails, especially `open04`, `pipe11`, `setfsuid01`-`setfsuid04`, `setfsgid01`-`setfsgid02`, and faccessat/fcntl families, because they protect the credential/fsid/open-permission fixes from regressing.
3. If this lane must contribute to stable300, the smallest next repair slices are:
   - `pipe02` / `pipe08`: split-libc pipe semantics (`TFAIL`/`TBROK` in RV musl historical evidence).
   - `open06` / `openat02`: open/openat error and permission semantics (`TBROK`/ENOSYS historical evidence).
   - `access02` / `access04`: access fixture and permission result semantics.
4. After any repair, require fresh RV + LA, musl + glibc targeted evidence parsed by `scripts/ltp_summary.py`; wrapper PASS alone is insufficient.
