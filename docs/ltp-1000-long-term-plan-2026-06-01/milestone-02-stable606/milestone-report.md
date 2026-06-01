# milestone-02-stable606 preflight report

## Goal

Promote the live stable baseline from 556 to the next milestone target of 606 trusted unique LTP stable cases on `dev/1000ltp-plan`, without counting blacklist/SKIP/status0 evidence and without hiding parser caveats.

## Current result

- Current stable list: 556 total / 556 unique / 0 duplicate.
- Target for this milestone: 606 total / 606 unique / 0 duplicate.
- Milestone status: **not complete / no stable promotion yet**.
- Stable list update in this preflight: none.
- Ultragoal checkpoint for G009: not run, because stable606 gate is not closed.

## Evidence generated in this preflight

Evidence directory: `target/ltp-1000-milestone-02-stable606/`.

1. `rv-m02-scout-001-20260601T154726Z.log`
   - 80-case RV scout across musl + glibc.
   - Parser result: 51 wrapper PASS / 109 wrapper FAIL rows; internal signals `TBROK=73`, `TFAIL=122`, `TCONF=24`; timeout matches 4; ENOSYS/not-implemented matches 6; panic/trap 0.
   - Clean RV two-libc candidates were limited to the 21 deferred clean rows from milestone-01.
2. `rv-socket01-postfix-20260601T160003Z.log` and `la-socket01-postfix-20260601T160247Z.log`
   - After the generic socket errno fix, `socket01` is clean on RV + LA x musl + glibc.
3. `rv-socket-adjacent-postfix-20260601T160853Z.log` and `la-socket-adjacent-postfix-20260601T160953Z.log`
   - Adjacent socket regression subset (`socket01`, existing stable `socket02`, `socketpair02`, `accept01`, `listen01`) is parser-clean on RV + LA x musl + glibc.
4. `rv-nanosleep01-rescout-20260601T160605Z.log` and `la-nanosleep01-rescout-20260601T160721Z.log`
   - Isolated `nanosleep01` rescout is parser-clean on RV + LA x musl + glibc.
   - Caveat: the earlier 80-case RV scout had one musl timing TFAIL for `nanosleep01`; it should be re-run in a later grouped gate before milestone promotion.

## Candidate bank after this preflight

- Deferred four-way clean bank inherited from milestone-01: 21 cases.
- New fixed/scouted candidates with current four-way targeted evidence: `socket01`, `nanosleep01`, `mmap04`, `vma01`, `times03`, `mmap14`, `mmap12`, `open10`, `creat08`, `chmod07`, `fchmod02`, `access04`, `chmod06`, `chown04`, `fchmod06`, `fchown04`, `pipe07`.
- Current candidate bank size for stable606 planning: at most 38 cases, still short of the +50 milestone.

## User-visible behavior / ABI impact

The initial preflight included one kernel-visible errno fix in `examples/shell/src/uspace/fd_socket.rs::sys_socket_bridge`:

- AF_INET `SOCK_RAW` with unsupported protocol now returns `EPROTONOSUPPORT` instead of `ESOCKTNOSUPPORT`.
- Other invalid AF_INET socket types now return `EINVAL` instead of `ESOCKTNOSUPPORT`.
- No LTP case/path/process/output is hardcoded.

No stable-list ABI surface changes are made in this preflight. See `abi-and-behavior-impact.md` for details, the proc-maps impact boundary, and the deliberately rejected `nice04` errno shortcut.

## Risk and caveats

- This is a preflight, not a promotion commit.
- `nanosleep01` has mixed scout history; keep it as a tentative candidate until a later grouped milestone gate proves stability.
- The 80-case RV scout exposed broad VFS/device/metadata blockers (`ENOSPC`, missing device/mount behavior, getdents symlink visibility, signal/itimer timeout) that require real semantic work.
- No blacklist changes were made.

## Next step

Continue G009 with real semantic fixes or low-risk clean candidates until at least 50 new unique cases can pass the full RV + LA x musl + glibc promotion gate. Do not update `LTP_STABLE_CASES` or checkpoint G009 from this preflight alone.

## Additional G009 progress on 2026-06-02

A generic `/proc/self/maps` improvement was added after the socket preflight:

