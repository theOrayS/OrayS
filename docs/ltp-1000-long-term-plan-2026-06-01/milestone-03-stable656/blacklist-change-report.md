# Milestone 03 stable656 blacklist change report

No blacklist changes were made in this checkpoint.

## Severe blockers observed

| Case | Blocker type | Blacklist decision |
| --- | --- | --- |
| `kill10` | RV scout produced panic/trap / early stop before glibc group | not blacklisted here; severe blocker recorded for isolated diagnosis |
| old `futex_wait03` scout row | timeout in both libcs before procfs repair | not blacklisted here; superseded by current clean targeted evidence |
| old `futex_wait05` scout/terminated rows | slept-too-long or incomplete LA regression before precise timer/periodic-deadline repair | not blacklisted here; superseded by current clean targeted and regression evidence |
| old `signal01` scout row | timeout before poll-wait proc-state repair | not blacklisted here; superseded by current clean targeted evidence |
| `shmat1` | mixed scout had long/hung behavior and was manually terminated | not blacklisted here; evidence is scouting-only |
| `mmap05` | LA musl+glibc still report `TFAIL=1` / SIGSEGV signal not received after RV became clean | not blacklisted here; recorded as LoongArch fault-signal repair candidate |
| `munmap01` | previously failed with wrapper code 139; now four-way clean after catchable synchronous `SIGSEGV` repair | not blacklisted here; counted only in the clean candidate pool, not promoted yet |
| `mmap13` | pre-fix `TFAIL` / SIGBUS signal not received | not blacklisted here; repaired by generic file-backed mmap SIGBUS-on-EOF handling and now tracked as a clean candidate |
| `readlinkat02` | LA musl `TFAIL` from musl zero-size wrapper rewriting to a one-byte syscall | not blacklisted here; ordinary libc/test boundary and not promotion evidence |
| `clone04` | RV glibc clean but RV musl `TBROK` / killed by SIGSEGV, with LTP hint toward musl `clone.c` wrapper behavior | not blacklisted here; ordinary libc-wrapper boundary and not promotion evidence |
| `mmap10_1` | missing testcase inventory | excluded from promotion; no blacklist change |
| `vma02` | `TCONF` libnuma requirement | excluded from promotion; no blacklist change |

## Closed arch-sweep mining

Re-mining `rv-arch002.log` and `la-arch012.log` did not change the blacklist. The not-stable four-way-clean filter was empty, and remaining failures/TCONF/TBROK/TFAIL/ENOSYS/timeout rows are blocker evidence only.

## Boundary

These failures are not hidden. They are not counted as PASS, not promoted to stable, and not converted into blacklist credit. If future full-sweep lanes need temporary blacklist isolation, the report must record the severe-blocker reason, source, and removal condition separately.

## `openat03` blocker update

No blacklist entry was added for `openat03`. The rejected `O_TMPFILE`/`linkat` emulation produced RV panic/trap evidence and was removed; the retained generic unsupported gate produces visible `TCONF`/wrapper FAIL on RV/LA x musl/glibc with zero panic/trap. This is an ordinary unresolved feature/VFS robustness blocker, not blacklist credit and not promotion evidence.
