# Task 2 worker-1 progress: Discovery+Stats lane

## Scope accepted

Worker-1 accepted the leader mailbox assignment to own **Discovery+Stats** only:
produce candidate matrix and first targeted batch recommendation under
`docs/ltp-score-improvement-2026-05-24/`, with no `.omx/ultragoal` mutation.

## Completed artifacts

- `docs/ltp-score-improvement-2026-05-24/discovery-candidate-matrix.md`
- `docs/ltp-score-improvement-2026-05-24/discovery-candidate-summary.json`
- `docs/ltp-score-improvement-2026-05-24/discovery-candidate-summary.txt`
- `docs/ltp-score-improvement-2026-05-24/first-targeted-batch-recommendation.md`
- `docs/ltp-score-improvement-2026-05-24/raw/candidate-matrix.json`
- `docs/ltp-score-improvement-2026-05-24/raw/first-targeted-batch-cases.txt`

Commit containing the artifacts: `4e9ef544 Document safe next LTP promotion candidates`.

## Discovery+Stats result

- Current source stable list: 75 cases per libc/arch.
- Candidate matrix: 46 stable-external cases grouped by process/credentials/scheduler,
  time/signal, and fs/fd/syscall-neighbor risk.
- First recommended batch: 16 process/credentials/scheduler cases:

```text
getpgid01,getsid01,getrusage02,gettimeofday02,gettid02,getgroups01,getresuid01,getresuid02,getresuid03,getresgid01,getresgid02,getresgid03,sched_getparam01,sched_getscheduler01,sched_getscheduler02,waitpid01
```

Expected clean targeted evidence if the batch is promotable:

- RV: 32 PASS LTP CASE (16 cases x musl/glibc), 0 FAIL.
- LA: 32 PASS LTP CASE (16 cases x musl/glibc), 0 FAIL.
- Combined: 64 PASS markers, 0 TFAIL/TBROK/TCONF, 0 timeout, 0 ENOSYS, 0 panic/trap.

## Verification performed

```sh
python3 -m json.tool docs/ltp-score-improvement-2026-05-24/discovery-candidate-summary.json
python3 -m json.tool docs/ltp-score-improvement-2026-05-24/raw/candidate-matrix.json
python3 - <<'PY'
# asserted stable_source_count=75, selected_batch_count=16,
# expected pass markers, and all selected cases outside stable source list
PY
git diff --check -- docs/ltp-score-improvement-2026-05-24
```

Result: PASS.

Known verification gap: `cargo fmt --all -- --check` is blocked in this OMX worktree by an
existing workspace/vendor mismatch at `vendor/rust-fatfs/Cargo.toml`; no Rust source was modified.

## Lane boundaries / open work

- No `.omx/ultragoal` files were edited.
- Worker-1 did not claim other workers' lanes.
- Task 5 (`Proc wait session ABI lane`) remains owner `worker-2` / pending.
- Task 6 (`Time signal ABI lane`) remains owner `worker-3` / in progress.
- Task 7 (`FS metadata verification lane`) remains owner `worker-4` / pending.