- `UserProcess` now tracks user-created mmap regions, including current protection bits and shared/private display state.
- `/proc/self/maps` now emits parseable dynamic mmap ranges in addition to executable/heap/stack rows.
- `MAP_FIXED`, `munmap`, `exec`, `fork`, and `mprotect` update or preserve this synthetic maps state generically.

Targeted evidence:

- `rv-proc-maps-mmap-vma-postfix2-20260601T162318Z.log`: `mmap04,vma01` RV musl+glibc PASS, parser-clean.
- `la-proc-maps-mmap-vma-postfix-20260601T162441Z.log`: `mmap04,vma01` LA musl+glibc PASS, parser-clean.
- `rv-proc-maps-mmap-regression-20260601T162607Z.log`: RV mmap/proc maps regression subset 22 PASS / 0 FAIL, no internal caveats.
- `la-proc-maps-mmap-regression-20260601T162755Z.log`: LA mmap/proc maps regression subset 22 PASS / 0 FAIL, no internal caveats.
- `proc-maps-mmap-regression-rv-la.promotion-candidates.txt`: combined four-way report; among the eleven clean rows, new not-yet-stable candidates are `mmap04` and `vma01`.

Promotion remained blocked after the proc-maps fix: the stable606 candidate bank was at most 25, still short of +50, and no final stable606 gate had been run.


## times03 CPU accounting follow-up

A generic `times()` accounting improvement was added after the proc-maps work:

- `UserProcess` now records a process start clock tick and waited-child user/system tick totals.
- `times()` now returns `USER_HZ` clock ticks instead of milliseconds and fills `tms_utime`, `tms_stime`, `tms_cutime`, and `tms_cstime` from process lifetime and waited-child accounting.
- `wait4`/`waitid` accumulate an exited waited child's self and descendant CPU counters before teardown.

Targeted evidence:

- `rv-times03-postfix-20260601T164216Z.log`: `times03` RV musl+glibc PASS, parser-clean.
- `la-times03-postfix-20260601T164436Z.log`: `times03` LA musl+glibc PASS, parser-clean.
- `rv-times03-regression-20260601T164708Z.log`: RV time regression subset 10 PASS / 0 FAIL, no internal caveats.
- `la-times03-regression-20260601T164956Z.log`: LA time regression subset 10 PASS / 0 FAIL, no internal caveats.
- `times03-regression-rv-la.promotion-candidates.txt`: combined four-way report; among the five clean rows, the new not-yet-stable candidate is `times03`.

Promotion remained blocked after the time follow-up: the stable606 candidate bank was at most 26, still short of +50, and no final stable606 gate had been run.


## mmap14 MAP_LOCKED / VmLck follow-up

A generic mmap/proc-status improvement was added after the `times03` work:

- `UserMmapRegion` now records whether a mapping was created with `MAP_LOCKED`.
- `MAP_LOCKED` mappings are eagerly populated and tracked as locked mmap bytes until `munmap`/`MAP_FIXED` removes or splits the range.
- `/proc/self/status` now reports `VmLck` from per-process locked mmap metadata.

Targeted evidence:

- `rv-mmap14-postfix-20260601T170355Z.log`: `mmap14` RV musl+glibc PASS, parser-clean.
- `la-mmap14-postfix-20260601T170553Z.log`: `mmap14` LA musl+glibc PASS, parser-clean.
- `rv-mmap14-regression-20260601T170753Z.log`: RV mmap/proc regression subset 24 PASS / 0 FAIL, no internal caveats.
- `la-mmap14-regression-20260601T171057Z.log`: LA mmap/proc regression subset 24 PASS / 0 FAIL, no internal caveats.
- `mmap14-regression-rv-la.promotion-candidates.txt`: combined four-way report; among the twelve clean rows, the new not-yet-stable candidate from this follow-up is `mmap14`.

Promotion remains blocked: the stable606 candidate bank is now at most 27, still short of +50, and no final stable606 gate has been run.

## mmap12 /proc/self/pagemap follow-up

A generic synthetic pagemap improvement was added after the `mmap14` work:

