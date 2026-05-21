# First targeted batch recommendation (2026-05-24)

## Recommendation

Run this first batch before any source edit or stable-list promotion:

```text
getpgid01,getsid01,getrusage02,gettimeofday02,gettid02,getgroups01,getresuid01,getresuid02,getresuid03,getresgid01,getresgid02,getresgid03,sched_getparam01,sched_getscheduler01,sched_getscheduler02,waitpid01
```

Why: these 16 cases are not in the current 75-case stable source list, exist in the common RV/LA inventory, and are concentrated in process/credential/scheduler/read-only metadata surfaces with existing dispatch support. They are the safest path from 75 toward 90+ if the fresh matrix is clean.

## PASS criteria

- RV and LA each show 32 PASS LTP CASE for this batch (16 cases x musl/glibc).
- No FAIL LTP CASE.
- Internal TFAIL/TBROK/TCONF is zero.
- Timeout, ENOSYS/not implemented, and panic/trap matches are zero.

## Stop/route rules

- If any case fails, do not promote the batch wholesale; split the clean subset with `scripts/ltp_summary.py --promotion-candidates` and route the failed signature to the ABI implementation lane.
- If all cases pass cleanly, promote those 16 names into `LTP_STABLE_CASES`, then run stable targeted RV+LA before final evaluator gates.
