# stable906 validation

## Stable count

`examples/shell/src/cmd.rs::LTP_STABLE_CASES` after promotion:

```text
906 total / 906 unique / 0 duplicate
```

Count command:

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
```

## Static/source validation

Commands run for the SysV msg implementation and stable list update:

```bash
rustfmt --edition 2021 --check examples/shell/src/uspace/sysv_msg.rs examples/shell/src/uspace/mod.rs examples/shell/src/uspace/syscall_dispatch.rs
cargo check --manifest-path examples/shell/Cargo.toml
rustfmt --edition 2021 --check examples/shell/src/cmd.rs examples/shell/src/uspace/sysv_msg.rs examples/shell/src/uspace/mod.rs examples/shell/src/uspace/syscall_dispatch.rs
git diff --check -- examples/shell/src/cmd.rs examples/shell/src/uspace/sysv_msg.rs examples/shell/src/uspace/mod.rs examples/shell/src/uspace/syscall_dispatch.rs
```

## Final new50 gate

RV final new50:

- Cases: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.summary.txt`
- Promotion report: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-08-stable906/rv-stable906-new50-final-gate-20260605T114502+0800.sha256.txt`
- Parser result: PASS LTP CASE 100; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

LA final new50:

- Cases: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.summary.txt`
- Promotion report: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.promotion-candidates.txt`
- Checksums: `target/ltp-1000-milestone-08-stable906/la-stable906-new50-final-gate-20260605T115135+0800.sha256.txt`
- Parser result: PASS LTP CASE 100; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

Combined four-way promotion report:

- `target/ltp-1000-milestone-08-stable906/stable906-new50-rvla-final-gate-20260605T115135+0800.txt`
- Required combos: RV/LA x musl/glibc = 4
- Promotion candidates: 50
- Blocked/incomplete cases: 0

## SysV msg implementation gate

This milestone added real syscall coverage for `msgget`, `msgsnd`, `msgrcv`, and `msgctl`.

RV SysV msg targeted gate:

- Cases: `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.summary.txt`
- Promotion report: `target/ltp-1000-milestone-08-stable906/rv-sysv-msg-fix-20260605T113226+0800.promotion-candidates.txt`
- Parser result: PASS LTP CASE 18; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

LA SysV msg targeted gate:

- Cases: `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.summary.txt`
- Promotion report: `target/ltp-1000-milestone-08-stable906/la-sysv-msg-fix-20260605T113700+0800.promotion-candidates.txt`
- Parser result: PASS LTP CASE 18; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

Combined SysV msg report:

- `target/ltp-1000-milestone-08-stable906/sysv-msg-rvla-fix-20260605T113700+0800.txt`
- Promotion candidates: 9
- Blocked/incomplete cases: 0

## Adjacent stable regression subset

Existing stable SysV shm cases were rerun because the new code enters the same SysV IPC syscall family and table/permission semantics are adjacent.

RV adjacent subset:

- Cases: `target/ltp-1000-milestone-08-stable906/rv-sysv-ipc-adjacent-stable-regression-20260605T120125+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/rv-sysv-ipc-adjacent-stable-regression-20260605T120125+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/rv-sysv-ipc-adjacent-stable-regression-20260605T120125+0800.summary.txt`
- Parser result: PASS LTP CASE 22; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

LA adjacent subset:

- Cases: `target/ltp-1000-milestone-08-stable906/la-sysv-ipc-adjacent-stable-regression-20260605T120551+0800.cases.txt`
- Raw log: `target/ltp-1000-milestone-08-stable906/la-sysv-ipc-adjacent-stable-regression-20260605T120551+0800.log`
- Summary: `target/ltp-1000-milestone-08-stable906/la-sysv-ipc-adjacent-stable-regression-20260605T120551+0800.summary.txt`
- Parser result: PASS LTP CASE 22; FAIL 0; internal TFAIL/TBROK/TCONF 0; timeout 0; ENOSYS 0; panic/trap 0.

Combined adjacent report:

- `target/ltp-1000-milestone-08-stable906/sysv-ipc-adjacent-stable-regression-rvla-20260605T120551+0800.txt`
- Required combos: RV/LA x musl/glibc = 4
- Clean stable SysV shm cases: 11
- Blocked/incomplete cases: 0

## Unverified / caveats

- This is a milestone targeted gate, not a full stable906 all-case sweep.
- Raw logs remain under `target/` and are not committed; paths/checksums are recorded here.
- `fallocate05` intentionally consumes substantial tmpfs/free-frame headroom during the run; final parser is clean, but memory delta is tracked in summaries.