- `/proc/self/pagemap` and `/proc/<pid>/pagemap` are now exposed as read-only synthetic procfs files.
- The fd implementation supports sparse `lseek`/`read` at pagemap-entry offsets and returns one native-endian `u64` per virtual page.
- Bit 63 (`present`) is set for pages visible in the process text approximation, heap, stack, and current tracked mmap regions when the pagemap file is opened. PFN/soft-dirty/swap bits remain intentionally zero.

Targeted evidence:

- `rv-mmap12-postfix-20260601T173127Z.log`: `mmap12` RV musl+glibc PASS, parser-clean.
- `la-mmap12-postfix-20260601T173441Z.log`: `mmap12` LA musl+glibc PASS, parser-clean.
- `mmap12-rv-la-postfix.promotion-candidates.txt`: combined four-way report; `mmap12` is clean across RV + LA x musl + glibc.
- `rv-mmap12-regression-20260601T174051Z.log`: RV mmap/proc/pagemap regression subset 24 PASS / 0 FAIL, no internal caveats.
- `la-mmap12-regression-20260601T174435Z.log`: LA mmap/proc/pagemap regression subset 24 PASS / 0 FAIL, no internal caveats.
- `mmap12-regression-rv-la.promotion-candidates.txt`: combined four-way regression report; all twelve rows are clean, with `mmap12` as the new not-yet-stable candidate from this follow-up.

Promotion remains blocked: the stable606 candidate bank is now at most 28, still short of +50, and no final stable606 gate has been run. Stable list remains 556 total / 556 unique / 0 duplicate.

## open10 / creat08 setgid inheritance follow-up

A generic VFS metadata improvement was added after the `mmap12` work:

- New file/FIFO/directory metadata recording now derives the child gid from the parent directory when the parent has `S_ISGID` set.
- New subdirectories under a setgid parent also inherit the setgid bit in the recorded mode.
- Non-setgid parents keep the previous process `fs_gid()` behavior. No LTP case/path/process/output is hardcoded.

Targeted evidence:

- `rv-open-creat-setgid-postfix-20260601T180048Z.log`: `open10,creat08` RV musl+glibc PASS, parser-clean.
- `la-open-creat-setgid-postfix-20260601T180132Z.log`: `open10,creat08` LA musl+glibc PASS, parser-clean.
- `open-creat-setgid-rv-la-postfix.promotion-candidates.txt`: combined four-way report; `open10` and `creat08` are clean across RV + LA x musl + glibc.
- `rv-open-creat-setgid-regression-20260601T180236Z.log`: RV VFS metadata regression subset 32 PASS / 0 FAIL, no internal caveats.
- `la-open-creat-setgid-regression-20260601T180348Z.log`: LA VFS metadata regression subset 32 PASS / 0 FAIL, no internal caveats.
- `open-creat-setgid-regression-rv-la.promotion-candidates.txt`: combined four-way regression report; all sixteen rows are clean, with `open10` and `creat08` as the new not-yet-stable candidates from this follow-up.

Promotion remains blocked: the stable606 candidate bank is now at most 30, still short of +50, and no final stable606 gate has been run. Stable list remains 556 total / 556 unique / 0 duplicate.


## chmod07 / fchmod02 group database follow-up

A generic synthetic `/etc/group` improvement was added after the setgid-create work:

- The default group database now includes conventional `daemon` and `users` entries in addition to `root` and `nogroup`.
- This resolves generic libc `getgrnam("users")` / fallback `getgrnam("daemon")` setup paths used by chmod/fchmod permission tests.
- No LTP case/path/process/output is hardcoded; the visible behavior change is the content of `/etc/group` for all callers.

Targeted evidence:

- `rv-groupdb-chmod-fchmod-20260601T181203Z.log`: `chmod07,fchmod02` RV musl+glibc PASS, parser-clean.
- `la-groupdb-chmod-fchmod-20260601T181243Z.log`: `chmod07,fchmod02` LA musl+glibc PASS, parser-clean.
- `groupdb-chmod-fchmod-rv-la.promotion-candidates.txt`: combined four-way report; `chmod07` and `fchmod02` are clean across RV + LA x musl + glibc.
- `rv-groupdb-chmod-regression-20260601T181338Z.log`: RV chmod/chown/open/creat regression subset 16 PASS / 0 FAIL, no internal caveats.
- `la-groupdb-chmod-regression-20260601T181429Z.log`: LA chmod/chown/open/creat regression subset 16 PASS / 0 FAIL, no internal caveats.
- `groupdb-chmod-regression-rv-la.promotion-candidates.txt`: combined four-way regression report; all eight rows are clean, with `chmod07` and `fchmod02` as the new not-yet-stable candidates from this follow-up.

