# milestone-04 stable706 report

## Goal

Advance `dev/1000ltp-plan` from stable656 to stable706 by promoting exactly 50 trustworthy unique LTP stable cases, without fake pass, blacklist counting, or evaluator/testsuite bypass.

## Result

- `examples/shell/src/cmd.rs::LTP_STABLE_CASES` now reports `706 total / 706 unique / 0 duplicate`.
- Promotion set: 50 new unique cases, listed in `targeted-cases.txt` and `promotion-candidates.md`.
- Final promoted-new50 gate is RV + LA × musl + glibc parser-clean: 100/100 PASS on RV and 100/100 PASS on LA, with no internal `TFAIL/TBROK/TCONF`, timeout, ENOSYS, panic, or trap.
- Team runtime state/mailbox for `complete-dev-1000ltp-c632b4a0` was unavailable (`No team state found`), so leader continued in solo mode and kept the leader-owned promotion gate.

## Main changes

- Added/expanded generic Linux-visible syscall behavior for `getcpu`, `syslog`, `clock_getres` alarm clock IDs, `copy_file_range`, and minimal `readahead` fd validation.
- Tightened xattr/path/robust-list/socket errno boundaries inherited from this milestone's earlier worker/leader lane work.
- Made effective clock resolution reporting match the current coarse timer model and updated regular-file write/copy timestamp overlay behavior.
- Added `/proc/sys/kernel/printk` synthetic procfs content.
- Updated the stable list with exactly the 50 parser-clean RV∩LA candidates.

## Evidence summary

Key final logs under `target/ltp-1000-milestone-04-stable706/`:

- RV promoted-new50: `rv-stable706-new50-final-gate-20260602T221318+0800.log` → `PASS LTP CASE: 100`, `FAIL LTP CASE: 0`, internal `{}`, timeout/ENOSYS/panic/trap `0`, promotion candidates `50`, blocked `0`.
- LA promoted-new50: `la-stable706-new50-final-gate-20260602T222043+0800.log` → `PASS LTP CASE: 100`, `FAIL LTP CASE: 0`, internal `{}`, timeout/ENOSYS/panic/trap `0`, promotion candidates `50`, blocked `0`.
- Adjacent already-stable regression subset: `rv-stable706-adjacent-regression8-20260602T222823+0800.log` and `la-stable706-adjacent-regression8-20260602T223016+0800.log` → each `PASS LTP CASE: 16`, `FAIL LTP CASE: 0`, internal `{}`.
- Full RV stable sweep caveat: `rv-stable706-final-gate-20260602T205711+0800.log` remained dirty (`PASS 1393`, `FAIL 19`, internal `{'TCONF': 4, 'TBROK': 4, 'TFAIL': 36}`, timeout `2`) and is disclosed as blocker evidence only, not promotion evidence.

Checksums for key logs/summaries are recorded in `validation-checksums.sha256`.

## Risks and caveats

- Full stable706 RV sweep is not clean. Targeted repairs cleared `clock_gettime04` and `copy_file_range03`, but full-stable blockers such as `mmap10`, `mmap-corruption01`, and `test_ioctl` remain for the next milestone before any final stable1000/full-suite claim.
- `readahead01` remains non-promoted because parser-visible TCONF persists from unsupported auxiliary fd families.
- `copy_file_range` is implemented as real chunked file copy, not kernel offload/reflink.
- Broad socket sockaddr/user-len behavior was touched earlier in this milestone; future socket promotion should rerun socket errno/readiness subsets before further expansion.

## Conclusion

Stable706 promotion is accepted for the 50 new cases because every promoted case has clean RV and LA evidence across musl and glibc, plus an adjacent already-stable regression subset. It is not a full-suite cleanliness claim; the full-stable RV caveat is a required follow-up blocker.

## Next step

Milestone-05 stable756 should first address or isolate the full-stable RV blockers, then prioritize candidates with clean small-scope semantics, likely process/time/capability or socket/proc/syntheticfs lanes, while avoiding large namespace/eventfd/fanotify/io_uring families unless a real subsystem plan exists.
