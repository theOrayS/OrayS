# LTP Discovery Candidates (2026-05-21)

Worker: `worker-3`  
Task: `7` / Discovery candidate batches

## Method

Inventory was collected read-only from the repository sdcard images with `debugfs`; no image was mounted or modified.

Commands used:

```sh
debugfs -R 'ls -p /{musl,glibc}/ltp/testcases/bin' /root/oskernel2026-orays/sdcard-{rv,la}.img
debugfs -R 'ls -p /{musl,glibc}/ltp/runtest' /root/oskernel2026-orays/sdcard-{rv,la}.img
debugfs -R 'rdump /{musl,glibc}/ltp/runtest /tmp/worker3-ltp-discovery/<arch>-<libc>-runtest' /root/oskernel2026-orays/sdcard-{rv,la}.img
```

The proposed batches below are discovery candidates only. They are not PASS claims and should be run through the hardened runner/summary path before promotion.

## Inventory counts

| Image/libc | `testcases/bin` regular executables | `runtest` files | `runtest` entries | unique case labels | unique command binaries |
| --- | ---: | ---: | ---: | ---: | ---: |
| `sdcard-rv.img` / `musl` | 2820 | 67 | 4056 | 3933 | 2108 |
| `sdcard-rv.img` / `glibc` | 2840 | 67 | 4056 | 3933 | 2108 |
| `sdcard-la.img` / `musl` | 2820 | 67 | 4056 | 3933 | 2108 |
| `sdcard-la.img` / `glibc` | 2840 | 67 | 4056 | 3933 | 2108 |

Notes:

- RV and LA inventories match for the same libc.
- The 20 glibc-only binaries are cpuset/controller helpers (`cpuset_*`), not first-batch candidates.
- The common executable intersection across all four image/libc combinations is 2820 binaries.
- Current core runner cases are: `access01`, `brk01`, `chdir01`, `clone01`, `close01`, `dup01`, `fcntl02`, `fork01`, `getpid01`, `mmap01`, `open01`, `pipe01`, `read01`, `stat01`, `wait401`, `write01`.

## Proposed first discovery batches

### `syscalls-basic-plus` (20 cases)

Low-risk adjacent syscall coverage near the current core set; all commands exist in the four image/libc inventories and have direct `runtest/syscalls` entries.

```text
access02 access03 access04
close02
dup02 dup03
fcntl01 fcntl03
getcwd01 getpid02 getppid01 getuid01 geteuid01 getgid01 getegid01
lseek01 lseek02
pipe02
read02
write02
```

Why first:

- Extends already-green core families (`access`, `close`, `dup`, `fcntl`, `getpid`, `pipe`, `read`, `write`).
- Mostly checks errno/metadata/basic fd semantics rather than stress behavior.
- Good at exposing narrow ABI gaps without requiring broad `/proc`, networking, cgroups, block devices, or ptrace.

### `fs-basic` (17 cases)

Filesystem metadata and namespace operations that are closer to current core than the heavy `runtest/fs` stress entries.

```text
creat01 creat03
open02 open03
stat02 lstat01
chmod01 fchmod01
mkdir02 rmdir01
link02 symlink01 readlink01 unlink05 rename01
ftruncate01
umask01
```

Why first:

- Targets metadata/open/rename/link behavior requested in the plan.
- Avoids `growfiles`, `rwtest`, quota, squashfs, and large data-integrity stress cases for the first pass.
- Likely failure signatures should map to `fd_table.rs`, `metadata.rs`/synthetic fs paths, or syscall dispatch errno handling.

### `proc-basic` (2 runtest labels)

Read-only `/proc` discovery. These should be treated as probe cases, not immediate promotion candidates, because they may expose broad synthetic `/proc` coverage gaps.

```text
proc01       -> proc01 -m 128
read_all_proc -> read_all -d /proc -q -r 3
```

Runner note:

- `proc01` has a binary, but the `runtest/fs` command supplies `-m 128`.
- `read_all_proc` is a case label using the common `read_all` binary with `/proc` arguments.
- If the initial selectable runner only accepts bare executable names, keep this batch in the discovery queue until command-line runtest entries are supported, or run it manually as label+argv probes.

### `time-signal-basic` (19 cases)

Basic clock/time and signal behavior with no timer stress or real-time scheduling dependency in the first batch.

```text
alarm02 alarm03
clock_getres01 clock_gettime01 clock_gettime02
gettimeofday01
nanosleep01 nanosleep02
time01 times01
kill02 kill03 pause01
rt_sigaction01 rt_sigprocmask01
sigaction01 sigprocmask01 sigpending02 sigsuspend01
```

Why first:

- Focuses on common time/signal ABI behavior before `timerfd_*`, POSIX timer, `tgkill/tkill`, and queued-signal cases.
- Expected failures should be separable into time ABI, signal ABI, and process lifecycle buckets.

## Explicitly deferred categories

Defer these until the harness has per-case timeout/failure accounting and the first batches are summarized:

- `crashme` / `crash01` and related user-fault tests: known LA hard blocker; should be handled in the separate hard-blocker lane.
- `cve`: older RV full-LTP attempt hit memory exhaustion near CVE cases; keep out of score-expansion batches initially.
- `controllers`, `cpuset`, `power_management*`, `numa`, `hugetlb`, `kvm`, `scsi_debug*`, `tpm_tools`: platform/kernel-feature heavy and not good first score wins.
- `net*`: useful later, but not first LTP score expansion while syscall/fs/process basics are still being widened.
- `runtest/fs` stress cases such as `growfiles`, `rwtest`, `fs_fill`, quota/remount, squashfs: high runtime/noise risk compared with syscall-level fs metadata cases.

## Suggested next runner order

1. Run `syscalls-basic-plus` first on RV musl+glibc and LA musl+glibc.
2. Run `fs-basic` only after syscall-basic failures are triaged, because fs failures may share root causes with open/stat/link syscall semantics.
3. Run `time-signal-basic` after process cleanup between cases is confirmed by the harness.
4. Keep `proc-basic` as a manual/label+argv discovery probe unless the batch runner supports full `runtest` command lines.

Promotion rule: do not promote any case into the stable scoring set until both architectures and both libc variants are measured, with internal `TFAIL/TBROK/TCONF`, timeout, panic/trap, and ENOSYS counted separately from wrapper status.
