# stable706 promotion candidates

## Promotion result

- Baseline before this milestone: `656 total / 656 unique / 0 duplicate`.
- Promotion target reached: `706 total / 706 unique / 0 duplicate`.
- Promoted cases: exactly `50` new unique LTP case names.
- Counting rule: only cases present in both final RV and final LA promotion-candidate parser reports, with `musl` and `glibc` clean wrapper PASS and no internal `TFAIL/TBROK/TCONF/ENOSYS/timeout/panic/trap`, were added to `LTP_STABLE_CASES`.
- Excluded evidence: blacklist, SKIP, status0, full-sweep partial TPASS, and parser-visible TCONF/TBROK/TFAIL.

## Promoted case set

```text
clock_adjtime01 clock_adjtime02 clock_getres01 copy_file_range01 copy_file_range03
creat06 fcntl14_64 fcntl15_64 fcntl30_64
fgetxattr01 fgetxattr03 flistxattr01 flistxattr02 flistxattr03
fremovexattr01 fremovexattr02 fsetxattr01 fsync01 futex_wake03
getcpu01 getpeername01 getsockname01 getsockopt01
lchown01 lchown02 lgetxattr01 lgetxattr02 listxattr02 listxattr03
llistxattr01 llistxattr02 llistxattr03 llseek01 lremovexattr01
munmap02 pause03 removexattr01 removexattr02
rename06 rename07 rename08 rename10 set_robust_list01
setsockopt01 setxattr01 sigaltstack01 socketpair01 statfs01_64
syslog11 syslog12
```

## Final evidence closure

Final new50 parser reports:

- RV: `target/ltp-1000-milestone-04-stable706/rv-stable706-new50-final-gate-20260602T221318+0800.log.promotion-rv.md` → `Promotion candidates: 50`, `Blocked/incomplete cases: 0`.
- LA: `target/ltp-1000-milestone-04-stable706/la-stable706-new50-final-gate-20260602T222043+0800.log.promotion-la.md` → `Promotion candidates: 50`, `Blocked/incomplete cases: 0`.

Earlier discovery/intersection accounting before the final new50 rerun:

```text
rv candidates: 64
la candidates: 58
rv ∩ la: 58
already stable in intersection: 8
new strict candidates: 50
```

The eight already-stable intersection cases were not counted again and were checked as an adjacent regression subset: `creat08`, `getitimer01`, `getxattr01`, `listxattr01`, `open10`, `ppoll01`, `socket01`, `times03`.
