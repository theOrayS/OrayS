# stable756 validation

## Live count check

```bash
python3 - <<'PY'
from pathlib import Path
import re
text = Path('examples/shell/src/cmd.rs').read_text()
start = text.index('const LTP_STABLE_CASES')
end = text.index('];', start)
cases = re.findall(r'"([^"]+)"', text[start:end])
print(len(cases), len(set(cases)), len(cases) - len(set(cases)))
PY
# 756 756 0
```

## Team/runtime reconciliation

```bash
omx team status complete-dev-1000ltp-c632b4a0 || true
# No team state found for complete-dev-1000ltp-c632b4a0
# mailbox/leader-fixed.json missing
```

The worker idle injections were stale/unactionable. The leader continued in solo mode and kept the promotion gate owner role.

## Final full stable756 gates

Commands:

```bash
LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 4h ./run-eval.sh rv
python3 scripts/ltp_summary.py target/ltp-1000-milestone-05-stable756/rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800.log > target/ltp-1000-milestone-05-stable756/rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800-summary.txt

LTP_CASES=stable LTP_CASE_TIMEOUT_SECS=90 timeout 4h ./run-eval.sh la
python3 scripts/ltp_summary.py target/ltp-1000-milestone-05-stable756/la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800.log > target/ltp-1000-milestone-05-stable756/la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800-summary.txt
```

| Arch | Raw log | Parser summary | Wrapper result | Parser result |
| --- | --- | --- | --- | --- |
| RV | `target/ltp-1000-milestone-05-stable756/rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800.log` | `target/ltp-1000-milestone-05-stable756/rv-stable756-final-after-pipe-poll-atomic-20260603T143606+0800-summary.txt` | `ltp-musl: 756 passed, 0 failed`; `ltp-glibc: 756 passed, 0 failed` | `PASS LTP CASE: 1512`, `FAIL LTP CASE: 0`, internal `{'TCONF': 4}`, timeout/ENOSYS/panic/trap `0`; only `read02` is `pass_with_tconf`. |
| LA | `target/ltp-1000-milestone-05-stable756/la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800.log` | `target/ltp-1000-milestone-05-stable756/la-stable756-final-after-pipe-poll-atomic-nontty-20260603T154154+0800-summary.txt` | `ltp-musl: 756 passed, 0 failed`; `ltp-glibc: 756 passed, 0 failed` | `PASS LTP CASE: 1512`, `FAIL LTP CASE: 0`, internal `{'TCONF': 4}`, timeout/ENOSYS/panic/trap `0`; only `read02` is `pass_with_tconf`. |

Conclusion: final stable756 gate is RV + LA × musl + glibc wrapper PASS. Parser-visible new failure categories are absent. The inherited `read02` O_DIRECT/tmpfs TCONF caveat remains disclosed.

## Targeted lane evidence logs

All paths are under `target/ltp-1000-milestone-05-stable756/`; generated summaries use `scripts/ltp_summary.py`.