Promotion remains blocked: the stable606 candidate bank is now at most 32, still short of +50, and no final stable606 gate has been run. Stable list remains 556 total / 556 unique / 0 duplicate.

## tmpfs read-only remount metadata follow-up

A generic mount/VFS metadata improvement was added after the group database work:

- `mount(..., MS_REMOUNT|MS_RDONLY, "tmpfs")` is now accepted only for an existing mount point and records the mount as read-only in per-process mount metadata.
- Path translation keeps mount source-root plus read-only state, and write-capable metadata paths now return `EROFS` when operating under a read-only mount.
- Parent search permission is checked before chown metadata permission checks so inaccessible path prefixes surface `EACCES` instead of a later ownership `EPERM`.
- No LTP case/path/process/output is hardcoded.

Targeted evidence:

- `rv-tmpfs-readonly-metadata-20260601T182849Z.log`: `access04,chmod06,chown04,fchmod06,fchown04` RV musl+glibc PASS, parser-clean.
- `la-tmpfs-readonly-metadata-20260601T182942Z.log`: same five cases LA musl+glibc PASS, parser-clean.
- `tmpfs-readonly-rv-la.promotion-candidates.txt`: combined four-way report; five candidates are clean across RV + LA x musl + glibc.
- `rv-tmpfs-readonly-regression-20260601T183034Z.log`: RV VFS permission regression subset 30 PASS / 0 FAIL, no internal caveats.
- `la-tmpfs-readonly-regression-20260601T183152Z.log`: LA VFS permission regression subset 30 PASS / 0 FAIL, no internal caveats.
- `tmpfs-readonly-regression-rv-la.promotion-candidates.txt`: combined four-way regression report; all fifteen rows are clean, with `access04`, `chmod06`, `chown04`, `fchmod06`, and `fchown04` as the new not-yet-stable candidates from this follow-up.

Promotion remained blocked after the tmpfs read-only mount follow-up: the stable606 candidate bank was at most 37, still short of +50, and no final stable606 gate had been run. Stable list remained 556 total / 556 unique / 0 duplicate.


## /proc/self/fd directory follow-up

A generic procfs FD-directory improvement was added after the tmpfs read-only mount work:

- `/proc/self/fd`, `/proc/<current-pid>/fd`, and `/dev/fd` can now be opened as read-only synthetic directories.
- `getdents64` on those directories enumerates `.`, `..`, and the process's currently open numeric fd entries dynamically, with numeric entries exposed as symlink-like `DT_LNK` rows.
- Directory fd behavior is generic (`read`/file-like I/O returns directory errors, `fstat`/path stat report a directory mode, `O_PATH` works through the existing synthetic path machinery). No LTP case/path/process/output is hardcoded.

Targeted evidence:

- `rv-proc-fd-pipe07-20260601T184539Z.log`: `pipe07` RV musl+glibc PASS, parser-clean.
- `la-proc-fd-pipe07-20260601T184915Z.log`: `pipe07` LA musl+glibc PASS, parser-clean.
- `proc-fd-pipe07-rv-la.promotion-candidates.txt`: combined four-way report; `pipe07` is clean across RV + LA x musl + glibc.
- `rv-proc-fd-regression-20260601T185013Z.log`: RV pipe/proc-fd/readlink/fcntl regression subset 40 PASS / 0 FAIL, no internal caveats.
- `la-proc-fd-regression-20260601T185013Z.log`: LA pipe/proc-fd/readlink/fcntl regression subset 40 PASS / 0 FAIL, no internal caveats.
- `proc-fd-regression-rv-la.promotion-candidates.txt`: combined four-way regression report; all twenty rows are clean, with `pipe07` as the new not-yet-stable candidate from this follow-up.

Promotion remains blocked: the stable606 candidate bank is now at most 38, still short of +50, and no final stable606 gate has been run. Stable list remains 556 total / 556 unique / 0 duplicate.
