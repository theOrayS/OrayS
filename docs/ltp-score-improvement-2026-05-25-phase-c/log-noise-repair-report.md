# Log-noise repair report — fops/root expected errno paths

Date: 2026-05-25
Team: `ltp-stable375-to-stab-eae749f6`
Ultragoal story: `G001-first-fix-the-high-frequency-remote`

## Result

Fixed the high-frequency remote warning source `kernel/fs/axfs/src/fops.rs::_open_dir_at()` by preserving `Err(AxError::NotADirectory)` while avoiding the `ax_err!` macro warning side effect. Also applied the same narrow no-warn pattern to two adjacent expected VFS negative paths in `kernel/fs/axfs/src/root.rs`:

- existing directory on `create_dir` still returns `AxError::AlreadyExists`;
- removing a directory through the file removal path still returns `AxError::IsADirectory`.

No success path was added. No errno was changed. The visible POSIX/Linux behavior is intended to remain ENOTDIR/EEXIST/EISDIR on the same failure paths.

## Code changes

| File | Previous path | New path | Visible behavior |
| --- | --- | --- | --- |
| `kernel/fs/axfs/src/fops.rs` | `return ax_err!(NotADirectory);` | `return Err(AxError::NotADirectory);` | still ENOTDIR/`AxError::NotADirectory`; warning suppressed |
| `kernel/fs/axfs/src/root.rs` | `ax_err!(AlreadyExists)` | `Err(AxError::AlreadyExists)` | still EEXIST/`AxError::AlreadyExists`; warning suppressed |
| `kernel/fs/axfs/src/root.rs` | `ax_err!(IsADirectory)` | `Err(AxError::IsADirectory)` | still EISDIR/`AxError::IsADirectory`; warning suppressed |

Worker 2 independently reviewed the patch and confirmed from `axerrno` macro behavior that `ax_err!` prints a warning while direct `Err(AxError::...)` does not.

## Remote-noise baseline before fix

From `raw/remote-output-noise-baseline.json` built from user-provided remote outputs:

| Remote output | `AxError::NotADirectory` | exact `axfs::fops:297` NotADirectory | `AxError::IsADirectory` | `AxError::AlreadyExists` | marker bad prefix | parser status |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `Riscv输出.txt` | 4510 | 4432 | 380 | 1 | 0 | glibc-only `375/0/0`, TFAIL/TBROK/timeout/ENOSYS/panic 0, known TCONF only |
| `LoongArch输出.txt` | 4507 | 4433 | 380 | 1 | 0 | glibc-only `375/0/0`, TFAIL/TBROK/timeout/ENOSYS/panic 0, known TCONF only |

This confirms the problem is output volume/noise, not hidden glibc LTP failure.

## Local validation after fix

### Build and static checks

- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- `make A=examples/shell ARCH=riscv64`: PASS, produced `kernel-rv` and remote-config `kernel-la` successfully.

### Targeted RV LTP subset

First subset (`access01,read02,ftest07,mem02`, timeout 60) proved `fops.rs` NotADirectory warnings were already gone, but `ftest07` timed out under the short 60s setting and is **not** promotion evidence:

- summary: `raw/log-noise-rv-subset-001-summary.txt`
- noise counts: `raw/log-noise-rv-subset-001-noise-counts.json`
- result: PASS 6 / FAIL 2; both failures are `ftest07` timeout code 137; TFAIL/TBROK/ENOSYS/panic 0; marker bad prefix 0.
- noise: exact `axfs::fops:297 [AxError::NotADirectory]` 0; `AxError::NotADirectory` 0; `AxError::IsADirectory` 16 before the adjacent root.rs no-warn patch.

Second subset (`access01,read02,mem02`, timeout 90) after both fops/root changes:

- summary: `raw/log-noise-rv-subset-002-summary.txt`
- noise counts: `raw/log-noise-rv-subset-002-noise-counts.json`
- PASS LTP CASE: 6; FAIL LTP CASE: 0.
- `ltp-musl`: 3 passed / 0 failed.
- `ltp-glibc`: 3 passed / 0 failed.
- internal TFAIL/TBROK: 0/0.
- internal TCONF: 4, all from known `read02` O_DIRECT-on-tmpfs transparency.
- timeout/ENOSYS/panic/trap: 0/0/0.
- marker bad prefix: 0.
- noise: exact `axfs::fops:297 [AxError::NotADirectory]` 0; `AxError::NotADirectory` 0; exact root `IsADirectory` 0; `AxError::IsADirectory` 0; `AxError::AlreadyExists` 0.

## Team evidence integrated

- Worker 1: wrote `worker1-discovery-candidate-matrix.md`; no newly promotable non-stable cases proven by existing artifacts alone.
- Worker 2: wrote `worker2-log-noise-vfs-report.md`; independently confirmed no-warn errno semantics and Batch A VFS candidate risks.
- Worker 3: wrote `worker3-fd-pipe-iovec-report.md`; FD/pipe/iovec candidates remain blocker/unknown, not clean promotion evidence.
- Worker 4: wrote `worker4-process-wait-signal-report.md`; `kill02` and `waitid07/08/10` remain blocked without aggregate/four-way proof.
- Worker 5: wrote `worker5-mmap-fs-guardrail-report.md`; mmap/fs-suite candidates are blocker/unknown and no new four-way-clean candidate was found.

Team shutdown reached `completed=5 failed=0`. Worker-5 merge reported a harmless untracked-file conflict because the leader had already copied the report into the same path.

## Conclusion

G001 is complete: the high-frequency `axfs::fops:297 [AxError::NotADirectory]` warning path is eliminated locally with no intended POSIX-visible errno change, marker prefixes remain clean, and small RV musl+glibc LTP smoke remains honest with only the known `read02` TCONF disclosed.