| Lane | RV evidence | LA evidence | Promoted / protected cases |
| --- | --- | --- | --- |
| eventfd/epoll base + advanced | `combined-eventfd-epoll-pipe-kconfig-pwait-clean25-promotion.md`; underlying RV logs include `rv-eventfd-epoll-pipe-clean-regression-after-timeout0-fastpath-20260603T010345+0800.log`, `rv-epoll-pwait01-05-after-pollwait-20260603T012904+0800.log` | combined report includes LA logs `la-eventfd-epoll-pipe-clean-regression-after-timeout0-fastpath-20260603T010705+0800.log`, `la-epoll-pwait01-05-after-pollwait-20260603T013129+0800.log` | 25 new eventfd/epoll cases; adjacent `pipe01`, `pipe06`, `pipe2_01`, `poll01`. |
| epoll zero-timeout regression | Failed repro: `rv-epoll_wait04-repro-20260603T141104+0800.log`; final repair: `rv-epoll_wait04-after-pipe-poll-atomic-20260603T142956+0800.log` | covered by final full LA stable756 gate | Protects promoted `epoll_wait04`; final targeted RV summary is `PASS LTP CASE: 2`, `FAIL LTP CASE: 0`, internal `{}`. |
| timerfd/signalfd | `rv-timerfd-signalfd-readlink-after-fd-impl-20260603T020142+0800.log`; `rv-signalfd01-timerfd-settime02-after-signalfd-return-20260603T020914+0800.log` | `la-timerfd-signalfd-readlink-clean10-after-fd-impl-20260603T022002+0800.log` | `timerfd_create01`, `timerfd_gettime01`, `timerfd_settime01`, `timerfd01`, `timerfd02`, `signalfd01`, `signalfd4_01`, `signalfd4_02`. |
| hard link / linkat | `rv-linkat-after-hardlink-overlay-20260603T023839+0800.log`; `rv-link08-after-cross-mount-order-20260603T024537+0800.log` | `la-link-clean5-after-hardlink-rename-20260603T030113+0800.log` | `link02`, `link04`, `link05`, `link08`, `linkat01`. |
| rename / renameat2 | `rv-rename-clean5-after-sparse-move-fix-20260603T032354+0800.log` | `la-rename-clean5-after-sparse-move-fix-20260603T033022+0800.log` | `rename09`, `rename12`, `rename13`, `renameat201`, `renameat202`. |
| pipe capacity / fcntl | `rv-fcntl35-pipe-regression-after-pipe-heap-capacity-20260603T035927+0800.log` | `la-fcntl35-pipe-regression-after-pipe-heap-capacity-20260603T040720+0800.log` | `fcntl35`, `fcntl35_64`; adjacent pipe regression cases. |
| open / creat / waitid | `rv-open-creat-waitid-clean5-scout-20260603T042820+0800.log` | `la-open-creat-waitid-clean5-20260603T043658+0800.log` | `open11`, `creat09`, `waitid07`, `waitid08`, `waitid10`. |
| `/dev/zero` mmap regression | `rv-mmap10-open11-after-devzero-mmap-20260603T050637+0800.log` | `la-mmap10-open11-after-devzero-mmap-20260603T051228+0800.log` | Protects `mmap10` and promoted `open11`. |
| SGID open/creat regression | `rv-open-creat-sgid-regression-after-root-exception-20260603T055320+0800.log` | `la-open-creat-sgid-regression-after-root-exception-nontty-20260603T055940+0800.log` | Protects `open09`, `open11`, `creat09`. |
| clock/shm full-stable repair | `rv-clock-shmt-after-sigtimedwait-clockres-20260603T071254+0800.log` | `la-clock-shmt-after-sigtimedwait-clockres-20260603T071453+0800.log` | Protects `clock_gettime04` and `shmt06`; both arch summaries `PASS LTP CASE: 4`, `FAIL LTP CASE: 0`. |
| LA `pipe2_02` repair | `rv-pipe2_02-after-read-short-fix-clean-20260603T112736+0800.log` | `la-pipe2_02-after-read-short-fix-clean-20260603T112736+0800.log` | Protects existing stable `pipe2_02`; both arch summaries `PASS LTP CASE: 2`, `FAIL LTP CASE: 0`. |

## Known non-promotion / excluded evidence

- `readlink03`, `readlinkat02`: LA musl had parser-visible `TFAIL`; not promoted.
- `timerfd_settime02`: RV glibc had `TBROK`; not promoted.
- `linkat02`: emits setup `TCONF`; not promoted.
- `mmap05`, select/getdents/statx/O_TMPFILE/ftruncate exploratory rows: not a four-way parser-clean promotion set.
- Dirty historical full-stable/diagnostic runs before the final repairs are blocker or diagnosis evidence only; they are not counted as promotion evidence.
- Blacklist/SKIP/status0/full-sweep partial TPASS rows are not counted.

## Checksum manifest

Key raw logs and parser outputs are checksummed in:

- `docs/ltp-1000-long-term-plan-2026-06-01/milestone-05-stable756/validation-checksums.sha256`

## Repository checks

```bash
df -h / /root                       # /dev/vda2 59G, 25G used, 32G available
cargo fmt                           # run before final docs
cargo check -p arceos-shell         # ok
git diff --check                    # ok
python3 stable-count-check          # 756 756 0
```

After documentation edits, `cargo fmt --check`, `cargo check -p arceos-shell`, and `git diff --check` were rerun before commit.
